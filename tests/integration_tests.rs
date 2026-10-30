//! Integration tests for zwift-race-finder
//! These tests verify the complete functionality of the application

mod common;
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
    fn test_explicit_route_metadata_supports_estimation() {
        let id = crate::common::install_route();
        let route = estimation::get_route_data(id).expect("explicit fixture must resolve");
        assert_eq!(route.name, "Synthetic test circuit");
        assert_eq!(route.distance_km, 10.0);
        let minutes = estimation::estimate_duration_from_route_id(id, 195).unwrap();
        assert!(minutes > 0);
        assert!(estimation::get_route_data(u32::MAX).is_none());
    }

}
