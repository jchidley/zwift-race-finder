-- Analyze actual speeds from Jack's race history
-- Focus on races where we can extract distance from notes

-- First, let's see what distances are recorded for each event type
SELECT 
    event_name,
    COUNT(*) as races,
    MIN(actual_minutes) as min_time,
    MAX(actual_minutes) as max_time,
    -- Extract distance from notes manually for common patterns
    CASE
        WHEN notes LIKE '%31km%' THEN 31
        WHEN notes LIKE '%37km%' THEN 37
        WHEN notes LIKE '%40km%' THEN 40
        WHEN notes LIKE '%41km%' THEN 41
        WHEN notes LIKE '%43km%' THEN 43
        WHEN notes LIKE '%45km%' THEN 45
        WHEN notes LIKE '%19km%' THEN 19
        WHEN notes LIKE '%23km%' THEN 23
        WHEN notes LIKE '%27km%' THEN 27
        WHEN notes LIKE '%36km%' THEN 36
        WHEN notes LIKE '%39km%' THEN 39
        ELSE NULL
    END as distance_km
FROM race_results
WHERE zwift_score < 200  -- Cat D
GROUP BY event_name, distance_km
HAVING distance_km IS NOT NULL
ORDER BY races DESC
LIMIT 20;

-- Calculate Jack's actual average speeds by category
SELECT 
    'Cat D (Jack)' as category,
    COUNT(*) as total_races,
    ROUND(AVG(
        CASE
            WHEN notes LIKE '%31km%' THEN 31.0 * 60 / actual_minutes
            WHEN notes LIKE '%37km%' THEN 37.0 * 60 / actual_minutes
            WHEN notes LIKE '%40km%' THEN 40.0 * 60 / actual_minutes
            WHEN notes LIKE '%41km%' THEN 41.0 * 60 / actual_minutes
            WHEN notes LIKE '%43km%' THEN 43.0 * 60 / actual_minutes
            WHEN notes LIKE '%45km%' THEN 45.0 * 60 / actual_minutes
            WHEN notes LIKE '%19km%' THEN 19.0 * 60 / actual_minutes
            WHEN notes LIKE '%23km%' THEN 23.0 * 60 / actual_minutes
            WHEN notes LIKE '%27km%' THEN 27.0 * 60 / actual_minutes
            WHEN notes LIKE '%36km%' THEN 36.0 * 60 / actual_minutes
            WHEN notes LIKE '%39km%' THEN 39.0 * 60 / actual_minutes
        END
    ), 1) as avg_speed_kmh
FROM race_results
WHERE zwift_score < 200
  AND (notes LIKE '%km%')
  AND actual_minutes > 0;

-- Show specific examples with calculated speeds
SELECT 
    event_name,
    actual_minutes,
    notes,
    CASE
        WHEN notes LIKE '%31km%' THEN 31
        WHEN notes LIKE '%37km%' THEN 37
        WHEN notes LIKE '%40km%' THEN 40
        WHEN notes LIKE '%41km%' THEN 41
        WHEN notes LIKE '%43km%' THEN 43
        WHEN notes LIKE '%45km%' THEN 45
        ELSE NULL
    END as distance_km,
    ROUND(CASE
        WHEN notes LIKE '%31km%' THEN 31.0 * 60 / actual_minutes
        WHEN notes LIKE '%37km%' THEN 37.0 * 60 / actual_minutes
        WHEN notes LIKE '%40km%' THEN 40.0 * 60 / actual_minutes
        WHEN notes LIKE '%41km%' THEN 41.0 * 60 / actual_minutes
        WHEN notes LIKE '%43km%' THEN 43.0 * 60 / actual_minutes
        WHEN notes LIKE '%45km%' THEN 45.0 * 60 / actual_minutes
    END, 1) as speed_kmh
FROM race_results
WHERE zwift_score < 200
  AND actual_minutes > 0
  AND (notes LIKE '%31km%' OR notes LIKE '%37km%' OR notes LIKE '%40km%' 
       OR notes LIKE '%41km%' OR notes LIKE '%43km%' OR notes LIKE '%45km%')
ORDER BY speed_kmh DESC
LIMIT 10;