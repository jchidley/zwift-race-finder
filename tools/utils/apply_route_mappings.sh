#!/bin/bash
# Apply route mappings to race results

set -euo pipefail

DB_PATH="${HOME}/.local/share/zwift-race-finder/races.db"

echo "🗺️  Applying route mappings..."
echo ""

# Apply each mapping and show results
sqlite3 "$DB_PATH" << 'SQL'
-- Volcano Circuit races  
UPDATE race_results 
SET route_id = 1016
WHERE route_id = 9999 
  AND event_name LIKE '%Volcano Circuit%';
SELECT 'Volcano Circuit mapped: ' || changes() as result;

-- Alpe du Zwift races
UPDATE race_results 
SET route_id = 6
WHERE route_id = 9999 
  AND (event_name LIKE '%Alpe Du Zwift%' OR event_name LIKE '%Alpe du Zwift%');
SELECT 'Alpe du Zwift mapped: ' || changes() as result;

-- Innsbruckring races
UPDATE race_results 
SET route_id = 236
WHERE route_id = 9999 
  AND event_name LIKE '%Innsbruckring%';
SELECT 'Innsbruckring mapped: ' || changes() as result;

-- Greater London races
UPDATE race_results 
SET route_id = 2
WHERE route_id = 9999 
  AND event_name LIKE '%Greater London%';
SELECT 'Greater London mapped: ' || changes() as result;

-- Richmond UCI races
UPDATE race_results 
SET route_id = 210
WHERE route_id = 9999 
  AND event_name LIKE '%Richmond UCI%';
SELECT 'Richmond UCI mapped: ' || changes() as result;

-- Sand and Sequoias races
UPDATE race_results 
SET route_id = 30
WHERE route_id = 9999 
  AND event_name LIKE '%Sand & Sequoias%';
SELECT 'Sand and Sequoias mapped: ' || changes() as result;

-- Tick Tock races
UPDATE race_results 
SET route_id = 22
WHERE route_id = 9999 
  AND event_name LIKE '%Tick Tock%';
SELECT 'Tick Tock mapped: ' || changes() as result;

-- Watopia Flat Route races
UPDATE race_results 
SET route_id = 3
WHERE route_id = 9999 
  AND event_name LIKE '%Watopia Flat Route%';
SELECT 'Watopia Flat Route mapped: ' || changes() as result;

-- Classique races
UPDATE race_results 
SET route_id = 1
WHERE route_id = 9999 
  AND event_name LIKE '%Classique%';
SELECT 'Classique mapped: ' || changes() as result;
SQL

# Update unknown_routes
echo ""
echo "Cleaning up unknown_routes..."
sqlite3 "$DB_PATH" << 'SQL'
DELETE FROM unknown_routes 
WHERE event_name IN (
    SELECT DISTINCT event_name 
    FROM race_results 
    WHERE route_id != 9999
);
SELECT 'Removed from unknown_routes: ' || changes() as result;
SQL

# Show summary
echo ""
echo "✅ Mapping complete!"
echo ""
echo "📊 Summary:"
sqlite3 -column -header "$DB_PATH" << 'SQL'
SELECT 
    COUNT(DISTINCT CASE WHEN route_id != 9999 THEN route_id END) as mapped_routes,
    COUNT(CASE WHEN route_id != 9999 THEN 1 END) as mapped_races,
    COUNT(CASE WHEN route_id = 9999 THEN 1 END) as unmapped_races,
    COUNT(DISTINCT CASE WHEN route_id = 9999 THEN event_name END) as unmapped_events
FROM race_results;
SQL

echo ""
echo "🏆 Top mapped routes:"
sqlite3 -column -header "$DB_PATH" << 'SQL'
SELECT 
    r.name as route_name,
    r.distance_km,
    r.elevation_m,
    COUNT(*) as races,
    ROUND(AVG(rr.actual_minutes), 0) as avg_minutes
FROM race_results rr
JOIN routes r ON rr.route_id = r.route_id
WHERE rr.route_id != 9999
GROUP BY rr.route_id
ORDER BY races DESC
LIMIT 10;
SQL