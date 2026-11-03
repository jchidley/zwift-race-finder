use crate::database::{Database, RouteData as DbRouteData};
use crate::duration_estimation::{
    get_route_difficulty_multiplier, get_route_difficulty_multiplier_from_gain_and_category,
};
use crate::models::RouteData;

pub fn get_route_data_from_db(route_id: u32) -> Option<DbRouteData> {
    match Database::new() {
        Ok(db) => db.get_route(route_id).ok().flatten(),
        Err(_) => None,
    }
}

pub fn get_route_data(route_id: u32) -> Option<RouteData> {
    if let Some(db_route) = get_route_data_from_db(route_id) {
        return Some(RouteData {
            distance_km: db_route.distance_km,
            elevation_m: db_route.elevation_m,
            name: Box::leak(db_route.name.into_boxed_str()),
            world: Box::leak(db_route.world.into_boxed_str()),
            surface: Box::leak(db_route.surface.into_boxed_str()),
            lead_in_distance_km: db_route.lead_in_distance_km,
            lead_in_elevation_m: db_route.lead_in_elevation_m,
        });
    }

    None
}

fn valid_gain(gain: f64) -> Option<f64> {
    (gain.is_finite() && gain >= 0.0 && gain <= f64::from(u32::MAX)).then_some(gain)
}

pub fn elevation_for_laps(route: &RouteData, laps: u32) -> Option<f64> {
    valid_gain(
        valid_gain(route.elevation_m?)? * f64::from(laps) + valid_gain(route.lead_in_elevation_m?)?,
    )
}

pub fn elevation_for_distance(route: &RouteData, distance_km: f64) -> Option<f64> {
    let lead_distance = route.lead_in_distance_km?;
    if !route.distance_km.is_finite()
        || route.distance_km <= 0.0
        || !lead_distance.is_finite()
        || lead_distance < 0.0
        || !distance_km.is_finite()
        || distance_km < lead_distance
    {
        return None;
    }
    let lap_gain = valid_gain(route.elevation_m?)?;
    let lead_gain = valid_gain(route.lead_in_elevation_m?)?;
    valid_gain(lap_gain * (distance_km - lead_distance) / route.distance_km + lead_gain)
}

pub fn estimate_duration_from_route_id(route_id: u32, zwift_score: u32) -> Option<u32> {
    let route = get_route_data(route_id)?;
    Some(estimate_route_duration(
        &route,
        route.distance_km + route.lead_in_distance_km?,
        zwift_score,
    ))
}

pub fn estimate_duration_with_distance(
    route_id: u32,
    distance_km: f64,
    zwift_score: u32,
) -> Option<u32> {
    let route = get_route_data(route_id)?;
    Some(estimate_route_duration(&route, distance_km, zwift_score))
}

fn estimate_route_duration(route: &RouteData, distance_km: f64, zwift_score: u32) -> u32 {
    let category = crate::category::get_category_from_score(zwift_score);
    let base_speed = crate::category::get_category_speed(category);
    let elevation = elevation_for_distance(route, distance_km)
        .map(|gain| (gain, distance_km))
        .or_else(|| route.elevation_m.map(|gain| (gain, route.distance_km)));
    let multiplier = elevation.map_or_else(
        || get_route_difficulty_multiplier(route.name),
        |(gain, distance)| {
            get_route_difficulty_multiplier_from_gain_and_category(distance, gain, category)
        },
    );
    (distance_km / (base_speed * multiplier) * f64::from(crate::constants::MINUTES_PER_HOUR)).round() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_route_rounds_only_the_final_minutes() {
        let route = RouteData {
            distance_km: 10.0,
            elevation_m: Some(100.0),
            lead_in_distance_km: Some(0.0),
            lead_in_elevation_m: Some(0.0),
            name: "Synthetic rounding circuit",
            world: "Test",
            surface: "road",
        };
        for (minutes, expected) in [
            (0.49, 0),
            (0.5, 1),
            (30.49, 30),
            (30.5, 31),
            (30.51, 31),
        ] {
            assert_eq!(
                estimate_route_duration(&route, minutes * 0.75, 650),
                expected
            );
        }
    }

    #[test]
    fn test_lead_in_distance_is_added() {
        let route_id = 4_000_001_101;
        let route = serde_json::from_value(serde_json::json!({
            "route_id": route_id, "name": "Synthetic lead-in circuit", "world": "Test",
            "distance_km": 10.0, "elevation_m": 50.0, "surface": "road",
            "lead_in_distance_km": 2.0, "lead_in_elevation_m": 10.0
        })).unwrap();
        crate::database::Database::new().unwrap().add_route(&route).unwrap();

        let with_lead_in = estimate_duration_from_route_id(route_id, 195);
        assert!(with_lead_in.is_some(), "Explicit synthetic route should be known");

        let route_data = get_route_data(route_id).unwrap();
        let without_lead_in =
            estimate_duration_with_distance(route_id, route_data.distance_km, 195);

        assert!(without_lead_in.is_some());

        assert!(
            with_lead_in.unwrap() >= without_lead_in.unwrap(),
            "Duration with lead-in ({}) should be >= without lead-in ({}). \
             Lead-in distance is {} km.",
            with_lead_in.unwrap(),
            without_lead_in.unwrap(),
            route_data.lead_in_distance_km.unwrap()
        );
    }
    #[test]
    fn duration_uses_event_gradient_when_lead_in_is_known() {
        let mut route = RouteData {
            distance_km: 2.0,
            elevation_m: Some(100.0),
            lead_in_distance_km: Some(2.0),
            lead_in_elevation_m: Some(0.0),
            name: "Test",
            world: "Test",
            surface: "road",
        };
        let actual = estimate_route_duration(&route, 4.0, 195);
        let category = crate::category::get_category_from_score(195);
        let speed = crate::category::get_category_speed(category);
        let multiplier =
            get_route_difficulty_multiplier_from_gain_and_category(4.0, 100.0, category);
        assert_eq!(actual, (4.0 / (speed * multiplier) * 60.0).round() as u32);
        route.lead_in_elevation_m = None;
        assert_ne!(actual, estimate_route_duration(&route, 4.0, 195));
    }
}
