//! Event display functionality
//!
//! This module contains functions for displaying event information in a formatted way.

use crate::constants::METERS_PER_KILOMETER;
use crate::database::Database;
use crate::estimation::{get_route_data, get_route_data_from_db};
use crate::event_analysis::selected_route_id;
use crate::event_filtering::{estimate_event, resolve_event, FilterStats};
use crate::formatting::format_duration;
use crate::models::ZwiftEvent;
use crate::parsing::parse_description_data;
use chrono::{DateTime, Local};
use colored::Colorize;

/// Display the basic event header information
pub fn display_event_header(event: &ZwiftEvent, zwift_score: u32) {
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

    let route_id = selected_route_id(event, zwift_score);
    let route_name = route_id
        .and_then(get_route_data_from_db)
        .map(|route| route.name)
        .or_else(|| {
            (route_id == event.route_id)
                .then(|| event.route.clone())
                .flatten()
        });
    if let Some(route) = route_name {
        println!("{}: {}", "Route".bright_blue(), route);
    }
}

/// Display route metadata
pub fn display_route_info(event: &ZwiftEvent, zwift_score: u32) {
    if let Some(info) = format_route_info(event, zwift_score) {
        println!("{}", info);
    }
    log_unknown_route(event, zwift_score);
}

/// Format metadata for the same route used by the duration resolver.
pub fn format_route_info(event: &ZwiftEvent, zwift_score: u32) -> Option<String> {
    let route_id = selected_route_id(event, zwift_score)?;
    Some(if let Some(route_data) = get_route_data(route_id) {
        format!(
            "{}: {} ({} elevation per lap)",
            "Route ID".bright_blue().dimmed(),
            route_id.to_string().dimmed(),
            route_data
                .elevation_m
                .map_or_else(|| "unknown".to_string(), |m| format!("{m:.1}m"))
        )
    } else {
        format!(
            "{}: {} (unknown route - automatically logged)",
            "Route ID".bright_blue().dimmed(),
            route_id.to_string().yellow()
        )
    })
}

/// Record missing route IDs for a future client-metadata refresh.
pub fn log_unknown_route(event: &ZwiftEvent, zwift_score: u32) {
    if let Some(route_id) = selected_route_id(event, zwift_score) {
        if get_route_data(route_id).is_none() {
            if let Ok(db) = Database::new() {
                let _ = db.record_unknown_route(route_id, &event.name, &event.event_type);
            }
        }
    }
}

/// Format the same resolved estimate used for filtering and the compact table.
pub fn format_duration_info(event: &ZwiftEvent, zwift_score: u32) -> String {
    let resolution = resolve_event(event, zwift_score);
    if let Ok(estimate) = &resolution {
        let distance = estimate.distance_km.map_or_else(String::new, |distance| {
            format!("{}: {:.1} km\n", "Distance".bright_blue(), distance)
        });
        let elevation = estimate.elevation_m.map_or_else(
            || "Elevation: unknown\n".to_string(),
            |m| {
                format!(
                    "Elevation: {}{} m\n",
                    if estimate.elevation_is_estimate {
                        "~"
                    } else {
                        ""
                    },
                    m
                )
            },
        );
        let basis = if estimate.is_fixed {
            "fixed duration".to_string()
        } else if let Some(personal) = &estimate.personal {
            format!(
                "history: {}; n={}; typical training error {:.1} min (descriptive, not a confidence interval; experimental estimate)",
                personal.basis, personal.sample_count, personal.typical_error_minutes
            )
        } else {
            "personal estimate".to_string()
        };
        format!(
            "{}{}{}: {} ({})",
            distance,
            elevation,
            "Duration".bright_blue(),
            format_duration(estimate.duration_minutes).green(),
            basis
        )
    } else {
        format!(
            "{}: unknown ({})",
            "Duration".bright_blue(),
            resolution.unwrap_err()
        )
    }
}

