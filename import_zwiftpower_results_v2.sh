#!/bin/bash
# Import ZwiftPower results from the standard download location
# This script uses a separate zwiftpower_results table to avoid conflicts

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOWNLOAD_FILE="/mnt/c/Users/YOUR_USERNAME/Downloads/zwiftpower_results.json"
LOCAL_FILE="${SCRIPT_DIR}/zwiftpower_results.json"
DB_PATH="${HOME}/.local/share/zwift-race-finder/races.db"

echo "🚴 ZwiftPower Results Importer V2"
echo "================================="
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
jq -r 'length' "$LOCAL_FILE" | xargs -I {} echo "   Total races: {}"
jq -r '.[0].date + " to " + .[-1].date' "$LOCAL_FILE" | xargs -I {} echo "   Date range: {}"

# Create database directory if needed
mkdir -p "$(dirname "$DB_PATH")"

# Convert JSON to SQL
echo ""
echo "🔄 Converting to SQL..."
cd "$SCRIPT_DIR"

# Create SQL import file
{
    # Create zwiftpower_results table for imported data
    cat << 'SQL'
CREATE TABLE IF NOT EXISTS zwiftpower_results (
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

-- Add indexes for faster lookups
CREATE INDEX IF NOT EXISTS idx_zwiftpower_results_date ON zwiftpower_results(race_date);
CREATE INDEX IF NOT EXISTS idx_zwiftpower_results_event ON zwiftpower_results(event_name);
SQL

    echo ""
    
    # Convert JSON to INSERT statements
    jq -r '.[] | 
        "INSERT OR IGNORE INTO zwiftpower_results (race_date, event_name, event_id, category, position, distance_km, estimated_minutes, zwift_score, route_name) VALUES (" +
        "\"" + .date + "\", " +
        "\"" + (.event_name | gsub("\""; "\"\"")) + "\", " +
        "\"" + (.event_id // "null") + "\", " +
        "\"" + .category + "\", " +
        "\"" + .position + "\", " +
        (.distance_km | tostring) + ", " +
        (.estimated_minutes | tostring) + ", " +
        "\"" + (.zwift_score // "null") + "\", " +
        "\"" + (.route_name // "null" | gsub("\""; "\"\"")) + "\");"' "$LOCAL_FILE"
} > import.sql

# Import to database
echo "💾 Importing to database..."
sqlite3 "$DB_PATH" < import.sql

# Show summary
echo ""
echo "✅ Import complete!"
echo ""
echo "📈 Summary:"
sqlite3 -column -header "$DB_PATH" <<'SQL'
-- Overall stats
SELECT COUNT(*) as total_races, 
       MIN(race_date) as first_race,
       MAX(race_date) as last_race
FROM zwiftpower_results;

-- Category breakdown
SELECT category, COUNT(*) as races, 
       ROUND(AVG(distance_km), 1) as avg_distance_km,
       ROUND(AVG(estimated_minutes), 0) as avg_minutes
FROM zwiftpower_results 
GROUP BY category
ORDER BY category;

-- Most frequent events
SELECT event_name, COUNT(*) as times_raced
FROM zwiftpower_results 
GROUP BY event_name 
HAVING COUNT(*) > 1
ORDER BY times_raced DESC 
LIMIT 5;

-- Events with unknown routes (need mapping)
SELECT DISTINCT event_name, distance_km
FROM zwiftpower_results 
WHERE (route_name IS NULL OR route_name = 'null')
  AND event_name NOT LIKE '%Workout%'
  AND event_name NOT LIKE '%Group Ride%'
LIMIT 10;
SQL

# Clean up
rm -f import.sql

# Optional: remove the downloaded file to avoid re-importing old data
echo ""
read -p "Remove downloaded file to avoid confusion? [y/N] " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    rm -f "$DOWNLOAD_FILE"
    echo "✅ Removed $DOWNLOAD_FILE"
fi

echo ""
echo "🎯 Next steps:"
echo "1. Check for unknown routes above and add them to the route database"
echo "2. Look for events with consistent names to identify route patterns"
echo "3. Run: cd ~/tools/rust/zwift-race-finder && cargo run -- --show-unknown-routes"
echo "4. Build regression tests with your actual data"