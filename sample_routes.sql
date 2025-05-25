-- Sample route data for Zwift Race Finder
-- This file contains common Zwift routes to get started
-- Run: sqlite3 ~/.local/share/zwift-race-finder/races.db < sample_routes.sql

-- Popular Watopia Routes
INSERT OR IGNORE INTO routes (route_id, distance_km, elevation_m, name, world, surface)
VALUES 
    -- Flat routes
    (3, 10.0, 52, 'Watopia Flat Route', 'Watopia', 'road'),
    (11, 17.1, 51, 'Tempus Fugit', 'Watopia', 'road'),
    (1015, 12.3, 46, 'Volcano Flat', 'Watopia', 'road'),
    (22, 21.3, 55, 'Tick Tock', 'Watopia', 'road'),
    
    -- Rolling routes
    (10, 12.5, 104, 'Watopia Figure 8', 'Watopia', 'road'),
    (2143464829, 33.4, 170, 'Watopia Flat Route Extended', 'Watopia', 'road'),
    (30, 19.7, 173, 'Sand and Sequoias', 'Watopia', 'road'),
    
    -- Hilly routes
    (14, 14.9, 168, 'The Pretzel', 'Watopia', 'road'),
    (1016, 4.4, 20, 'Volcano Circuit', 'Watopia', 'road'),
    (1017, 23.7, 155, 'Volcano Climb', 'Watopia', 'road'),
    
    -- Mountain routes
    (6, 12.2, 1035, 'Alpe du Zwift', 'Watopia', 'road'),
    (16, 12.5, 1642, 'Ven-Top', 'France', 'road'),
    (1263588526, 8.5, 350, 'Zwift KOM', 'Watopia', 'road');

-- London Routes
INSERT OR IGNORE INTO routes (route_id, distance_km, elevation_m, name, world, surface)
VALUES 
    (1, 14.9, 141, 'London Loop', 'London', 'road'),
    (2, 11.7, 71, 'Greater London Flat', 'London', 'road'),
    (2536286505, 20.5, 215, 'London Pretzel', 'London', 'road'),
    (3873257593, 23.8, 251, 'PRL Half', 'London', 'road'),
    (2737483347, 173.3, 2310, 'PRL Full', 'London', 'road');

-- Richmond Routes
INSERT OR IGNORE INTO routes (route_id, distance_km, elevation_m, name, world, surface)
VALUES 
    (210, 16.2, 142, 'Richmond UCI Worlds', 'Richmond', 'road'),
    (38, 11.0, 84, '2015 UCI Worlds', 'Richmond', 'road'),
    (3268894719, 18.7, 166, 'Richmond Flat', 'Richmond', 'road');

-- Innsbruck Routes
INSERT OR IGNORE INTO routes (route_id, distance_km, elevation_m, name, world, surface)
VALUES 
    (236, 8.8, 111, 'Innsbruckring', 'Innsbruck', 'road'),
    (3919348289, 23.5, 232, 'Innsbruck KOM After Party', 'Innsbruck', 'road'),
    (2739835984, 7.9, 373, 'Innsbruck KOM', 'Innsbruck', 'road');

-- Makuri Islands Routes
INSERT OR IGNORE INTO routes (route_id, distance_km, elevation_m, name, world, surface)
VALUES 
    (3742187716, 24.5, 168, 'Castle to Castle', 'Makuri Islands', 'road'),
    (2927651296, 64.4, 481, 'Makuri Pretzel', 'Makuri Islands', 'road'),
    (178089894, 13.7, 162, 'Country to Coastal', 'Makuri Islands', 'road'),
    (591978455, 21.7, 208, 'Two Village Loop', 'Makuri Islands', 'road');

-- Crit City Routes
INSERT OR IGNORE INTO routes (route_id, distance_km, elevation_m, name, world, surface)
VALUES 
    (1258415487, 1.9, 11, 'Bell Lap', 'Crit City', 'road'),
    (2698009951, 8.0, 66, 'Downtown Dolphin', 'Crit City', 'road');

-- Gravel Routes
INSERT OR IGNORE INTO routes (route_id, distance_km, elevation_m, name, world, surface)
VALUES 
    (3884086981, 40.7, 289, 'Gravel Growler', 'Makuri Islands', 'gravel'),
    (3645149188, 60.5, 397, 'The Muckle Yin', 'Scotland', 'gravel'),
    (2053478708, 19.0, 261, 'Loch Loop', 'Scotland', 'mixed');

-- Show summary after import
SELECT 
    world,
    COUNT(*) as route_count,
    ROUND(AVG(distance_km), 1) as avg_distance_km,
    ROUND(AVG(elevation_m), 0) as avg_elevation_m
FROM routes
GROUP BY world
ORDER BY route_count DESC;