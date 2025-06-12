# Database Management

## Overview

Zwift Race Finder uses SQLite for local storage of route data, race results, and unknown routes that need investigation.

## Database Location

- **Primary**: `~/.local/share/zwift-race-finder/races.db`
- **Created automatically** on first run
- **Permissions**: User read/write only

## Schema

### routes
```sql
CREATE TABLE routes (
    route_id INTEGER PRIMARY KEY,
    distance_km REAL NOT NULL,
    elevation_m INTEGER NOT NULL,
    lead_in_km REAL DEFAULT 0,
    lead_in_elevation_m INTEGER DEFAULT 0,
    name TEXT NOT NULL,
    world TEXT NOT NULL,
    surface TEXT DEFAULT 'road',
    slug TEXT,
    UNIQUE(name, world)
);
```

### race_results
```sql
CREATE TABLE race_results (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    route_id INTEGER,
    category TEXT,
    time_minutes REAL,
    event_name TEXT,
    event_date TEXT,
    zwift_racing_score INTEGER,
    FOREIGN KEY (route_id) REFERENCES routes(route_id)
);
```

### unknown_routes
```sql
CREATE TABLE unknown_routes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_title TEXT NOT NULL,
    route_id INTEGER NOT NULL,
    sport TEXT,
    first_seen TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    occurrence_count INTEGER DEFAULT 1,
    UNIQUE(event_title, route_id)
);
```

## Backup Strategy

### Manual Backup
```bash
# Simple backup with timestamp
cp ~/.local/share/zwift-race-finder/races.db \
   ~/.local/share/zwift-race-finder/races_$(date +%Y%m%d_%H%M%S).db

# Before major operations
cp ~/.local/share/zwift-race-finder/races.db \
   ~/.local/share/zwift-race-finder/races.db.backup
```

### Automated Backup (Optional)
Add to crontab for daily backups:
```bash
0 2 * * * cp ~/.local/share/zwift-race-finder/races.db ~/.local/share/zwift-race-finder/backups/races_$(date +\%Y\%m\%d).db
```

## Common Operations

### Adding New Routes

1. **Find Route Information**:
   - Check ZwiftHacks.com for route_id
   - Get metrics from Zwift Insider or WhatsOnZwift
   - Note lead-in distance if available

2. **Insert Route**:
```sql
INSERT INTO routes (route_id, distance_km, elevation_m, lead_in_km, 
                   lead_in_elevation_m, name, world, surface, slug)
VALUES (12345, 25.3, 341, 2.1, 15, 'New Route Name', 'Watopia', 'road', 'new-route-name');
```

3. **Update Mappings**:
```sql
-- Add to route_mappings.sql for event name matching
UPDATE events SET route_id = 12345 WHERE event_title LIKE '%New Route%';
```

### Importing Race Results

See [Data Import Guide](../guides/DATA_IMPORT.md) for detailed workflows.

Quick import:
```bash
# From ZwiftPower export
./tools/zwiftpower/import_zwiftpower_dev.sh

# Apply route mappings
sqlite3 ~/.local/share/zwift-race-finder/races.db < sql/mappings/route_mappings.sql
```

### Querying Unknown Routes

```sql
-- Show most common unknown routes
SELECT event_title, route_id, occurrence_count 
FROM unknown_routes 
ORDER BY occurrence_count DESC 
LIMIT 20;

-- Routes seen in last 7 days
SELECT * FROM unknown_routes 
WHERE first_seen > datetime('now', '-7 days')
ORDER BY first_seen DESC;
```

### Database Maintenance

```bash
# Vacuum to reclaim space
sqlite3 ~/.local/share/zwift-race-finder/races.db "VACUUM;"

# Analyze for query optimization
sqlite3 ~/.local/share/zwift-race-finder/races.db "ANALYZE;"

# Check integrity
sqlite3 ~/.local/share/zwift-race-finder/races.db "PRAGMA integrity_check;"
```

## Migration Commands

### Add New Column
```sql
-- Example: Add a new column with default
ALTER TABLE routes ADD COLUMN is_event_only BOOLEAN DEFAULT 0;

-- Update existing data if needed
UPDATE routes SET is_event_only = 1 WHERE name LIKE '%Tour%';
```

### Schema Updates
Always:
1. Backup database first
2. Test on copy of database
3. Update all code that uses the table
4. Update test data

## SQL Utilities

### Useful Queries

```sql
-- Routes by popularity in races
SELECT r.name, r.world, COUNT(rr.id) as race_count
FROM routes r
LEFT JOIN race_results rr ON r.route_id = rr.route_id
GROUP BY r.route_id
ORDER BY race_count DESC;

-- Average times by route and category
SELECT r.name, rr.category, 
       AVG(rr.time_minutes) as avg_time,
       COUNT(*) as sample_size
FROM race_results rr
JOIN routes r ON rr.route_id = r.route_id
GROUP BY r.route_id, rr.category
ORDER BY r.name, rr.category;

-- Find routes needing mapping
SELECT DISTINCT event_title, route_id
FROM unknown_routes
WHERE route_id = 9999
ORDER BY occurrence_count DESC;
```

## Troubleshooting

### Database Locked
```bash
# Find processes using the database
lsof ~/.local/share/zwift-race-finder/races.db

# Kill if necessary (use with caution)
kill -9 <PID>
```

### Corrupted Database
```bash
# Try to recover
sqlite3 ~/.local/share/zwift-race-finder/races.db ".recover" | 
sqlite3 ~/.local/share/zwift-race-finder/races_recovered.db

# Verify and replace if successful
sqlite3 ~/.local/share/zwift-race-finder/races_recovered.db "PRAGMA integrity_check;"
```

### Reset Database
```bash
# Remove and regenerate (loses all data!)
rm ~/.local/share/zwift-race-finder/races.db
cargo run  # Will create fresh database
```

## Best Practices

1. **Always backup** before bulk operations
2. **Use transactions** for multiple updates
3. **Index foreign keys** for performance
4. **Keep route_id stable** - it's the primary reference
5. **Document schema changes** in migration files
6. **Test queries** on backup copy first