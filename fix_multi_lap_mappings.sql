-- Fix Multi-Lap Race Mappings
-- These races are showing incorrect predictions because they're multi-lap events
-- Created: 2025-05-27

-- 3R Racing generic events are actually 3-lap Volcano Flat races
-- Volcano Flat: 12.3km x 3 laps = 36.9km total
-- At 30.9 km/h, this should take ~72 minutes, matching actual times (52-79 min)
UPDATE race_results 
SET route_id = 3369744027  -- Keep Volcano Flat but note it's 3 laps
WHERE event_name = '3R Racing'
  AND route_id = 3369744027;

-- Add note about multi-lap nature
-- Since we can't change the distance in the routes table (it's for single lap),
-- we need to handle this in the application logic or create a new mapping table

-- Team DRAFT Monday Race appears to be 2 laps of Castle to Castle
-- Castle to Castle: 24.5km x 2 laps = 49km total
-- At 30.9 km/h, this should take ~95 minutes, close to actual times (75-91 min)
UPDATE race_results 
SET route_id = 3742187716  -- Keep Castle to Castle but note it's 2 laps
WHERE event_name = 'Team DRAFT Monday Race'
  AND route_id = 3742187716;

-- Create a multi-lap mapping table to handle these cases
CREATE TABLE IF NOT EXISTS multi_lap_events (
    event_name_pattern TEXT PRIMARY KEY,
    route_id INTEGER NOT NULL,
    lap_count INTEGER NOT NULL,
    notes TEXT,
    FOREIGN KEY (route_id) REFERENCES routes(route_id)
);

-- Insert known multi-lap event patterns
INSERT OR REPLACE INTO multi_lap_events (event_name_pattern, route_id, lap_count, notes) VALUES
('3R Racing', 3369744027, 3, 'Generic 3R Racing is 3 laps of Volcano Flat'),
('Team DRAFT Monday Race', 3742187716, 2, 'Monday race is 2 laps of Castle to Castle'),
('KISS Racing', 2139400188, 3, 'KISS Racing is typically 3 laps based on 65min for 35km route'),
('EVR Winter Series', 3742187716, 2, 'EVR Winter Series appears to be 2+ laps based on 92-98min times'),
('DIRT Dadurday Chase Race', 2139400188, 2, 'Chase races often 2 laps, 65-71min for 35km route');

-- Show the impact of multi-lap recognition
SELECT 
    'Multi-Lap Event Analysis' as Report;

SELECT 
    mle.event_name_pattern,
    r.name as route_name,
    r.distance_km as single_lap_km,
    mle.lap_count,
    r.distance_km * mle.lap_count as total_km,
    COUNT(rr.id) as race_count,
    AVG(rr.actual_minutes) as avg_actual_minutes,
    ROUND((r.distance_km * mle.lap_count) / 30.9 * 60) as expected_minutes
FROM multi_lap_events mle
JOIN routes r ON mle.route_id = r.route_id
LEFT JOIN race_results rr ON rr.event_name = mle.event_name_pattern AND rr.route_id = mle.route_id
GROUP BY mle.event_name_pattern
ORDER BY race_count DESC;

-- Check remaining high-error events
SELECT 
    'Remaining High Error Events' as Report;

SELECT 
    rr.event_name,
    r.name as route_name,
    COUNT(*) as race_count,
    AVG(rr.actual_minutes) as avg_actual,
    r.distance_km,
    ROUND(r.distance_km / 30.9 * 60) as expected_single_lap,
    ROUND(AVG(rr.actual_minutes) / (r.distance_km / 30.9 * 60)) as likely_laps
FROM race_results rr
JOIN routes r ON rr.route_id = r.route_id
WHERE rr.route_id != 9999
GROUP BY rr.event_name, r.route_id
HAVING ABS(avg_actual - (r.distance_km / 30.9 * 60)) > 20  -- More than 20 min difference
ORDER BY race_count DESC
LIMIT 10;