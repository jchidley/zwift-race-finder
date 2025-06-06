//! Configuration tests
//! These tests verify configuration loading and precedence

use std::env;
use std::fs;
use tempfile::TempDir;
use zwift_race_finder::config::Config;

#[test]
fn test_default_config() {
    let config = Config::default();

    // Test defaults structure - these have default values
    assert_eq!(config.defaults.zwift_score, Some(195));
    assert_eq!(config.defaults.category, Some("D".to_string()));
    assert_eq!(config.defaults.weight_kg, Some(86.0));
    assert_eq!(config.defaults.height_m, Some(1.82));
    assert!(config.defaults.ftp_watts.is_none());

    // Test preferences - defaults should have Some values
    assert_eq!(config.preferences.default_duration, Some(120));
    assert_eq!(config.preferences.default_tolerance, Some(30));
    assert_eq!(config.preferences.default_days, Some(1));
}

#[test]
fn test_config_from_toml() {
    let toml_content = r#"
[defaults]
zwift_score = 250
category = "C"
weight_kg = 80.0
ftp_watts = 280

[preferences]
default_duration = 90
default_tolerance = 20
default_days = 3

[display]
use_colors = false
debug = true
"#;

    let config: Config = toml::from_str(toml_content).unwrap();

    assert_eq!(config.defaults.zwift_score, Some(250));
    assert_eq!(config.defaults.category, Some("C".to_string()));
    assert_eq!(config.defaults.weight_kg, Some(80.0));
    assert_eq!(config.defaults.ftp_watts, Some(280));

    assert_eq!(config.preferences.default_duration, Some(90));
    assert_eq!(config.preferences.default_tolerance, Some(20));
    assert_eq!(config.preferences.default_days, Some(3));

    assert_eq!(config.display.use_colors, Some(false));
    assert_eq!(config.display.debug, Some(true));
}

#[test]
fn test_config_file_loading() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    let config_content = r#"
[defaults]
zwift_score = 300
weight_kg = 75.0

[preferences]
default_duration = 45
"#;

    fs::write(&config_path, config_content).unwrap();

    // Config::load() looks for config.toml in current directory
    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(&temp_dir).unwrap();

    let config = Config::load().unwrap();

    assert_eq!(config.defaults.zwift_score, Some(300));
    assert_eq!(config.defaults.weight_kg, Some(75.0));
    assert_eq!(config.preferences.default_duration, Some(45));

    env::set_current_dir(original_dir).unwrap();
}

#[test]
fn test_partial_config() {
    // Test that partial configs work (only some fields specified)
    let toml_content = r#"
[defaults]
zwift_score = 350
"#;

    let config: Config = toml::from_str(toml_content).unwrap();

    // Specified value
    assert_eq!(config.defaults.zwift_score, Some(350));

    // Other fields in [defaults] section become None when not specified
    assert_eq!(config.defaults.category, None); // Not specified in partial config
                                                // But preferences section uses default because entire section is missing
    assert_eq!(config.preferences.default_duration, Some(120)); // Uses default
}

#[test]
fn test_empty_config() {
    // Empty config should parse successfully
    let config: Config = toml::from_str("").unwrap();

    // All fields should use defaults from Default trait
    assert_eq!(config.defaults.zwift_score, Some(195)); // Default value
    assert_eq!(config.preferences.default_duration, Some(120)); // Default value
}

#[test]
fn test_invalid_config_handling() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("bad_config.toml");

    // Write invalid TOML
    fs::write(&config_path, "invalid toml content { ] }").unwrap();

    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(&temp_dir).unwrap();

    // Should fall back to defaults on error
    let config = Config::load().unwrap_or_default();
    assert_eq!(config.defaults.zwift_score, Some(195)); // Default value

    env::set_current_dir(original_dir).unwrap();
}
