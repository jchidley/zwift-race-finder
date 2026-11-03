//! Configuration management for Zwift Race Finder using TOML

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Main configuration structure for the application
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Config {
    /// Default rider settings
    #[serde(default)]
    pub defaults: Defaults,
    /// User preferences
    #[serde(default)]
    pub preferences: Preferences,
}

/// Default rider settings
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Defaults {
    /// Zwift Racing Score (0-999)
    pub zwift_score: Option<u32>,
    /// Racing category (A/B/C/D/E)
    pub category: Option<String>,
}

/// User preferences for searching
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Preferences {
    /// Default race duration in minutes
    pub default_duration: Option<u32>,
    /// Default tolerance in minutes
    pub default_tolerance: Option<u32>,
    /// Default days ahead to search
    pub default_days: Option<u32>,
}

/// Secrets loaded from environment variables
#[derive(Debug, Clone, Default)]
pub struct Secrets {
    /// ZwiftPower profile ID
    pub zwiftpower_profile_id: Option<String>,
    /// ZwiftPower session ID
    pub zwiftpower_session_id: Option<String>,
}

impl Default for Defaults {
    fn default() -> Self {
        Defaults {
            zwift_score: Some(195),
            category: Some("D".to_string()),
        }
    }
}

impl Default for Preferences {
    fn default() -> Self {
        Preferences {
            default_duration: Some(120),
            default_tolerance: Some(30),
            default_days: Some(1),
        }
    }
}

impl Config {
    /// Load configuration from files and environment
    pub fn load() -> Result<Self> {
        // Priority order for config:
        // 1. Environment variables (highest priority)
        // 2. Local config.toml
        // 3. ~/.config/zwift-race-finder/config.toml
        // 4. ~/.local/share/zwift-race-finder/config.toml (survives updates)
        // 5. Default values

        let config_paths = vec![
            PathBuf::from("config.toml"),
            dirs::config_dir()
                .map(|mut path| {
                    path.push("zwift-race-finder");
                    path.push("config.toml");
                    path
                })
                .unwrap_or_default(),
            dirs::data_dir()
                .map(|mut path| {
                    path.push("zwift-race-finder");
                    path.push("config.toml");
                    path
                })
                .unwrap_or_default(),
        ];

        // Load from file if found
        let mut config = Config::default();
        for path in config_paths {
            if path.exists() {
                let contents = fs::read_to_string(&path)?;
                config = toml::from_str(&contents)?;
                break;
            }
        }

        // Apply environment variable overrides
        config.apply_env_overrides();

        Ok(config)
    }

    /// Apply environment variable overrides to config
    fn apply_env_overrides(&mut self) {
        // Override defaults
        if let Ok(score) = std::env::var("ZWIFT_SCORE") {
            if let Ok(score) = score.parse::<u32>() {
                self.defaults.zwift_score = Some(score);
            }
        }

        if let Ok(cat) = std::env::var("ZWIFT_CATEGORY") {
            self.defaults.category = Some(cat);
        }

        // Override preferences
        if let Ok(duration) = std::env::var("ZWIFT_DEFAULT_DURATION") {
            if let Ok(duration) = duration.parse::<u32>() {
                self.preferences.default_duration = Some(duration);
            }
        }

        if let Ok(tolerance) = std::env::var("ZWIFT_DEFAULT_TOLERANCE") {
            if let Ok(tolerance) = tolerance.parse::<u32>() {
                self.preferences.default_tolerance = Some(tolerance);
            }
        }

        if let Ok(days) = std::env::var("ZWIFT_DEFAULT_DAYS") {
            if let Ok(days) = days.parse::<u32>() {
                self.preferences.default_days = Some(days);
            }
        }
    }
}

impl Secrets {
    /// Load secrets from environment variables
    pub fn load() -> Self {
        // Load secrets from environment variables (set via direnv/ak or equivalent)
        Secrets {
            zwiftpower_profile_id: std::env::var("ZWIFTPOWER_PROFILE_ID").ok(),
            zwiftpower_session_id: std::env::var("ZWIFTPOWER_SESSION_ID").ok(),
        }
    }
}

/// Combined configuration including secrets
pub struct FullConfig {
    /// Main configuration
    pub config: Config,
    /// Secret credentials
    pub secrets: Secrets,
}

impl Default for FullConfig {
    fn default() -> Self {
        FullConfig {
            config: Config::default(),
            secrets: Secrets::default(),
        }
    }
}

impl FullConfig {
    /// Load full configuration including secrets
    pub fn load() -> Result<Self> {
        Ok(FullConfig {
            config: Config::load()?,
            secrets: Secrets::load(),
        })
    }

    /// Get ZwiftPower profile ID
    pub fn zwiftpower_profile_id(&self) -> Option<&String> {
        self.secrets.zwiftpower_profile_id.as_ref()
    }

    /// Get ZwiftPower session ID
    pub fn zwiftpower_session_id(&self) -> Option<&String> {
        self.secrets.zwiftpower_session_id.as_ref()
    }

    /// Get default Zwift score
    pub fn default_zwift_score(&self) -> Option<u32> {
        self.config.defaults.zwift_score
    }

    /// Get default category
    pub fn default_category(&self) -> Option<&String> {
        self.config.defaults.category.as_ref()
    }
}
