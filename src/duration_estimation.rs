use crate::category::{get_category_from_score, get_category_speed};
use crate::constants::MINUTES_PER_HOUR;

pub fn get_route_difficulty_multiplier_from_gain_and_category(
    distance_km: f64,
    elevation_m: f64,
    category: &str,
) -> f64 {
    let meters_per_km = elevation_m / distance_km;

    let breakpoints: &[(f64, f64)] = &[
        (0.0, 1.1),
        (5.0, 1.05),
        (10.0, 1.0),
        (15.0, 0.93),
        (20.0, 0.85),
        (30.0, 0.70),
        (40.0, 0.55),
        (60.0, 0.45),
        (100.0, 0.30),
    ];

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

    if meters_per_km > 15.0 {
        let category_factor = match category {
            "A++" | "A" | "A+" | "A-" => 1.0,
            "B" | "B+" | "B-" => 0.97,
            "C" | "C+" | "C-" => 0.93,
            "D" | "D+" | "D-" => 0.85,
            "E" | "E+" | "E-" => 0.75,
            _ => 0.85,
        };
        base_multiplier *= category_factor;
    }

    base_multiplier
}

pub fn get_route_difficulty_multiplier(route_name: &str) -> f64 {
    let route_lower = route_name.to_lowercase();

    if route_lower.contains("alpe") || route_lower.contains("ventoux") {
        0.7
    } else if route_lower.contains("epic") || route_lower.contains("mountain") {
        0.8
    } else if route_lower.contains("flat") || route_lower.contains("tempus") {
        1.1
    } else {
        1.0
    }
}