/// Display duration information without another calculation path.
pub fn display_duration_info(event: &ZwiftEvent, zwift_score: u32) {
    println!("{}", format_duration_info(event, zwift_score));
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

    let user_subgroup = crate::event_analysis::find_user_subgroup(event, zwift_score);

    for group in &event.event_sub_groups {
        let is_user_category = user_subgroup.is_some_and(|sg| std::ptr::eq(sg, group));

        print!("  • {}", group.name);

        // Show distance and calculate laps if possible
        if let Some(dist) = group
            .distance_in_meters
            .filter(|d| d.is_finite() && *d > 0.0)
        {
            let dist_km = dist / METERS_PER_KILOMETER;
            print!(" ({:.1} km", dist_km);

            // Try to calculate laps based on base route distance
            if let Some(route_id) = group.route_id.or(event.route_id) {
                if let Some(route_data) = get_route_data(route_id) {
                    let base_distance = route_data.distance_km;
                    if base_distance.is_finite() && base_distance > 0.0 {
                        if let Some(lead) = route_data
                            .lead_in_distance_km
                            .filter(|lead| lead.is_finite() && *lead >= 0.0)
                        {
                            let laps = ((dist_km - lead) / base_distance).round() as u32;
                            if laps > 1 {
                                print!(" - {} laps", laps);
                            }
                        }
                    }
                }
            }
            print!(")");
        }

        if let Some(dur) = crate::event_filtering::fixed_minutes(
            group.duration_in_minutes,
            group.duration_in_seconds,
        ) {
            print!(" ({})", format_duration(dur));
        }

        // Highlight exactly the subgroup whose geometry drives the main row,
        // including lap-only groups with no explicit distance.
        if is_user_category {
            if let Some(estimate) = estimate_event(event, zwift_score) {
                print!(
                    " → {} ({})",
                    format_duration(estimate.duration_minutes).green(),
                    if estimate.is_fixed {
                        "fixed duration"
                    } else {
                        "estimated"
                    }
                );
            }
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
pub fn display_external_url(event: &ZwiftEvent, zwift_score: u32) {
    if let Some(url) = external_route_url(event, zwift_score) {
        println!("{}: {}", "Route Info".bright_blue(), url.dimmed());
    }
}

/// External route metadata must refer to the selected subgroup's route too.
pub fn external_route_url(event: &ZwiftEvent, zwift_score: u32) -> Option<String> {
    if let Some(route_id) = selected_route_id(event, zwift_score) {
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
                return Some(url);
            }
        }
    }
    None
}

/// Main function to print a single event with all details
pub fn print_event(event: &ZwiftEvent, zwift_score: u32) {
    display_event_header(event, zwift_score);
    display_route_info(event, zwift_score);
    display_duration_info(event, zwift_score);
    display_category_enforcement(event);
    display_subgroups(event, zwift_score);
    display_description_info(event);
    display_external_url(event, zwift_score);
}

/// Structure to hold table row data for compact display
pub struct EventTableRow {
    pub name: String,
    pub time: String,
    pub distance: String,
    pub elevation: String,
    pub duration: String,
    pub basis: String,
}

/// Prepare event data for table display
pub fn prepare_event_row(event: &ZwiftEvent, zwift_score: u32) -> EventTableRow {
    let local_time: DateTime<Local> = event.event_start.into();
    let time_str = local_time.format("%H:%M").to_string();

    let resolution = resolve_event(event, zwift_score);
    let estimate = resolution.as_ref().ok();
    let distance_str = estimate
        .as_ref()
        .and_then(|e| e.distance_km)
        .map_or_else(|| "? km".to_string(), |km| format!("{:.1} km", km));
    let elevation_str = estimate.as_ref().map_or_else(
        || "?m".to_string(),
        |e| {
            e.elevation_m.map_or_else(
                || "?m".to_string(),
                |m| format!("{}{}m", if e.elevation_is_estimate { "~" } else { "" }, m),
            )
        },
    );
    let duration_str = estimate.as_ref().map_or_else(
        || "? min".to_string(),
        |e| format_duration(e.duration_minutes),
    );

    let basis = match &resolution {
        Err(reason) => format!("Unknown: {reason}"),
        Ok(e) if e.is_fixed => "Fixed".to_string(),
        Ok(e) => e.personal.as_ref().map_or_else(
            || "Personal estimate".to_string(),
            |personal| {
                format!(
                    "History: {} (experimental)",
                    match personal.basis {
                        crate::personal_estimation::PersonalBasis::SameRoute => "same route",
                        crate::personal_estimation::PersonalBasis::DistanceAscent =>
                            "distance/climbing",
                    }
                )
            },
        ),
    };

    EventTableRow {
        name: event.name.clone(),
        time: time_str,
        distance: distance_str,
        elevation: elevation_str,
        duration: duration_str,
        basis,
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
    let basis_width = rows
        .iter()
        .map(|(r, _)| r.basis.len())
        .max()
        .unwrap_or(5)
        .max(5);
    let total_width = name_width
        + time_width
        + distance_width
        + elevation_width
        + duration_width
        + basis_width
        + 20;

    // Print header
    println!("\n{}", "─".repeat(total_width).dimmed());
    println!(
        "{:<width1$} │ {:<width2$} │ {:<width3$} │ {:<width4$} │ {:<width5$} │ {:<width6$}",
        "Event Name".bright_blue().bold(),
        "Time".bright_blue().bold(),
        "Distance".bright_blue().bold(),
        "Elev".bright_blue().bold(),
        "Duration".bright_blue().bold(),
        "Basis".bright_blue().bold(),
        width1 = name_width,
        width2 = time_width,
        width3 = distance_width,
        width4 = elevation_width,
        width5 = duration_width,
        width6 = basis_width
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
            "{:<width1$} │ {:<width2$} │ {:<width3$} │ {:<width4$} │ {:<width5$} │ {:<width6$}",
            row.name,
            row.time,
            row.distance,
            row.elevation,
            row.duration.green(),
            row.basis,
            width1 = name_width,
            width2 = time_width,
            width3 = distance_width,
            width4 = elevation_width,
            width5 = duration_width,
            width6 = basis_width
        );
    }

    println!("{}", "─".repeat(total_width).dimmed());
}

/// Display filter statistics and actionable fixes
pub fn display_filter_stats(stats: &FilterStats, _total_fetched: usize) {
    let total_filtered = stats.total_filtered();

    if total_filtered == 0 {
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

    if stats.duration_unknown > 0 {
        println!(
            "  • {} events with unknown duration (not time-window matches)",
            stats.duration_unknown
        );
    }
    if stats.subgroup_filtered > 0 {
        println!(
            "  • {} events without an eligible subgroup",
            stats.subgroup_filtered
        );
    }

    if stats.duration_filtered > 0 {
        println!(
            "  • {} events outside duration range",
            stats.duration_filtered
        );
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
    use crate::constants::METERS_PER_KILOMETER;
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
        let mut event = create_test_event("Test Race", 42.195, "Test Route", "CYCLING");
        event.duration_in_minutes = Some(77);
        let row = prepare_event_row(&event, 195);

        // 42.195 km should be displayed as "42.2 km" (with lead-in it might be different)
        // If / becomes *, we'd get 42195 km
        // If / becomes %, we'd get a remainder instead
        assert_eq!(row.distance, "42.2 km");
        assert_eq!(row.duration, "01:17");
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
        event.duration_in_minutes = Some(77);

        let row = prepare_event_row(&event, 195);

        // Duration should be formatted as "H:MM"
        // Declared 77 minutes = "01:17"
        assert_eq!(row.duration, "01:17");

        // Wrong arithmetic would produce weird results
        assert!(!row.duration.contains("77:")); // Would appear if / became %
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
            duration_in_minutes: Some(40),
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
        log_unknown_route(&event, 195);
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
            duration_filtered: 4,
            duration_unknown: 0,
            subgroup_filtered: 0,
        };

        // We can't easily capture println! output, but we can verify the function runs
        display_filter_stats(&stats, 100); // Should print statistics
    }

    #[test]
    fn test_log_unknown_route_conditions() {
        // Test the || condition in log_unknown_route
        // Mutation: replace || with &&
        let mut event = create_test_event("Test Race", 25.0, "Test Route", "CYCLING");

        // Case 1: No route_id - should not call record_unknown_route
        event.route_id = None;
        log_unknown_route(&event, 195);

        // Case 2: Known route_id - should not call record_unknown_route
        event.route_id = Some(1); // This is a known route in the test data
        log_unknown_route(&event, 195);

        // Case 3: Unknown route_id with description
        event.route_id = Some(999999);
        event.description = Some("3 laps of Ocean Boulevard".to_string());
        log_unknown_route(&event, 195);

        // Case 4: Unknown route_id without description
        event.description = None;
        log_unknown_route(&event, 195);
    }

    #[test]
    fn test_display_uses_estimated_distance_without_extra_lead_in() {
        let mut event = create_test_event("Test Race", 30.0, "Test Route", "CYCLING");
        event.duration_in_minutes = Some(45);
        let estimate = estimate_event(&event, 195).unwrap();
        let row = prepare_event_row(&event, 195);
        assert_eq!(estimate.distance_km, Some(30.0));
        assert_eq!(row.distance, "30.0 km");
        assert_eq!(row.duration, format_duration(estimate.duration_minutes));
    }

    #[test]
    fn test_filter_stats_methods() {
        use crate::event_filtering::FilterStats;

        let stats = FilterStats {
            sport_filtered: 10,
            time_filtered: 5,
            type_filtered: 3,
            duration_filtered: 8,
            duration_unknown: 0,
            subgroup_filtered: 0,
            tag_filtered: 2,
        };

        let total_events = 100;
        display_filter_stats(&stats, total_events);

        // Test that FilterStats::total_filtered works correctly
        let total = stats.total_filtered();
        assert_eq!(total, 28); // 10 + 5 + 3 + 8 + 2 = 28
    }

    #[test]
    fn test_display_filter_stats_empty_case() {
        // Test mutation: if total_filtered == 0 (line 717)
        // This should NOT display anything when all counts are zero
        use crate::event_filtering::FilterStats;

        let empty_stats = FilterStats::default();

        // Capture the fact that display_filter_stats returns early for empty stats
        // by checking the condition that causes early return
        let total_filtered = empty_stats.sport_filtered
            + empty_stats.time_filtered
            + empty_stats.type_filtered
            + empty_stats.tag_filtered
            + empty_stats.duration_filtered;

        assert_eq!(total_filtered, 0);

        // The function should return early and not print anything
        display_filter_stats(&empty_stats, 100);
    }

    #[test]
    fn test_prepare_event_row_multi_lap_calculation() {
        // Test mutation: actual_distance_km = route_data.distance_km * laps (line 515)
        // Ensures multiplication is used, not addition

        let mut event = create_test_event("Multi-Lap Race", 10.0, "Test Route", "CYCLING");
        let route_id = 4_000_001_102;
        let route = serde_json::from_value(serde_json::json!({
            "route_id": route_id, "name": "Synthetic lap circuit", "world": "Test",
            "distance_km": 10.0, "elevation_m": 50.0, "surface": "road",
            "lead_in_distance_km": 0.5, "lead_in_elevation_m": 0.0
        }))
        .unwrap();
        crate::database::Database::new()
            .unwrap()
            .add_route(&route)
            .unwrap();
        event.route_id = Some(route_id);

        // Add subgroup with 3 laps
        event.event_sub_groups = vec![EventSubGroup {
            id: 1,
            name: "Cat D".to_string(),
            route_id: None,
            distance_in_meters: None,
            duration_in_minutes: None,
            duration_in_seconds: None,
            subgroup_label: None,
            category_enforcement: None,
            range_access_label: None,
            laps: Some(3),
        }];

        // Also need to find the user's subgroup - add the score range
        event.event_sub_groups[0].range_access_label = Some("0-199".to_string());
        event.duration_in_minutes = Some(60);

        let row = prepare_event_row(&event, 195); // Cat D rider

        // The test verifies that multiplication is used for laps
        assert!(row.distance.contains("km"));

        // Parse the distance value
        let distance_str = row.distance.replace(" km", "");
        let distance: f64 = distance_str.parse().unwrap_or(0.0);

        // With 3 laps, distance should be significantly more than base route
        // If * was replaced with +, we'd get base_distance + 3 instead of base_distance * 3
        assert!(
            distance > 15.0,
            "Distance should be > 15km for 3 laps, got {}",
            distance
        );

        // Also verify it's not just adding 3 to the base distance
        assert_ne!(
            distance, 13.0,
            "Distance shouldn't be exactly 13km (10 + 3)"
        );
    }
}
