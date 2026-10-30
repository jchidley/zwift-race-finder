//! Database module for managing Zwift route data and race results
//!
//! Stores route information and actual race completion times

use anyhow::{ensure, Result};
use colored::Colorize;
use rusqlite::{params, Connection, OptionalExtension};
use std::path::{Path, PathBuf};

#[path = "database_history.rs"]
mod history_storage;
pub use history_storage::{HistoryImportSummary, HistoryObservation};

/// Route data including distance, elevation, and lead-in information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RouteData {
    /// Unique route identifier
    #[allow(dead_code)]
    pub route_id: u32,
    /// Route distance in kilometers
    pub distance_km: f64,
    /// Total elevation gain in meters
    pub elevation_m: Option<f64>,
    /// Route name
    pub name: String,
    /// Zwift world (Watopia, London, etc.)
    pub world: String,
    /// Surface type (road, gravel, mixed)
    pub surface: String,
    /// Lead-in distance in kilometers
    pub lead_in_distance_km: Option<f64>,
    /// Lead-in elevation in meters
    #[allow(dead_code)]
    pub lead_in_elevation_m: Option<f64>,
    /// Lead-in distance for free ride mode
    #[allow(dead_code)]
    pub lead_in_distance_free_ride_km: Option<f64>,
    /// Lead-in elevation for free ride mode
    #[allow(dead_code)]
    pub lead_in_elevation_free_ride_m: Option<f64>,
    /// Lead-in distance for meetups
    #[allow(dead_code)]
    pub lead_in_distance_meetups_km: Option<f64>,
    /// Lead-in elevation for meetups
    #[allow(dead_code)]
    pub lead_in_elevation_meetups_m: Option<f64>,
    /// URL slug for WhatsOnZwift
    pub slug: Option<String>,
}

/// Local client provenance. Lower rank is more authoritative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
#[allow(dead_code)] // Used by the importer; this module also builds privately in the CLI.
pub enum ClientSourceKind {
    ClientCache,
    ExtractedClient,
}

