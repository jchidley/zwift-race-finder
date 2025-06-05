//! Duration estimation functions for Zwift races

use crate::category::{get_category_from_score, get_category_speed};

/// Calculate difficulty multiplier based on elevation gain per km
pub fn get_route_difficulty_multiplier_from_elevation(distance_km: f64, elevation_m: u32) -> f64 {
    let meters_per_km = elevation_m as f64 / distance_km;
    
    match meters_per_km {
        m if m < 5.0 => 1.1,   // Very flat (like Tempus Fugit)
        m if m < 10.0 => 1.0,  // Flat to rolling
        m if m < 20.0 => 0.9,  // Rolling hills
        m if m < 40.0 => 0.8,  // Hilly
        _ => 0.7,              // Very hilly (like Mt. Fuji or Alpe)
    }
}

/// Route difficulty multipliers (some routes are hillier)
pub fn get_route_difficulty_multiplier(route_name: &str) -> f64 {
    let route_lower = route_name.to_lowercase();

    if route_lower.contains("alpe") || route_lower.contains("ventoux") {
        0.7 // Very hilly, slower
    } else if route_lower.contains("epic") || route_lower.contains("mountain") {
        0.8 // Hilly
    } else if route_lower.contains("flat") || route_lower.contains("tempus") {
        1.1 // Flat, faster
    } else {
        1.0 // Default
    }
}

/// Estimate duration for a specific distance and route, considering pack dynamics
pub fn estimate_duration_for_category(distance_km: f64, route_name: &str, zwift_score: u32) -> u32 {
    // Get category-based speed
    let category = get_category_from_score(zwift_score);
    let base_speed = get_category_speed(category);

    let difficulty_multiplier = get_route_difficulty_multiplier(route_name);
    let effective_speed = base_speed * difficulty_multiplier;

    let duration_hours = distance_km / effective_speed;
    (duration_hours * 60.0) as u32
}

/// Calculate duration with dual-speed model (pack vs solo)
pub fn calculate_duration_with_dual_speed(
    distance_km: f64,
    elevation_m: u32,
    zwift_score: u32,
    weight_kg: f64,
    ftp_watts: f64,
) -> u32 {
    let category = get_category_from_score(zwift_score);
    
    // Base pack speeds (km/h) - from actual race data
    let pack_speed = match category {
        "E" => 27.0,
        "D" => 30.9,
        "C" => 34.5,
        "B" => 37.0,
        "A" => 39.0,
        "A+" => 41.0,
        _ => 30.9,
    };
    
    // Solo speed is 77% of pack speed (based on empirical data)
    let solo_speed = pack_speed * 0.77;
    
    // Calculate drop probability based on elevation and W/kg
    let watts_per_kg = ftp_watts / weight_kg;
    let avg_gradient = (elevation_m as f64 / (distance_km * 1000.0)) * 100.0;
    
    // Drop probability increases with gradient and decreases with W/kg
    let drop_probability = if avg_gradient > 2.0 {
        // On hilly routes, lower W/kg riders more likely to drop
        let w_kg_factor = match watts_per_kg {
            w if w < 2.5 => 0.8,  // Very likely to drop
            w if w < 3.0 => 0.6,  // Likely to drop
            w if w < 3.5 => 0.4,  // May drop
            w if w < 4.0 => 0.2,  // Unlikely to drop
            _ => 0.1,             // Very unlikely to drop
        };
        w_kg_factor * (avg_gradient / 10.0).min(1.0)
    } else {
        0.1 // Low drop probability on flat routes
    };
    
    // Weighted average of pack and solo times
    let pack_time = (distance_km / pack_speed) * 60.0;
    let solo_time = (distance_km / solo_speed) * 60.0;
    let estimated_time = pack_time * (1.0 - drop_probability) + solo_time * drop_probability;
    
    estimated_time as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_route_difficulty_multiplier_from_elevation() {
        // Test very flat routes (< 5m/km)
        assert_eq!(get_route_difficulty_multiplier_from_elevation(40.0, 100), 1.1);
        
        // Test flat to rolling (5-10m/km)
        assert_eq!(get_route_difficulty_multiplier_from_elevation(40.0, 300), 1.0);
        
        // Test rolling hills (10-20m/km)
        assert_eq!(get_route_difficulty_multiplier_from_elevation(40.0, 600), 0.9);
        
        // Test hilly (20-40m/km)
        assert_eq!(get_route_difficulty_multiplier_from_elevation(40.0, 1200), 0.8);
        
        // Test very hilly (>40m/km)
        assert_eq!(get_route_difficulty_multiplier_from_elevation(12.2, 1035), 0.7);
    }

    #[test]
    fn test_estimate_duration_for_category() {
        // Test Cat D rider on Alpe du Zwift (slow climb)
        // 12.2km / (30.9 km/h * 0.7) * 60 = ~33.8 minutes
        let alpe_duration = estimate_duration_for_category(12.2, "Alpe du Zwift", 195);
        assert!(alpe_duration >= 33 && alpe_duration <= 35,
            "Alpe (12.2km with 1035m elevation) should take 33-35 min for Cat D, got {}", alpe_duration);
        
        // Test Cat C rider on Tempus Fugit (fast flat)
        // 17.3km / (34.5 km/h * 1.1) * 60 = ~27.3 minutes
        let tempus_time = estimate_duration_for_category(17.3, "Tempus Fugit", 250);
        assert!(tempus_time >= 26 && tempus_time <= 29,
            "Tempus Fugit should take 26-29 min for Cat C, got {}", tempus_time);
    }
}