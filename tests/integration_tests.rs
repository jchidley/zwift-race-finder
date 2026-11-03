//! Integration tests for zwift-race-finder
//! These tests verify the complete functionality of the application

mod common;
use std::process::Command;
use zwift_race_finder::estimation;

/// Helper to run the binary with arguments
fn run_command(args: &[&str]) -> (String, String, bool) {
    let state = tempfile::tempdir().expect("create isolated CLI state");
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_zwift-race-finder"));
    cmd.env_clear()
        .env("HOME", state.path())
        .env("XDG_DATA_HOME", state.path().join("data"))
        .env("XDG_CONFIG_HOME", state.path().join("config"))
        .current_dir(state.path())
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
    // CLI state is isolated by run_command.
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
