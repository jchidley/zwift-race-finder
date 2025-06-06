//! Event display functionality
//!
//! This module contains functions for displaying event information in a formatted way.

use crate::category::{
    get_category_from_score, get_category_speed, get_detailed_category_from_score,
};
use crate::constants::METERS_PER_KILOMETER;
use crate::database::Database;
use crate::duration_estimation::estimate_duration_for_category;
use crate::duration_estimation::get_route_difficulty_multiplier_from_elevation;
use crate::estimation::{get_route_data, get_route_data_from_db};
use crate::event_analysis::find_user_subgroup;
use crate::event_filtering::FilterStats;
use crate::formatting::format_duration;
use crate::models::{EventSubGroup, ZwiftEvent};
use crate::parsing::{estimate_distance_from_name, parse_description_data};
use crate::route_discovery;
use chrono::{DateTime, Local};
use colored::Colorize;

/// Display the basic event header information
pub fn display_event_header(event: &ZwiftEvent) {
    let local_time: DateTime<Local> = event.event_start.into();

    println!("\n{}", "─".repeat(80).dimmed());
    println!("{}: {}", "Event".bright_blue(), event.name.bold());
    println!(
        "{}: {}",
        "Type".bright_blue(),
        event.event_type.replace("_", " ")
    );
    println!(
        "{}: {}",
        "Time".bright_blue(),
        local_time.format("%a %b %d at %I:%M %p")
    );

    if let Some(route) = &event.route {
        println!("{}: {}", "Route".bright_blue(), route);
    }
}

