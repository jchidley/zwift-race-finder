//! Configuration management for Zwift Race Finder using TOML

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Main configuration structure for the application
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    /// Default rider settings
    #[serde(default)]
    pub defaults: Defaults,
    /// Import configuration
    #[serde(default)]
    pub import: ImportConfig,
    /// User preferences
    #[serde(default)]
    pub preferences: Preferences,
    /// Display settings
    #[serde(default)]
    pub display: Display,
}

/// Default rider settings
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Defaults {
    /// Zwift Racing Score (0-999)
    pub zwift_score: Option<u32>,
    /// Racing category (A/B/C/D/E)
    pub category: Option<String>,
    /// Height in meters
    pub height_m: Option<f32>,
    /// Weight in kilograms
    pub weight_kg: Option<f32>,
    /// Functional Threshold Power in watts
    pub ftp_watts: Option<u32>,
}

/// Import configuration for data sources
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ImportConfig {
    /// Windows username for WSL path mapping
    pub windows_username: Option<String>,
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

/// Display settings
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Display {
    /// Whether to use colored output
    pub use_colors: Option<bool>,
    /// Whether to show debug output
    pub debug: Option<bool>,
}

/// Secrets loaded from environment or secure storage
#[derive(Debug, Clone)]
pub struct Secrets {
    /// ZwiftPower profile ID
    pub zwiftpower_profile_id: Option<String>,
    /// ZwiftPower session ID
    pub zwiftpower_session_id: Option<String>,
}

impl Default for Secrets {
    fn default() -> Self {
        Secrets {
            zwiftpower_profile_id: None,
            zwiftpower_session_id: None,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            defaults: Defaults::default(),
            import: ImportConfig::default(),
            preferences: Preferences::default(),
            display: Display::default(),
        }
    }
}

impl Default for Defaults {
    fn default() -> Self {
        Defaults {
            zwift_score: Some(195),
            category: Some("D".to_string()),
            height_m: Some(1.82),  // Jack's height
            weight_kg: Some(86.0), // Typical weight from race data
            ftp_watts: None,       // Will be set via config or calculated
        }
    }
}

impl Default for ImportConfig {
    fn default() -> Self {
        ImportConfig {
            windows_username: None,
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

impl Default for Display {
    fn default() -> Self {
        Display {
            use_colors: Some(true),
            debug: Some(false),
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
        
        if let Ok(weight) = std::env::var("ZWIFT_WEIGHT_KG") {
            if let Ok(weight) = weight.parse::<f32>() {
                self.defaults.weight_kg = Some(weight);
            }
        }
        
        if let Ok(height) = std::env::var("ZWIFT_HEIGHT_M") {
            if let Ok(height) = height.parse::<f32>() {
                self.defaults.height_m = Some(height);
            }
        }
        
        if let Ok(ftp) = std::env::var("ZWIFT_FTP_WATTS") {
            if let Ok(ftp) = ftp.parse::<u32>() {
                self.defaults.ftp_watts = Some(ftp);
            }
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
    
    /// Get the download path for imported files
    #[allow(dead_code)]
    pub fn get_download_path(&self) -> String {
        let username = self.import.windows_username.clone()
            .or_else(|| std::env::var("WINDOWS_USERNAME").ok())
            .unwrap_or_else(|| "YOUR_USERNAME".to_string());
            
        if username != "YOUR_USERNAME" {
            format!("/mnt/c/Users/{}/Downloads", username)
        } else {
            format!("{}/Downloads", std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string()))
        }
    }
    
    /// Save config to the user's data directory (survives updates)
    #[allow(dead_code)]
    pub fn save(&self) -> Result<()> {
        let config_dir = dirs::data_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))?
            .join("zwift-race-finder");
        
        // Create directory if it doesn't exist
        fs::create_dir_all(&config_dir)?;
        
        let config_path = config_dir.join("config.toml");
        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        
        eprintln!("Config saved to: {}", config_path.display());
        Ok(())
    }
    
    /// Get the path where user config would be saved
    #[allow(dead_code)]
    pub fn get_user_config_path() -> Option<PathBuf> {
        dirs::data_dir().map(|mut path| {
            path.push("zwift-race-finder");
            path.push("config.toml");
            path
        })
    }
}

impl Secrets {
    /// Load secrets from environment variables
    pub fn load() -> Self {
        // Load secrets from environment variables (set by Bitwarden)
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
    
    // Compatibility methods
    /// Get ZwiftPower profile ID
    #[allow(dead_code)]
    pub fn zwiftpower_profile_id(&self) -> Option<&String> {
        self.secrets.zwiftpower_profile_id.as_ref()
    }
    
    /// Get ZwiftPower session ID
    #[allow(dead_code)]
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
    
    /// Get Windows username for WSL
    #[allow(dead_code)]
    pub fn windows_username(&self) -> Option<&String> {
        self.config.import.windows_username.as_ref()
    }
    
    /// Get default weight in kg
    #[allow(dead_code)]
    pub fn default_weight_kg(&self) -> Option<f32> {
        self.config.defaults.weight_kg
    }
    
    /// Get default height in meters
    #[allow(dead_code)]
    pub fn default_height_m(&self) -> Option<f32> {
        self.config.defaults.height_m
    }
    
    /// Get default FTP in watts
    #[allow(dead_code)]
    pub fn default_ftp_watts(&self) -> Option<u32> {
        self.config.defaults.ftp_watts
    }
}
