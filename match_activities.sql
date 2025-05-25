-- Match by event name and date (within 1 day)
WITH matches AS (
    SELECT 
        r.id as race_id,
        s.id as strava_id,
        r.event_name,
        s.name as strava_name,
        r.race_date,
        date(s.start_date) as strava_date,
        r.actual_minutes as db_minutes,
        ROUND(s.moving_time_seconds / 60.0) as strava_minutes,
        ROUND(s.distance_m / 1000.0, 1) as strava_km,
        -- Calculate match score
        CASE 
            WHEN r.event_name = s.name THEN 100
            WHEN LOWER(r.event_name) LIKE '%' || LOWER(SUBSTR(s.name, 1, 20)) || '%' THEN 80
            WHEN LOWER(s.name) LIKE '%' || LOWER(SUBSTR(r.event_name, 1, 20)) || '%' THEN 70
            ELSE 50
        END as name_match_score
    FROM race_results r
    CROSS JOIN strava_activities s
    WHERE 
        r.strava_activity_id IS NULL
        AND ABS(julianday(date(s.start_date)) - julianday(date(r.race_date))) <= 1
        AND (
            LOWER(r.event_name) LIKE '%' || LOWER(SUBSTR(s.name, 1, 20)) || '%'
            OR LOWER(s.name) LIKE '%' || LOWER(r.event_name) || '%'
            OR r.event_name = s.name
        )
)
SELECT 
    race_id,
    strava_id,
    event_name,
    strava_name,
    race_date,
    strava_date,
    db_minutes,
    strava_minutes,
    strava_km,
    name_match_score
FROM matches
ORDER BY race_id, name_match_score DESC;
