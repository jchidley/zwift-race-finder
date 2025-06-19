// ABOUTME: OCR configuration loading and management for different resolutions
// Replaces hardcoded regions with dynamic configuration from JSON files

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;

/// Region definition from config file
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegionConfig {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// Complete OCR configuration for a specific resolution
#[derive(Debug, Serialize, Deserialize)]
pub struct OcrConfig {
    pub version: String,
    pub resolution: String,
    pub zwift_version: String,
    pub created: String,
    pub regions: HashMap<String, RegionConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calibration_image: Option<String>,
}

/// OCR configuration manager
pub struct OcrConfigManager {
    configs_dir: PathBuf,
    current_config: Option<OcrConfig>,
}

impl OcrConfigManager {
    /// Create a new config manager
    pub fn new(configs_dir: PathBuf) -> Self {
        Self {
            configs_dir,
            current_config: None,
        }
    }

    /// Load configuration for a specific resolution
    pub fn load_for_resolution(&mut self, width: u32, height: u32) -> Result<()> {
        let resolution = format!("{}x{}", width, height);
        
        // Look for config files matching the resolution
        let entries = fs::read_dir(&self.configs_dir)
            .context("Failed to read ocr-configs directory")?;
        
        let mut best_match: Option<(PathBuf, String)> = None;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
                    // Check if filename starts with the resolution
                    if filename.starts_with(&resolution) {
                        // Extract version if present (e.g., "1920x1080_v1.67.0")
                        let version = filename.strip_prefix(&resolution)
                            .and_then(|s| s.strip_prefix('_'))
                            .unwrap_or("default")
                            .to_string();
                        
                        // For now, take the first match (could implement version selection later)
                        best_match = Some((path, version));
                        break;
                    }
                }
            }
        }
        
        if let Some((path, _version)) = best_match {
            self.load_from_file(&path)?;
            Ok(())
        } else {
            anyhow::bail!("No configuration found for resolution {}x{}", width, height)
        }
    }

    /// Load configuration from a specific file
    pub fn load_from_file(&mut self, path: &Path) -> Result<()> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {:?}", path))?;
        
        let config: OcrConfig = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {:?}", path))?;
        
        self.current_config = Some(config);
        Ok(())
    }

    /// Get a specific region configuration
    pub fn get_region(&self, name: &str) -> Option<&RegionConfig> {
        self.current_config.as_ref()
            .and_then(|c| c.regions.get(name))
    }

    /// Get all regions
    pub fn get_all_regions(&self) -> Option<&HashMap<String, RegionConfig>> {
        self.current_config.as_ref()
            .map(|c| &c.regions)
    }

    /// Check if a configuration is loaded
    pub fn has_config(&self) -> bool {
        self.current_config.is_some()
    }

    /// Get the current configuration metadata
    pub fn get_config_info(&self) -> Option<(String, String)> {
        self.current_config.as_ref()
            .map(|c| (c.resolution.clone(), c.zwift_version.clone()))
    }
}

/// Convert region config to tuple format for compatibility with existing code
impl RegionConfig {
    pub fn as_tuple(&self) -> (u32, u32, u32, u32) {
        (self.x, self.y, self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_test_config() -> OcrConfig {
        let mut regions = HashMap::new();
        regions.insert(
            "speed".to_string(),
            RegionConfig {
                x: 158,
                y: 75,
                width: 70,
                height: 35,
                note: Some("km/h value".to_string()),
            },
        );
        regions.insert(
            "power".to_string(),
            RegionConfig {
                x: 148,
                y: 28,
                width: 90,
                height: 50,
                note: Some("watts".to_string()),
            },
        );

        OcrConfig {
            version: "1.0.0".to_string(),
            resolution: "1920x1080".to_string(),
            zwift_version: "1.67.0".to_string(),
            created: "2025-01-12".to_string(),
            regions,
            notes: Some("Test config".to_string()),
            calibration_image: None,
        }
    }

    #[test]
    fn test_load_config_from_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("1920x1080_v1.67.0.json");
        
        let config = create_test_config();
        let json = serde_json::to_string_pretty(&config).unwrap();
        
        let mut file = fs::File::create(&config_path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
        
        let mut manager = OcrConfigManager::new(temp_dir.path().to_path_buf());
        manager.load_from_file(&config_path).unwrap();
        
        assert!(manager.has_config());
        
        let speed_region = manager.get_region("speed").unwrap();
        assert_eq!(speed_region.x, 158);
        assert_eq!(speed_region.y, 75);
    }

    #[test]
    fn test_load_for_resolution() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("1920x1080_v1.67.0.json");
        
        let config = create_test_config();
        let json = serde_json::to_string_pretty(&config).unwrap();
        
        let mut file = fs::File::create(&config_path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
        
        let mut manager = OcrConfigManager::new(temp_dir.path().to_path_buf());
        manager.load_for_resolution(1920, 1080).unwrap();
        
        let (resolution, version) = manager.get_config_info().unwrap();
        assert_eq!(resolution, "1920x1080");
        assert_eq!(version, "1.67.0");
    }

    #[test]
    fn test_region_as_tuple() {
        let region = RegionConfig {
            x: 100,
            y: 200,
            width: 300,
            height: 400,
            note: None,
        };
        
        assert_eq!(region.as_tuple(), (100, 200, 300, 400));
    }
}