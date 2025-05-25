#!/bin/bash
# Route research helper - analyzes race data to help identify routes

set -euo pipefail

DB_PATH="${HOME}/.local/share/zwift-race-finder/races.db"

# Function to analyze a specific event pattern
analyze_event() {
    local pattern="$1"
    echo "🔍 Analyzing events matching: $pattern"
    echo "================================================"
    
    sqlite3 -column -header "$DB_PATH" << SQL
SELECT 
    rr.event_name,
    COUNT(*) as races,
    ROUND(AVG(rr.actual_minutes), 0) as avg_minutes,
    MIN(rr.actual_minutes) as min_minutes,
    MAX(rr.actual_minutes) as max_minutes,
    GROUP_CONCAT(DISTINCT substr(rr.race_date, 1, 7)) as months_raced
FROM race_results rr
WHERE rr.event_name LIKE '%${pattern}%'
GROUP BY rr.event_name
ORDER BY races DESC;
SQL
    echo ""
}

# Main menu
if [[ $# -eq 0 ]]; then
    echo "🚴 Zwift Route Research Helper"
    echo "=============================="
    echo ""
    
    # Show events with obvious route info
    echo "📍 Events with route info in name:"
    sqlite3 -column -header "$DB_PATH" << 'SQL'
SELECT 
    event_name,
    times_seen,
    CASE
        WHEN event_name LIKE '%Volcano Flat%' THEN 'Volcano Flat (1015)'
        WHEN event_name LIKE '%Volcano Circuit%' THEN 'Volcano Circuit (1016)'
        WHEN event_name LIKE '%Alpe Du Zwift%' THEN 'Alpe du Zwift (6)'
        WHEN event_name LIKE '%Innsbruckring%' THEN 'Innsbruckring (236)'
        WHEN event_name LIKE '%Watopia Figure 8%' THEN 'Watopia Figure 8 (10)'
        WHEN event_name LIKE '%The Pretzel%' THEN 'The Pretzel (14)'
        WHEN event_name LIKE '%Tempus Fugit%' THEN 'Tempus Fugit (11)'
        ELSE 'Unknown'
    END as likely_route
FROM unknown_routes
WHERE event_name LIKE '%km%' 
   OR event_name LIKE '%Volcano%'
   OR event_name LIKE '%Alpe%'
   OR event_name LIKE '%Innsbruck%'
   OR event_name LIKE '%Figure%'
ORDER BY times_seen DESC
LIMIT 15;
SQL
    
    echo ""
    echo "📊 Event series analysis:"
    echo ""
    echo "3R Events pattern:"
    analyze_event "3R%"
    
    echo "Usage: $0 <search_pattern>"
    echo "Example: $0 'EVO CC'"
else
    analyze_event "$1"
fi

# Show helpful queries
echo ""
echo "💡 Helpful SQL queries:"
echo ""
echo "-- Find all races for a specific event:"
echo "SELECT * FROM race_results WHERE event_name LIKE '%PATTERN%';"
echo ""
echo "-- Compare your times across similar events:"
echo "SELECT event_name, actual_minutes, race_date FROM race_results"
echo "WHERE event_name LIKE '%Volcano%' ORDER BY actual_minutes;"