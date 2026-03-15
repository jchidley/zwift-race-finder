//! Duration estimation functions for Zwift races

use crate::category::{get_category_from_score, get_category_speed};
use crate::constants::MINUTES_PER_HOUR;

/// Calculate difficulty multiplier based on elevation gain per km
///
/// Uses piecewise linear interpolation for smooth transitions between
/// terrain categories. On steep climbs (>15 m/km), lower racing categories
/// are disproportionately slower due to lower w/kg.
pub fn get_route_difficulty_multiplier_from_elevation(distance_km: f64, elevation_m: u32) -> f64 {
    get_route_difficulty_multiplier_from_elevation_and_category(distance_km, elevation_m, "C")
}

/// Category-aware difficulty multiplier based on elevation gain per km
///
/// Lower categories (D, E) suffer more on climbs because w/kg matters more
/// than raw watts on steep gradients. The category penalty is derived from
/// real race data: on flats, all categories ride near their empirical speed,
/// but on climbs >20 m/km, Cat D achieves only 48% of flat speed vs Cat C's 54%.
pub fn get_route_difficulty_multiplier_from_elevation_and_category(
    distance_km: f64,
    elevation_m: u32,
    category: &str,
) -> f64 {
    let meters_per_km = elevation_m as f64 / distance_km;

    // Piecewise linear interpolation breakpoints: (m/km, multiplier)
    // Calibrated against 125 real race results across all terrain types
    let breakpoints: &[(f64, f64)] = &[
        (0.0, 1.1),   // Very flat (Tempus Fugit ~1 m/km)
        (5.0, 1.05),  // Flat (most Watopia routes)
        (10.0, 1.0),  // Rolling (transition zone)
        (15.0, 0.93), // Rolling hills
        (20.0, 0.85), // Hilly (Mountain 8 ~21 m/km)
        (30.0, 0.70), // Very hilly
        (40.0, 0.55), // Mountain (approaching Alpe territory)
        (60.0, 0.45), // Extreme climb (Road to Sky ~60 m/km)
        (100.0, 0.30), // Theoretical maximum
    ];

    // Interpolate between breakpoints
    let mut base_multiplier = breakpoints.last().unwrap().1;
    for i in 0..breakpoints.len() - 1 {
        if meters_per_km < breakpoints[i + 1].0 {
            let (x0, y0) = breakpoints[i];
            let (x1, y1) = breakpoints[i + 1];
            let t = (meters_per_km - x0) / (x1 - x0);
            base_multiplier = y0 + t * (y1 - y0);
            break;
        }
    }

    // Category penalty on climbs: lower categories have less w/kg,
    // which matters much more on steep gradients than on flat terrain.
    // On flat: all categories ride near their empirical category speed (ratio ≈ 1.0)
    // On climbs: Cat D achieves ~48% of flat speed, Cat C ~54% (from race data)
    if meters_per_km > 15.0 {
        let category_factor = match category {
            "A++" | "A" | "A+" | "A-" => 1.0,
            "B" | "B+" | "B-" => 0.97,
            "C" | "C+" | "C-" => 0.93,
            "D" | "D+" | "D-" => 0.85,
            "E" | "E+" | "E-" => 0.75,
            _ => 0.85, // Default to Cat D
        };
        base_multiplier *= category_factor;
    }

    base_multiplier
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_route_difficulty_multiplier_from_elevation() {
        // The default (no category) uses Cat C as baseline
        // Test very flat routes (< 5m/km) — interpolated between 1.1 and 1.05
        let flat = get_route_difficulty_multiplier_from_elevation(40.0, 100); // 2.5 m/km
        assert!(flat > 1.07 && flat < 1.1, "Very flat should be ~1.08, got {}", flat);

        // Test flat to rolling (5-10m/km) — interpolated between 1.05 and 1.0
        let rolling = get_route_difficulty_multiplier_from_elevation(40.0, 300); // 7.5 m/km
        assert!(rolling > 0.97 && rolling < 1.03, "Rolling should be ~1.025, got {}", rolling);

        // Test rolling hills (10-20m/km) — interpolated between 1.0 and 0.85
        let hilly = get_route_difficulty_multiplier_from_elevation(40.0, 600); // 15 m/km
        assert!(hilly > 0.88 && hilly < 0.96, "Hilly should be ~0.93, got {}", hilly);

        // Test very hilly (>40m/km) — with Cat C penalty
        let mountain = get_route_difficulty_multiplier_from_elevation(12.2, 1035); // 84.8 m/km
        assert!(mountain > 0.25 && mountain < 0.40, "Mountain should be ~0.33, got {}", mountain);
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
        // The default function uses Cat C as baseline
        // With piecewise linear interpolation, values are continuous

        // Very flat (< 5 m/km): between 1.1 and 1.05
        let m1 = get_route_difficulty_multiplier_from_elevation(20.0, 50); // 2.5 m/km
        assert!(m1 > 1.07, "2.5 m/km should be > 1.07, got {}", m1);

        let m2 = get_route_difficulty_multiplier_from_elevation(10.0, 40); // 4 m/km
        assert!(m2 > 1.05, "4 m/km should be > 1.05, got {}", m2);

        // Flat to rolling (5-10 m/km): between 1.05 and 1.0
        let m3 = get_route_difficulty_multiplier_from_elevation(20.0, 150); // 7.5 m/km
        assert!(m3 > 1.0 && m3 < 1.05, "7.5 m/km should be ~1.025, got {}", m3);

        let m4 = get_route_difficulty_multiplier_from_elevation(30.0, 270); // 9 m/km
        assert!(m4 > 0.99 && m4 < 1.03, "9 m/km should be ~1.01, got {}", m4);

        // Rolling hills (10-15 m/km): between 1.0 and 0.93
        let m5 = get_route_difficulty_multiplier_from_elevation(20.0, 300); // 15 m/km
        assert!(m5 > 0.90 && m5 < 0.95, "15 m/km should be ~0.93, got {}", m5);

        // Hilly (15-20 m/km, Cat C penalty applies): ~0.89 * 0.93 = ~0.83
        let m6 = get_route_difficulty_multiplier_from_elevation(40.0, 760); // 19 m/km
        assert!(m6 > 0.78 && m6 < 0.88, "19 m/km Cat C should be ~0.83, got {}", m6);

        // Very hilly (25 m/km): with Cat C penalty
        let m7 = get_route_difficulty_multiplier_from_elevation(20.0, 500); // 25 m/km
        assert!(m7 > 0.65 && m7 < 0.78, "25 m/km Cat C should be ~0.72, got {}", m7);

        // Mountain (50 m/km): with Cat C penalty
        let m8 = get_route_difficulty_multiplier_from_elevation(10.0, 500); // 50 m/km
        assert!(m8 > 0.40 && m8 < 0.52, "50 m/km Cat C should be ~0.47, got {}", m8);

        // Edge cases
        let m9 = get_route_difficulty_multiplier_from_elevation(0.1, 1); // 10 m/km
        assert!(m9 > 0.98 && m9 < 1.02, "10 m/km should be ~1.0, got {}", m9);

        let m10 = get_route_difficulty_multiplier_from_elevation(1000.0, 5000); // 5 m/km
        assert!((m10 - 1.05).abs() < 0.01, "5 m/km should be 1.05, got {}", m10);
    }

    #[test]
    fn test_category_aware_elevation_multiplier() {
        // On flat terrain, category shouldn't matter
        let flat_c = get_route_difficulty_multiplier_from_elevation_and_category(40.0, 100, "C");
        let flat_d = get_route_difficulty_multiplier_from_elevation_and_category(40.0, 100, "D");
        assert_eq!(flat_c, flat_d, "On flat terrain, category shouldn't affect multiplier");

        // On steep climbs, lower categories should get a lower multiplier
        let climb_a = get_route_difficulty_multiplier_from_elevation_and_category(10.0, 600, "A"); // 60 m/km
        let climb_c = get_route_difficulty_multiplier_from_elevation_and_category(10.0, 600, "C");
        let climb_d = get_route_difficulty_multiplier_from_elevation_and_category(10.0, 600, "D");
        let climb_e = get_route_difficulty_multiplier_from_elevation_and_category(10.0, 600, "E");

        assert!(climb_a > climb_c, "Cat A should have higher multiplier than C on climbs");
        assert!(climb_c > climb_d, "Cat C should have higher multiplier than D on climbs");
        assert!(climb_d > climb_e, "Cat D should have higher multiplier than E on climbs");

        // Road to Sky test: 17.6km, 1047m = 59.5 m/km
        // Cat D actual: 110 min → speed 9.6 km/h → multiplier = 9.6/30.9 = 0.31
        let rts_d = get_route_difficulty_multiplier_from_elevation_and_category(17.6, 1047, "D");
        assert!(rts_d > 0.25 && rts_d < 0.45,
            "Road to Sky Cat D multiplier should be ~0.35, got {}", rts_d);

        // Cat C actual: 66 min → speed 16.0 km/h → multiplier = 16.0/33.0 = 0.48
        let rts_c = get_route_difficulty_multiplier_from_elevation_and_category(17.6, 1047, "C");
        assert!(rts_c > 0.35 && rts_c < 0.55,
            "Road to Sky Cat C multiplier should be ~0.42, got {}", rts_c);
    }

    // NOTE: test_weighted_average_calculation, test_drop_probability_match_guards,
    // and test_minutes_calculation were removed during test audit.
    // They tested reimplemented inline logic (no production function calls).
    // The dual-speed pack/solo model they tested was removed from production code.
    // The formulas they verified are already tested indirectly through
    // estimate_duration_for_category and property tests.
}
