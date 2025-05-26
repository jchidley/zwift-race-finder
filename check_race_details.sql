-- Check 3R Racing details
SELECT 'Checking 3R Racing - actual times vs single lap prediction' as info;
SELECT event_name, actual_minutes, route_id, race_date 
FROM race_results 
WHERE event_name LIKE '%3R Racing%' 
ORDER BY race_date DESC;

-- Check prediction for single lap
SELECT '3R Racing route (Volcano Flat): ' || distance_km || 'km, ' || elevation_m || 'm' as route_info,
       ROUND(distance_km / 30.9 * 60) as expected_minutes_single_lap
FROM routes WHERE route_id = 3369744027;

-- Check Team DRAFT Monday details
SELECT '' as blank;
SELECT 'Checking Team DRAFT Monday Race - actual times vs single lap prediction' as info;
SELECT event_name, actual_minutes, route_id, race_date 
FROM race_results 
WHERE event_name LIKE '%Team DRAFT Monday Race%' 
ORDER BY race_date DESC;

-- Check prediction for single lap
SELECT 'Team DRAFT route (Castle to Castle): ' || distance_km || 'km, ' || elevation_m || 'm' as route_info,
       ROUND(distance_km / 30.9 * 60) as expected_minutes_single_lap
FROM routes WHERE route_id = 3742187716;