pub fn estimate_duration_for_category(distance_km: f64, route_name: &str, zwift_score: u32) -> u32 {
    let category = get_category_from_score(zwift_score);
    let base_speed = get_category_speed(category);

    let difficulty_multiplier = get_route_difficulty_multiplier(route_name);
    let effective_speed = base_speed * difficulty_multiplier;

    let duration_hours = distance_km / effective_speed;
    (duration_hours * MINUTES_PER_HOUR as f64).round() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fallback_rounds_only_the_final_minutes() {
        for (minutes, expected) in [
            (0.0, 0),
            (0.49, 0),
            (0.5, 1),
            (30.49, 30),
            (30.5, 31),
            (30.51, 31),
        ] {
            assert_eq!(
                estimate_duration_for_category(minutes * 0.75, "Synthetic neutral route", 650),
                expected
            );
        }
    }

    #[test]
    fn test_get_route_difficulty_multiplier_from_elevation() {
        let flat = get_route_difficulty_multiplier_from_gain_and_category(40.0, f64::from(100), "C");
        assert!(
            flat > 1.07 && flat < 1.1,
            "Very flat should be ~1.08, got {}",
            flat
        );

        let rolling = get_route_difficulty_multiplier_from_gain_and_category(40.0, f64::from(300), "C");
        assert!(
            rolling > 0.97 && rolling < 1.03,
            "Rolling should be ~1.025, got {}",
            rolling
        );

        let hilly = get_route_difficulty_multiplier_from_gain_and_category(40.0, f64::from(600), "C");
        assert!(
            hilly > 0.88 && hilly < 0.96,
            "Hilly should be ~0.93, got {}",
            hilly
        );

        let mountain = get_route_difficulty_multiplier_from_gain_and_category(12.2, f64::from(1035), "C");
        assert!(
            mountain > 0.25 && mountain < 0.40,
            "Mountain should be ~0.33, got {}",
            mountain
        );
    }

    #[test]
    fn test_estimate_duration_for_category() {
        let alpe_duration = estimate_duration_for_category(12.2, "Alpe du Zwift", 195);
        assert!(
            alpe_duration >= 33 && alpe_duration <= 35,
            "Alpe (12.2km with 1035m elevation) should take 33-35 min for Cat D, got {}",
            alpe_duration
        );

        let tempus_time = estimate_duration_for_category(17.3, "Tempus Fugit", 250);
        assert!(
            tempus_time >= 26 && tempus_time <= 29,
            "Tempus Fugit should take 26-29 min for Cat C, got {}",
            tempus_time
        );
    }

    #[test]
    fn test_duration_estimation_for_cat_d() {

        let watopia_time = estimate_duration_for_category(40.0, "Watopia", 195);
        assert_eq!(watopia_time, 78);

        let alpe_time = estimate_duration_for_category(30.0, "Alpe du Zwift", 195);
        assert_eq!(alpe_time, 83);

        let tempus_time = estimate_duration_for_category(35.0, "Tempus Fugit", 195);
        assert_eq!(tempus_time, 62);
    }

    #[test]
    fn test_get_route_difficulty_multiplier() {
        assert_eq!(get_route_difficulty_multiplier("Alpe du Zwift"), 0.7);
        assert_eq!(get_route_difficulty_multiplier("Road to Sky"), 1.0);
        assert_eq!(get_route_difficulty_multiplier("Ven-Top"), 1.0);
        assert_eq!(get_route_difficulty_multiplier("Mont Ventoux"), 0.7);
        assert_eq!(get_route_difficulty_multiplier("ALPE DU ZWIFT"), 0.7);

        assert_eq!(get_route_difficulty_multiplier("Epic KOM"), 0.8);
        assert_eq!(get_route_difficulty_multiplier("Mountain Route"), 0.8);

        assert_eq!(get_route_difficulty_multiplier("Tempus Fugit"), 1.1);
        assert_eq!(get_route_difficulty_multiplier("Flatlands"), 1.1);

        assert_eq!(get_route_difficulty_multiplier("Regular Route"), 1.0);
        assert_eq!(get_route_difficulty_multiplier(""), 1.0);
    }

    #[test]
    fn test_specific_route_multipliers() {
        let flat_distance = 40.0;
        let zwift_score = 195;

        let tempus = estimate_duration_for_category(flat_distance, "Tempus Fugit", zwift_score);
        let alpe = estimate_duration_for_category(flat_distance, "Alpe du Zwift", zwift_score);
        let normal = estimate_duration_for_category(flat_distance, "Regular Route", zwift_score);

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

        assert_eq!(tempus, 71);
        assert_eq!(normal, 78);
        assert_eq!(alpe, 111);
    }

    #[test]
    fn test_edge_case_estimations() {
        let sprint_duration = estimate_duration_for_category(5.0, "Sprint Route", 195);
        assert!(
            sprint_duration >= 8 && sprint_duration <= 12,
            "Sprint (5km) should take 8-12 minutes, got {}",
            sprint_duration
        );

        let fondo_duration = estimate_duration_for_category(100.0, "Epic Route", 195);
        assert!(
            fondo_duration >= 180 && fondo_duration <= 250,
            "Gran Fondo (100km) should take 3-4.2 hours, got {} min",
            fondo_duration
        );

        let zero_duration = estimate_duration_for_category(0.0, "Zero Route", 195);
        assert_eq!(zero_duration, 0, "Zero distance should give zero duration");

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

        let m1 = get_route_difficulty_multiplier_from_gain_and_category(20.0, f64::from(50), "C");
        assert!(m1 > 1.07, "2.5 m/km should be > 1.07, got {}", m1);

        let m2 = get_route_difficulty_multiplier_from_gain_and_category(10.0, f64::from(40), "C");
        assert!(m2 > 1.05, "4 m/km should be > 1.05, got {}", m2);

        let m3 = get_route_difficulty_multiplier_from_gain_and_category(20.0, f64::from(150), "C");
        assert!(
            m3 > 1.0 && m3 < 1.05,
            "7.5 m/km should be ~1.025, got {}",
            m3
        );

        let m4 = get_route_difficulty_multiplier_from_gain_and_category(30.0, f64::from(270), "C");
        assert!(m4 > 0.99 && m4 < 1.03, "9 m/km should be ~1.01, got {}", m4);

        let m5 = get_route_difficulty_multiplier_from_gain_and_category(20.0, f64::from(300), "C");
        assert!(
            m5 > 0.90 && m5 < 0.95,
            "15 m/km should be ~0.93, got {}",
            m5
        );

        let m6 = get_route_difficulty_multiplier_from_gain_and_category(40.0, f64::from(760), "C");
        assert!(
            m6 > 0.78 && m6 < 0.88,
            "19 m/km Cat C should be ~0.83, got {}",
            m6
        );

        let m7 = get_route_difficulty_multiplier_from_gain_and_category(20.0, f64::from(500), "C");
        assert!(
            m7 > 0.65 && m7 < 0.78,
            "25 m/km Cat C should be ~0.72, got {}",
            m7
        );

        let m8 = get_route_difficulty_multiplier_from_gain_and_category(10.0, f64::from(500), "C");
        assert!(
            m8 > 0.40 && m8 < 0.52,
            "50 m/km Cat C should be ~0.47, got {}",
            m8
        );

        let m9 = get_route_difficulty_multiplier_from_gain_and_category(0.1, f64::from(1), "C");
        assert!(m9 > 0.98 && m9 < 1.02, "10 m/km should be ~1.0, got {}", m9);

        let m10 = get_route_difficulty_multiplier_from_gain_and_category(1000.0, f64::from(5000), "C");
        assert!(
            (m10 - 1.05).abs() < 0.01,
            "5 m/km should be 1.05, got {}",
            m10
        );
    }

    #[test]
    fn test_category_aware_elevation_multiplier() {
        let flat_c = get_route_difficulty_multiplier_from_gain_and_category(40.0, f64::from(100), "C");
        let flat_d = get_route_difficulty_multiplier_from_gain_and_category(40.0, f64::from(100), "D");
        assert_eq!(
            flat_c, flat_d,
            "On flat terrain, category shouldn't affect multiplier"
        );

        let climb_a = get_route_difficulty_multiplier_from_gain_and_category(10.0, f64::from(600), "A");
        let climb_c = get_route_difficulty_multiplier_from_gain_and_category(10.0, f64::from(600), "C");
        let climb_d = get_route_difficulty_multiplier_from_gain_and_category(10.0, f64::from(600), "D");
        let climb_e = get_route_difficulty_multiplier_from_gain_and_category(10.0, f64::from(600), "E");

        assert!(
            climb_a > climb_c,
            "Cat A should have higher multiplier than C on climbs"
        );
        assert!(
            climb_c > climb_d,
            "Cat C should have higher multiplier than D on climbs"
        );
        assert!(
            climb_d > climb_e,
            "Cat D should have higher multiplier than E on climbs"
        );

        let rts_d = get_route_difficulty_multiplier_from_gain_and_category(17.6, f64::from(1047), "D");
        assert!(
            rts_d > 0.25 && rts_d < 0.45,
            "Road to Sky Cat D multiplier should be ~0.35, got {}",
            rts_d
        );

        let rts_c = get_route_difficulty_multiplier_from_gain_and_category(17.6, f64::from(1047), "C");
        assert!(
            rts_c > 0.35 && rts_c < 0.55,
            "Road to Sky Cat C multiplier should be ~0.42, got {}",
            rts_c
        );
    }

}
