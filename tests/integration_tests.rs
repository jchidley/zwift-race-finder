//! Integration tests for zwift-race-finder
//! These tests verify the complete functionality of the application

use std::process::Command;
use zwift_race_finder::estimation;
use zwift_race_finder::parsing::parse_lap_count;

/// Helper to run the binary with arguments
fn run_command(args: &[&str]) -> (String, String, bool) {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--bin")
        .arg("zwift-race-finder")
        .arg("--")
        .args(args);

    let output = cmd.output().expect("Failed to execute command");

    (
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
        output.status.success(),
    )
}

#[test]
fn test_help_command() {
    let (stdout, _, success) = run_command(&["--help"]);

    assert!(success);
    assert!(stdout.contains("Find Zwift races") || stdout.contains("Zwift Race Finder"));
    assert!(stdout.contains("--zwift-score"));
    assert!(stdout.contains("--duration"));
    assert!(stdout.contains("--tolerance"));
}

#[test]
fn test_invalid_duration() {
    let (_, stderr, success) = run_command(&["--duration", "-10"]);

    assert!(!success);
    assert!(
        stderr.contains("invalid value")
            || stderr.contains("Duration must be positive")
            || stderr.contains("unexpected argument")
    );
}

#[test]
fn test_invalid_event_type() {
    let (_stdout, stderr, success) = run_command(&["--event-type", "invalid_type"]);

    // Invalid event type still runs but shows warning to stderr
    assert!(success);
    assert!(stderr.contains("Unknown event type") || stderr.contains("invalid_type"));
}

#[test]
fn test_new_routes_only_flag() {
    // Test that new-routes-only flag works
    let (stdout, _, _) = run_command(&["--new-routes-only"]);

    // Should either work or give appropriate message
    assert!(stdout.contains("routes") || stdout.contains("No results"));
}

#[test]
fn test_database_command() {
    // Test that database-dependent commands run without crashing
    // The actual database is created in ~/.local/share/zwift-race-finder/
    let (stdout, stderr, success) = run_command(&["--show-unknown-routes"]);

    // Command should either succeed or fail gracefully
    if !success {
        // If it failed, should give a reasonable error message
        assert!(stderr.contains("Error") || stderr.contains("Failed"));
    } else {
        // If it succeeded, should have some output
        assert!(stdout.contains("Unknown") || stdout.contains("No unknown routes"));
    }
}

#[cfg(test)]
mod cli_parsing_tests {
    use super::*;

    #[test]
    fn test_multiple_tags() {
        let (stdout, stderr, success) =
            run_command(&["--tags", "ranked,zracing", "--exclude-tags", "women_only"]);

        // Should parse successfully and not crash
        // Success depends on API availability, but should not have parsing errors
        if !success {
            // If it failed, should be due to API issues, not parsing
            assert!(
                stderr.contains("Error") || stdout.contains("No results") || stdout.contains("API")
            );
        }
    }

    #[test]
    fn test_record_result_format() {
        // Test invalid format
        let (_, stderr, success) = run_command(&["--record-result", "invalid_format"]);

        assert!(!success);
        assert!(
            stderr.contains("Invalid result format")
                || stderr.contains("comma-separated")
                || stderr.contains("Format: --record-result")
        );
    }

    #[test]
    fn test_verbose_mode() {
        let (stdout, _, success) = run_command(&["--verbose", "--duration", "30"]);

        // In verbose mode, output should be different from table mode
        if success && stdout.contains("Event") {
            // If we got results, verbose mode should show detailed info
            assert!(
                stdout.contains("Route:")
                    || stdout.contains("Distance:")
                    || stdout.contains("Category:")
            );
        } else {
            // At minimum, should have some output (error or no results message)
            assert!(!stdout.is_empty());
        }
    }
}

#[cfg(test)]
mod output_format_tests {
    use super::*;

    #[test]
    fn test_table_output_default() {
        // Default should be table format
        let (stdout, _, success) = run_command(&["--duration", "60"]);

        // Should have table elements or error message
        if success && !stdout.contains("No results") && !stdout.contains("Error") {
            // If we got results in table format, should have headers
            assert!(
                stdout.contains("Event") || stdout.contains("Time") || stdout.contains("Duration")
            );
        } else {
            // Should have error or no results message
            assert!(
                stdout.contains("Error") || stdout.contains("No results") || stdout.contains("API")
            );
        }
    }

    // Route discovery integration tests
    // (Moved from route_discovery.rs to avoid module import issues)

