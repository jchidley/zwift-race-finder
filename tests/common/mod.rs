pub fn install_route() -> u32 {
    let route = serde_json::from_value(serde_json::json!({
        "route_id": 4_000_001_100u32, "name": "Synthetic test circuit", "world": "Test",
        "distance_km": 10.0, "elevation_m": 50.0, "surface": "road",
        "lead_in_distance_km": 0.5, "lead_in_elevation_m": null
    })).unwrap();
    zwift_race_finder::database::Database::new().unwrap().add_route(&route).unwrap();
    4_000_001_100
}
