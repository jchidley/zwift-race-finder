use std::env;
use std::fs;
use tempfile::TempDir;
use zwift_race_finder::config::Config;

#[test]
fn test_default_config() {
    let config = Config::default();

    assert_eq!(config.defaults.zwift_score, Some(195));
    assert_eq!(config.defaults.category, Some("D".to_string()));

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

[preferences]
default_duration = 90
default_tolerance = 20
default_days = 3

"#;

    let config: Config = toml::from_str(toml_content).unwrap();

    assert_eq!(config.defaults.zwift_score, Some(250));
    assert_eq!(config.defaults.category, Some("C".to_string()));

    assert_eq!(config.preferences.default_duration, Some(90));
    assert_eq!(config.preferences.default_tolerance, Some(20));
    assert_eq!(config.preferences.default_days, Some(3));
}

#[test]
fn test_config_file_loading() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    let config_content = r#"
[defaults]
zwift_score = 300

[preferences]
default_duration = 45
"#;

    fs::write(&config_path, config_content).unwrap();

    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(&temp_dir).unwrap();

    let config = Config::load().unwrap();

    assert_eq!(config.defaults.zwift_score, Some(300));
    assert_eq!(config.preferences.default_duration, Some(45));

    env::set_current_dir(original_dir).unwrap();
}

#[test]
fn test_partial_config() {
    let toml_content = r#"
[defaults]
zwift_score = 350
"#;

    let config: Config = toml::from_str(toml_content).unwrap();

    assert_eq!(config.defaults.zwift_score, Some(350));

    assert_eq!(config.defaults.category, None);
    assert_eq!(config.preferences.default_duration, Some(120));
}

#[test]
fn test_empty_config() {
    let config: Config = toml::from_str("").unwrap();

    assert_eq!(config.defaults.zwift_score, Some(195));
    assert_eq!(config.preferences.default_duration, Some(120));
}

#[test]
fn test_invalid_config_handling() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    fs::write(&config_path, "invalid toml content { ] }").unwrap();

    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(&temp_dir).unwrap();

    assert!(
        Config::load().is_err(),
        "Malformed selected config must return an error"
    );

    env::set_current_dir(original_dir).unwrap();
}

#[test]
fn test_removed_settings_are_ignored() {
    let config: Config = toml::from_str(
        r#"
[defaults]
zwift_score = 250
category = "C"
weight_kg = 75.0
height_m = 1.75
ftp_watts = 250
[preferences]
default_duration = 60
default_tolerance = 15
default_days = 2
[display]
use_colors = false
debug = true
[import]
windows_username = "synthetic-user"
"#,
    )
    .unwrap();
    assert_eq!(config.defaults.zwift_score, Some(250));
    assert_eq!(config.defaults.category.as_deref(), Some("C"));
    assert_eq!(config.preferences.default_duration, Some(60));
    assert_eq!(config.preferences.default_tolerance, Some(15));
    assert_eq!(config.preferences.default_days, Some(2));
    let serialized = toml::to_string(&config).unwrap();
    for removed in [
        "weight_kg",
        "height_m",
        "ftp_watts",
        "[display]",
        "[import]",
    ] {
        assert!(!serialized.contains(removed));
    }
}

#[test]
fn test_example_config_matches_defaults() {
    let config: Config = toml::from_str(include_str!("../config.example.toml")).unwrap();
    assert_eq!(
        toml::to_string(&config).unwrap(),
        toml::to_string(&Config::default()).unwrap()
    );
}