    #[test]
    fn test_multi_lap_race_detection() {
        // Test that we can detect multi-lap races from event names
        assert_eq!(parse_lap_count("3R Volcano Flat Race - 3 Laps"), Some(3));
        assert_eq!(parse_lap_count("Race - 5 laps"), Some(5));
        assert_eq!(parse_lap_count("2 Lap Race"), Some(2));
        assert_eq!(parse_lap_count("Regular Race"), None);

        // Test distance calculation for multi-lap races
        if let Some(route_data) = estimation::get_route_data(123) {
            // Volcano Flat
            let base_distance = route_data.distance_km;

            // Single lap
            assert_eq!(
                get_multi_lap_distance("Regular Race", base_distance),
                base_distance
            );

            // 3 laps
            assert_eq!(
                get_multi_lap_distance("3 Lap Race", base_distance),
                base_distance * 3.0
            );
        }
    }

    fn get_multi_lap_distance(event_name: &str, base_distance: f64) -> f64 {
        if let Some(laps) = parse_lap_count(event_name) {
            base_distance * laps as f64
        } else {
            base_distance
        }
    }

    #[test]
    fn test_get_multi_lap_distance() {
        // Test single lap (no lap count)
        assert_eq!(get_multi_lap_distance("Regular Race", 10.0), 10.0);
        assert_eq!(get_multi_lap_distance("Sprint Race", 5.5), 5.5);

        // Test multi-lap races
        assert_eq!(get_multi_lap_distance("2 Lap Race", 10.0), 20.0);
        assert_eq!(get_multi_lap_distance("3 Lap Sprint", 5.0), 15.0);
        assert_eq!(get_multi_lap_distance("4 laps of Volcano", 12.5), 50.0);
        assert_eq!(get_multi_lap_distance("5 Laps Challenge", 8.0), 40.0);
        assert_eq!(get_multi_lap_distance("10 lap time trial", 2.5), 25.0);

        // Test edge cases
        assert_eq!(get_multi_lap_distance("", 10.0), 10.0);
        assert_eq!(get_multi_lap_distance("Race with laps in name", 10.0), 10.0);
    }

    #[test]
    fn test_route_id_regression_with_actual_results() {
        // This test will use Jack's actual race results once provided
        // For now, we test the route_id infrastructure

        // Test that our known routes exist
        let known_routes = vec![
            (1258415487, "Bell Lap"),
            (2143464829, "Watopia Flat Route"),
            (2927651296, "Makuri Pretzel"),
            (3742187716, "Castle to Castle"),
            (2698009951, "Downtown Dolphin"),
            (2663908549, "Mt. Fuji"),
        ];

        for (route_id, name) in known_routes {
            assert!(
                estimation::get_route_data(route_id).is_some(),
                "Route {} ({}) should exist in database",
                route_id,
                name
            );
        }

        // Test duration estimates are reasonable for Cat D (195 score)
        struct RouteExpectation {
            route_id: u32,
            name: &'static str,
            min_minutes: u32,
            max_minutes: u32,
        }

        let expectations = vec![
            RouteExpectation {
                route_id: 1258415487, // Bell Lap (14.1km, 59m elevation)
                name: "Bell Lap",
                min_minutes: 22, // 14.1km at ~38 km/h (flat boost)
                max_minutes: 28,
            },
            RouteExpectation {
                route_id: 2698009951, // Downtown Dolphin (22.9km, 80m elevation)
                name: "Downtown Dolphin",
                min_minutes: 40, // 22.9km at ~34 km/h
                max_minutes: 48,
            },
            RouteExpectation {
                route_id: 2663908549, // Mt. Fuji (20.3km, 1159m elevation)
                name: "Mt. Fuji",
                min_minutes: 52, // Very hilly route, 20.3km at ~23 km/h
                max_minutes: 70,
            },
        ];

        for exp in expectations {
            if let Some(duration) = estimation::estimate_duration_from_route_id(exp.route_id, 195) {
                assert!(
                    duration >= exp.min_minutes && duration <= exp.max_minutes,
                    "Route {} ({}) estimate {} should be {}-{} min for Cat D",
                    exp.route_id,
                    exp.name,
                    duration,
                    exp.min_minutes,
                    exp.max_minutes
                );
            } else {
                panic!(
                    "Route {} ({}) should have duration estimate",
                    exp.route_id, exp.name
                );
            }
        }

        // TODO: Add Jack's actual race results here
        // Example format:
        // struct ActualResult {
        //     route_id: u32,
        //     actual_minutes: u32,
        //     date: &'static str,
        // }
        //
        // let jacks_results = vec![
        //     ActualResult { route_id: 2698009951, actual_minutes: 52, date: "2025-01" },
        // ];
    }
}
