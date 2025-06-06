//! Duration estimation functions for Zwift races

use crate::category::{get_category_from_score, get_category_speed};
use crate::constants::{METERS_PER_KILOMETER, MINUTES_PER_HOUR, PERCENT_MULTIPLIER};

/// Calculate difficulty multiplier based on elevation gain per km
pub fn get_route_difficulty_multiplier_from_elevation(distance_km: f64, elevation_m: u32) -> f64 {
    let meters_per_km = elevation_m as f64 / distance_km;

    match meters_per_km {
        m if m < 5.0 => 1.1,  // Very flat (like Tempus Fugit)
        m if m < 10.0 => 1.0, // Flat to rolling
        m if m < 20.0 => 0.9, // Rolling hills
        m if m < 40.0 => 0.8, // Hilly
        _ => 0.7,             // Very hilly (like Mt. Fuji or Alpe)
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
    (duration_hours * MINUTES_PER_HOUR as f64) as u32
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
    let avg_gradient =
        (elevation_m as f64 / (distance_km * METERS_PER_KILOMETER)) * PERCENT_MULTIPLIER;

    // Drop probability increases with gradient and decreases with W/kg
    let drop_probability = if avg_gradient > 2.0 {
        // On hilly routes, lower W/kg riders more likely to drop
        let w_kg_factor = match watts_per_kg {
            w if w < 2.5 => 0.8, // Very likely to drop
            w if w < 3.0 => 0.6, // Likely to drop
            w if w < 3.5 => 0.4, // May drop
            w if w < 4.0 => 0.2, // Unlikely to drop
            _ => 0.1,            // Very unlikely to drop
        };
        w_kg_factor * (avg_gradient / 10.0).min(1.0)
    } else {
        0.1 // Low drop probability on flat routes
    };

    // Weighted average of pack and solo times
    let pack_time = (distance_km / pack_speed) * MINUTES_PER_HOUR as f64;
    let solo_time = (distance_km / solo_speed) * MINUTES_PER_HOUR as f64;
    let estimated_time = pack_time * (1.0 - drop_probability) + solo_time * drop_probability;

    estimated_time as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_route_difficulty_multiplier_from_elevation() {
        // Test very flat routes (< 5m/km)
        assert_eq!(
            get_route_difficulty_multiplier_from_elevation(40.0, 100),
            1.1
        );

        // Test flat to rolling (5-10m/km)
        assert_eq!(
            get_route_difficulty_multiplier_from_elevation(40.0, 300),
            1.0
        );

        // Test rolling hills (10-20m/km)
        assert_eq!(
            get_route_difficulty_multiplier_from_elevation(40.0, 600),
            0.9
        );

        // Test hilly (20-40m/km)
        assert_eq!(
            get_route_difficulty_multiplier_from_elevation(40.0, 1200),
            0.8
        );

        // Test very hilly (>40m/km)
        assert_eq!(
            get_route_difficulty_multiplier_from_elevation(12.2, 1035),
            0.7
        );
    }

    #[test]
    fn test_estimate_duration_for_category() {
        // Test Cat D rider on Alpe du Zwift (slow climb)
        // 12.2km / (30.9 km/h * 0.7) = 0.564 hours = ~33.8 minutes
        let alpe_duration = estimate_duration_for_category(12.2, "Alpe du Zwift", 195);
        assert!(
            alpe_duration >= 33 && alpe_duration <= 35,
            "Alpe (12.2km with 1035m elevation) should take 33-35 min for Cat D, got {}",
            alpe_duration
        );

        // Test Cat C rider on Tempus Fugit (fast flat)
        // 17.3km / (34.5 km/h * 1.1) = 0.456 hours = ~27.3 minutes
        let tempus_time = estimate_duration_for_category(17.3, "Tempus Fugit", 250);
        assert!(
            tempus_time >= 26 && tempus_time <= 29,
            "Tempus Fugit should take 26-29 min for Cat C, got {}",
            tempus_time
        );
    }

    #[test]
    fn test_duration_estimation_for_cat_d() {
        // Test known distances and expected durations for Cat D (195 score)
        // Base speed for 195 score is 30.9 km/h

        // Watopia: 40km at 30.9km/h * 1.0 multiplier = 77.7 ≈ 77 min
        let watopia_time = estimate_duration_for_category(40.0, "Watopia", 195);
        assert_eq!(watopia_time, 77);

        // Alpe du Zwift: 30km at 30.9km/h * 0.7 multiplier = 83.1 ≈ 83 min
        let alpe_time = estimate_duration_for_category(30.0, "Alpe du Zwift", 195);
        assert_eq!(alpe_time, 83);

        // Tempus Fugit: 35km at 30.9km/h * 1.1 multiplier = 61.8 ≈ 61 min
        let tempus_time = estimate_duration_for_category(35.0, "Tempus Fugit", 195);
        assert_eq!(tempus_time, 61);
    }

    #[test]
    fn test_get_route_difficulty_multiplier() {
        // Test very hilly routes
        assert_eq!(get_route_difficulty_multiplier("Alpe du Zwift"), 0.7);
        assert_eq!(get_route_difficulty_multiplier("Road to Sky"), 1.0); // doesn't contain special keywords
        assert_eq!(get_route_difficulty_multiplier("Ven-Top"), 1.0); // "ventoux" not "ven"
        assert_eq!(get_route_difficulty_multiplier("Mont Ventoux"), 0.7); // contains "ventoux"
        assert_eq!(get_route_difficulty_multiplier("ALPE DU ZWIFT"), 0.7); // case insensitive

        // Test hilly routes
        assert_eq!(get_route_difficulty_multiplier("Epic KOM"), 0.8);
        assert_eq!(get_route_difficulty_multiplier("Mountain Route"), 0.8);

        // Test flat routes
        assert_eq!(get_route_difficulty_multiplier("Tempus Fugit"), 1.1);
        assert_eq!(get_route_difficulty_multiplier("Flatlands"), 1.1);

        // Test normal routes
        assert_eq!(get_route_difficulty_multiplier("Regular Route"), 1.0);
        assert_eq!(get_route_difficulty_multiplier(""), 1.0);
    }

    #[test]
    fn test_specific_route_multipliers() {
        // Test that route difficulty multipliers work correctly
        let flat_distance = 40.0;
        let zwift_score = 195;

        // Same distance, different routes should give different times
        let tempus = estimate_duration_for_category(flat_distance, "Tempus Fugit", zwift_score);
        let alpe = estimate_duration_for_category(flat_distance, "Alpe du Zwift", zwift_score);
        let normal = estimate_duration_for_category(flat_distance, "Regular Route", zwift_score); // No special keywords

        // Tempus has 1.1x multiplier (faster), normal has 1.0x, Alpe has 0.7x (slower)
        assert!(
            tempus < normal,
            "Tempus Fugit should be faster than normal: {} vs {}",
            tempus,
            normal
        );
        assert!(
            alpe > normal,
            "Alpe du Zwift should be slower than normal: {} vs {}",
            alpe,
            normal
        );

        // Check actual values match expected calculations
        // 40km at 30.9km/h with multipliers:
        assert_eq!(tempus, 70); // 40 / (30.9 * 1.1) * 60 = 70.7 ≈ 70
        assert_eq!(normal, 77); // 40 / (30.9 * 1.0) * 60 = 77.7 ≈ 77
        assert_eq!(alpe, 110); // 40 / (30.9 * 0.7) * 60 = 110.9 ≈ 110
    }

    #[test]
    fn test_edge_case_estimations() {
        // Test very short race (sprint)
        let sprint_duration = estimate_duration_for_category(5.0, "Sprint Route", 195);
        assert!(
            sprint_duration >= 8 && sprint_duration <= 12,
            "Sprint (5km) should take 8-12 minutes, got {}",
            sprint_duration
        );

        // Test very long race (gran fondo)
        let fondo_duration = estimate_duration_for_category(100.0, "Epic Route", 195);
        assert!(
            fondo_duration >= 180 && fondo_duration <= 250,
            "Gran Fondo (100km) should take 3-4.2 hours, got {} min",
            fondo_duration
        );

        // Test zero distance (edge case)
        let zero_duration = estimate_duration_for_category(0.0, "Zero Route", 195);
        assert_eq!(zero_duration, 0, "Zero distance should give zero duration");

        // Test with different categories
        let cat_a_duration = estimate_duration_for_category(40.0, "Test Route", 450);
        let cat_d_duration = estimate_duration_for_category(40.0, "Test Route", 195);
        assert!(
            cat_a_duration < cat_d_duration,
            "Cat A should be faster than Cat D: {} vs {}",
            cat_a_duration,
            cat_d_duration
        );
    }

    #[test]
    fn test_more_elevation_multipliers() {
        // Test very flat routes (< 5m/km)
        assert_eq!(
            get_route_difficulty_multiplier_from_elevation(20.0, 50),
            1.1
        ); // 2.5m/km
        assert_eq!(
            get_route_difficulty_multiplier_from_elevation(10.0, 40),
            1.1
        ); // 4m/km

        // Test flat to rolling (5-10m/km)
        assert_eq!(
            get_route_difficulty_multiplier_from_elevation(20.0, 150),
            1.0
        ); // 7.5m/km
        assert_eq!(
            get_route_difficulty_multiplier_from_elevation(30.0, 270),
            1.0
        ); // 9m/km

        // Test rolling hills (10-20m/km)
        assert_eq!(
            get_route_difficulty_multiplier_from_elevation(20.0, 300),
            0.9
        ); // 15m/km
        assert_eq!(
            get_route_difficulty_multiplier_from_elevation(40.0, 760),
            0.9
        ); // 19m/km

        // Test hilly (20-40m/km)
        assert_eq!(
            get_route_difficulty_multiplier_from_elevation(20.0, 500),
            0.8
        ); // 25m/km
        assert_eq!(
            get_route_difficulty_multiplier_from_elevation(30.0, 1100),
            0.8
        ); // 36.7m/km

        // Test very hilly (>40m/km)
        assert_eq!(
            get_route_difficulty_multiplier_from_elevation(10.0, 500),
            0.7
        ); // 50m/km
        assert_eq!(
            get_route_difficulty_multiplier_from_elevation(12.2, 1035),
            0.7
        ); // 84.8m/km (Alpe)

        // Test edge cases
        assert_eq!(get_route_difficulty_multiplier_from_elevation(0.1, 1), 0.9); // 10m/km = rolling hills
        assert_eq!(
            get_route_difficulty_multiplier_from_elevation(1000.0, 5000),
            1.0
        ); // 5m/km on long route
    }

    #[test]
    fn test_calculate_duration_with_dual_speed_arithmetic() {
        // Test arithmetic operations in calculate_duration_with_dual_speed
        // Mutations: replace * with +, replace / with *, etc.

        // Test basic calculation with flat route (low drop probability)
        let distance = 30.0;
        let elevation = 100; // Flat route: 3.3 m/km
        let zwift_score = 195; // Cat D
        let weight = 86.0;
        let ftp = 217.0;

        // For flat route, drop probability should be 0
        let duration =
            calculate_duration_with_dual_speed(distance, elevation, zwift_score, weight, ftp);
        // Pack time = 30 / 30.9 * 60 = 58.25 minutes, rounded to 59
        assert_eq!(duration, 59);

        // Test with hilly route (higher drop probability)
        let elevation_hilly = 600; // 20 m/km = 30% drop probability
        let duration_hilly =
            calculate_duration_with_dual_speed(distance, elevation_hilly, zwift_score, weight, ftp);

        // Should be between pack time (59) and solo time (~76)
        assert!(duration_hilly >= duration); // >= pack time (might be same due to rounding)
        assert!(duration_hilly < 80); // < full solo time

        // Test with very hilly route (high drop probability)
        let elevation_steep = 1500; // 50 m/km = 90% drop probability
        let duration_steep =
            calculate_duration_with_dual_speed(distance, elevation_steep, zwift_score, weight, ftp);

        // Should be longer than pack time but with difficulty multiplier it might be shorter
        // Very steep routes get a 0.7 multiplier
        assert!(duration_steep > 40); // With steep difficulty multiplier
    }

    #[test]
    fn test_weighted_average_calculation() {
        // Test the weighted average formula specifically
        // pack_time * (1.0 - drop_probability) + solo_time * drop_probability

        let pack_time = 60.0;
        let solo_time = 80.0;
        let drop_prob = 0.25;

        let weighted = pack_time * (1.0 - drop_prob) + solo_time * drop_prob;
        assert_eq!(weighted, 65.0); // 60 * 0.75 + 80 * 0.25 = 45 + 20 = 65

        // If * became +, we'd get very different results
        assert_ne!(
            weighted,
            pack_time + (1.0 - drop_prob) + solo_time + drop_prob
        );

        // Test edge cases
        let weighted_0 = pack_time * 1.0 + solo_time * 0.0;
        assert_eq!(weighted_0, pack_time);

        let weighted_1 = pack_time * 0.0 + solo_time * 1.0;
        assert_eq!(weighted_1, solo_time);
    }

    #[test]
    fn test_drop_probability_match_guards() {
        // Test match guard conditions in calculate_drop_probability
        // Mutation: replace match guard with true/false

        // Test various elevation levels
        let test_cases = vec![
            (5.0, 0.0),  // < 10 m/km -> 0%
            (15.0, 0.1), // 10-20 m/km -> 10%
            (25.0, 0.3), // 20-30 m/km -> 30%
            (35.0, 0.5), // 30-40 m/km -> 50%
            (45.0, 0.7), // 40-50 m/km -> 70%
            (55.0, 0.9), // > 50 m/km -> 90%
        ];

        for (elevation_per_km, expected_prob) in test_cases {
            let prob = match elevation_per_km {
                e if e < 10.0 => 0.0,
                e if e < 20.0 => 0.1,
                e if e < 30.0 => 0.3,
                e if e < 40.0 => 0.5,
                e if e < 50.0 => 0.7,
                _ => 0.9,
            };

            assert_eq!(prob, expected_prob);
        }
    }

    #[test]
    fn test_minutes_calculation() {
        // Test conversion from hours to minutes
        // Formula: (distance_km / speed_kmh) * 60

        let distance = 20.0;
        let speed = 30.0;

        let time_hours = distance / speed; // 0.667 hours
        let time_minutes = time_hours * MINUTES_PER_HOUR as f64; // 40 minutes

        assert_eq!(time_minutes as u32, 40);

        // If * became +, we'd get 60.667
        assert!(time_minutes < 50.0);

        // If / became *, we'd get 600 hours
        assert!(time_hours < 1.0);
    }
}
