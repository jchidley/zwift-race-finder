#!/bin/bash
# Development import script - imports ZwiftPower results for testing
# Matches the Rust database schema exactly

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Load config if exists (try TOML first, then JSON)
if [[ -f "${SCRIPT_DIR}/config.toml" ]]; then
    # Parse TOML using grep/sed (simple approach for single value)
    WINDOWS_USERNAME=$(grep -E "^windows_username\s*=" "${SCRIPT_DIR}/config.toml" | sed 's/.*=\s*"\(.*\)"/\1/' | tr -d "'")
elif [[ -f "${SCRIPT_DIR}/config.json" ]]; then
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

echo "🚴 Development Import - ZwiftPower Results"
echo "========================================="
echo ""

# Check if download file exists
if [[ ! -f "$DOWNLOAD_FILE" ]]; then
    echo "❌ Error: No file found at $DOWNLOAD_FILE"
    exit 1
fi

# Copy file to local directory
echo "📋 Copying from Downloads..."
cp "$DOWNLOAD_FILE" "$LOCAL_FILE"

# Show what we found
echo "📊 Found results file:"
TOTAL_RACES=$(jq -r 'length' "$LOCAL_FILE")
echo "   Total races: $TOTAL_RACES"

# For development, let's start fresh
echo ""
echo "🧹 Cleaning existing data for fresh import..."
sqlite3 "$DB_PATH" << 'SQL'
-- Keep routes table but clear race results
DELETE FROM race_results;
DELETE FROM unknown_routes;

-- Drop any temporary tables
DROP TABLE IF EXISTS zwiftpower_results;
DROP TABLE IF EXISTS zwiftpower_import;
SQL

# Import into race_results with placeholder route_ids
echo ""
echo "💾 Importing races..."

# We'll use a simple mapping: if we can't find a route, use route_id 9999
# and track it in unknown_routes
jq -r '.[] | 
    # For now, just use placeholder route_id 9999 for everything
    # The Rust program will track these as unknown
    "INSERT INTO race_results (route_id, event_name, actual_minutes, zwift_score, race_date, notes) VALUES (" +
    "9999, " +
    "\"" + (.event_name | gsub("\""; "\"\"")) + "\", " +
    ((.estimated_minutes // 60) | tostring) + ", " +
    ((.zwift_score // "500") | gsub("[^0-9.]"; "") | if . == "" then "500" else . end) + ", " +
    "\"" + .date + "\", " +
    "\"Category " + .category + " - Position " + .position + " - " + (.distance_km | tostring) + "km\");"' "$LOCAL_FILE" > import.sql

# Also track unique events in unknown_routes
jq -r '[.[] | {event_name: .event_name, event_type: .category}] | unique_by(.event_name) | .[] |
    "INSERT OR IGNORE INTO unknown_routes (route_id, event_name, event_type) VALUES (" +
    "9999, " +
    "\"" + (.event_name | gsub("\""; "\"\"")) + "\", " +
    "\"" + .event_type + "\");"' "$LOCAL_FILE" >> import.sql

# Import to database
sqlite3 "$DB_PATH" < import.sql

# Show what we imported
echo ""
echo "✅ Import complete!"
echo ""
echo "📊 Summary:"
sqlite3 -column -header "$DB_PATH" << 'SQL'
-- Total imported
SELECT COUNT(*) as total_races_imported FROM race_results;

-- Score distribution
SELECT 
    CASE 
        WHEN zwift_score < 400 THEN '< 400'
        WHEN zwift_score < 500 THEN '400-500'
        WHEN zwift_score < 600 THEN '500-600'
        ELSE '600+'
    END as score_range,
    COUNT(*) as count
FROM race_results
GROUP BY score_range
ORDER BY score_range;

-- Unknown routes to map
SELECT COUNT(DISTINCT event_name) as unique_unknown_events FROM unknown_routes;
SQL

# Show sample of unknown routes
echo ""
echo "🗺️  Sample unknown routes that need mapping:"
sqlite3 -column -header "$DB_PATH" << 'SQL'
SELECT event_name, event_type, times_seen
FROM unknown_routes
ORDER BY times_seen DESC
LIMIT 10;
SQL

# Clean up
rm -f import.sql

echo ""
echo "🎯 Next steps:"
echo "1. Run the Rust program to see unknown routes:"
echo "   cd ~/tools/rust/zwift-race-finder && cargo run -- --show-unknown-routes"
echo ""
echo "2. Add route mappings for common events"
echo ""
echo "3. For testing, you now have ${TOTAL_RACES} races in the database!"

# Optional: remove the downloaded file
echo ""
read -p "Remove downloaded file? [y/N] " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    rm -f "$DOWNLOAD_FILE"
    echo "✅ Removed $DOWNLOAD_FILE"
fi