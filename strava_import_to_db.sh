#!/bin/bash
# Import Strava activity data into the race results database
# Matches activities to existing races and updates actual times

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ACTIVITIES_FILE="${SCRIPT_DIR}/strava_zwift_activities.json"
DB_PATH="${HOME}/.local/share/zwift-race-finder/races.db"

if [[ ! -f "$ACTIVITIES_FILE" ]]; then
    echo "❌ No activities file found. Run ./strava_fetch_activities.sh first"
    exit 1
fi

echo "🚴 Importing Strava Activities to Database"
echo "========================================="
echo ""

# First, let's add a strava_activity_id column if it doesn't exist
echo "📊 Updating database schema..."
sqlite3 "$DB_PATH" << 'SQL'
-- Check if column exists before adding
SELECT CASE 
    WHEN COUNT(*) = 0 THEN 'ALTER TABLE race_results ADD COLUMN strava_activity_id INTEGER;'
    ELSE 'SELECT "strava_activity_id column already exists";'
END
FROM pragma_table_info('race_results') 
WHERE name = 'strava_activity_id';
SQL

# Execute the result of the above query
sqlite3 "$DB_PATH" "$(sqlite3 "$DB_PATH" << 'SQL'
SELECT CASE 
    WHEN COUNT(*) = 0 THEN 'ALTER TABLE race_results ADD COLUMN strava_activity_id INTEGER;'
    ELSE 'SELECT "strava_activity_id column already exists";'
END
FROM pragma_table_info('race_results') 
WHERE name = 'strava_activity_id';
SQL
)"

# Add index and create tables
sqlite3 "$DB_PATH" << 'SQL'
-- Add index for faster lookups
CREATE INDEX IF NOT EXISTS idx_race_results_strava ON race_results(strava_activity_id);

