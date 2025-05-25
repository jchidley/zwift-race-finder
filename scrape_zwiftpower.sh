#!/bin/bash
# Scrape Jack's ZwiftPower profile to extract race results for regression testing
# Uses curl, jq, and standard Unix tools per CLAUDE.md bash-first philosophy

set -euo pipefail

# Configuration
ZWIFTPOWER_ID="1106548"
PROFILE_URL="https://zwiftpower.com/profile.php?z=${ZWIFTPOWER_ID}"
RESULTS_API="https://zwiftpower.com/api3.php?do=profile_results&z=${ZWIFTPOWER_ID}"
DB_PATH="${HOME}/.local/share/zwift-race-finder/races.db"

echo "🚴 ZwiftPower Results Scraper"
echo "============================="
echo "Profile: ${PROFILE_URL}"

# Create temp directory for work
TEMP_DIR=$(mktemp -d)
trap "rm -rf ${TEMP_DIR}" EXIT

# Function to convert time string to minutes
time_to_minutes() {
    local time_str="$1"
    # Handle formats: "1:23:45" or "23:45" or "45"
    local parts=(${time_str//:/ })
    local minutes=0
    
    case ${#parts[@]} in
        3) minutes=$((${parts[0]} * 60 + ${parts[1]})) ;;
        2) minutes=$((${parts[0]} * 60 + ${parts[1]})) ;;
        1) minutes=${parts[0]} ;;
    esac
    
    echo "$minutes"
}

# Try API endpoint first (cleaner data)
echo "Fetching race results from ZwiftPower API..."
if curl -s -H "Accept: application/json" "${RESULTS_API}" -o "${TEMP_DIR}/results.json"; then
    # Check if we got valid JSON
    if jq empty "${TEMP_DIR}/results.json" 2>/dev/null; then
        echo "✓ Got API response"
        
        # Extract race results
        jq -r '.data[] | 
            select(.event_title != null) |
            [
                .event_date,
                .event_title,
                .category,
                .position_in_cat,
                .finish_time,
                .event_id // "unknown"
            ] | @csv' "${TEMP_DIR}/results.json" > "${TEMP_DIR}/races.csv" || true
            
        # Count results
        RACE_COUNT=$(wc -l < "${TEMP_DIR}/races.csv")
        echo "Found ${RACE_COUNT} race results"
    else
        echo "⚠️  API returned invalid JSON, falling back to HTML scraping"
        USE_HTML=1
    fi
else
    echo "⚠️  API request failed, falling back to HTML scraping"
    USE_HTML=1
fi

# Fallback to HTML scraping
if [[ "${USE_HTML:-0}" == "1" ]]; then
    echo "Fetching HTML profile page..."
    curl -s -A "Mozilla/5.0" "${PROFILE_URL}" -o "${TEMP_DIR}/profile.html"
    
    # Extract race results table using grep/sed
    # This is brittle but follows bash-first approach
    grep -A 1000 "table-striped" "${TEMP_DIR}/profile.html" | \
    grep -B 1000 "</table>" | \
    grep -E "<td.*>(.*)</td>" | \
    sed -E 's/<[^>]*>//g' | \
    sed 's/&nbsp;/ /g' > "${TEMP_DIR}/races_raw.txt"
    
    # Group every 7 lines (typical columns per race)
    paste -d ',' - - - - - - - < "${TEMP_DIR}/races_raw.txt" > "${TEMP_DIR}/races.csv" 2>/dev/null || true
fi

# Process results and prepare SQL
echo "Processing race data..."
{
    echo "BEGIN TRANSACTION;"
    echo "CREATE TABLE IF NOT EXISTS zwiftpower_results ("
    echo "    id INTEGER PRIMARY KEY AUTOINCREMENT,"
    echo "    date TEXT NOT NULL,"
    echo "    event_name TEXT NOT NULL,"
    echo "    category TEXT,"
    echo "    position TEXT,"
    echo "    time_minutes INTEGER NOT NULL,"
    echo "    time_text TEXT,"
    echo "    event_id TEXT,"
    echo "    scraped_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP"
    echo ");"
    
    while IFS=',' read -r date event cat pos time_text event_id; do
        # Clean up fields
        date=$(echo "$date" | tr -d '"' | xargs)
        event=$(echo "$event" | tr -d '"' | xargs)
        time_text=$(echo "$time_text" | tr -d '"' | xargs)
        
        # Convert time to minutes
        minutes=$(time_to_minutes "$time_text")
        
        if [[ -n "$date" && -n "$event" && "$minutes" -gt 0 ]]; then
            echo "INSERT INTO zwiftpower_results (date, event_name, category, position, time_minutes, time_text, event_id)"
            echo "VALUES ('$date', '${event//\'/\'\'}', '$cat', '$pos', $minutes, '$time_text', '${event_id:-unknown}');"
        fi
    done < "${TEMP_DIR}/races.csv"
    
    echo "COMMIT;"
} > "${TEMP_DIR}/import.sql"

# Import to SQLite
if [[ -f "$DB_PATH" ]]; then
    echo "Importing results to database..."
    sqlite3 "$DB_PATH" < "${TEMP_DIR}/import.sql"
    
    # Show summary
    echo ""
    echo "Summary of imported races:"
    echo "========================="
    sqlite3 "$DB_PATH" <<EOF
SELECT COUNT(*) || ' total races' FROM zwiftpower_results;
SELECT 'Shortest: ' || MIN(time_minutes) || ' minutes' FROM zwiftpower_results;
SELECT 'Longest: ' || MAX(time_minutes) || ' minutes' FROM zwiftpower_results;
SELECT 'Average: ' || ROUND(AVG(time_minutes)) || ' minutes' FROM zwiftpower_results;

.mode column
.headers on
SELECT event_name, COUNT(*) as times_raced 
FROM zwiftpower_results 
GROUP BY event_name 
ORDER BY times_raced DESC 
LIMIT 10;
EOF
else
    echo "⚠️  Database not found at $DB_PATH"
    echo "Run 'zwift-race-finder' first to create the database"
fi

# Try to match events with route IDs
echo ""
echo "Attempting to match events with known routes..."
sqlite3 "$DB_PATH" <<EOF 2>/dev/null || true
UPDATE zwiftpower_results zr
SET route_id = (
    SELECT route_id FROM routes r 
    WHERE zr.event_name LIKE '%' || r.name || '%'
    LIMIT 1
)
WHERE route_id IS NULL;

SELECT 'Matched ' || COUNT(*) || ' events with route IDs' 
FROM zwiftpower_results 
WHERE route_id IS NOT NULL;
EOF

echo ""
echo "✅ Done! Results saved to database."
echo ""
echo "Next steps:"
echo "1. Review unmatched events: sqlite3 $DB_PATH 'SELECT DISTINCT event_name FROM zwiftpower_results WHERE route_id IS NULL;'"
echo "2. Add route mappings for common events"
echo "3. Use this data for regression testing"