#[allow(dead_code)] // Used by the importer; this module also builds privately in the CLI.
impl ClientSourceKind {
    fn rank(self) -> u8 {
        match self {
            Self::ClientCache => 0,
            Self::ExtractedClient => 1,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(dead_code)] // Used by the importer; this module also builds privately in the CLI.
pub struct ClientRouteSource {
    pub kind: ClientSourceKind,
    pub path: String,
    pub sha256: String,
    pub entry: String,
}

/// Normalized metadata plus original source identity and availability flags.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(dead_code)] // Used by the importer; this module also builds privately in the CLI.
pub struct ClientRouteRecord {
    #[serde(flatten)]
    pub route: RouteData,
    pub source: ClientRouteSource,
    pub event_only: Option<String>,
    pub sport_type: Option<String>,
    pub supported_laps: Option<String>,
}

#[derive(Debug, Default)]
#[allow(dead_code)] // Used by the importer; this module also builds privately in the CLI.
pub struct ClientImportSummary {
    pub written: usize,
    pub skipped: usize,
}

#[allow(dead_code)] // Used by the importer; this module also builds privately in the CLI.
impl ClientRouteRecord {
    pub fn validate(&self) -> Result<()> {
        let route = &self.route;
        ensure!(
            !route.name.trim().is_empty() && !route.world.trim().is_empty(),
            "Route {} has no name/world",
            route.route_id
        );
        ensure!(
            route.distance_km.is_finite() && route.distance_km > 0.0,
            "Invalid distance for route {}",
            route.route_id
        );
        for value in [
            route.lead_in_distance_km,
            route.lead_in_distance_free_ride_km,
            route.lead_in_distance_meetups_km,
            route.elevation_m,
            route.lead_in_elevation_m,
            route.lead_in_elevation_free_ride_m,
            route.lead_in_elevation_meetups_m,
        ]
        .into_iter()
        .flatten()
        {
            ensure!(
                value.is_finite() && value >= 0.0 && value <= f64::from(u32::MAX),
                "Invalid route/lead-in measurement for {}",
                route.route_id
            );
        }
        ensure!(
            !self.source.path.is_empty()
                && !self.source.entry.is_empty()
                && self.source.sha256.len() == 64
                && self.source.sha256.bytes().all(|b| b.is_ascii_hexdigit()),
            "Missing or invalid source provenance for route {}",
            route.route_id
        );
        Ok(())
    }
}

/// Race result with actual completion time
#[derive(Debug, Clone)]
pub struct RaceResult {
    /// Database ID
    #[allow(dead_code)]
    pub id: Option<i64>,
    /// Route ID
    pub route_id: u32,
    /// Event name
    pub event_name: String,
    /// Actual race duration in minutes
    pub actual_minutes: u32,
    /// Zwift Racing Score at time of race
    pub zwift_score: Option<u32>,
    /// Race date (YYYY-MM-DD format)
    pub race_date: String,
    /// Optional notes
    pub notes: Option<String>,
}

/// Rider physical stats for physics calculations
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RiderStats {
    /// Height in meters
    #[allow(dead_code)]
    pub height_m: f64,
    /// Weight in kilograms
    pub weight_kg: f64,
    /// Functional Threshold Power in watts
    #[allow(dead_code)]
    pub ftp_watts: Option<u32>,
}

/// Database connection and operations
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Create a new database connection
    pub fn new() -> Result<Self> { Self::open(&get_database_path()?) }

    pub fn default_path() -> Result<PathBuf> { get_database_path() }

    pub fn open(db_path: &Path) -> Result<Self> {
        if let Some(parent) = db_path.parent().filter(|p| !p.as_os_str().is_empty()) { std::fs::create_dir_all(parent)?; }
        let conn = Connection::open(&db_path).map_err(|e| {
            eprintln!("\n{} Failed to open database", "❌ Error:".red().bold());
            eprintln!("   Path: {}", db_path.display());
            eprintln!("   {}", e);
            eprintln!("\n{}", "💡 Suggestions:".yellow());
            eprintln!("   • Check if the directory exists and is writable");
            eprintln!(
                "   • Try: mkdir -p {}",
                db_path.parent().unwrap_or(&db_path).display()
            );
            eprintln!();
            e
        })?;

        Self::from_connection(conn)
    }

    /// Clone existing state in memory; preview never initializes or migrates the target.
    pub fn preview(path: &Path) -> Result<Self> {
        let mut conn = Connection::open_in_memory()?;
        if path.exists() {
            let source = Connection::open_with_flags(path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)?;
            rusqlite::backup::Backup::new(&source, &mut conn)?.run_to_completion(100, std::time::Duration::from_millis(10), None)?;
        }
        Self::from_connection(conn)
    }

    fn from_connection(conn: Connection) -> Result<Self> {
        conn.execute_batch("PRAGMA foreign_keys=ON")?;
        let db = Database { conn };
        {
            let tx = db.conn.unchecked_transaction()?;
            db.create_tables()?;
            db.migrate_elevation_knowledge()?;
            if history_storage::migrate(&db.conn)? {
                ensure!(db.conn.prepare("PRAGMA foreign_key_check")?.query([])?.next()?.is_none(), "Foreign-key violations prevent migration; inspect a backup before proceeding");
            }
            tx.commit()?;
        }
        Ok(db)
    }

    fn create_tables(&self) -> Result<()> {
        // Routes table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS routes (
                route_id INTEGER PRIMARY KEY,
                distance_km REAL NOT NULL,
                elevation_m REAL NOT NULL,
                name TEXT NOT NULL,
                world TEXT NOT NULL,
                surface TEXT NOT NULL DEFAULT 'road',
                lead_in_distance_km REAL DEFAULT 0.0,
                lead_in_elevation_m REAL DEFAULT 0,
                lead_in_distance_free_ride_km REAL,
                lead_in_elevation_free_ride_m INTEGER,
                lead_in_distance_meetups_km REAL,
                lead_in_elevation_meetups_m INTEGER,
                slug TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        history_storage::create_table(&self.conn, "race_results")?;

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

        // Route aliases: maps alternative route IDs (used by Zwift API for
        // event-only variants) to canonical route IDs in our routes table.
        // Zwift often uses different internal route IDs for the same physical
        // route depending on whether it's a free-ride or event-only context.
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS route_aliases (
                alias_route_id INTEGER PRIMARY KEY,
                canonical_route_id INTEGER NOT NULL,
                notes TEXT,
                FOREIGN KEY (canonical_route_id) REFERENCES routes(route_id)
            )",
            [],
        )?;

        Ok(())
    }

