# Database Reference

## Location

`~/.local/share/zwift-race-finder/races.db` (SQLite, created automatically on first run)

## Schema

### routes
```sql
CREATE TABLE IF NOT EXISTS routes (
    route_id INTEGER PRIMARY KEY,
    distance_km REAL NOT NULL,
    elevation_m INTEGER NOT NULL,
    name TEXT NOT NULL,
    world TEXT NOT NULL,
    surface TEXT NOT NULL DEFAULT 'road',
    lead_in_distance_km REAL DEFAULT 0.0,
    lead_in_elevation_m INTEGER DEFAULT 0,
    lead_in_distance_free_ride_km REAL,
    lead_in_elevation_free_ride_m INTEGER,
    lead_in_distance_meetups_km REAL,
    lead_in_elevation_meetups_m INTEGER,
    slug TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### race_results
```sql
CREATE TABLE IF NOT EXISTS race_results (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    route_id INTEGER NOT NULL,
    event_name TEXT NOT NULL,
    actual_minutes INTEGER NOT NULL,
    zwift_score INTEGER NOT NULL,
    race_date TIMESTAMP NOT NULL,
    notes TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (route_id) REFERENCES routes(route_id)
);
```

### unknown_routes
```sql
CREATE TABLE IF NOT EXISTS unknown_routes (
    route_id INTEGER PRIMARY KEY,
    event_name TEXT NOT NULL,
    event_type TEXT,
    first_seen TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    times_seen INTEGER DEFAULT 1
);
```

### route_completion
```sql
CREATE TABLE IF NOT EXISTS route_completion (
    route_id INTEGER PRIMARY KEY,
    completed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    actual_time_minutes INTEGER,
    notes TEXT,
    FOREIGN KEY (route_id) REFERENCES routes(route_id)
);
```

### rider_stats
```sql
CREATE TABLE IF NOT EXISTS rider_stats (
    id INTEGER PRIMARY KEY,
    height_m REAL DEFAULT 1.82,
    weight_kg REAL,
    ftp_watts INTEGER,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);
```

Note: `rider_stats` data is stored but **not used** in the duration estimation algorithm.

### route_aliases
```sql
CREATE TABLE IF NOT EXISTS route_aliases (
    alias_route_id INTEGER PRIMARY KEY,
    canonical_route_id INTEGER NOT NULL,
    notes TEXT,
    FOREIGN KEY (canonical_route_id) REFERENCES routes(route_id)
);
```

Maps alternative Zwift API route IDs (event-only variants) to canonical route IDs in the `routes` table. Zwift uses different internal IDs for the same physical route depending on whether it's a free-ride or event-only context. `Database::get_route()` checks aliases transparently on miss.

Populated by `sql/mappings/route_aliases.sql`. Currently 11 aliases covering ~2,640 previously-unresolvable event sightings.

### route_discovery_attempts
```sql
CREATE TABLE IF NOT EXISTS route_discovery_attempts (
    route_id INTEGER PRIMARY KEY,
    event_name TEXT NOT NULL,
    last_attempt TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    found BOOLEAN DEFAULT 0,
    distance_km REAL,
    elevation_m INTEGER,
    world TEXT,
    surface TEXT,
    notes TEXT
);
```

## Route ID Notes

- Route IDs are `u32` in Rust (0 to 4,294,967,295)
- SQLite stores them as `INTEGER` (signed 64-bit)
- Zwift API may serialize large IDs as negative numbers (e.g., -2129086892 = 2165880404 as u32)
- Import code deserializes as `i64`, then casts to `u32`
- `--record-result` requires the route to exist in `routes` table first (FK constraint)

## Common Queries

```sql
-- Most-raced routes
SELECT r.name, r.world, COUNT(rr.id) as race_count
FROM routes r
JOIN race_results rr ON r.route_id = rr.route_id
GROUP BY r.route_id
ORDER BY race_count DESC;

-- Unknown routes by frequency
SELECT event_name, route_id, times_seen
FROM unknown_routes
ORDER BY times_seen DESC
LIMIT 20;

-- Route count
SELECT COUNT(*) FROM routes;
```

## Backup

```bash
cp ~/.local/share/zwift-race-finder/races.db \
   ~/.local/share/zwift-race-finder/races_$(date +%Y%m%d).db
```

## Reset

```bash
rm ~/.local/share/zwift-race-finder/races.db
# Next run recreates the database with empty tables
```
