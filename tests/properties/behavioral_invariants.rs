use proptest::prelude::*;
use zwift_race_finder::{
    duration_estimation::estimate_duration_for_category,
    estimation::{estimate_duration_from_route_id, estimate_duration_with_distance},
};

fn route_names() -> Vec<&'static str> {
    vec![
        "Watopia's Waistband",
        "Hilly Route",
        "Volcano Circuit",
        "Tempus Fugit",
        "The Pretzel",
        "Mountain Route",
        "Jungle Circuit",
        "Road to Sky",
        "Two Village Loop",
        "Downtown Dolphin",
        "Bell Lap",
        "Castle to Castle",
        "Sand and Sequoias",
        "Three Sisters",
        "Four Horsemen",
    ]
}

fn route_name_strategy() -> impl Strategy<Value = &'static str> {
    prop::sample::select(route_names())
}

proptest! {
    #[test]
    fn duration_monotonic_with_distance(
        route_name in route_name_strategy(),
        score in 100..500u32,
        d1 in 0.1..500.0,
        d2 in 0.1..500.0,
    ) {
        let dur1 = estimate_duration_for_category(d1, route_name, score);
        let dur2 = estimate_duration_for_category(d2, route_name, score);

        if (d1 - d2).abs() < 0.1 {
            assert!((dur1 as i32 - dur2 as i32).abs() <= 1,
                "Very close distances should have similar durations");
        } else if d1 < d2 {
            assert!(dur1 <= dur2,
                "Distance {} km must not take more time than {} km (got {} min vs {} min)",
                d1, d2, dur1, dur2);
        } else {
            assert!(dur1 >= dur2,
                "Distance {} km must not take less time than {} km (got {} min vs {} min)",
                d1, d2, dur1, dur2);
        }
    }

    #[test]
    fn duration_inverse_with_skill(
        route_name in route_name_strategy(),
        distance in 10.0..100.0,
        s1 in 100..600u32,
        s2 in 100..600u32,
    ) {
        let dur1 = estimate_duration_for_category(distance, route_name, s1);
        let dur2 = estimate_duration_for_category(distance, route_name, s2);

        if s1 == s2 {
            assert_eq!(dur1, dur2,
                "Same score {} must give same duration", s1);
        }
        else if s1 < s2 {
            assert!(dur1 >= dur2,
                "Rider with score {} must not be faster than rider with score {} (got {} min vs {} min)",
                s1, s2, dur1, dur2);
        } else {
            assert!(dur1 <= dur2,
                "Rider with score {} must not be slower than rider with score {} (got {} min vs {} min)",
                s1, s2, dur1, dur2);
        }
    }

    #[test]
    fn duration_within_reasonable_bounds(
        route_name in route_name_strategy(),
        distance in 0.1..300.0,
        score in 50..600u32,
    ) {
        let duration = estimate_duration_for_category(distance, route_name, score);

        if distance >= 1.0 {
            assert!(duration > 0,
                "Duration must be positive for {} km at score {}", distance, score);
        }

        if distance < 200.0 {
            assert!(duration < 600,
                "Duration must be less than 10 hours for {} km at score {}", distance, score);
        } else {
            assert!(duration < 1440,
                "Duration must be less than 24 hours for {} km at score {}", distance, score);
        }
    }

    #[test]
    fn category_transitions_affect_speed(
        route_name in route_name_strategy(),
        distance in 20.0..60.0,
    ) {
        let cat_d_high = estimate_duration_for_category(distance, route_name, 199);
        let cat_c_low = estimate_duration_for_category(distance, route_name, 200);
        let cat_c_high = estimate_duration_for_category(distance, route_name, 299);
        let cat_b_low = estimate_duration_for_category(distance, route_name, 300);

        assert!(cat_c_low <= cat_d_high,
            "Cat C (score 200) should not be slower than Cat D (score 199)");
        assert!(cat_b_low <= cat_c_high,
            "Cat B (score 300) should not be slower than Cat C (score 299)");
    }

    #[test]
    fn route_difficulty_affects_duration(
        distance in 20.0..80.0,
        score in 150..350u32,
    ) {
        let flat_duration = estimate_duration_for_category(distance, "Tempus Fugit", score);
        let hilly_duration = estimate_duration_for_category(distance, "Hilly Route", score);
        let mountain_duration = estimate_duration_for_category(distance, "Road to Sky", score);

        if distance > 30.0 {
            assert!(flat_duration <= hilly_duration,
                "Flat route should not be slower than hilly route for {} km", distance);
            assert!(hilly_duration <= mountain_duration,
                "Hilly route should not be slower than mountain route for {} km", distance);
        }
    }

    #[test]
    fn route_id_estimation_deterministic(
        route_id in 1..100u32,
        score in 100..500u32,
    ) {
        let result1 = estimate_duration_from_route_id(route_id, score);
        let result2 = estimate_duration_from_route_id(route_id, score);

        assert_eq!(result1, result2,
            "Same route_id {} and score {} must give same result", route_id, score);
    }

    #[test]
    fn custom_distance_affects_duration(
        route_id in 1..30u32,
        base_distance in 10.0..50.0,
        multiplier in 0.5..2.0,
        score in 150..350u32,
    ) {
        let distance1 = base_distance;
        let distance2 = base_distance * multiplier;

        let dur1 = estimate_duration_with_distance(route_id, distance1, score);
        let dur2 = estimate_duration_with_distance(route_id, distance2, score);

        if let (Some(d1), Some(d2)) = (dur1, dur2) {
            if multiplier > 1.1 {
                assert!(d2 >= d1,
                    "Distance {} km should take longer than {} km", distance2, distance1);
            } else if multiplier < 0.9 {
                assert!(d2 <= d1,
                    "Distance {} km should take less than {} km", distance2, distance1);
            }
        }
    }

    #[test]
    fn below_half_minute_estimates_round_to_zero(
        route_name in route_name_strategy(),
        distance in 0.001..0.01,
        score in 100..500u32,
    ) {
        prop_assert_eq!(estimate_duration_for_category(distance, route_name, score), 0);
    }

    #[test]
    fn zero_score_handling(
        route_name in route_name_strategy(),
        distance in 10.0..50.0,
    ) {
        let duration = estimate_duration_for_category(distance, route_name, 0);

        assert!(duration > 0, "Zero score should still produce valid duration");

        let cat_d_duration = estimate_duration_for_category(distance, route_name, 150);
        assert!(duration >= cat_d_duration,
            "Unrated rider (score 0) should not be faster than Cat D");
    }
}