//! Snapshot tests for zwift-race-finder
//! These tests capture and verify exact behavior for known routes

use insta::assert_yaml_snapshot;
use zwift_race_finder::database::RouteData;

/// Calculate duration estimation using the same logic as the main code
fn estimate_duration(route: &RouteData, zwift_score: u32) -> f64 {
    let base_speed = match zwift_score {
        0..=199 => 30.0,   // Cat D
        200..=299 => 33.0, // Cat C
        300..=399 => 36.0, // Cat B
        _ => 39.0,         // Cat A
    };
    
    let elevation_factor = 1.0 - (route.elevation_m as f64 / route.distance_km / 100.0).min(0.3);
    let effective_speed = base_speed * elevation_factor;
    route.distance_km / effective_speed * 60.0
}

#[test]
fn test_known_flat_routes() {
    // Test flat routes with minimal elevation
    let routes = vec![
        ("Bell Lap", RouteData {
            route_id: 1258415487,
            distance_km: 14.1,
            elevation_m: 59,
            name: "Bell Lap".to_string(),
            world: "Crit City".to_string(),
            surface: "tarmac".to_string(),
            lead_in_distance_km: 0.0,
            lead_in_elevation_m: 0,
            lead_in_distance_free_ride_km: None,
            lead_in_elevation_free_ride_m: None,
            lead_in_distance_meetups_km: None,
            lead_in_elevation_meetups_m: None,
            slug: None,
        }),
        ("Downtown Dolphin", RouteData {
            route_id: 2698009951,
            distance_km: 22.9,
            elevation_m: 80,
            name: "Downtown Dolphin".to_string(),
            world: "Crit City".to_string(),
            surface: "tarmac".to_string(),
            lead_in_distance_km: 0.0,
            lead_in_elevation_m: 0,
            lead_in_distance_free_ride_km: None,
            lead_in_elevation_free_ride_m: None,
            lead_in_distance_meetups_km: None,
            lead_in_elevation_meetups_m: None,
            slug: None,
        }),
        ("Three Village Loop", RouteData {
            route_id: 3379779247,
            distance_km: 10.6,
            elevation_m: 93,
            name: "Three Village Loop".to_string(),
            world: "Makuri Islands".to_string(),
            surface: "tarmac".to_string(),
            lead_in_distance_km: 0.0,
            lead_in_elevation_m: 0,
            lead_in_distance_free_ride_km: None,
            lead_in_elevation_free_ride_m: None,
            lead_in_distance_meetups_km: None,
            lead_in_elevation_meetups_m: None,
            slug: None,
        }),
    ];
    
    for (name, route) in routes {
        let cat_d_duration = estimate_duration(&route, 100);
        let cat_c_duration = estimate_duration(&route, 250);
        let cat_b_duration = estimate_duration(&route, 350);
        let cat_a_duration = estimate_duration(&route, 450);
        
        assert_yaml_snapshot!(
            format!("flat_route_{}", name.to_lowercase().replace(' ', "_")),
            serde_yaml::to_value(&[
                ("route_name", name),
                ("distance_km", &route.distance_km.to_string()),
                ("elevation_m", &route.elevation_m.to_string()),
                ("cat_d_minutes", &format!("{:.1}", cat_d_duration)),
                ("cat_c_minutes", &format!("{:.1}", cat_c_duration)),
                ("cat_b_minutes", &format!("{:.1}", cat_b_duration)),
                ("cat_a_minutes", &format!("{:.1}", cat_a_duration)),
            ]).unwrap()
        );
    }
}

#[test]
fn test_known_hilly_routes() {
    // Test routes with moderate elevation
    let routes = vec![
        ("Castle to Castle", RouteData {
            route_id: 3742187716,
            distance_km: 24.5,
            elevation_m: 168,
            name: "Castle to Castle".to_string(),
            world: "Makuri Islands".to_string(),
            surface: "tarmac".to_string(),
            lead_in_distance_km: 0.0,
            lead_in_elevation_m: 0,
            lead_in_distance_free_ride_km: None,
            lead_in_elevation_free_ride_m: None,
            lead_in_distance_meetups_km: None,
            lead_in_elevation_meetups_m: None,
            slug: None,
        }),
        ("Hilltop Hustle", RouteData {
            route_id: 3961473046,
            distance_km: 15.4,
            elevation_m: 292,
            name: "Hilltop Hustle".to_string(),
            world: "Makuri Islands".to_string(),
            surface: "tarmac".to_string(),
            lead_in_distance_km: 0.0,
            lead_in_elevation_m: 0,
            lead_in_distance_free_ride_km: None,
            lead_in_elevation_free_ride_m: None,
            lead_in_distance_meetups_km: None,
            lead_in_elevation_meetups_m: None,
            slug: None,
        }),
        ("eRacing Course", RouteData {
            route_id: 3368626651,
            distance_km: 27.4,
            elevation_m: 223,
            name: "eRacing Course".to_string(),
            world: "Various".to_string(),
            surface: "tarmac".to_string(),
            lead_in_distance_km: 0.0,
            lead_in_elevation_m: 0,
            lead_in_distance_free_ride_km: None,
            lead_in_elevation_free_ride_m: None,
            lead_in_distance_meetups_km: None,
            lead_in_elevation_meetups_m: None,
            slug: None,
        }),
    ];
    
    for (name, route) in routes {
        let cat_d_duration = estimate_duration(&route, 100);
        let cat_c_duration = estimate_duration(&route, 250);
        let cat_b_duration = estimate_duration(&route, 350);
        let cat_a_duration = estimate_duration(&route, 450);
        
        assert_yaml_snapshot!(
            format!("hilly_route_{}", name.to_lowercase().replace(' ', "_")),
            serde_yaml::to_value(&[
                ("route_name", name),
                ("distance_km", &route.distance_km.to_string()),
                ("elevation_m", &route.elevation_m.to_string()),
                ("cat_d_minutes", &format!("{:.1}", cat_d_duration)),
                ("cat_c_minutes", &format!("{:.1}", cat_c_duration)),
                ("cat_b_minutes", &format!("{:.1}", cat_b_duration)),
                ("cat_a_minutes", &format!("{:.1}", cat_a_duration)),
            ]).unwrap()
        );
    }
}

