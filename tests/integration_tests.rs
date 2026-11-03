mod common;
use std::process::Command;
use zwift_race_finder::estimation;

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

    assert!(success);
    assert!(stderr.contains("Unknown event type") || stderr.contains("invalid_type"));
}


#[test]
fn test_database_command() {
    let (stdout, stderr, success) = run_command(&["--show-unknown-routes"]);

    if !success {
        assert!(stderr.contains("Error") || stderr.contains("Failed"));
    } else {
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

        if !success {
            assert!(
                stderr.contains("Error") || stdout.contains("No results") || stdout.contains("API")
            );
        }
    }


    #[test]
    fn test_verbose_mode() {
        let (stdout, _, success) = run_command(&["--verbose", "--duration", "30"]);

        if success && stdout.contains("Event") {
            assert!(
                stdout.contains("Route:")
                    || stdout.contains("Distance:")
                    || stdout.contains("Category:")
            );
        } else {
            assert!(!stdout.is_empty());
        }
    }
}

#[cfg(test)]
mod output_format_tests {
    use super::*;

    #[test]
    fn test_table_output_default() {
        let (stdout, _, success) = run_command(&["--duration", "60"]);

        if success && !stdout.contains("No results") && !stdout.contains("Error") {
            assert!(
                stdout.contains("Event") || stdout.contains("Time") || stdout.contains("Duration")
            );
        } else {
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
