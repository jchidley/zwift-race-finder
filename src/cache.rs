//! Cache-related functionality for storing user stats

use anyhow::Result;
use chrono::{Utc, DateTime};
use std::fs;
use std::path::PathBuf;
use crate::models::{UserStats, CachedStats};

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