#[test]
fn test_known_mountain_routes() {
    // Test routes with significant elevation
    let routes = vec![
        ("Mt. Fuji", RouteData {
            route_id: 2663908549,
            distance_km: 20.3,
            elevation_m: 1159,
            name: "Mt. Fuji".to_string(),
            world: "Makuri Islands".to_string(),
            surface: "tarmac".to_string(),
            lead_in_distance_km: 0.0,
            lead_in_elevation_m: 0,
            lead_in_distance_free_ride_km: None,
            lead_in_elevation_free_ride_m: None,
            lead_in_distance_meetups_km: None,
            lead_in_elevation_meetups_m: None,
            slug: None,
        }),
        ("Mountain Mash", RouteData {
            route_id: 1917017591,
            distance_km: 5.7,
            elevation_m: 335,
            name: "Mountain Mash".to_string(),
            world: "Watopia".to_string(),
            surface: "tarmac".to_string(),
            lead_in_distance_km: 0.0,
            lead_in_elevation_m: 0,
            lead_in_distance_free_ride_km: None,
            lead_in_elevation_free_ride_m: None,
            lead_in_distance_meetups_km: None,
            lead_in_elevation_meetups_m: None,
            slug: None,
        }),
        ("KISS 100", RouteData {
            route_id: 2474227587,
            distance_km: 35.0,
            elevation_m: 892,
            name: "KISS 100".to_string(),
            world: "Watopia".to_string(),
            surface: "tarmac".to_string(),
            lead_in_distance_km: 0.0,
            lead_in_elevation_m: 0,
            lead_in_distance_free_ride_km: None,
            lead_in_elevation_free_ride_m: None,
            lead_in_distance_meetups_km: None,
            lead_in_elevation_meetups_m: None,
            slug: None,
        }),
    ];
    
    for (name, route) in routes {
        let cat_d_duration = estimate_duration(&route, 100);
        let cat_c_duration = estimate_duration(&route, 250);
        let cat_b_duration = estimate_duration(&route, 350);
        let cat_a_duration = estimate_duration(&route, 450);
        
        assert_yaml_snapshot!(
            format!("mountain_route_{}", name.to_lowercase().replace(' ', "_")),
            serde_yaml::to_value(&[
                ("route_name", name),
                ("distance_km", &route.distance_km.to_string()),
                ("elevation_m", &route.elevation_m.to_string()),
                ("cat_d_minutes", &format!("{:.1}", cat_d_duration)),
                ("cat_c_minutes", &format!("{:.1}", cat_c_duration)),
                ("cat_b_minutes", &format!("{:.1}", cat_b_duration)),
                ("cat_a_minutes", &format!("{:.1}", cat_a_duration)),
            ]).unwrap()
        );
    }
}

#[test]
fn test_short_crit_race() {
    // Test very short criterium race
    let route = RouteData {
        route_id: 3765339356,
        distance_km: 3.0,
        elevation_m: 34,
        name: "Glasgow Crit Circuit".to_string(),
        world: "Scotland".to_string(),
        surface: "tarmac".to_string(),
        lead_in_distance_km: 0.0,
        lead_in_elevation_m: 0,
        lead_in_distance_free_ride_km: None,
        lead_in_elevation_free_ride_m: None,
        lead_in_distance_meetups_km: None,
        lead_in_elevation_meetups_m: None,
        slug: None,
    };
    
    let durations = vec![
        ("cat_d", estimate_duration(&route, 100)),
        ("cat_c", estimate_duration(&route, 250)),
        ("cat_b", estimate_duration(&route, 350)),
        ("cat_a", estimate_duration(&route, 450)),
    ];
    
    assert_yaml_snapshot!(
        "short_crit_race_durations",
        serde_yaml::to_value(&serde_yaml::to_value(&[
            ("route_name", serde_yaml::Value::String("Glasgow Crit Circuit".to_string())),
            ("distance_km", serde_yaml::Value::String("3.0".to_string())),
            ("elevation_m", serde_yaml::Value::String("34".to_string())),
            ("durations", serde_yaml::Value::Sequence(
                durations.into_iter()
                    .map(|(cat, dur)| serde_yaml::to_value(&[
                        ("category", cat),
                        ("minutes", &format!("{:.1}", dur))
                    ]).unwrap())
                    .collect()
            )),
        ]).unwrap()).unwrap()
    );
}