-- Create a table for Strava activities if needed
CREATE TABLE IF NOT EXISTS strava_activities (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    start_date TEXT NOT NULL,
    distance_m REAL NOT NULL,
    moving_time_seconds INTEGER NOT NULL,
    elapsed_time_seconds INTEGER NOT NULL,
    average_speed_mps REAL,
    average_watts INTEGER,
    average_heartrate INTEGER,
    imported_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
SQL

# Import all Strava activities
echo "💾 Importing Strava activities..."
jq -r '.[] | 
    "INSERT OR REPLACE INTO strava_activities (id, name, start_date, distance_m, moving_time_seconds, elapsed_time_seconds, average_speed_mps, average_watts, average_heartrate) VALUES (" +
    (.id | tostring) + ", " +
    "\"" + (.name | gsub("\""; "\"\"")) + "\", " +
    "\"" + .start_date + "\", " +
    (.distance | tostring) + ", " +
    (.moving_time | tostring) + ", " +
    (.elapsed_time | tostring) + ", " +
    (.average_speed // 0 | tostring) + ", " +
    (.average_watts // 0 | tostring) + ", " +
    (.average_heartrate // 0 | tostring) + ");"' "$ACTIVITIES_FILE" | sqlite3 "$DB_PATH"

# Now let's try to match activities to race results
echo ""
echo "🔍 Matching Strava activities to race results..."

# Create a matching SQL script
cat > match_activities.sql << 'SQL'
-- Match by event name and date (within 1 day)
WITH matches AS (
    SELECT 
        r.id as race_id,
        s.id as strava_id,
        r.event_name,
        s.name as strava_name,
        r.race_date,
        date(s.start_date) as strava_date,
        r.actual_minutes as db_minutes,
        ROUND(s.moving_time_seconds / 60.0) as strava_minutes,
        ROUND(s.distance_m / 1000.0, 1) as strava_km,
        -- Calculate match score
        CASE 
            WHEN r.event_name = s.name THEN 100
            WHEN LOWER(r.event_name) LIKE '%' || LOWER(SUBSTR(s.name, 1, 20)) || '%' THEN 80
            WHEN LOWER(s.name) LIKE '%' || LOWER(SUBSTR(r.event_name, 1, 20)) || '%' THEN 70
            ELSE 50
        END as name_match_score
    FROM race_results r
    CROSS JOIN strava_activities s
    WHERE 
        r.strava_activity_id IS NULL
        AND ABS(julianday(date(s.start_date)) - julianday(date(r.race_date))) <= 1
        AND (
            LOWER(r.event_name) LIKE '%' || LOWER(SUBSTR(s.name, 1, 20)) || '%'
            OR LOWER(s.name) LIKE '%' || LOWER(r.event_name) || '%'
            OR r.event_name = s.name
        )
)
SELECT 
    race_id,
    strava_id,
    event_name,
    strava_name,
    race_date,
    strava_date,
    db_minutes,
    strava_minutes,
    strava_km,
    name_match_score
FROM matches
ORDER BY race_id, name_match_score DESC;
SQL

# Find matches
MATCHES=$(sqlite3 -header -column "$DB_PATH" < match_activities.sql)

echo "$MATCHES" | head -20
echo ""

# Count potential matches
MATCH_COUNT=$(sqlite3 "$DB_PATH" "SELECT COUNT(DISTINCT id) FROM race_results WHERE strava_activity_id IS NULL AND EXISTS (SELECT 1 FROM strava_activities s WHERE ABS(julianday(date(s.start_date)) - julianday(date(race_results.race_date))) <= 1)")

echo "📊 Found potential matches for $MATCH_COUNT race results"
echo ""

# Ask for confirmation
read -p "Do you want to update race results with Strava data? [y/N] " -n 1 -r
echo ""

if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "🔄 Updating race results with actual times from Strava..."
    
    # Update with best matches (highest score per race)
    # Using temporary table to work around SQLite correlated subquery limitations
    sqlite3 "$DB_PATH" << 'SQL'
.bail on

-- Create temporary mapping table
-- Match by date only since race_results only has dates without times
DROP TABLE IF EXISTS race_strava_matches;
CREATE TEMP TABLE race_strava_matches AS
SELECT 
    r.id as race_id,
    s.id as strava_id,
    ROUND(s.moving_time_seconds / 60.0) as actual_minutes,
    -- For display only - shows hours difference (will be large due to date-only storage)
    ABS(julianday(s.start_date) - julianday(r.race_date || ' 00:00:00')) * 24 as time_diff_hours,
    CASE 
        -- Perfect name match
        WHEN r.event_name = s.name THEN 100
        -- Good name matches
        WHEN LOWER(r.event_name) LIKE '%' || LOWER(SUBSTR(s.name, 1, 20)) || '%' THEN 80
        WHEN LOWER(s.name) LIKE '%' || LOWER(r.event_name) || '%' THEN 70
        -- Both are races (common for Zwift activities) - high score for same day
        WHEN LOWER(s.name) LIKE '%race%' AND LOWER(r.event_name) LIKE '%race%' 
             AND DATE(s.start_date) = DATE(r.race_date) THEN 85
        -- Same date is a strong indicator when names don't match well
        WHEN DATE(s.start_date) = DATE(r.race_date) THEN 75
        ELSE 20
    END as match_score
FROM race_results r
JOIN strava_activities s ON DATE(s.start_date) = DATE(r.race_date)
WHERE r.strava_activity_id IS NULL;

-- Show potential matches for debugging
SELECT 
    r.event_name,
    s.name as strava_name,
    m.time_diff_hours,
    m.match_score,
    m.actual_minutes
FROM race_strava_matches m
JOIN race_results r ON r.id = m.race_id
JOIN strava_activities s ON s.id = m.strava_id
ORDER BY m.match_score DESC
LIMIT 10;

-- Update race_results with best matches
UPDATE race_results
SET 
    strava_activity_id = (
        SELECT strava_id 
        FROM race_strava_matches m
        WHERE m.race_id = race_results.id
        ORDER BY match_score DESC
        LIMIT 1
    ),
    actual_minutes = (
        SELECT actual_minutes
        FROM race_strava_matches m
        WHERE m.race_id = race_results.id
        ORDER BY match_score DESC
        LIMIT 1
    )
WHERE id IN (SELECT DISTINCT race_id FROM race_strava_matches);

-- Clean up
DROP TABLE IF EXISTS race_strava_matches;
SQL
    
    # Show results
    UPDATED=$(sqlite3 "$DB_PATH" "SELECT changes();")
    echo "✅ Updated $UPDATED race results with Strava data"
fi

# Show summary
echo ""
echo "📊 Database Summary:"
sqlite3 -column -header "$DB_PATH" << 'SQL'
SELECT 
    COUNT(*) as total_races,
    COUNT(strava_activity_id) as matched_to_strava,
    ROUND(AVG(CASE WHEN strava_activity_id IS NOT NULL THEN actual_minutes END), 1) as avg_race_minutes,
    MIN(race_date) as earliest_race,
    MAX(race_date) as latest_race
FROM race_results;
SQL

# Clean up
rm -f match_activities.sql

echo ""
echo "🎯 Next steps:"
echo "1. Run regression tests: cd ~/tools/rust/zwift-race-finder && cargo test regression"
echo "2. Check unmatched races: sqlite3 $DB_PATH \"SELECT event_name, race_date FROM race_results WHERE strava_activity_id IS NULL ORDER BY race_date DESC LIMIT 10;\""