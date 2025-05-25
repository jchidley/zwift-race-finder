-- Route Mappings for Zwift Race Finder
-- Maps common event names to proper route IDs
-- Based on Jack's race history and route research

-- First, let's add the most common routes to the routes table
-- Route data from ZwiftHacks.com and Zwift Insider

-- 3R Racing routes (most common in history)
-- 3R often uses Volcano Flat or Volcano Circuit
INSERT OR IGNORE INTO routes (route_id, distance_km, elevation_m, name, world, surface) VALUES
(3369744027, 12.3, 45, "Volcano Flat", "Watopia", "road"),
(3742187716, 24.5, 168, "Castle to Castle", "Makuri Islands", "road"),
(2143464829, 33.4, 170, "Watopia Flat Route", "Watopia", "road");

-- EVO CC often uses classic Watopia routes
INSERT OR IGNORE INTO routes (route_id, distance_km, elevation_m, name, world, surface) VALUES
(1258415487, 14.1, 59, "Bell Lap", "Crit City", "road"),
(2698009951, 22.9, 80, "Downtown Dolphin", "Crit City", "road");

-- Team DRAFT Monday Race - often uses variable routes
INSERT OR IGNORE INTO routes (route_id, distance_km, elevation_m, name, world, surface) VALUES
(2474227587, 27.8, 218, "Cobbled Climbs", "Richmond", "mixed"),
(2927651296, 67.5, 654, "Makuri Pretzel", "Makuri Islands", "road");

-- KISS Racing - uses their custom 100km route
INSERT OR IGNORE INTO routes (route_id, distance_km, elevation_m, name, world, surface) VALUES
(2474227587, 100.0, 892, "KISS 100", "Watopia", "road");

-- Ottawa TopSpeed Race
INSERT OR IGNORE INTO routes (route_id, distance_km, elevation_m, name, world, surface) VALUES
(1656629976, 19.8, 142, "Ottawa TopSpeed", "Various", "road");

-- Now update race_results to use proper route IDs based on event names
-- This is a best-effort mapping based on common patterns

-- 3R Racing events
UPDATE race_results 
SET route_id = 3369744027  -- Volcano Flat
WHERE event_name = '3R Racing' 
   OR event_name LIKE '3R Volcano Flat%'
   OR event_name LIKE '3R Volcano Circuit%';

-- EVO CC Race Series - often uses Bell Lap
UPDATE race_results 
SET route_id = 1258415487  -- Bell Lap
WHERE event_name = 'EVO CC Race Series';

-- Team DRAFT Monday Race - varies, but often Makuri routes
UPDATE race_results 
SET route_id = 3742187716  -- Castle to Castle
WHERE event_name = 'Team DRAFT Monday Race';

-- KISS Racing
UPDATE race_results 
SET route_id = 2474227587  -- KISS 100
WHERE event_name = 'KISS Racing';

-- Ottawa TopSpeed Race
UPDATE race_results 
SET route_id = 1656629976  -- Ottawa TopSpeed
WHERE event_name = 'Ottawa TopSpeed Race';

-- EVR Winter Series - often uses Watopia Flat
UPDATE race_results 
SET route_id = 2143464829  -- Watopia Flat Route
WHERE event_name = 'EVR Winter Series';

-- DIRT races often use gravel routes
UPDATE race_results 
SET route_id = 2474227587  -- Cobbled Climbs (mixed surface)
WHERE event_name LIKE 'DIRT%';

-- Show summary of mappings
SELECT 
    route_id,
    COUNT(*) as races_mapped,
    GROUP_CONCAT(DISTINCT event_name) as event_names
FROM race_results 
WHERE route_id != 9999
GROUP BY route_id
ORDER BY races_mapped DESC;

-- Show remaining unmapped events
SELECT 
    event_name,
    COUNT(*) as times_raced
FROM race_results 
WHERE route_id = 9999
GROUP BY event_name
ORDER BY times_raced DESC
LIMIT 20;