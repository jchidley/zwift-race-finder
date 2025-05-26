// Database module for managing Zwift route data and race results
// Stores route information and Jack's actual race completion times

use anyhow::Result;
use rusqlite::{params, Connection, OptionalExtension};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RouteData {
    pub route_id: u32,
    pub distance_km: f64,
    pub elevation_m: u32,
    pub name: String,
    pub world: String,
    pub surface: String, // "road", "gravel", "mixed"
}

#[derive(Debug, Clone)]
pub struct RaceResult {
    pub id: Option<i64>,
    pub route_id: u32,
    pub event_name: String,
    pub actual_minutes: u32,
    pub zwift_score: u32,
    pub race_date: String,  // Stored as text in database
    pub notes: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RiderStats {
    pub height_m: f64,
    pub weight_kg: f64,
    pub ftp_watts: Option<u32>,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let db_path = get_database_path()?;
        let conn = Connection::open(db_path)?;
        
        let db = Database { conn };
        db.create_tables()?;
        db.seed_initial_data()?;
        
        Ok(db)
    }
    
    fn create_tables(&self) -> Result<()> {
        // Routes table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS routes (
                route_id INTEGER PRIMARY KEY,
                distance_km REAL NOT NULL,
                elevation_m INTEGER NOT NULL,
                name TEXT NOT NULL,
                world TEXT NOT NULL,
                surface TEXT NOT NULL DEFAULT 'road',
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        
        // Race results table for regression testing
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS race_results (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                route_id INTEGER NOT NULL,
                event_name TEXT NOT NULL,
                actual_minutes INTEGER NOT NULL,
                zwift_score INTEGER NOT NULL,
                race_date TIMESTAMP NOT NULL,
                notes TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (route_id) REFERENCES routes(route_id)
            )",
            [],
        )?;
        
