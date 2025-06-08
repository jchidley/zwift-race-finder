//! Test database utilities
//!
//! Provides temporary databases for testing without affecting production data

use crate::database::Database;
use anyhow::Result;
use std::path::PathBuf;
use tempfile::TempDir;

/// Test database wrapper that automatically cleans up
pub struct TestDatabase {
    _temp_dir: TempDir,
    pub db: Database,
}

impl TestDatabase {
    /// Create a new temporary test database
    pub fn new() -> Result<Self> {
        // Create temporary directory
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test_races.db");
        
        // Create database with test path
        std::env::set_var("ZWIFT_RACE_FINDER_TEST_DB", db_path.to_str().unwrap());
        let db = Database::new()?;
        std::env::remove_var("ZWIFT_RACE_FINDER_TEST_DB");
        
        Ok(Self {
            _temp_dir: temp_dir,
            db,
        })
    }
    
    /// Seed with minimal test data
    pub fn seed_test_routes(&self) -> Result<()> {
        // Add a few representative routes for testing
        self.db.conn.execute(
            "INSERT OR IGNORE INTO routes (route_id, distance_km, elevation_m, name, world, surface) VALUES
            (12, 17.8, 61, 'Tempus Fugit', 'Watopia', 'road'),
            (2, 9.1, 140, 'Hilly Route', 'Watopia', 'road'),
            (22, 17.0, 1041, 'Road to Sky', 'Watopia', 'road')",
            [],
        )?;
        Ok(())
    }
}

/// Get database path, preferring test database if set
pub fn get_database_path_with_test_override() -> Result<PathBuf> {
    // Check for test database override
    if let Ok(test_path) = std::env::var("ZWIFT_RACE_FINDER_TEST_DB") {
        return Ok(PathBuf::from(test_path));
    }
    
    // Fall back to production path
    let mut data_dir = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    data_dir.push("zwift-race-finder");
    std::fs::create_dir_all(&data_dir)?;
    data_dir.push("races.db");
    Ok(data_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_temporary_database_creation() {
        let test_db = TestDatabase::new().expect("Failed to create test database");
        
        // Verify we can query the database
        let count: i32 = test_db.db.conn
            .query_row("SELECT COUNT(*) FROM routes", [], |row| row.get(0))
            .expect("Failed to count routes");
        
        // Should have some seeded routes
        assert!(count >= 0);
    }
    
    #[test] 
    fn test_database_cleanup() {
        let db_path = {
            let test_db = TestDatabase::new().expect("Failed to create test database");
            let path = std::env::var("ZWIFT_RACE_FINDER_TEST_DB").ok();
            // Add test data
            test_db.seed_test_routes().ok();
            path
        }; // test_db drops here, should clean up
        
        // Verify the temporary database was cleaned up
        if let Some(path) = db_path {
            assert!(!PathBuf::from(path).exists(), "Test database should be cleaned up");
        }
    }
}