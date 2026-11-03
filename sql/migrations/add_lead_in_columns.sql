ALTER TABLE routes ADD COLUMN lead_in_distance_km REAL DEFAULT 0.0;
ALTER TABLE routes ADD COLUMN lead_in_elevation_m INTEGER DEFAULT 0;
ALTER TABLE routes ADD COLUMN lead_in_distance_free_ride_km REAL DEFAULT 0.0;
ALTER TABLE routes ADD COLUMN lead_in_elevation_free_ride_m INTEGER DEFAULT 0;
ALTER TABLE routes ADD COLUMN lead_in_distance_meetups_km REAL DEFAULT 0.0;
ALTER TABLE routes ADD COLUMN lead_in_elevation_meetups_m INTEGER DEFAULT 0;
ALTER TABLE routes ADD COLUMN slug TEXT;