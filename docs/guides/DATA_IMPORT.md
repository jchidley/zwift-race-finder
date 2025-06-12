# Data Import Guide

## Overview

Zwift Race Finder can import data from multiple sources to improve predictions and discover new routes.

## ZwiftPower Import

### Prerequisites
- Active ZwiftPower account linked to your Zwift account
- Web browser with developer console access

### Export Process

1. **Navigate to Your Profile**:
   - Log into ZwiftPower.com
   - Go to your profile page

2. **Extract Race Data**:
   ```bash
   # Copy the extraction script to clipboard
   cat tools/zwiftpower/zwiftpower_profile_extractor.js | xclip -selection clipboard
   
   # Or on macOS:
   cat tools/zwiftpower/zwiftpower_profile_extractor.js | pbcopy
   ```

3. **Run in Browser Console**:
   - Open browser developer tools (F12)
   - Go to Console tab
   - Paste and run the script
   - Save the downloaded `zwiftpower_results.json`

4. **Import to Database**:
   ```bash
   # For development/testing
   ./tools/zwiftpower/import_zwiftpower_dev.sh
   
   # For production
   ./tools/zwiftpower/import_zwiftpower.sh
   ```

### What Gets Imported
- Race finish times
- Route information
- Category/score data
- Event dates
- Power/weight data (if available)

## Strava Import

### Prerequisites
- Strava account with Zwift activities
- Strava API application (create at https://www.strava.com/settings/api)

### Setup

1. **Configure Strava App**:
   ```bash
   # Copy example config
   cp tools/import/strava/strava_config.json.example strava_config.json
   
   # Edit with your app details
   vim strava_config.json
   ```

2. **Authenticate**:
   ```bash
   # Run authentication flow
   ./tools/import/strava/strava_auth.sh
   
   # Or with secure storage
   ./tools/import/strava/strava_auth_secure.sh
   ```

3. **Fetch Activities**:
   ```bash
   # Fetch recent Zwift activities
   ./tools/import/strava/strava_fetch_activities.sh
   
   # Import to database
   ./tools/import/strava/strava_import_to_db.sh
   ```

### What Gets Imported
- Actual race completion times
- Route names from activity titles
- Distance and elevation
- Average speed/power
- Date and time

## Route Data Import

### WhatsOnZwift Data

1. **Fetch Route Data**:
   ```bash
   cd tools/import/routes
   
   # Fetch all routes
   ./fetch_whatsonzwift_route_data.sh
   
   # Parse and import
   python3 fetch_whatsonzwift_route_data_parser.py
   ```

2. **Import to Database**:
   ```bash
   # Import parsed routes
   python3 import_zwift_data_routes.py
   ```

### Manual Route Addition

For routes not in automated sources:

```sql
-- Connect to database
sqlite3 ~/.local/share/zwift-race-finder/races.db

-- Add new route
INSERT INTO routes (route_id, distance_km, elevation_m, lead_in_km, 
                   lead_in_elevation_m, name, world, surface, slug)
VALUES (
    12345,           -- route_id from ZwiftHacks
    25.3,            -- distance in km
    341,             -- elevation in meters
    2.1,             -- lead-in distance
    15,              -- lead-in elevation
    'Epic Route',    -- route name
    'Watopia',       -- world
    'road',          -- surface type
    'epic-route'     -- URL slug
);
```

## Route Mapping

### Apply Standard Mappings

```bash
# Apply all route mappings
./tools/utils/apply_route_mappings.sh

# Or manually
sqlite3 ~/.local/share/zwift-race-finder/races.db < sql/mappings/route_mappings.sql
```

### Fix Unmapped Routes

```bash
# Show routes needing mapping
cargo run -- --show-unknown-routes

# Apply fixes
./tools/utils/fix_unmapped_routes.sh
```

### Multi-Lap Race Mappings

```sql
-- Special handling for multi-lap races
sqlite3 ~/.local/share/zwift-race-finder/races.db < sql/mappings/fix_multi_lap_mappings.sql
```

## Event Description Import

For Racing Score events that need description parsing:

```bash
# Fetch event descriptions
cd tools/import/routes
./fetch_event_descriptions.sh

# This helps identify:
# - Distance information in descriptions
# - Multi-stage event details
# - Special event formats
```

## Data Validation

### Verify Import Success

```sql
-- Check imported race results
SELECT COUNT(*) as total_races FROM race_results;

-- Check route coverage
SELECT COUNT(DISTINCT route_id) as unique_routes FROM routes;

-- Find missing data
SELECT event_title, COUNT(*) as occurrences
FROM unknown_routes
WHERE route_id = 9999
GROUP BY event_title
ORDER BY occurrences DESC
LIMIT 10;
```

### Test Predictions

```bash
# Run regression tests with imported data
cargo test regression

# Check accuracy
cargo run -- --test-predictions
```

## Troubleshooting

### Common Issues

1. **Duplicate Imports**:
   ```sql
   -- Remove duplicates (keeping first occurrence)
   DELETE FROM race_results
   WHERE rowid NOT IN (
       SELECT MIN(rowid)
       FROM race_results
       GROUP BY event_name, event_date, category
   );
   ```

2. **Missing Route Mappings**:
   - Check event title variations
   - Look for route_id on ZwiftHacks
   - Add to `route_mappings.sql`

3. **Import Script Fails**:
   - Check file permissions
   - Verify JSON format
   - Check database path

### Data Cleanup

```bash
# Backup before cleanup
cp ~/.local/share/zwift-race-finder/races.db ~/.local/share/zwift-race-finder/races.db.pre-cleanup

# Remove old/test data
sqlite3 ~/.local/share/zwift-race-finder/races.db "DELETE FROM race_results WHERE event_date < date('now', '-2 years');"

# Vacuum database
sqlite3 ~/.local/share/zwift-race-finder/races.db "VACUUM;"
```

## Best Practices

1. **Regular Imports**: Weekly for active racers
2. **Backup First**: Always backup before bulk imports
3. **Verify Data**: Check a few records manually
4. **Map Routes**: Investigate unknown routes promptly
5. **Share Mappings**: Contribute route discoveries back