//! Route and duration estimation functions
//! 
//! This module provides functions for estimating race durations based on routes,
//! distances, and rider capabilities. These functions are used by both the main
//! application and regression testing.

use crate::database::{Database, RouteData as DbRouteData};
use crate::models::RouteData;
use crate::duration_estimation::estimate_duration_for_category;

/// Get route data from the database
pub fn get_route_data_from_db(route_id: u32) -> Option<DbRouteData> {
    match Database::new() {
        Ok(db) => db.get_route(route_id).ok().flatten(),
        Err(_) => None,
    }
}

/// Get route data and convert to models::RouteData
pub fn get_route_data(route_id: u32) -> Option<RouteData> {
    // First try database
    if let Some(db_route) = get_route_data_from_db(route_id) {
        // Map fields from database RouteData to models RouteData
        return Some(RouteData {
            distance_km: db_route.distance_km,
            elevation_m: db_route.elevation_m,
            name: Box::leak(db_route.name.into_boxed_str()),
            world: Box::leak(db_route.world.into_boxed_str()),
            surface: Box::leak(db_route.surface.into_boxed_str()),
            lead_in_distance_km: db_route.lead_in_distance_km,
        });
    }
    
    // Fallback to hardcoded data for common routes
    match route_id {
        // Women's races - typically shorter criteriums
        1258415487 => Some(RouteData {
            distance_km: 14.1,
            elevation_m: 59,
            name: "Bell Lap",
            world: "Crit City",
            surface: "road",
            lead_in_distance_km: 0.5,
        }),
        _ => None,
    }
}

/// Estimate duration based on route_id only
pub fn estimate_duration_from_route_id(route_id: u32, zwift_score: u32) -> Option<u32> {
    let route_data = get_route_data(route_id)?;
    let duration = estimate_duration_for_category(route_data.distance_km, route_data.name, zwift_score);
    Some(duration)
}

/// Estimate duration with a specific distance (for multi-lap races)
pub fn estimate_duration_with_distance(route_id: u32, distance_km: f64, zwift_score: u32) -> Option<u32> {
    let route_data = get_route_data(route_id)?;
    
    // Use route name for elevation-based difficulty estimation
    let duration = estimate_duration_for_category(distance_km, route_data.name, zwift_score);
    Some(duration)
}