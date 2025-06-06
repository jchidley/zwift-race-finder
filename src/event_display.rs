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
use crate::formatting::format_duration;
use crate::models::{EventSubGroup, ZwiftEvent};
use crate::parsing::{estimate_distance_from_name, parse_description_data};
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
fn log_unknown_route(event: &ZwiftEvent) {
    if let Ok(db) = Database::new() {
        if let Some(route_id) = event.route_id {
            // Determine if it's a distance-based or time-based race
            let is_fixed_duration =
                event.duration_in_minutes.is_some() || event.duration_in_seconds.is_some();
            let event_type = if is_fixed_duration {
                "time"
            } else {
                "distance"
            };

            if let Err(e) = db.record_unknown_route(route_id, &event.name, event_type) {
                eprintln!("Failed to log unknown route: {}", e);
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