    /// Add knowledge flags without rebuilding tables or altering rider history.
    /// NULL flags preserve legacy positive values; legacy zero means unknown.
    /// Initialization owns the surrounding transaction.
    fn migrate_elevation_knowledge(&self) -> Result<()> {
        let columns: Vec<String> = self.conn
            .prepare("PRAGMA table_info(routes)")?
            .query_map([], |row| row.get(1))?
            .collect::<Result<_, _>>()?;
        for column in ["elevation_known", "lead_in_elevation_known"] {
            if !columns.iter().any(|name| name == column) {
                self.conn.execute(
                    &format!(
                        "ALTER TABLE routes ADD COLUMN {column} INTEGER CHECK ({column} IN (0, 1))"
                    ),
                    [],
                )?;
            }
        }
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS route_sources (
                route_id INTEGER PRIMARY KEY REFERENCES routes(route_id),
                metadata TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    /// Whether any route metadata has been explicitly imported or retained.
    pub fn has_routes(&self) -> Result<bool> {
        Ok(self.conn.query_row("SELECT EXISTS(SELECT 1 FROM routes)", [], |row| row.get(0))?)
    }

    /// Get route data by ID, falling back to alias lookup
    ///
    /// If the route_id isn't found directly, checks the route_aliases table
    /// for a mapping to a canonical route_id. This handles Zwift's use of
    /// different internal IDs for the same physical route (event-only vs
    /// free-ride variants).
    pub fn get_route(&self, route_id: u32) -> Result<Option<RouteData>> {
        // Direct lookup first
        if let Some(route) = self.get_route_direct(route_id)? {
            return Ok(Some(route));
        }

        // Try alias lookup
        let canonical_id: Option<u32> = self
            .conn
            .query_row(
                "SELECT canonical_route_id FROM route_aliases WHERE alias_route_id = ?1",
                [route_id],
                |row| row.get(0),
            )
            .optional()?;

        if let Some(canon_id) = canonical_id {
            return self.get_route_direct(canon_id);
        }

        Ok(None)
    }

    /// Get route data by direct ID (no alias fallback)
    pub fn get_route_direct(&self, route_id: u32) -> Result<Option<RouteData>> {
        let mut stmt = self.conn.prepare(
            "SELECT route_id, distance_km,
                    CASE WHEN elevation_known = 1 OR (elevation_known IS NULL AND elevation_m > 0)
                         THEN elevation_m END, name, world, surface,
                    lead_in_distance_km,
                    CASE WHEN lead_in_elevation_known = 1 OR (lead_in_elevation_known IS NULL AND lead_in_elevation_m > 0)
                         THEN lead_in_elevation_m END,
                    lead_in_distance_free_ride_km, lead_in_elevation_free_ride_m,
                    lead_in_distance_meetups_km, lead_in_elevation_meetups_m, slug
             FROM routes WHERE route_id = ?1",
        )?;

        let route = stmt
            .query_row([route_id], |row| {
                Ok(RouteData {
                    route_id: row.get(0)?,
                    distance_km: row.get(1)?,
                    elevation_m: row.get(2)?,
                    name: row.get(3)?,
                    world: row.get(4)?,
                    surface: row.get(5)?,
                    lead_in_distance_km: row.get(6)?,
                    lead_in_elevation_m: row.get(7)?,
                    lead_in_distance_free_ride_km: row.get(8).ok(),
                    lead_in_elevation_free_ride_m: row.get(9).ok(),
                    lead_in_distance_meetups_km: row.get(10).ok(),
                    lead_in_elevation_meetups_m: row.get(11).ok(),
                    slug: row.get(12).ok(),
                })
            })
            .optional()?;

        Ok(route)
    }

    /// Add a new route to the database
    #[allow(dead_code)]
    pub fn has_client_route(&self, route_id: u32) -> Result<bool> {
        Ok(self.conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM route_sources WHERE route_id=?1)",
            params![route_id],
            |row| row.get(0),
        )?)
    }

