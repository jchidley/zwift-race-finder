-- Manual Route Mappings for Common Zwift Race Series
-- Based on research of typical routes used by these series
-- Created: 2025-05-27

-- First, add routes that weren't already in the database
-- Route data from whatsonzwift.com and zwiftinsider.com

-- Watopia's Waistband for TT Club Racing
INSERT OR IGNORE INTO routes (route_id, distance_km, elevation_m, name, world, surface) VALUES
(3733109212, 25.4, 96, "Watopia's Waistband", "Watopia", "road");

-- Common sprint race routes for various series
INSERT OR IGNORE INTO routes (route_id, distance_km, elevation_m, name, world, surface) VALUES
-- These are estimates based on typical routes used, actual route_id would need confirmation
(9001, 29.6, 150, "Sydkysten Typical Route", "Various", "road"),  -- Placeholder ID
(9002, 40.0, 200, "CAT & MOUSE Chase Route", "Various", "road"),  -- Placeholder ID
(9003, 49.0, 300, "ZHR Morning Tea Route", "London", "road");     -- Placeholder ID

-- Newly discovered routes from research
INSERT OR IGNORE INTO routes (route_id, distance_km, elevation_m, name, world, surface) VALUES
(1917017591, 5.7, 335, "Mountain Mash", "Watopia", "road"),      -- For Restart Monday Mash
(2128890027, 18.9, 16, "Tempus Fugit", "Watopia", "road"),       -- For TEAM VTO POWERPUSH
(3366225080, 19.2, 59, "Tick Tock", "Watopia", "road");          -- For The Bump Sprint Race

-- Map events to routes based on research findings

-- EVO CC Race Series
-- Note: They rotate routes, but often use shorter 30-40km routes
-- Without specific rotation schedule, we'll use a common sprint route
UPDATE race_results 
SET route_id = 3369744027  -- Volcano Flat as default
WHERE event_name LIKE 'EVO CC%Race Series%'
  AND route_id = 9999;

-- Sydkysten Race - 29.6km route
UPDATE race_results 
SET route_id = 9001  -- Placeholder for Sydkysten route
WHERE (event_name LIKE '%Sydkysten%Race%' 
   OR event_name LIKE '%KST Race%')
  AND route_id = 9999;

-- Tofu Tornado Race - varies 32-70km, typically around 35km
UPDATE race_results 
SET route_id = 3369744027  -- Using Volcano Flat as approximation
WHERE event_name LIKE '%Tofu Tornado%'
  AND route_id = 9999;

-- CAT & MOUSE KZR CHASE RACE - 40km chase format
UPDATE race_results 
SET route_id = 9002  -- Placeholder for chase route
WHERE event_name LIKE '%CAT & MOUSE%CHASE%'
  AND route_id = 9999;

-- DBR races (Danish Bike Riders)
-- They run multiple races throughout the week with varying routes
UPDATE race_results 
SET route_id = 2143464829  -- Watopia Flat Route as default
WHERE event_name LIKE 'DBR%Race%'
  AND route_id = 9999;

-- ZHR Morning Tea Race - 49km London crit
UPDATE race_results 
SET route_id = 9003  -- Placeholder for ZHR route
WHERE event_name LIKE 'ZHR Morning Tea Race%'
  AND route_id = 9999;

-- Zwift TT Club Racing - Watopia's Waistband
UPDATE race_results 
SET route_id = 3733109212  -- Watopia's Waistband
WHERE event_name LIKE '%TT Club Racing%Waistband%'
  AND route_id = 9999;

-- Additional mappings for high-frequency events

-- ZSUN events often use shorter routes
UPDATE race_results 
SET route_id = 1258415487  -- Bell Lap
WHERE event_name LIKE 'ZSUN%'
  AND route_id = 9999;

-- Team VEGAN races
UPDATE race_results 
SET route_id = 3369744027  -- Volcano Flat
WHERE event_name LIKE '%VEGAN%'
  AND route_id = 9999;

-- Restart Monday Mash - likely uses Mountain Mash route (55x occurrences)
-- Mountain Mash: 5.7km, 335m elevation, Categories C&D Zwift Games route
UPDATE race_results 
SET route_id = 1917017591  -- Current route_id being used by Monday events
WHERE event_name = 'Restart Monday Mash'
  AND route_id = 9999;

-- TEAM VTO POWERPUSH - uses Tempus Fugit route (37x occurrences)
-- Tempus Fugit: 18.9km, 16m elevation, flattest route in Zwift
UPDATE race_results 
SET route_id = 2128890027  -- Route ID from unknown_routes table
WHERE event_name LIKE '%VTO POWERPUSH%'
  AND route_id = 9999;

-- The Bump Sprint Race - uses Tick Tock route (27x occurrences)
-- Tick Tock: 19.2km, 59m elevation, Desert Pens start
UPDATE race_results 
SET route_id = 3366225080  -- Route ID from unknown_routes table
WHERE event_name = 'The Bump Sprint Race'
  AND route_id = 9999;

-- Show summary of new mappings
SELECT 
    'Summary of Manual Route Mappings' as Report;

SELECT 
    CASE 
        WHEN route_id = 9001 THEN 'Sydkysten Route (placeholder)'
        WHEN route_id = 9002 THEN 'CAT & MOUSE Route (placeholder)'
        WHEN route_id = 9003 THEN 'ZHR Route (placeholder)'
        ELSE r.name 
    END as route_name,
    route_id,
    COUNT(*) as events_mapped,
    GROUP_CONCAT(DISTINCT 
        CASE 
            WHEN event_name LIKE 'EVO CC%' THEN 'EVO CC Series'
            WHEN event_name LIKE '%Sydkysten%' THEN 'Sydkysten Race'
            WHEN event_name LIKE '%Tofu Tornado%' THEN 'Tofu Tornado'
            WHEN event_name LIKE '%CAT & MOUSE%' THEN 'CAT & MOUSE'
            WHEN event_name LIKE 'DBR%' THEN 'DBR Races'
            WHEN event_name LIKE 'ZHR%' THEN 'ZHR Races'
            WHEN event_name LIKE '%TT Club%' THEN 'TT Club Racing'
            ELSE event_name
        END
    ) as series_names
FROM race_results rr
LEFT JOIN routes r ON rr.route_id = r.route_id
WHERE rr.route_id IN (
    3733109212,  -- Watopia's Waistband
    3369744027,  -- Volcano Flat
    2143464829,  -- Watopia Flat Route
    1258415487,  -- Bell Lap
    9001, 9002, 9003  -- Placeholders
)
GROUP BY rr.route_id
ORDER BY events_mapped DESC;

-- Show remaining high-frequency unmapped events
SELECT 
    'Remaining Unmapped High-Frequency Events' as Report;

SELECT 
    event_name,
    COUNT(*) as occurrences
FROM race_results 
WHERE route_id = 9999
GROUP BY event_name
HAVING COUNT(*) >= 3
ORDER BY occurrences DESC
LIMIT 20;

-- Notes for future improvements:
-- 1. Route IDs 9001-9003 are placeholders and need real route_ids from Zwift API
-- 2. EVO CC rotates routes weekly - would benefit from date-based mapping
-- 3. DBR races vary by day of week - could map Monday/Tuesday/etc to specific routes
-- 4. Some series like Tofu Tornado have XL versions with different distances
-- 5. Chase races often use flatter routes to maintain pack dynamics