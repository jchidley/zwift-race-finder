//! Cache-related functionality for storing user stats

use crate::models::{CachedStats, UserStats};
use anyhow::Result;
use chrono::Utc;
use std::fs;
use std::path::PathBuf;

pub fn get_cache_file() -> Result<PathBuf> {
    let mut cache_dir = dirs::cache_dir().unwrap_or_else(|| PathBuf::from("."));
    cache_dir.push("zwift-race-finder");
    fs::create_dir_all(&cache_dir)?;
    cache_dir.push("user_stats.json");
    Ok(cache_dir)
}

pub fn load_cached_stats() -> Result<Option<UserStats>> {
    let cache_file = get_cache_file()?;

    if !cache_file.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(cache_file)?;
    let cached: CachedStats = serde_json::from_str(&content)?;

    // Use cache if it's less than 24 hours old
    let age = Utc::now().signed_duration_since(cached.cached_at);
    if age.num_hours() < 24 {
        Ok(Some(cached.stats))
    } else {
        Ok(None)
    }
}

pub fn save_cached_stats(stats: &UserStats) -> Result<()> {
    let cache_file = get_cache_file()?;
    let cached = CachedStats {
        stats: stats.clone(),
        cached_at: Utc::now(),
    };

    let content = serde_json::to_string_pretty(&cached)?;
    fs::write(cache_file, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_get_cache_file() {
        let cache_file = get_cache_file().unwrap();
        assert!(cache_file.to_string_lossy().contains("zwift-race-finder"));
        assert!(cache_file.to_string_lossy().contains("user_stats.json"));
    }

    #[test]
    fn test_load_and_save_cached_stats() {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new().unwrap();
        let cache_file = temp_dir
            .path()
            .join("zwift-race-finder")
            .join("user_stats.json");

        // Override the cache directory for testing
        std::env::set_var("XDG_CACHE_HOME", temp_dir.path());

        // Test loading when cache doesn't exist
        let result = load_cached_stats().unwrap();
        assert!(
            result.is_none(),
            "Should return None when cache doesn't exist"
        );

        // Create test stats
        let test_stats = UserStats {
            zwift_score: 250,
            category: "C".to_string(),
            username: "TestUser".to_string(),
        };

        // Save stats
        save_cached_stats(&test_stats).unwrap();
        assert!(cache_file.exists(), "Cache file should be created");

        // Load stats back
        let loaded = load_cached_stats().unwrap();
        assert!(loaded.is_some(), "Should load cached stats");
        let loaded_stats = loaded.unwrap();
        assert_eq!(loaded_stats.zwift_score, 250);
        assert_eq!(loaded_stats.category, "C");
        assert_eq!(loaded_stats.username, "TestUser");

        // Test cache expiration by modifying the file
        let content = fs::read_to_string(&cache_file).unwrap();
        let mut cached: CachedStats = serde_json::from_str(&content).unwrap();
        cached.cached_at = Utc::now() - chrono::Duration::hours(25); // Make it 25 hours old
        let expired_content = serde_json::to_string(&cached).unwrap();
        fs::write(&cache_file, expired_content).unwrap();

        // Should return None for expired cache
        let expired_result = load_cached_stats().unwrap();
        assert!(
            expired_result.is_none(),
            "Should return None for expired cache"
        );

        // Clean up
        std::env::remove_var("XDG_CACHE_HOME");
    }
}