        // Unknown routes table for data collection
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS unknown_routes (
                route_id INTEGER PRIMARY KEY,
                event_name TEXT NOT NULL,
                event_type TEXT,
                first_seen TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                times_seen INTEGER DEFAULT 1
            )",
            [],
        )?;
        
        // Rider stats table for physics calculations
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS rider_stats (
                id INTEGER PRIMARY KEY,
                height_m REAL DEFAULT 1.82,
                weight_kg REAL,
                ftp_watts INTEGER,
                updated_at TEXT DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        
        // Route discovery attempts table to avoid repeated searches
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS route_discovery_attempts (
                route_id INTEGER PRIMARY KEY,
                event_name TEXT NOT NULL,
                last_attempt TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                found BOOLEAN DEFAULT 0,
                distance_km REAL,
                elevation_m INTEGER,
                world TEXT,
                surface TEXT,
                route_name TEXT
            )",
            [],
        )?;
        
        Ok(())
    }
    
    fn seed_initial_data(&self) -> Result<()> {
        // Check if we already have data
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM routes",
            [],
            |row| row.get(0),
        )?;
        
        if count > 0 {
            return Ok(()); // Already seeded
        }
        
        // Seed with known routes
        let routes: Vec<(u32, f64, u32, &str, &str, &str)> = vec![
            (1258415487, 14.1, 59, "Bell Lap", "Crit City", "road"),
            (2143464829, 33.4, 170, "Watopia Flat Route", "Watopia", "road"),
            (2927651296, 67.5, 654, "Makuri Pretzel", "Makuri Islands", "road"),
            (3742187716, 24.5, 168, "Castle to Castle", "Makuri Islands", "road"),
            (2698009951, 22.9, 80, "Downtown Dolphin", "Crit City", "road"),
            (2663908549, 20.3, 1159, "Mt. Fuji", "Makuri Islands", "road"),
            (3368626651, 27.4, 223, "eRacing Course", "Various", "road"),
            (1656629976, 19.8, 142, "Ottawa TopSpeed", "Various", "road"),
            (2474227587, 100.0, 892, "KISS 100", "Watopia", "road"),
            (3395698268, 60.0, 543, "R3R 60km", "Various", "road"),
        ];
        
        for (id, dist, elev, name, world, surface) in routes {
            self.conn.execute(
                "INSERT OR IGNORE INTO routes (route_id, distance_km, elevation_m, name, world, surface) 
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![id, dist, elev, name, world, surface],
            )?;
        }
        
        Ok(())
    }
    
    pub fn get_route(&self, route_id: u32) -> Result<Option<RouteData>> {
        let mut stmt = self.conn.prepare(
            "SELECT route_id, distance_km, elevation_m, name, world, surface 
             FROM routes WHERE route_id = ?1"
        )?;
        
        let route = stmt.query_row([route_id], |row| {
            Ok(RouteData {
                route_id: row.get(0)?,
                distance_km: row.get(1)?,
                elevation_m: row.get(2)?,
                name: row.get(3)?,
                world: row.get(4)?,
                surface: row.get(5)?,
            })
        }).optional()?;
        
        Ok(route)
    }
    
    pub fn add_route(&self, route: &RouteData) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO routes (route_id, distance_km, elevation_m, name, world, surface) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                route.route_id,
                route.distance_km,
                route.elevation_m,
                route.name,
                route.world,
                route.surface
            ],
        )?;
        Ok(())
    }
    
    pub fn record_unknown_route(&self, route_id: u32, event_name: &str, event_type: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO unknown_routes (route_id, event_name, event_type) 
             VALUES (?1, ?2, ?3)
             ON CONFLICT(route_id) DO UPDATE SET 
                times_seen = times_seen + 1,
                event_name = ?2",
            params![route_id, event_name, event_type],
        )?;
        Ok(())
    }
    
    pub fn add_race_result(&self, result: &RaceResult) -> Result<()> {
        self.conn.execute(
            "INSERT INTO race_results (route_id, event_name, actual_minutes, zwift_score, race_date, notes) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                result.route_id,
                result.event_name,
                result.actual_minutes,
                result.zwift_score,
                result.race_date,
                result.notes
            ],
        )?;
        Ok(())
    }
    
    pub fn get_race_results_for_route(&self, route_id: u32, zwift_score: u32) -> Result<Vec<RaceResult>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, route_id, event_name, actual_minutes, zwift_score, race_date, notes 
             FROM race_results 
             WHERE route_id = ?1 AND zwift_score BETWEEN ?2 - 10 AND ?2 + 10
             ORDER BY race_date DESC"
        )?;
        
        let results = stmt.query_map(params![route_id, zwift_score], |row| {
            Ok(RaceResult {
                id: Some(row.get(0)?),
                route_id: row.get(1)?,
                event_name: row.get(2)?,
                actual_minutes: row.get(3)?,
                zwift_score: row.get(4)?,
                race_date: row.get(5)?,
                notes: row.get(6)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
        
        Ok(results)
    }
    
    pub fn get_average_race_time(&self, route_id: u32, _zwift_score: u32) -> Result<Option<u32>> {
        // Get average from recent results (last 3 months are most reliable)
        // But if no recent results, use all historical data
        let recent_result: Option<f64> = self.conn.query_row(
            "SELECT AVG(actual_minutes) 
             FROM race_results 
             WHERE route_id = ?1 
               AND race_date >= date('now', '-3 months')",
            [route_id],
            |row| row.get(0)
        ).optional()?;
        
        // If no recent results, fall back to all results
        let result = if recent_result.is_some() {
            recent_result
        } else {
            self.conn.query_row(
                "SELECT AVG(actual_minutes) 
                 FROM race_results 
                 WHERE route_id = ?1",
                [route_id],
                |row| row.get(0)
            ).optional()?
        };
        
        Ok(result.map(|avg| avg.round() as u32))
    }
    
    pub fn get_all_routes(&self) -> Result<Vec<RouteData>> {
        let mut stmt = self.conn.prepare(
            "SELECT route_id, distance_km, elevation_m, name, world, surface 
             FROM routes"
        )?;
        
        let routes = stmt.query_map([], |row| {
            Ok(RouteData {
                route_id: row.get(0)?,
                distance_km: row.get(1)?,
                elevation_m: row.get(2)?,
                name: row.get(3)?,
                world: row.get(4)?,
                surface: row.get(5)?,
            })
        })?.collect::<Result<Vec<_>, _>>()?;
        
        Ok(routes)
    }

    pub fn get_all_race_results(&self) -> Result<Vec<RaceResult>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, route_id, event_name, actual_minutes, zwift_score, race_date, notes 
             FROM race_results 
             ORDER BY race_date DESC"
        )?;
        
        let results = stmt.query_map([], |row| {
            // Handle zwift_score as either integer or real
            let zwift_score_raw: Result<u32, _> = row.get(4);
            let zwift_score = match zwift_score_raw {
                Ok(val) => val,
                Err(_) => {
                    // Try as f64 and convert
                    let val: f64 = row.get(4)?;
                    val.round() as u32
                }
            };
            
            Ok(RaceResult {
                id: row.get(0)?,
                route_id: row.get(1)?,
                event_name: row.get(2)?,
                actual_minutes: row.get(3)?,
                zwift_score,
                race_date: row.get(5)?,
                notes: row.get(6)?,
            })
        })?
        .collect::<Result<Vec<_>, rusqlite::Error>>()?;
        
        Ok(results)
    }
    
    pub fn get_unknown_routes(&self) -> Result<Vec<(u32, String, i32)>> {
        let mut stmt = self.conn.prepare(
            "SELECT route_id, event_name, times_seen 
             FROM unknown_routes 
             ORDER BY times_seen DESC, route_id"
        )?;
        
        let routes = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
        
        Ok(routes)
    }
    
    pub fn get_rider_stats(&self) -> Result<Option<RiderStats>> {
        let result = self.conn.query_row(
            "SELECT height_m, weight_kg, ftp_watts FROM rider_stats WHERE id = 1",
            [],
            |row| {
                Ok(RiderStats {
                    height_m: row.get(0)?,
                    weight_kg: row.get(1)?,
                    ftp_watts: row.get(2)?,
                })
            }
        ).optional()?;
        
        Ok(result)
    }
    
    /// Check if we've already tried to discover this route recently
    pub fn should_attempt_discovery(&self, route_id: u32) -> Result<bool> {
        let result: Option<i64> = self.conn.query_row(
            "SELECT COUNT(*) FROM route_discovery_attempts 
             WHERE route_id = ?1 
             AND datetime(last_attempt) > datetime('now', '-10 minutes')",
            params![route_id],
            |row| row.get(0),
        ).optional()?;
        
        // If no recent attempt found, we should try
        Ok(result.unwrap_or(0) == 0)
    }
    
    /// Record a discovery attempt
    pub fn record_discovery_attempt(&self, route_id: u32, event_name: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO route_discovery_attempts (route_id, event_name, found) 
             VALUES (?1, ?2, 0)
             ON CONFLICT(route_id) DO UPDATE SET 
                last_attempt = CURRENT_TIMESTAMP,
                event_name = ?2",
            params![route_id, event_name],
        )?;
        Ok(())
    }
    
    /// Save discovered route data
    pub fn save_discovered_route(&self, route_id: u32, distance_km: f64, elevation_m: u32, 
                                 world: &str, surface: &str, route_name: &str) -> Result<()> {
        // Update discovery attempts table
        self.conn.execute(
            "UPDATE route_discovery_attempts 
             SET found = 1, distance_km = ?2, elevation_m = ?3, 
                 world = ?4, surface = ?5, route_name = ?6
             WHERE route_id = ?1",
            params![route_id, distance_km, elevation_m, world, surface, route_name],
        )?;
        
        // Insert into routes table
        self.conn.execute(
            "INSERT INTO routes (route_id, distance_km, elevation_m, name, world, surface) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(route_id) DO UPDATE SET
                distance_km = ?2,
                elevation_m = ?3,
                name = ?4,
                world = ?5,
                surface = ?6",
            params![route_id, distance_km, elevation_m, route_name, world, surface],
        )?;
        
        Ok(())
    }
}

fn get_database_path() -> Result<PathBuf> {
    let mut data_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    data_dir.push("zwift-race-finder");
    std::fs::create_dir_all(&data_dir)?;
    data_dir.push("races.db");
    Ok(data_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_database_creation() {
        let db = Database::new().unwrap();
        
        // Test getting a known route
        let route = db.get_route(1258415487).unwrap();
        assert!(route.is_some());
        let route = route.unwrap();
        assert_eq!(route.name, "Bell Lap");
        assert_eq!(route.distance_km, 14.1);
    }
    
    #[test]
    fn test_race_result_storage() {
        let db = Database::new().unwrap();
        
        let result = RaceResult {
            id: None,
            route_id: 1258415487,
            event_name: "Test Race".to_string(),
            actual_minutes: 32,
            zwift_score: 195,
            race_date: Utc::now().format("%Y-%m-%d").to_string(),
            notes: Some("Test result".to_string()),
        };
        
        db.add_race_result(&result).unwrap();
        
        let results = db.get_race_results_for_route(1258415487, 195).unwrap();
        assert!(!results.is_empty());
        assert_eq!(results[0].actual_minutes, 32);
    }
}