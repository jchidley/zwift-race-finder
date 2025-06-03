#!/bin/bash
# Fix unknown_routes table to show all unique events

set -euo pipefail

DB_PATH="${HOME}/.local/share/zwift-race-finder/races.db"

echo "🔧 Fixing unknown_routes tracking..."
echo ""

# First check the schema
echo "Checking unknown_routes schema..."
sqlite3 "$DB_PATH" ".schema unknown_routes"
echo ""

# Clear and repopulate unknown_routes
sqlite3 "$DB_PATH" << 'SQL'
-- Clear existing unknown routes
DELETE FROM unknown_routes;

-- Get next available route_id starting from 10000
-- Insert all unique events with unique route_ids
WITH numbered_events AS (
    SELECT 
        event_name,
        COUNT(*) as times_seen,
        ROW_NUMBER() OVER (ORDER BY COUNT(*) DESC) + 9999 as new_route_id
    FROM race_results
    WHERE route_id = 9999
    GROUP BY event_name
)
INSERT INTO unknown_routes (route_id, event_name, event_type, times_seen)
SELECT new_route_id, event_name, 'race', times_seen
FROM numbered_events;
SQL

# Show results
echo "✅ Updated unknown_routes table"
echo ""
echo "📊 Summary:"
sqlite3 -column -header "$DB_PATH" << 'SQL'
SELECT COUNT(*) as total_unknown_events,
       SUM(times_seen) as total_races
FROM unknown_routes;
SQL

echo ""
echo "🔝 Top 20 unknown routes by frequency:"
sqlite3 -column -header "$DB_PATH" << 'SQL'
SELECT event_name, times_seen
FROM unknown_routes
ORDER BY times_seen DESC
LIMIT 20;
SQL

echo ""
echo "📈 Events by pattern:"
sqlite3 -column -header "$DB_PATH" << 'SQL'
SELECT 
    CASE 
        WHEN event_name LIKE '%3R%' THEN '3R Events'
        WHEN event_name LIKE '%WTRL%' THEN 'WTRL Events'
        WHEN event_name LIKE '%ZRacing%' THEN 'ZRacing Events'
        WHEN event_name LIKE '%EVO%' THEN 'EVO Events'
        WHEN event_name LIKE '%KISS%' THEN 'KISS Events'
        WHEN event_name LIKE '%DIRT%' THEN 'DIRT Events'
        WHEN event_name LIKE '%Team DRAFT%' THEN 'Team DRAFT Events'
        ELSE 'Other Events'
    END as event_series,
    COUNT(*) as unique_events,
    SUM(times_seen) as total_races
FROM unknown_routes
GROUP BY event_series
ORDER BY total_races DESC;
SQL