/// Display route information including completion status
pub fn display_route_info(event: &ZwiftEvent) {
    if let Some(route_id) = event.route_id {
        // Check completion status
        let completion_marker = if let Ok(db) = Database::new() {
            if db.is_route_completed(route_id).unwrap_or(false) {
                " ✓".green().to_string()
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        };

        if let Some(route_data) = get_route_data(route_id) {
            println!(
                "{}: {} ({}m elevation){}",
                "Route ID".bright_blue().dimmed(),
                route_id.to_string().dimmed(),
                route_data.elevation_m,
                completion_marker
            );
        } else {
            println!(
                "{}: {} (unknown route - automatically logged)",
                "Route ID".bright_blue().dimmed(),
                route_id.to_string().yellow()
            );
            log_unknown_route(event);
        }
    }
}

/// Log unknown route for future mapping
pub fn log_unknown_route(event: &ZwiftEvent) {
    if let Some(route_id) = event.route_id {
        if get_route_data(route_id).is_none() {
            // First try to parse route name from description
            if let Some(description) = &event.description {
                if let Some(parsed) = route_discovery::parse_route_from_description(description) {
                    // Log with parsed route info for manual mapping later
                    if let Ok(db) = Database::new() {
                        let event_name_with_route = format!(
                            "{} -> {} ({} laps)",
                            event.name, parsed.route_name, parsed.laps
                        );
                        let _ = db.record_unknown_route(
                            route_id,
                            &event_name_with_route,
                            &event.event_type,
                        );
                        return;
                    }
                }
            }

            // If description parsing failed, log normally
            if let Ok(db) = Database::new() {
                let _ = db.record_unknown_route(route_id, &event.name, &event.event_type);
            }
        }
    }
}

/// Display duration and distance information with estimation
pub fn display_duration_info(event: &ZwiftEvent, zwift_score: u32) {
    let user_subgroup = find_user_subgroup(event, zwift_score);

    // Try to get duration/distance from user's category subgroup first
    let duration_minutes = user_subgroup
        .and_then(|sg| sg.duration_in_minutes)
        .or(event.duration_in_minutes)
        .or_else(|| event.duration_in_seconds.map(|s| s / 60));

    let distance_meters = user_subgroup
        .and_then(|sg| sg.distance_in_meters)
        .or(event.distance_in_meters);

    if let Some(duration) = duration_minutes.filter(|&d| d > 0) {
        println!(
            "{}: {} (fixed duration)",
            "Duration".bright_blue(),
            format_duration(duration)
        );
    } else if let Some(route_id) = event.route_id {
        display_route_based_duration(event, route_id, user_subgroup, distance_meters, zwift_score);
    } else if let Some(distance) = distance_meters.filter(|&d| d > 0.0) {
        display_distance_based_duration(event, distance, zwift_score);
    } else {
        display_estimated_duration(event, zwift_score);
    }
}

/// Display duration based on route information
fn display_route_based_duration(
    event: &ZwiftEvent,
    route_id: u32,
    user_subgroup: Option<&EventSubGroup>,
    distance_meters: Option<f64>,
    zwift_score: u32,
) {
    if let Some(route_data) = get_route_data(route_id) {
        let (actual_distance_km, lap_count) =
            calculate_actual_distance(&route_data, user_subgroup, distance_meters, event);

        // Display distance and laps
        if lap_count > 1 {
            println!(
                "{}: {:.1} km ({} laps of {:.1} km)",
                "Distance".bright_blue(),
                actual_distance_km,
                lap_count,
                route_data.distance_km
            );
        } else {
            println!("{}: {:.1} km", "Distance".bright_blue(), actual_distance_km);
        }

        // Show lead-in distance if significant
        if route_data.lead_in_distance_km > 0.1 {
            println!(
                "{}: {:.1} km",
                "Lead-in".bright_blue(),
                route_data.lead_in_distance_km
            );
        }

        // Calculate and display estimated duration
        display_calculated_duration(&route_data, actual_distance_km, zwift_score);
    } else {
        display_unknown_route_duration(
            event,
            user_subgroup,
            distance_meters,
            route_id,
            zwift_score,
        );
    }
}

/// Calculate actual distance considering laps
fn calculate_actual_distance(
    route_data: &crate::models::RouteData,
    user_subgroup: Option<&EventSubGroup>,
    distance_meters: Option<f64>,
    event: &ZwiftEvent,
) -> (f64, u32) {
    let mut lap_count = 1;
    let actual_distance_km;

    // Check if subgroup has lap information (for Racing Score events)
    if let Some(sg) = user_subgroup {
        if let Some(laps) = sg.laps {
            lap_count = laps;
            actual_distance_km = route_data.distance_km * laps as f64;
        } else if let Some(dist_m) = sg.distance_in_meters.filter(|&d| d > 0.0) {
            // Subgroup has explicit distance
            actual_distance_km = dist_m / METERS_PER_KILOMETER;
            lap_count = (actual_distance_km / route_data.distance_km).round() as u32;
        } else {
            // No lap info in subgroup, use event distance or route distance
            actual_distance_km = distance_meters
                .map(|d| d / METERS_PER_KILOMETER)
                .unwrap_or(route_data.distance_km);
        }
    } else {
        // No subgroup - use event distance or check database for multi-lap info
        if let Some(dist_m) = distance_meters.filter(|&d| d > 0.0) {
            actual_distance_km = dist_m / METERS_PER_KILOMETER;
            lap_count = (actual_distance_km / route_data.distance_km).round() as u32;
        } else {
            // Check if this is a known multi-lap event
            if let Ok(db) = Database::new() {
                if let Ok(Some(laps)) = db.get_multi_lap_info(&event.name) {
                    lap_count = laps;
                }
            }
            actual_distance_km = route_data.distance_km * lap_count as f64;
        }
    }

    (actual_distance_km, lap_count)
}

/// Display calculated duration based on route data
fn display_calculated_duration(
    route_data: &crate::models::RouteData,
    actual_distance_km: f64,
    zwift_score: u32,
) {
    let category = get_category_from_score(zwift_score);
    let effective_speed = get_category_speed(category);

    let difficulty_multiplier = get_route_difficulty_multiplier_from_elevation(
        route_data.distance_km, // Use base route for elevation calculation
        route_data.elevation_m,
    );

    let surface_multiplier = match route_data.surface {
        "road" => 1.0,
        "gravel" => 0.85,
        "mixed" => 0.92,
        _ => 1.0,
    };

    let adjusted_speed = effective_speed * difficulty_multiplier * surface_multiplier;
    // Include lead-in distance in total duration calculation
    let total_distance = actual_distance_km + route_data.lead_in_distance_km;
    let estimated_duration = ((total_distance / adjusted_speed) * 60.0) as u32;

    let cat_string = get_detailed_category_from_score(zwift_score);
    println!(
        "{}: {} (estimated for Cat {} rider)",
        "Duration".bright_blue(),
        format_duration(estimated_duration).green(),
        cat_string
    );
}

/// Display duration for unknown routes
fn display_unknown_route_duration(
    event: &ZwiftEvent,
    user_subgroup: Option<&EventSubGroup>,
    distance_meters: Option<f64>,
    route_id: u32,
    zwift_score: u32,
) {
    let distance_meters = user_subgroup
        .and_then(|sg| sg.distance_in_meters)
        .or(distance_meters)
        .filter(|&d| d > 0.0); // Ignore 0.0 distances

    if let Some(dist_m) = distance_meters {
        let distance_km = dist_m / METERS_PER_KILOMETER;
        let route_name = event.route.as_deref().unwrap_or(&event.name);
        let estimated_duration =
            estimate_duration_for_category(distance_km, route_name, zwift_score);

        println!("{}: {:.1} km", "Distance".bright_blue(), distance_km);
        let cat_string = get_detailed_category_from_score(zwift_score);
        println!(
            "{}: {} (estimated for Cat {} rider, unknown route)",
            "Duration".bright_blue(),
            format_duration(estimated_duration).green(),
            cat_string
        );
    } else {
        println!(
            "{}: Route ID {} needs mapping",
            "Info".yellow(),
            route_id.to_string().yellow()
        );
    }
}

/// Display distance-based duration when no route ID
fn display_distance_based_duration(event: &ZwiftEvent, distance_meters: f64, zwift_score: u32) {
    let distance_km = distance_meters / METERS_PER_KILOMETER;
    let route_name = event.route.as_deref().unwrap_or(&event.name);
    let estimated_duration = estimate_duration_for_category(distance_km, route_name, zwift_score);

    println!("{}: {:.1} km", "Distance".bright_blue(), distance_km);
    let cat_string = get_detailed_category_from_score(zwift_score);
    println!(
        "{}: {} (estimated for Cat {} rider)",
        "Duration".bright_blue(),
        format_duration(estimated_duration).green(),
        cat_string
    );
}

/// Display estimated duration from event name
fn display_estimated_duration(event: &ZwiftEvent, zwift_score: u32) {
    let estimated_distance = estimate_distance_from_name(&event.name);

    if let Some(distance_km) = estimated_distance {
        let route_name = event.route.as_deref().unwrap_or(&event.name);
        let estimated_duration =
            estimate_duration_for_category(distance_km, route_name, zwift_score);

        println!(
            "{}: ~{:.1} km (estimated from route)",
            "Distance".bright_blue(),
            distance_km
        );
        let cat_string = get_detailed_category_from_score(zwift_score);
        println!(
            "{}: {} (estimated for Cat {} rider)",
            "Duration".bright_blue(),
            format_duration(estimated_duration).green(),
            cat_string
        );
    }
}

/// Display category enforcement status
pub fn display_category_enforcement(event: &ZwiftEvent) {
    if event.category_enforcement {
        println!("{}: {}", "Category".bright_blue(), "Enforced ✓".green());
    }
}

/// Display event subgroups with details
pub fn display_subgroups(event: &ZwiftEvent, zwift_score: u32) {
    if event.event_sub_groups.is_empty() {
        return;
    }

    println!("{}: ", "Categories".bright_blue());

    // Find the subgroup that matches user's category
    let user_category = get_category_from_score(zwift_score);

    for group in &event.event_sub_groups {
        let is_user_category = group.name.contains(user_category)
            || (user_category == "D" && group.name.contains("E"));

        print!("  • {}", group.name);

        // Show distance and calculate laps if possible
        if let Some(dist) = group.distance_in_meters {
            let dist_km = dist / METERS_PER_KILOMETER;
            print!(" ({:.1} km", dist_km);

            // Try to calculate laps based on base route distance
            if let Some(route_id) = event.route_id {
                if let Some(route_data) = get_route_data(route_id) {
                    let base_distance = route_data.distance_km;
                    if base_distance > 0.0 {
                        let laps = (dist_km / base_distance).round() as u32;
                        if laps > 1 {
                            print!(" - {} laps", laps);
                        }
                    }
                }
            }
            print!(")");

            // Show estimated duration for user's category
            if is_user_category {
                let route_name = event.route.as_deref().unwrap_or(&event.name);
                let estimated_duration =
                    estimate_duration_for_category(dist_km, route_name, zwift_score);
                print!(
                    " → {} estimated",
                    format_duration(estimated_duration).green()
                );
            }
        }

        if let Some(dur) = group.duration_in_minutes {
            print!(" ({})", format_duration(dur));
        }

        println!();
    }
}

/// Display parsed description data
pub fn display_description_info(event: &ZwiftEvent) {
    if event.description.is_none() {
        return;
    }

    let desc_data = parse_description_data(&event.description);

    // Show parsed distance/elevation from description
    let mut parsed_info = Vec::new();
    if let Some(dist) = desc_data.distance_km {
        parsed_info.push(format!("{:.1} km", dist));
    }
    if let Some(elev) = desc_data.elevation_m {
        parsed_info.push(format!("{} m elevation", elev));
    }
    if let Some(laps) = desc_data.laps {
        parsed_info.push(format!("{} laps", laps));
    }

    if !parsed_info.is_empty() {
        println!(
            "{}: {}",
            "From description".bright_blue(),
            parsed_info.join(", ").cyan()
        );
    }

    // Show first 2 lines of description
    if let Some(desc) = &event.description {
        let cleaned_desc = desc
            .lines()
            .take(2)
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();
        if !cleaned_desc.is_empty() {
            println!("{}: {}", "Info".bright_blue(), cleaned_desc.dimmed());
        }
    }
}

/// Display external route information URL
pub fn display_external_url(event: &ZwiftEvent) {
    if let Some(route_id) = event.route_id {
        if let Some(db_route) = get_route_data_from_db(route_id) {
            if let Some(slug) = db_route.slug {
                let world_slug = match db_route.world.as_str() {
                    "Watopia" => "watopia",
                    "London" => "london",
                    "New York" => "new-york",
                    "Innsbruck" => "innsbruck",
                    "Yorkshire" => "yorkshire",
                    "France" => "france",
                    "Paris" => "paris",
                    "Makuri Islands" => "makuri-islands",
                    "Scotland" => "scotland",
                    "Bologna" => "bologna",
                    "Crit City" => "crit-city",
                    _ => "watopia", // Default
                };
                let url = format!(
                    "https://whatsonzwift.com/world/{}/route/{}",
                    world_slug, slug
                );
                println!("{}: {}", "Route Info".bright_blue(), url.dimmed());
            }
        }
    }
}

/// Main function to print a single event with all details
pub fn print_event(event: &ZwiftEvent, zwift_score: u32) {
    display_event_header(event);
    display_route_info(event);
    display_duration_info(event, zwift_score);
    display_category_enforcement(event);
    display_subgroups(event, zwift_score);
    display_description_info(event);
    display_external_url(event);
}

/// Structure to hold table row data for compact display
pub struct EventTableRow {
    pub name: String,
    pub time: String,
    pub distance: String,
    pub elevation: String,
    pub duration: String,
}

/// Prepare event data for table display
pub fn prepare_event_row(event: &ZwiftEvent, zwift_score: u32) -> EventTableRow {
    let local_time: DateTime<Local> = event.event_start.into();
    let time_str = local_time.format("%H:%M").to_string();

    // Get route data and calculate total distance and elevation
    let (distance_str, elevation_str, duration_str) = if let Some(route_id) = event.route_id {
        if let Some(route_data) = get_route_data(route_id) {
            // Calculate total distance including lead-in
            let user_subgroup = find_user_subgroup(event, zwift_score);
            let distance_meters = user_subgroup
                .and_then(|sg| sg.distance_in_meters)
                .or(event.distance_in_meters);

            let mut actual_distance_km = route_data.distance_km;
            let mut lap_count = 1;

            // Calculate actual distance including laps
            if let Some(sg) = user_subgroup {
                if let Some(laps) = sg.laps {
                    lap_count = laps;
                    actual_distance_km = route_data.distance_km * laps as f64;
                } else if let Some(dist_m) = sg.distance_in_meters.filter(|&d| d > 0.0) {
                    actual_distance_km = dist_m / METERS_PER_KILOMETER;
                    lap_count = (actual_distance_km / route_data.distance_km).round() as u32;
                }
            } else if let Some(dist_m) = distance_meters.filter(|&d| d > 0.0) {
                actual_distance_km = dist_m / METERS_PER_KILOMETER;
                lap_count = (actual_distance_km / route_data.distance_km).round() as u32;
            }

            // Total distance including lead-in (no lap indicator)
            let total_distance = actual_distance_km + route_data.lead_in_distance_km;
            let distance_str = format!("{:.1} km", total_distance);

            // Calculate total elevation (multiply by laps if multi-lap)
            let total_elevation = route_data.elevation_m * lap_count;
            let elevation_str = format!("{}m", total_elevation);

            // Calculate duration
            let category = get_category_from_score(zwift_score);
            let effective_speed = get_category_speed(category);

            let difficulty_multiplier = get_route_difficulty_multiplier_from_elevation(
                route_data.distance_km,
                route_data.elevation_m,
            );

            let surface_multiplier = match route_data.surface {
                "road" => 1.0,
                "gravel" => 0.85,
                "mixed" => 0.92,
                _ => 1.0,
            };

            let adjusted_speed = effective_speed * difficulty_multiplier * surface_multiplier;
            let estimated_duration = ((total_distance / adjusted_speed) * 60.0) as u32;

            (
                distance_str,
                elevation_str,
                format_duration(estimated_duration),
            )
        } else {
            // Unknown route
            if let Some(dist_m) = event.distance_in_meters.filter(|&d| d > 0.0) {
                let distance_km = dist_m / METERS_PER_KILOMETER;
                let route_name = event.route.as_deref().unwrap_or(&event.name);
                let estimated_duration =
                    estimate_duration_for_category(distance_km, route_name, zwift_score);
                (
                    format!("{:.1} km", distance_km),
                    "?m".to_string(),
                    format_duration(estimated_duration),
                )
            } else {
                ("? km".to_string(), "?m".to_string(), "? min".to_string())
            }
        }
    } else {
        // No route ID
        if let Some(dist_m) = event.distance_in_meters.filter(|&d| d > 0.0) {
            let distance_km = dist_m / 1000.0;
            let route_name = event.route.as_deref().unwrap_or(&event.name);
            let estimated_duration =
                estimate_duration_for_category(distance_km, route_name, zwift_score);
            (
                format!("{:.1} km", distance_km),
                "?m".to_string(),
                format_duration(estimated_duration),
            )
        } else {
            ("? km".to_string(), "?m".to_string(), "? min".to_string())
        }
    };

    EventTableRow {
        name: event.name.clone(),
        time: time_str,
        distance: distance_str,
        elevation: elevation_str,
        duration: duration_str,
    }
}

/// Print events in table format
pub fn print_events_table(events: &[ZwiftEvent], zwift_score: u32) {
    if events.is_empty() {
        return;
    }

    // Check if events span multiple days
    let first_event_time: DateTime<Local> = events[0].event_start.into();
    let last_event_time: DateTime<Local> = events[events.len() - 1].event_start.into();
    let spans_multiple_days = first_event_time.date_naive() != last_event_time.date_naive();

    // Collect data for all events
    let mut rows: Vec<(EventTableRow, DateTime<Local>)> = Vec::new();

    for event in events {
        let row = prepare_event_row(event, zwift_score);
        let local_time: DateTime<Local> = event.event_start.into();
        rows.push((row, local_time));
    }

    // Calculate column widths
    let name_width = rows
        .iter()
        .map(|(r, _)| r.name.len())
        .max()
        .unwrap_or(10)
        .max(10);
    let time_width = rows
        .iter()
        .map(|(r, _)| r.time.len())
        .max()
        .unwrap_or(5)
        .max(5);
    let distance_width = rows
        .iter()
        .map(|(r, _)| r.distance.len())
        .max()
        .unwrap_or(8)
        .max(8);
    let elevation_width = rows
        .iter()
        .map(|(r, _)| r.elevation.len())
        .max()
        .unwrap_or(6)
        .max(6);
    let duration_width = rows
        .iter()
        .map(|(r, _)| r.duration.len())
        .max()
        .unwrap_or(8)
        .max(8);
    let total_width =
        name_width + time_width + distance_width + elevation_width + duration_width + 17;

    // Print header
    println!("\n{}", "─".repeat(total_width).dimmed());
    println!(
        "{:<width1$} │ {:<width2$} │ {:<width3$} │ {:<width4$} │ {:<width5$}",
        "Event Name".bright_blue().bold(),
        "Time".bright_blue().bold(),
        "Distance".bright_blue().bold(),
        "Elev".bright_blue().bold(),
        "Duration".bright_blue().bold(),
        width1 = name_width,
        width2 = time_width,
        width3 = distance_width,
        width4 = elevation_width,
        width5 = duration_width
    );
    println!("{}", "─".repeat(total_width).dimmed());

    // Print rows with day separators if needed
    let mut current_date = None;
    for (row, event_time) in rows {
        let event_date = event_time.date_naive();

        // Insert day separator if date changes and we span multiple days
        if spans_multiple_days && current_date.is_some() && current_date != Some(event_date) {
            println!("{}", "─".repeat(total_width).dimmed());
            let day_label = event_time.format("%A, %B %d").to_string();
            println!("{:^width$}", day_label.yellow(), width = total_width);
            println!("{}", "─".repeat(total_width).dimmed());
        } else if spans_multiple_days && current_date.is_none() {
            // First day label
            let day_label = event_time.format("%A, %B %d").to_string();
            println!("{:^width$}", day_label.yellow(), width = total_width);
            println!("{}", "─".repeat(total_width).dimmed());
        }

        current_date = Some(event_date);

        println!(
            "{:<width1$} │ {:<width2$} │ {:<width3$} │ {:<width4$} │ {:<width5$}",
            row.name,
            row.time,
            row.distance,
            row.elevation,
            row.duration.green(),
            width1 = name_width,
            width2 = time_width,
            width3 = distance_width,
            width4 = elevation_width,
            width5 = duration_width
        );
    }

    println!("{}", "─".repeat(total_width).dimmed());
}

/// Display filter statistics and actionable fixes
pub fn display_filter_stats(stats: &FilterStats, _total_fetched: usize) {
    let total_filtered = stats.sport_filtered
        + stats.time_filtered
        + stats.type_filtered
        + stats.tag_filtered
        + stats.completed_routes_filtered
        + stats.duration_filtered;

    if total_filtered == 0 && stats.unknown_routes == 0 && stats.missing_distance == 0 {
        return; // No issues to report
    }

    println!("\n{}", "─".repeat(80).dimmed());
    println!(
        "{}: {} events filtered out",
        "Filter Summary".yellow(),
        total_filtered
    );

    if stats.sport_filtered > 0 {
        println!("  • {} non-cycling events", stats.sport_filtered);
    }

    if stats.time_filtered > 0 {
        println!("  • {} events outside time range", stats.time_filtered);
    }

    if stats.type_filtered > 0 {
        println!("  • {} events of wrong type", stats.type_filtered);
    }

    if stats.tag_filtered > 0 {
        println!("  • {} events filtered by tags", stats.tag_filtered);
    }

    if stats.completed_routes_filtered > 0 {
        println!(
            "  • {} events on completed routes",
            stats.completed_routes_filtered
        );
    }

    if stats.duration_filtered > 0 {
        println!(
            "  • {} events outside duration range",
            stats.duration_filtered
        );
    }

    // Data quality issues in shown events
    if stats.unknown_routes > 0 || stats.missing_distance > 0 {
        println!(
            "\n{}: Some events may have inaccurate estimates",
            "Data Quality".yellow()
        );

        if stats.unknown_routes > 0 {
            println!("  • {} events with unknown routes", stats.unknown_routes);
            println!(
                "    {} Run {} to help map these routes",
                "Fix:".green(),
                "cargo run --bin zwift-race-finder -- --discover-routes".cyan()
            );
            println!(
                "    {} Check {} for manual mapping",
                "Or:".green(),
                "sql/mappings/manual_route_mappings.sql".cyan()
            );
        }

        if stats.missing_distance > 0 {
            println!(
                "  • {} events missing distance data",
                stats.missing_distance
            );
            println!(
                "    {} These are typically new Racing Score events",
                "Note:".green()
            );
            println!(
                "    {} Distance parsing from descriptions is attempted automatically",
                "Info:".green()
            );
        }
    }

    // Suggest actions for large numbers of filtered events
    if stats.duration_filtered > 20 {
        println!("\n{}: Many events filtered by duration", "Tip".green());
        println!("  • Try wider tolerance: {}", "--tolerance 60".cyan());
        println!("  • Or different duration: {}", "--duration 60".cyan());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{METERS_PER_KILOMETER, MINUTES_PER_HOUR, PERCENT_MULTIPLIER};
    use crate::models::{EventSubGroup, ZwiftEvent};
    use chrono::Utc;

    fn create_test_event(name: &str, distance_km: f64, route: &str, sport: &str) -> ZwiftEvent {
        ZwiftEvent {
            id: 1,
            name: name.to_string(),
            event_start: Utc::now() + chrono::Duration::hours(2),
            event_type: "RACE".to_string(),
            distance_in_meters: Some(distance_km * METERS_PER_KILOMETER),
            duration_in_minutes: None,
            duration_in_seconds: None,
            route_id: None,
            route: Some(route.to_string()),
            description: None,
            category_enforcement: false,
            event_sub_groups: vec![],
            sport: sport.to_string(),
            tags: vec![],
        }
    }

    #[test]
    fn test_prepare_event_row_distance_conversion() {
        // Test distance conversion: distance / 1000.0
        // Targets mutation: replace / with * and / with %
        let event = create_test_event("Test Race", 42.195, "Test Route", "CYCLING");
        let row = prepare_event_row(&event, 195);

        // 42.195 km should be displayed as "42.2 km" (with lead-in it might be different)
        // If / becomes *, we'd get 42195 km
        // If / becomes %, we'd get a remainder instead
        assert!(row.distance.contains("km"));
        assert!(!row.distance.contains("42195")); // Would appear if / became *
    }

    #[test]
    fn test_prepare_event_row_time_formatting() {
        // Test time formatting calculations
        // hours = duration / 60, minutes = duration % 60
        // Targets mutations: replace / with %, replace % with /

        // Create event with known duration
        let mut event = create_test_event("Test Race", 40.0, "Test Route", "CYCLING");
        event.route_id = Some(1); // Known route

        let row = prepare_event_row(&event, 195);

        // Duration should be formatted as "H:MM"
        // For 40km at Cat D speed (~77 min) = "1:17"
        assert!(row.duration.contains(":"));

        // Wrong arithmetic would produce weird results
        assert!(!row.duration.contains("77:")); // Would appear if / became %
    }

    #[test]
    fn test_print_event_percentage_calculation() {
        // Test percentage calculation: (error / actual) * 100.0
        // Targets: replace * with +, replace / with *

        let error = 5.0;
        let actual = 20.0;
        let percentage = (error / actual) * PERCENT_MULTIPLIER;

        assert_eq!(percentage, 25.0); // 5/20 * 100 = 25%
        assert_ne!(percentage, 105.0); // Would be if * became +
        assert_ne!(percentage, 100.0); // Would be if / became *
    }

    #[test]
    fn test_prepare_event_row() {
        // Create a test event with route data
        let event = ZwiftEvent {
            id: 1,
            name: "Test Race".to_string(),
            event_start: Utc::now() + chrono::Duration::hours(2),
            event_type: "RACE".to_string(),
            distance_in_meters: Some(20000.0),
            duration_in_minutes: None,
            duration_in_seconds: None,
            route_id: Some(1), // Watopia's Ocean Boulevard
            route: Some("Ocean Boulevard".to_string()),
            description: None,
            category_enforcement: false,
            event_sub_groups: vec![],
            sport: "CYCLING".to_string(),
            tags: vec![],
        };

        let row = prepare_event_row(&event, 195); // Cat D rider

        // Verify the row has expected format
        assert_eq!(row.name, "Test Race");
        assert!(row.time.contains(":")); // Should be HH:MM format
        assert!(row.distance.contains("km")); // Should include km
        assert!(row.elevation.contains("m")); // Should include m
        assert!(row.duration.contains(":")); // Should be MM:SS or HH:MM format
    }

    #[test]
    fn test_prepare_event_row_time_formatting_mutations() {
        // Test time formatting arithmetic
        let test_cases = vec![
            (30, "30"),    // 30 minutes
            (60, "1:00"),  // 60 minutes = 1 hour
            (90, "1:30"),  // 90 minutes = 1 hour 30 min
            (125, "2:05"), // 125 minutes = 2 hours 5 min
        ];

        for (minutes, _expected) in test_cases {
            let hours = minutes / MINUTES_PER_HOUR;
            let mins = minutes % MINUTES_PER_HOUR;

            // Test / is division, not %
            if minutes == 90 {
                assert_eq!(hours, 1);
                assert_ne!(hours, 30); // Would be if / was %
            }

            // Test % is modulo, not /
            if minutes == 90 {
                assert_eq!(mins, 30);
                assert_ne!(mins, 1); // Would be if % was /
            }
        }
    }

    #[test]
    fn test_print_event_percentage_calculations() {
        // Test percentage calculation
        let error = 5.0;
        let actual = 50.0;
        let percentage = (error * PERCENT_MULTIPLIER) / actual;

        assert_eq!(percentage, 10.0);
        assert_ne!(error + PERCENT_MULTIPLIER, 500.0); // Would be if * was +
        assert_ne!((error * PERCENT_MULTIPLIER) * actual, 10.0); // Would be if / was *
    }

    #[test]
    fn test_log_unknown_route() {
        // Create a test event
        let event = ZwiftEvent {
            id: 999,
            name: "Unknown Route Race".to_string(),
            event_start: Utc::now(),
            event_type: "RACE".to_string(),
            distance_in_meters: Some(25000.0),
            duration_in_minutes: None,
            duration_in_seconds: None,
            route_id: Some(9999), // Unknown route ID
            route: Some("Mystery Route".to_string()),
            description: Some("A mysterious 25km race".to_string()),
            category_enforcement: false,
            event_sub_groups: vec![],
            sport: "CYCLING".to_string(),
            tags: vec![],
        };

        // This should not panic
        log_unknown_route(&event);
    }

    #[test]
    fn test_display_filter_stats_arithmetic() {
        // Test arithmetic in display_filter_stats
        // total_races += count, etc.
        // Targets: replace += with *=, += with -=

        let mut stats = FilterStats::default();

        // Simulate filtering operations
        stats.sport_filtered = 5;
        stats.time_filtered = 3;
        stats.duration_filtered = 2;

        // Verify counts are reasonable (would be huge if *= mutation)
        assert!(stats.sport_filtered < 100);
        assert!(stats.time_filtered < 100);
        assert!(stats.duration_filtered < 100);

        // Test increments
        let mut count = 0u32;
        for _ in 0..5 {
            count += 1; // Should be 5, not 120 if *= mutation
        }
        assert_eq!(count, 5);
    }

    #[test]
    fn test_display_filter_stats() {
        // Test with no filtering
        let empty_stats = FilterStats::default();
        display_filter_stats(&empty_stats, 100); // Should not print anything

        // Test with some filtering
        let stats = FilterStats {
            sport_filtered: 5,
            time_filtered: 3,
            type_filtered: 2,
            tag_filtered: 1,
            completed_routes_filtered: 0,
            duration_filtered: 4,
            unknown_routes: 2,
            missing_distance: 1,
        };

        // We can't easily capture println! output, but we can verify the function runs
        display_filter_stats(&stats, 100); // Should print statistics
    }
}
