# Database Reference

## Location

`~/.local/share/zwift-race-finder/races.db` (SQLite, created automatically on first run)

## Schema

### routes
```sql
CREATE TABLE IF NOT EXISTS routes (
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
);
```

The application adds nullable `elevation_known` and `lead_in_elevation_known` integer flags (checked to 0/1) through a repeatable migration. Flag 1 means known, including zero; flag 0 means unknown. Legacy NULL flags retain positive elevation but treat zero as unknown. Rust exposes optional fractional elevations and optional lead-in distance. Existing INTEGER-affinity elevation columns accept fractional REAL values without rebuilding the table.

### route_sources
```sql
CREATE TABLE IF NOT EXISTS route_sources (
    route_id INTEGER PRIMARY KEY REFERENCES routes(route_id),
    metadata TEXT NOT NULL
);
```

`metadata` is the normalized client record, source kind/path/entry/SHA-256, and event/sport/lap flags as JSON. Client route updates and provenance are committed atomically with UPSERT; route creation timestamps and dependent race history are preserved. Supported inputs are selected client cache, then previously extracted client XML for missing IDs. Existing community/legacy rows remain until explicitly updated; there is no supported community importer. A later extracted-only import cannot replace a cache-derived record. See [client-only import](../howto/CLIENT_ROUTE_DATA.md).

### race_results
```sql
CREATE TABLE IF NOT EXISTS race_results (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    route_id INTEGER NOT NULL,
    event_name TEXT NOT NULL,
    actual_minutes INTEGER NOT NULL,
    zwift_score INTEGER,
    race_date TIMESTAMP NOT NULL,
    notes TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    actual_seconds REAL,
    event_start_utc TEXT,
    event_distance_km REAL,
    laps INTEGER,
    result_category TEXT,
    gun_time_seconds REAL,
    event_type TEXT,
    subgroup_id TEXT,
    source TEXT,
    source_rider_id TEXT,
    source_result_id TEXT,
    source_event_id TEXT,
    source_sha256 TEXT,
    source_payload_json TEXT,
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

The old name-matched alias SQL has been retired; it included approximate and reversed-route substitutions. Existing alias rows are preserved, but their validity requires separate review. Client imports use direct numeric IDs, not guessed aliases.

Existing retired completion/rider/discovery tables are preserved but not created
or used by the current finder. Their historical schemas remain in Git at `v0.3.0`.

## Route ID Notes

Rust uses `u32`; SQLite uses signed integer storage and supports the complete
unsigned-32 range. Validate and normalize signed client representations at input
boundaries; do not truncate through `i32`. `Database::get_route()` tries direct
numeric lookup before aliases. Names are not identity.

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

## Source-qualified performance history

`race_results` now accepts SQL NULL for unknown historical `zwift_score`. The transactional migration preserves old IDs, notes, creation timestamps, indexes/triggers and AUTOINCREMENT history; unsupported custom/dependent schemas fail rather than lose data. Application database initialization performs this migration, so back up before first use of the updated binaries.

Additional nullable columns are `actual_seconds`, `event_start_utc`, `event_distance_km`, `laps`, `result_category`, `gun_time_seconds`, `event_type`, `subgroup_id`, `source`, `source_rider_id`, `source_result_id`, `source_event_id`, `source_sha256`, and `source_payload_json`. Minute-only legacy results keep NULL exact seconds. Modern imports preserve exact seconds and use rounded `actual_minutes` for existing consumers. The unique partial source-identity index covers `(source, source_rider_id, source_result_id)`.

`Database::performance_history()` returns observations regardless of score availability. Score-dependent validation reports/excludes unknown scores without deleting their records. `Database::preview()` uses a SQLite backup into memory and does not migrate the original database. See [design](../explanation/HISTORY_DATA_MODEL.md) and [workflow](../howto/ZWIFTPOWER_EXPORT_STEPS.md).
