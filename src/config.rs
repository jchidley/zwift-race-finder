use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Config {
    #[serde(default)]
    pub defaults: Defaults,
    #[serde(default)]
    pub preferences: Preferences,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Defaults {
    pub zwift_score: Option<u32>,
    pub category: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Preferences {
    pub default_duration: Option<u32>,
    pub default_tolerance: Option<u32>,
    pub default_days: Option<u32>,
}

#[derive(Debug, Clone, Default)]
pub struct Secrets {
    pub zwiftpower_profile_id: Option<String>,
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
    pub fn load() -> Result<Self> {

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

        let mut config = Config::default();
        for path in config_paths {
            if path.exists() {
                let contents = fs::read_to_string(&path)?;
                config = toml::from_str(&contents)?;
                break;
            }
        }

        config.apply_env_overrides();

        Ok(config)
    }

    fn apply_env_overrides(&mut self) {
        if let Ok(score) = std::env::var("ZWIFT_SCORE") {
            if let Ok(score) = score.parse::<u32>() {
                self.defaults.zwift_score = Some(score);
            }
        }

        if let Ok(cat) = std::env::var("ZWIFT_CATEGORY") {
            self.defaults.category = Some(cat);
        }

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
    pub fn load() -> Self {
        Secrets {
            zwiftpower_profile_id: std::env::var("ZWIFTPOWER_PROFILE_ID").ok(),
            zwiftpower_session_id: std::env::var("ZWIFTPOWER_SESSION_ID").ok(),
        }
    }
}

pub struct FullConfig {
    pub config: Config,
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
    pub fn load() -> Result<Self> {
        Ok(FullConfig {
            config: Config::load()?,
            secrets: Secrets::load(),
        })
    }

    pub fn zwiftpower_profile_id(&self) -> Option<&String> {
        self.secrets.zwiftpower_profile_id.as_ref()
    }

    pub fn zwiftpower_session_id(&self) -> Option<&String> {
        self.secrets.zwiftpower_session_id.as_ref()
    }

    pub fn default_zwift_score(&self) -> Option<u32> {
        self.config.defaults.zwift_score
    }

    pub fn default_category(&self) -> Option<&String> {
        self.config.defaults.category.as_ref()
    }
}
