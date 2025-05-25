#!/bin/bash
# Import ZwiftPower results and merge with existing race_results table
# This handles the route_id lookup/mapping challenge

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOWNLOAD_FILE="/mnt/c/Users/YOUR_USERNAME/Downloads/zwiftpower_results.json"
LOCAL_FILE="${SCRIPT_DIR}/zwiftpower_results.json"
DB_PATH="${HOME}/.local/share/zwift-race-finder/races.db"

echo "🚴 ZwiftPower Results Import & Merge"
echo "===================================="
echo ""

# Check if download file exists
if [[ ! -f "$DOWNLOAD_FILE" ]]; then
    echo "❌ Error: No file found at $DOWNLOAD_FILE"
    echo ""
    echo "To extract your results:"
    echo "1. Go to your ZwiftPower profile page"
    echo "2. Open browser console (F12)"
    echo "3. Run: cat ~/tools/rust/zwift-race-finder/extract_zwiftpower_final.js | xclip -selection clipboard"
    echo "4. Paste and run in console"
    echo "5. File will download automatically"
    exit 1
fi

# Copy file to local directory
echo "📋 Copying from Downloads..."
cp "$DOWNLOAD_FILE" "$LOCAL_FILE"

# Show what we found
echo "📊 Found results file:"
TOTAL_RACES=$(jq -r 'length' "$LOCAL_FILE")
echo "   Total races: $TOTAL_RACES"
jq -r '.[0].date + " to " + .[-1].date' "$LOCAL_FILE" | xargs -I {} echo "   Date range: {}"

# Create database directory if needed
mkdir -p "$(dirname "$DB_PATH")"

# First, let's clean up the old zwiftpower_results table if it exists
echo ""
echo "🧹 Cleaning up old tables..."
sqlite3 "$DB_PATH" "DROP TABLE IF EXISTS zwiftpower_results;"

