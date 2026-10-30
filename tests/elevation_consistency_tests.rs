use zwift_race_finder::{
    estimation::{elevation_for_distance, elevation_for_laps},
    models::RouteData,
};

fn route(elevation_m: Option<f64>, lead_in_elevation_m: Option<f64>) -> RouteData {
    RouteData {
        distance_km: 2.0,
        elevation_m,
        lead_in_distance_km: Some(0.5),
        lead_in_elevation_m,
        name: "Test Circuit",
        world: "Test",
        surface: "road",
    }
}

#[test]
fn laps_add_lead_in_once_not_proportionally() {
    let route = route(Some(20.0), Some(3.0));
    assert_eq!(elevation_for_laps(&route, 3), Some(63.0));
    assert_eq!(elevation_for_distance(&route, 6.5), Some(63.0));
    assert_eq!(elevation_for_distance(&route, 3.5), Some(33.0));
}

#[test]
fn zero_is_not_missing() {
    assert_eq!(
        elevation_for_laps(&route(Some(0.0), Some(0.0)), 15),
        Some(0.0)
    );
    assert_eq!(elevation_for_laps(&route(None, Some(0.0)), 15), None);
    assert_eq!(elevation_for_laps(&route(Some(17.0), None), 15), None);
    assert_eq!(elevation_for_distance(&route(None, Some(0.0)), 30.0), None);
    assert_eq!(elevation_for_distance(&route(Some(17.0), None), 30.0), None);
}

#[test]
fn short_partial_lead_in_or_invalid_distance_has_no_invented_gain() {
    let mut route = route(Some(20.0), Some(3.0));
    assert_eq!(elevation_for_distance(&route, 0.25), None);
    assert_eq!(elevation_for_distance(&route, f64::NAN), None);
    route.distance_km = 0.0;
    assert_eq!(elevation_for_distance(&route, 5.0), None);
}

#[test]
fn overflowing_lap_total_is_unknown_not_wrapped() {
    assert_eq!(
        elevation_for_laps(&route(Some(f64::from(u32::MAX)), Some(1.0)), 2),
        None
    );
}

#[test]
fn downtown_dolphin_fifteen_laps_is_255m_not_scaled_lead_in_climbing() {
    let route = RouteData {
        distance_km: 1.965,
        elevation_m: Some(17.0),
        lead_in_distance_km: Some(0.094),
        lead_in_elevation_m: Some(0.0),
        name: "Downtown Dolphin",
        world: "crit-city",
        surface: "road",
    };
    assert_eq!(elevation_for_laps(&route, 15), Some(255.0));
}

#[test]
fn client_precision_is_retained_until_the_event_total_is_displayed() {
    let route = RouteData {
        distance_km: 1.9653959960938,
        elevation_m: Some(16.6176948547),
        lead_in_distance_km: Some(0.0938338317871),
        lead_in_elevation_m: Some(0.0),
        name: "Downtown Dolphin",
        world: "Crit City",
        surface: "road",
    };
    let gain = elevation_for_laps(&route, 15).unwrap();
    assert!((gain - 249.2654228205).abs() < 1e-9);
    assert_eq!(gain.round() as u32, 249);
    assert_ne!(
        gain.round() as u32,
        route.elevation_m.unwrap().round() as u32 * 15
    );
}
