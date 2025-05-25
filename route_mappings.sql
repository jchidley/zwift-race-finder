-- Route mappings for Zwift Race Finder
-- Based on analysis of Jack's race history

-- First, let's add the routes we can identify from event names
-- Route IDs from ZwiftHacks.com

-- Volcano routes
INSERT OR IGNORE INTO routes (route_id, distance_km, elevation_m, name, world, surface)
VALUES 
    (1015, 12.2, 46, 'Volcano Flat', 'Watopia', 'road'),
    (1016, 4.4, 20, 'Volcano Circuit', 'Watopia', 'road'),
    (1017, 21.3, 155, 'Volcano Climb', 'Watopia', 'road');

-- Alpe du Zwift
INSERT OR IGNORE INTO routes (route_id, distance_km, elevation_m, name, world, surface)
VALUES (6, 12.2, 1035, 'Alpe du Zwift', 'Watopia', 'road');

-- Innsbruckring
INSERT OR IGNORE INTO routes (route_id, distance_km, elevation_m, name, world, surface)
VALUES (236, 8.8, 77, 'Innsbruckring', 'Innsbruck', 'road');

-- Other common routes
INSERT OR IGNORE INTO routes (route_id, distance_km, elevation_m, name, world, surface)
VALUES 
    (10, 12.5, 104, 'Watopia Figure 8', 'Watopia', 'road'),
    (11, 17.1, 51, 'Tempus Fugit', 'Watopia', 'road'),
    (14, 14.9, 168, 'The Pretzel', 'Watopia', 'road'),
    (2, 10.0, 52, 'Greater London Flat', 'London', 'road'),
    (1, 7.5, 48, 'Classique', 'London', 'road'),
    (210, 16.2, 142, 'Richmond UCI', 'Richmond', 'road'),
    (30, 19.7, 173, 'Sand and Sequoias', 'Watopia', 'road'),
    (22, 21.3, 55, 'Tick Tock', 'Watopia', 'road'),
    (3, 10.1, 54, 'Watopia Flat Route', 'Watopia', 'road');

-- Now update race_results to use proper route_ids where we can match
-- This is based on the event names containing route information

-- Volcano Flat races
UPDATE race_results 
SET route_id = 1015
WHERE route_id = 9999 
  AND event_name LIKE '%Volcano Flat%';

-- Volcano Circuit races  
UPDATE race_results 
SET route_id = 1016
WHERE route_id = 9999 
  AND event_name LIKE '%Volcano Circuit%';

-- Alpe du Zwift races
UPDATE race_results 
SET route_id = 6
WHERE route_id = 9999 
  AND (event_name LIKE '%Alpe Du Zwift%' OR event_name LIKE '%Alpe du Zwift%');

-- Innsbruckring races
UPDATE race_results 
SET route_id = 236
WHERE route_id = 9999 
  AND event_name LIKE '%Innsbruckring%';

-- Update unknown_routes to remove mapped events
DELETE FROM unknown_routes 
WHERE event_name IN (
    SELECT DISTINCT event_name 
    FROM race_results 
    WHERE route_id != 9999
);

-- Show summary of mappings
SELECT 'Routes mapped:' as status, COUNT(DISTINCT route_id) as count 
FROM race_results WHERE route_id != 9999
UNION ALL
SELECT 'Routes unmapped:', COUNT(DISTINCT event_name)
FROM race_results WHERE route_id = 9999;