# Create a temporary import table
echo "📥 Creating temporary import table..."
sqlite3 "$DB_PATH" << 'SQL'
CREATE TABLE IF NOT EXISTS zwiftpower_import (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    race_date TEXT NOT NULL,
    event_name TEXT NOT NULL,
    event_id TEXT,
    category TEXT,
    position TEXT,
    distance_km REAL,
    estimated_minutes INTEGER,
    zwift_score TEXT,
    route_name TEXT,
    imported_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
SQL

# Convert JSON to SQL
echo "🔄 Converting JSON to SQL..."
cd "$SCRIPT_DIR"

# Import to temporary table
jq -r '.[] | 
    "INSERT INTO zwiftpower_import (race_date, event_name, event_id, category, position, distance_km, estimated_minutes, zwift_score, route_name) VALUES (" +
    "\"" + .date + "\", " +
    "\"" + (.event_name | gsub("\""; "\"\"")) + "\", " +
    "\"" + (.event_id // "null") + "\", " +
    "\"" + .category + "\", " +
    "\"" + .position + "\", " +
    (.distance_km | tostring) + ", " +
    (.estimated_minutes | tostring) + ", " +
    "\"" + (.zwift_score // "null") + "\", " +
    "\"" + (.route_name // "null" | gsub("\""; "\"\"")) + "\");"' "$LOCAL_FILE" > import.sql

echo "💾 Importing to temporary table..."
sqlite3 "$DB_PATH" < import.sql

# Show import summary
echo ""
echo "📊 Import Summary:"
sqlite3 -column -header "$DB_PATH" <<'SQL'
-- Overall stats
SELECT COUNT(*) as imported_races, 
       MIN(race_date) as first_race,
       MAX(race_date) as last_race
FROM zwiftpower_import;

-- Category breakdown
SELECT category, COUNT(*) as races
FROM zwiftpower_import 
GROUP BY category
ORDER BY category;
SQL

# Now let's analyze what can be merged
echo ""
echo "🔍 Analyzing route mappings..."
sqlite3 -column -header "$DB_PATH" <<'SQL'
-- Show events that might have routes
SELECT event_name, COUNT(*) as times, 
       ROUND(AVG(distance_km), 1) as avg_km,
       ROUND(AVG(estimated_minutes), 0) as avg_min
FROM zwiftpower_import
WHERE event_name NOT LIKE '%Workout%'
  AND event_name NOT LIKE '%Group Ride%'
  AND distance_km > 0
GROUP BY event_name
HAVING COUNT(*) > 1
ORDER BY times DESC
LIMIT 10;
SQL

# Show how many races already exist in race_results
EXISTING_COUNT=$(sqlite3 "$DB_PATH" "SELECT COUNT(*) FROM race_results;")
echo ""
echo "📈 Existing race_results: $EXISTING_COUNT"

# Ask user what to do
echo ""
echo "Options:"
echo "1) Keep imported data in temporary table for manual route mapping"
echo "2) Try to auto-match some routes based on event names"
echo "3) Show more analysis before deciding"
echo "4) Cancel and clean up"
echo ""
read -p "Choose option [1-4]: " -n 1 -r
echo ""

case $REPLY in
    1)
        echo "✅ Data imported to zwiftpower_import table"
        echo ""
        echo "Next steps:"
        echo "1. Query zwiftpower_import to find route patterns"
        echo "2. Add missing routes to the routes table"
        echo "3. Manually insert matched races into race_results with proper route_ids"
        ;;
    2)
        echo "🤖 Attempting auto-matching..."
        # Try to match some common patterns
        sqlite3 "$DB_PATH" <<'SQL'
-- Create a mapping table for common events
CREATE TEMP TABLE event_route_map AS
SELECT DISTINCT event_name, 
       CASE 
           WHEN event_name LIKE '%Volcano%' THEN 1001  -- Example route_id
           WHEN event_name LIKE '%Watopia%Figure%8%' THEN 1002
           -- Add more mappings as needed
           ELSE NULL
       END as suggested_route_id
FROM zwiftpower_import;

SELECT event_name, suggested_route_id 
FROM event_route_map 
WHERE suggested_route_id IS NOT NULL
LIMIT 10;
SQL
        ;;
    3)
        echo "📊 Detailed analysis..."
        sqlite3 -column -header "$DB_PATH" <<'SQL'
-- Events by distance
SELECT ROUND(distance_km/5)*5 as distance_bucket, 
       COUNT(*) as races
FROM zwiftpower_import
WHERE distance_km > 0
GROUP BY distance_bucket
ORDER BY distance_bucket;

-- Most common event name patterns
SELECT 
    CASE 
        WHEN event_name LIKE '%WTRL%' THEN 'WTRL Events'
        WHEN event_name LIKE '%ZRacing%' THEN 'ZRacing Events'
        WHEN event_name LIKE '%3R%' THEN '3R Events'
        WHEN event_name LIKE '%SISU%' THEN 'SISU Events'
        ELSE 'Other'
    END as event_type,
    COUNT(*) as count
FROM zwiftpower_import
GROUP BY event_type
ORDER BY count DESC;
SQL
        ;;
    4)
        echo "🧹 Cleaning up..."
        sqlite3 "$DB_PATH" "DROP TABLE IF EXISTS zwiftpower_import;"
        rm -f import.sql
        echo "✅ Cancelled and cleaned up"
        exit 0
        ;;
esac

# Clean up
rm -f import.sql

echo ""
echo "💡 To query the imported data:"
echo "sqlite3 ~/.local/share/zwift-race-finder/races.db"
echo ""
echo "Example queries:"
echo "-- Find all races for a specific route"
echo "SELECT * FROM zwiftpower_import WHERE event_name LIKE '%Volcano%';"
echo ""
echo "-- Find unique event patterns"
echo "SELECT DISTINCT event_name FROM zwiftpower_import ORDER BY event_name;"