    /// Import all records atomically. Client cache beats extracted XML; either
    /// replaces legacy/community metadata. Identical imports do not rewrite rows.
    #[allow(dead_code)] // Called by the separate import_client_routes binary through the library.
    pub fn import_client_routes(
        &self,
        records: &[ClientRouteRecord],
        dry_run: bool,
    ) -> Result<ClientImportSummary> {
        let mut ids = std::collections::HashSet::new();
        for record in records {
            record.validate()?;
            ensure!(
                ids.insert(record.route.route_id),
                "Duplicate client route ID {}",
                record.route.route_id
            );
        }
        let tx = self.conn.unchecked_transaction()?;
        let mut summary = ClientImportSummary::default();
        for record in records {
            let metadata = serde_json::to_string(record)?;
            let previous: Option<String> = self
                .conn
                .query_row(
                    "SELECT metadata FROM route_sources WHERE route_id=?1",
                    params![record.route.route_id],
                    |row| row.get(0),
                )
                .optional()?;
            if let Some(previous) = previous {
                let previous_record: ClientRouteRecord = serde_json::from_str(&previous)?;
                if previous == metadata
                    || previous_record.source.kind.rank() < record.source.kind.rank()
                {
                    summary.skipped += 1;
                    continue;
                }
            }
            if !dry_run {
                self.write_route(&record.route)?;
                self.conn.execute(
                    "INSERT INTO route_sources (route_id, metadata) VALUES (?1, ?2)
                    ON CONFLICT(route_id) DO UPDATE SET metadata=excluded.metadata",
                    params![record.route.route_id, metadata],
                )?;
            }
            summary.written += 1;
        }
        tx.commit()?;
        Ok(summary)
    }

    /// Insert explicit legacy/test metadata; never overwrite client provenance.
    /// Production route ingestion uses `import_client_routes`, not this helper.
    pub fn add_route(&self, route: &RouteData) -> Result<()> {
        ensure!(
            !self.has_client_route(route.route_id)?,
            "Route {} is client-derived; refresh it with import_client_routes",
            route.route_id
        );
        self.write_route(route)
    }

