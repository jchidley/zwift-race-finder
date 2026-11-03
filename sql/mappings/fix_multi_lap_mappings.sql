UPDATE race_results
SET route_id = 3369744027
WHERE event_name = '3R Racing'
  AND route_id = 3369744027;


UPDATE race_results
SET route_id = 3742187716
WHERE event_name = 'Team DRAFT Monday Race'
  AND route_id = 3742187716;

CREATE TABLE IF NOT EXISTS multi_lap_events (
    event_name_pattern TEXT PRIMARY KEY,
    route_id INTEGER NOT NULL,
    lap_count INTEGER NOT NULL,
    notes TEXT,
    FOREIGN KEY (route_id) REFERENCES routes(route_id)
);

INSERT OR REPLACE INTO multi_lap_events (event_name_pattern, route_id, lap_count, notes) VALUES
('3R Racing', 3369744027, 3, 'Generic 3R Racing is 3 laps of Volcano Flat'),
('Team DRAFT Monday Race', 3742187716, 2, 'Monday race is 2 laps of Castle to Castle'),
('KISS Racing', 2139400188, 3, 'KISS Racing is typically 3 laps based on 65min for 35km route'),
('EVR Winter Series', 3742187716, 2, 'EVR Winter Series appears to be 2+ laps based on 92-98min times'),
('DIRT Dadurday Chase Race', 2139400188, 2, 'Chase races often 2 laps, 65-71min for 35km route');

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
HAVING ABS(avg_actual - (r.distance_km / 30.9 * 60)) > 20
ORDER BY race_count DESC
LIMIT 10;