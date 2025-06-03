-- Apply manual route mappings
BEGIN TRANSACTION;

-- EVO CC Race Series (already done)
-- UPDATE race_results SET route_id = 3369744027 WHERE event_name LIKE 'EVO CC%Race Series%' AND route_id = 9999;

-- Sydkysten Race
UPDATE race_results SET route_id = 9001 WHERE (event_name LIKE '%Sydkysten%Race%' OR event_name LIKE '%KST Race%') AND route_id = 9999;

-- Tofu Tornado Race
UPDATE race_results SET route_id = 3369744027 WHERE event_name LIKE '%Tofu Tornado%' AND route_id = 9999;

-- CAT & MOUSE KZR CHASE RACE
UPDATE race_results SET route_id = 9002 WHERE event_name LIKE '%CAT & MOUSE%CHASE%' AND route_id = 9999;

-- DBR races
UPDATE race_results SET route_id = 2143464829 WHERE event_name LIKE 'DBR%Race%' AND route_id = 9999;

-- ZHR Morning Tea Race
UPDATE race_results SET route_id = 9003 WHERE event_name LIKE 'ZHR Morning Tea Race%' AND route_id = 9999;

-- Zwift TT Club Racing - Watopia's Waistband
UPDATE race_results SET route_id = 3733109212 WHERE event_name LIKE '%TT Club Racing%Waistband%' AND route_id = 9999;

-- ZSUN events
UPDATE race_results SET route_id = 1258415487 WHERE event_name LIKE 'ZSUN%' AND route_id = 9999;

-- Team VEGAN races
UPDATE race_results SET route_id = 3369744027 WHERE event_name LIKE '%VEGAN%' AND route_id = 9999;

COMMIT;

-- Show results
SELECT 'Mapping Results:' as Report;
SELECT 
    CASE route_id
        WHEN 3369744027 THEN 'Volcano Flat'
        WHEN 2143464829 THEN 'Watopia Flat Route'
        WHEN 3733109212 THEN 'Watopias Waistband'
        WHEN 1258415487 THEN 'Bell Lap'
        WHEN 9001 THEN 'Sydkysten (placeholder)'
        WHEN 9002 THEN 'CAT & MOUSE (placeholder)'
        WHEN 9003 THEN 'ZHR Route (placeholder)'
        ELSE 'Unknown'
    END as route_name,
    route_id,
    COUNT(*) as events_mapped
FROM race_results 
WHERE route_id IN (3369744027, 2143464829, 3733109212, 1258415487, 9001, 9002, 9003)
GROUP BY route_id
ORDER BY events_mapped DESC;

SELECT '---' as separator;
SELECT 'Still unmapped: ' || COUNT(*) as remaining FROM race_results WHERE route_id = 9999;