    fn write_route(&self, route: &RouteData) -> Result<()> {
        self.conn.execute(
            "INSERT INTO routes (route_id, distance_km, elevation_m, name, world, surface,
                                           lead_in_distance_km, lead_in_elevation_m,
                                           lead_in_distance_free_ride_km, lead_in_elevation_free_ride_m,
                                           lead_in_distance_meetups_km, lead_in_elevation_meetups_m, slug,
                                           elevation_known, lead_in_elevation_known)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)
             ON CONFLICT(route_id) DO UPDATE SET
                distance_km=excluded.distance_km, elevation_m=excluded.elevation_m,
                name=excluded.name, world=excluded.world, surface=excluded.surface,
                lead_in_distance_km=excluded.lead_in_distance_km,
                lead_in_elevation_m=excluded.lead_in_elevation_m,
                lead_in_distance_free_ride_km=excluded.lead_in_distance_free_ride_km,
                lead_in_elevation_free_ride_m=excluded.lead_in_elevation_free_ride_m,
                lead_in_distance_meetups_km=excluded.lead_in_distance_meetups_km,
                lead_in_elevation_meetups_m=excluded.lead_in_elevation_meetups_m,
                slug=excluded.slug, elevation_known=excluded.elevation_known,
                lead_in_elevation_known=excluded.lead_in_elevation_known,
                updated_at=CURRENT_TIMESTAMP",
            params![
                route.route_id,
                route.distance_km,
                route.elevation_m.unwrap_or(0.0),
                route.name,
                route.world,
                route.surface,
                route.lead_in_distance_km,
                route.lead_in_elevation_m.unwrap_or(0.0),
                route.lead_in_distance_free_ride_km,
                route.lead_in_elevation_free_ride_m,
                route.lead_in_distance_meetups_km,
                route.lead_in_elevation_meetups_m,
                route.slug,
                route.elevation_m.is_some(),
                route.lead_in_elevation_m.is_some()
            ],
        )?;
        Ok(())
    }

    /// Get route data by name
    #[allow(dead_code)]
    pub fn get_route_by_name(&self, name: &str) -> Result<Option<RouteData>> {
        let mut stmt = self.conn.prepare(
            "SELECT route_id, distance_km,
                    CASE WHEN elevation_known = 1 OR (elevation_known IS NULL AND elevation_m > 0)
                         THEN elevation_m END, name, world, surface,
                    lead_in_distance_km,
                    CASE WHEN lead_in_elevation_known = 1 OR (lead_in_elevation_known IS NULL AND lead_in_elevation_m > 0)
                         THEN lead_in_elevation_m END,
                    lead_in_distance_free_ride_km, lead_in_elevation_free_ride_m,
                    lead_in_distance_meetups_km, lead_in_elevation_meetups_m, slug
             FROM routes 
             WHERE LOWER(name) = LOWER(?1)
             LIMIT 1",
        )?;

        let route = stmt
            .query_row([name], |row| {
                Ok(RouteData {
                    route_id: row.get(0)?,
                    distance_km: row.get(1)?,
                    elevation_m: row.get(2)?,
                    name: row.get(3)?,
                    world: row.get(4)?,
                    surface: row.get(5)?,
                    lead_in_distance_km: row.get(6)?,
                    lead_in_elevation_m: row.get(7)?,
                    lead_in_distance_free_ride_km: row.get(8).ok(),
                    lead_in_elevation_free_ride_m: row.get(9).ok(),
                    lead_in_distance_meetups_km: row.get(10).ok(),
                    lead_in_elevation_meetups_m: row.get(11).ok(),
                    slug: row.get(12).ok(),
                })
            })
            .optional()?;

        Ok(route)
    }

    /// Record an unknown route for future investigation
    pub fn record_unknown_route(
        &self,
        route_id: u32,
        event_name: &str,
        event_type: &str,
    ) -> Result<()> {
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


    /// Get race results for a specific route
    #[allow(dead_code)]
    pub fn get_race_results_for_route(
        &self,
        route_id: u32,
        zwift_score: u32,
    ) -> Result<Vec<RaceResult>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, route_id, event_name, actual_minutes, zwift_score, race_date, notes 
             FROM race_results 
             WHERE route_id = ?1 AND zwift_score BETWEEN ?2 - 10 AND ?2 + 10
             ORDER BY race_date DESC",
        )?;

        let results = stmt
            .query_map(params![route_id, zwift_score], |row| {
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

    /// Get average race time for a route
    #[allow(dead_code)]
    pub fn get_average_race_time(&self, route_id: u32, _zwift_score: u32) -> Result<Option<u32>> {
        // Get average from recent results (last 3 months are most reliable)
        // But if no recent results, use all historical data
        let recent_result: Option<f64> = self
            .conn
            .query_row(
                "SELECT AVG(actual_minutes) 
             FROM race_results 
             WHERE route_id = ?1 
               AND race_date >= date('now', '-3 months')",
                [route_id],
                |row| row.get(0),
            )
            .optional()?;

        // If no recent results, fall back to all results
        let result = if recent_result.is_some() {
            recent_result
        } else {
            self.conn
                .query_row(
                    "SELECT AVG(actual_minutes) 
                 FROM race_results 
                 WHERE route_id = ?1",
                    [route_id],
                    |row| row.get(0),
                )
                .optional()?
        };

        Ok(result.map(|avg| avg.round() as u32))
    }

    /// Get all routes from the database
    #[allow(dead_code)]
    pub fn get_all_routes(&self) -> Result<Vec<RouteData>> {
        let mut stmt = self.conn.prepare(
            "SELECT route_id, distance_km,
                    CASE WHEN elevation_known = 1 OR (elevation_known IS NULL AND elevation_m > 0)
                         THEN elevation_m END, name, world, surface,
                    lead_in_distance_km,
                    CASE WHEN lead_in_elevation_known = 1 OR (lead_in_elevation_known IS NULL AND lead_in_elevation_m > 0)
                         THEN lead_in_elevation_m END,
                    lead_in_distance_free_ride_km, lead_in_elevation_free_ride_m,
                    lead_in_distance_meetups_km, lead_in_elevation_meetups_m, slug
             FROM routes",
        )?;

        let routes = stmt
            .query_map([], |row| {
                Ok(RouteData {
                    route_id: row.get(0)?,
                    distance_km: row.get(1)?,
                    elevation_m: row.get(2)?,
                    name: row.get(3)?,
                    world: row.get(4)?,
                    surface: row.get(5)?,
                    lead_in_distance_km: row.get(6)?,
                    lead_in_elevation_m: row.get(7)?,
                    lead_in_distance_free_ride_km: row.get(8).ok(),
                    lead_in_elevation_free_ride_m: row.get(9).ok(),
                    lead_in_distance_meetups_km: row.get(10).ok(),
                    lead_in_elevation_meetups_m: row.get(11).ok(),
                    slug: row.get(12).ok(),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(routes)
    }

    /// Get all race results
    pub fn get_all_race_results(&self) -> Result<Vec<RaceResult>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, route_id, event_name, actual_minutes, zwift_score, race_date, notes 
             FROM race_results 
             ORDER BY race_date DESC",
        )?;

        let results = stmt
            .query_map([], |row| {
                let zwift_score = row.get::<_, Option<f64>>(4)?.map(|v| v.round() as u32);

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

    /// Get unknown routes that need mapping
    pub fn get_unknown_routes(&self) -> Result<Vec<(u32, String, i32)>> {
        let mut stmt = self.conn.prepare(
            "SELECT route_id, event_name, times_seen 
             FROM unknown_routes 
             ORDER BY times_seen DESC, route_id",
        )?;

        let routes = stmt
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(routes)
    }

    /// Get rider stats from the database
    #[allow(dead_code)]
    pub fn get_rider_stats(&self) -> Result<Option<RiderStats>> {
        let result = self
            .conn
            .query_row(
                "SELECT height_m, weight_kg, ftp_watts FROM rider_stats WHERE id = 1",
                [],
                |row| {
                    Ok(RiderStats {
                        height_m: row.get(0)?,
                        weight_kg: row.get(1)?,
                        ftp_watts: row.get(2)?,
                    })
                },
            )
            .optional()?;

        Ok(result)
    }

    /// Get lap count for multi-lap events
    #[allow(dead_code)]
    pub fn get_multi_lap_info(&self, event_name: &str) -> Result<Option<u32>> {
        // Try exact match first
        let result = self
            .conn
            .query_row(
                "SELECT lap_count FROM multi_lap_events WHERE event_name_pattern = ?1",
                params![event_name],
                |row| row.get(0),
            )
            .optional()?;

        if result.is_some() {
            return Ok(result);
        }

        // Try pattern match - check if event name contains the pattern
        let result = self
            .conn
            .query_row(
                "SELECT lap_count FROM multi_lap_events 
             WHERE ?1 LIKE '%' || event_name_pattern || '%'
             LIMIT 1",
                params![event_name],
                |row| row.get(0),
            )
            .optional()?;

        Ok(result)
    }





    /// Get all routes with basic info (for validation)
    #[allow(dead_code)]
    pub fn get_all_routes_basic(&self) -> Result<Vec<(u32, String, f64, u32)>> {
        let mut stmt = self.conn.prepare(
            "SELECT route_id, name, distance_km, CAST(ROUND(elevation_m) AS INTEGER) FROM routes",
        )?;

        let routes = stmt
            .query_map([], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(routes)
    }

    /// Get race results for validation
    #[allow(dead_code)]
    pub fn get_race_results_for_validation(&self) -> Result<Vec<(u32, String, u32, u32)>> {
        let mut stmt = self.conn.prepare(
            "SELECT route_id, event_name, actual_minutes, zwift_score 
             FROM race_results 
             WHERE route_id != 9999 AND zwift_score IS NOT NULL",
        )?;

        let results = stmt
            .query_map([], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(results)
    }
}

fn get_database_path() -> Result<PathBuf> {
    let mut data_dir = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    data_dir.push("zwift-race-finder");
    data_dir.push("races.db");
    Ok(data_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_creation() {
        let db = Database { conn: Connection::open_in_memory().unwrap() };
        db.create_tables().unwrap();
        db.migrate_elevation_knowledge().unwrap();
        assert_eq!(db.conn.query_row("SELECT COUNT(*) FROM routes", [], |r| r.get::<_, u32>(0)).unwrap(), 0);
        assert!(db.get_route(1258415487).unwrap().is_none());
    }

    #[test]
    fn test_race_result_query() {
        let db = Database { conn: Connection::open_in_memory().unwrap() };
        db.create_tables().unwrap();
        db.migrate_elevation_knowledge().unwrap();
        db.conn.execute("INSERT INTO routes (route_id, distance_km, elevation_m, name, world, surface) VALUES (1258415487, 10, 50, 'Synthetic storage fixture', 'Test', 'road')", []).unwrap();

        db.conn.execute("INSERT INTO race_results (route_id,event_name,actual_minutes,zwift_score,race_date,notes) VALUES (1258415487,'Synthetic query fixture',32,195,'2025-02-02','Synthetic note')", []).unwrap();

        let results = db.get_race_results_for_route(1258415487, 195).unwrap();
        assert!(!results.is_empty());
        assert_eq!(results[0].actual_minutes, 32);
    }

    #[test]
    fn test_database_route_validation() {
        // Test that all routes in database have valid data
        if let Ok(db) = Database::new() {
            if let Ok(routes) = db.get_all_routes() {
                for route in routes {
                    // Distance should be reasonable
                    assert!(
                        route.distance_km > 0.0 && route.distance_km < 200.0,
                        "Route {} has unrealistic distance: {} km",
                        route.name,
                        route.distance_km
                    );

                    // Elevation gain per km should be reasonable
                    let elevation_per_km =
                        f64::from(route.elevation_m.unwrap_or(0.0)) / route.distance_km;
                    assert!(
                        elevation_per_km < 100.0,
                        "Route {} has unrealistic elevation: {} m/km",
                        route.name,
                        elevation_per_km
                    );

                    // Surface should be valid
                    assert!(
                        matches!(route.surface.as_str(), "road" | "gravel" | "mixed"),
                        "Route {} has invalid surface: {}",
                        route.name,
                        route.surface
                    );
                }
            }
        }
    }

    #[test]
    fn legacy_elevation_migration_is_repeatable_and_preserves_history() {
        let db = Database {
            conn: Connection::open_in_memory().unwrap(),
        };
        db.create_tables().unwrap();
        db.conn.execute("PRAGMA foreign_keys = ON", []).unwrap();
        db.conn.execute("INSERT INTO routes (route_id, distance_km, elevation_m, name, world, surface,
            lead_in_elevation_m, created_at) VALUES (4000000000, 2, 0, 'Circuit', 'Test', 'road', 0, '2000-01-01')", []).unwrap();
        db.conn.execute("INSERT INTO race_results (route_id, actual_minutes, zwift_score, event_name, race_date)
            VALUES (4000000000, 30, 195, 'Test race', '2026-10-29')", []).unwrap();
        db.migrate_elevation_knowledge().unwrap();
        db.migrate_elevation_knowledge().unwrap();
        let mut route = db.get_route_direct(4_000_000_000).unwrap().unwrap();
        assert_eq!(route.elevation_m, None);
        assert_eq!(route.lead_in_elevation_m, None);
        route.elevation_m = Some(0.0);
        route.lead_in_elevation_m = Some(0.0);
        db.add_route(&route).unwrap();
        let flat = db.get_route_direct(route.route_id).unwrap().unwrap();
        assert_eq!(flat.elevation_m, Some(0.0));
        assert_eq!(flat.lead_in_elevation_m, Some(0.0));
        assert_eq!(
            db.get_route_by_name("Circuit")
                .unwrap()
                .unwrap()
                .elevation_m,
            Some(0.0)
        );
        assert_eq!(db.get_all_routes().unwrap()[0].elevation_m, Some(0.0));
        route.elevation_m = Some(17.0);
        db.add_route(&route).unwrap();
        assert_eq!(
            db.get_route_direct(route.route_id)
                .unwrap()
                .unwrap()
                .elevation_m,
            Some(17.0)
        );
        let created: String = db
            .conn
            .query_row(
                "SELECT created_at FROM routes WHERE route_id=4000000000",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(created, "2000-01-01");
        assert_eq!(db.get_all_race_results().unwrap().len(), 1);
        db.conn.execute("INSERT INTO route_aliases (alias_route_id, canonical_route_id) VALUES (4000000001, 4000000000)", []).unwrap();
        assert_eq!(
            db.get_route(4_000_000_001).unwrap().unwrap().elevation_m,
            Some(17.0)
        );
    }

    #[test]
    fn legacy_positive_elevation_remains_known() {
        let db = Database {
            conn: Connection::open_in_memory().unwrap(),
        };
        db.create_tables().unwrap();
        db.conn.execute("INSERT INTO routes (route_id, distance_km, elevation_m, name, world, surface, lead_in_elevation_m)
            VALUES (1, 2, 17, 'Circuit', 'Test', 'road', 3)", []).unwrap();
        db.migrate_elevation_knowledge().unwrap();
        let route = db.get_route_direct(1).unwrap().unwrap();
        assert_eq!(route.elevation_m, Some(17.0));
        assert_eq!(route.lead_in_elevation_m, Some(3.0));
    }
}
