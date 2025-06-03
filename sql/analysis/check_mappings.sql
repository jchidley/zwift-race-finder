-- Check manual mappings results
SELECT 'Events mapped from route 9999:' as Report;

SELECT 
    event_name,
    route_id,
    actual_minutes
FROM race_results 
WHERE event_name LIKE 'EVO CC%'
   OR event_name LIKE '%Sydkysten%'
   OR event_name LIKE '%Tofu Tornado%'
   OR event_name LIKE '%CAT % MOUSE%'
   OR event_name LIKE 'DBR%Race%'
   OR event_name LIKE 'ZHR Morning Tea%'
   OR event_name LIKE '%TT Club%Waistband%'
ORDER BY event_name, race_date DESC
LIMIT 20;

SELECT '---' as separator;
SELECT 'Summary by route:' as Report;

SELECT 
    route_id,
    COUNT(*) as events_mapped
FROM race_results 
WHERE route_id IN (3733109212, 3369744027, 2143464829, 1258415487, 9001, 9002, 9003)
GROUP BY route_id
ORDER BY events_mapped DESC;

SELECT '---' as separator;
SELECT 'Remaining unmapped (9999):' as Report;
SELECT COUNT(*) as still_unmapped FROM race_results WHERE route_id = 9999;