#!/bin/bash
# Import ZwiftPower results from the standard download location
# This script assumes you've already run the JavaScript extractor in your browser

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Load config if exists
if [[ -f "${SCRIPT_DIR}/config.json" ]]; then
    WINDOWS_USERNAME=$(jq -r '.windows_username // empty' "${SCRIPT_DIR}/config.json")
fi

# Use environment variable, config, or fallback to default
WINDOWS_USERNAME="${WINDOWS_USERNAME:-${ZWIFTPOWER_WINDOWS_USERNAME:-YOUR_USERNAME}}"

# Set paths
if [[ "$WINDOWS_USERNAME" != "YOUR_USERNAME" ]]; then
    DOWNLOAD_FILE="/mnt/c/Users/${WINDOWS_USERNAME}/Downloads/zwiftpower_results.json"
else
    # Fallback to Linux Downloads directory
    DOWNLOAD_FILE="${HOME}/Downloads/zwiftpower_results.json"
fi

LOCAL_FILE="${SCRIPT_DIR}/zwiftpower_results.json"
DB_PATH="${HOME}/.local/share/zwift-race-finder/races.db"

echo "🚴 ZwiftPower Results Importer"
echo "=============================="
echo ""

# Check if download file exists
if [[ ! -f "$DOWNLOAD_FILE" ]]; then
    echo "❌ Error: No file found at $DOWNLOAD_FILE"
    echo ""
    echo "To extract your results:"
    echo "1. Go to your ZwiftPower profile page"
    echo "2. Open browser console (F12)"
    echo "3. Run: cat ~/tools/rust/zwift-race-finder/extract_zwiftpower_v2.js | xclip -selection clipboard"
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
    # Create table if not exists
    cat << 'SQL'
CREATE TABLE IF NOT EXISTS race_results (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date TEXT NOT NULL,
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

-- Add index for faster lookups
CREATE INDEX IF NOT EXISTS idx_race_results_date ON race_results(date);
CREATE INDEX IF NOT EXISTS idx_race_results_event ON race_results(event_name);
SQL

    echo ""
    
    # Convert JSON to INSERT statements
    jq -r '.[] | 
        "INSERT OR IGNORE INTO race_results (date, event_name, event_id, category, position, distance_km, estimated_minutes, zwift_score, route_name) VALUES (" +
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
       MIN(date) as first_race,
       MAX(date) as last_race
FROM race_results;

-- Category breakdown
SELECT category, COUNT(*) as races, 
       ROUND(AVG(distance_km), 1) as avg_distance_km,
       ROUND(AVG(estimated_minutes), 0) as avg_minutes
FROM race_results 
GROUP BY category
ORDER BY category;

-- Most frequent events
SELECT event_name, COUNT(*) as times_raced
FROM race_results 
GROUP BY event_name 
HAVING COUNT(*) > 1
ORDER BY times_raced DESC 
LIMIT 5;

-- Events with unknown routes (need mapping)
SELECT DISTINCT event_name
FROM race_results 
WHERE route_name IS NULL OR route_name = 'null'
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
echo "2. Run: cd ~/tools/rust/zwift-race-finder && cargo run -- --show-unknown-routes"
echo "3. Build regression tests with your actual data"