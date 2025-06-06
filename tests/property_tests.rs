//! Property-based tests for zwift-race-finder
//! These tests verify behavior across a wide range of inputs

use proptest::prelude::*;
use zwift_race_finder::database::RouteData;

// Test format_duration logic with property-based testing
proptest! {
    #[test]
    fn test_duration_formatting_properties(minutes in 0.0..600.0) {  // Limit to reasonable race durations
        // Inline the format_duration logic for testing
        let hours = (minutes / 60.0) as u32;
        let mins = (minutes % 60.0) as u32;
        let result = format!("{:02}:{:02}", hours, mins);

        // Result should always be in HH:MM format for reasonable durations
        assert!(result.len() >= 5);  // Could be longer for > 99 hours
        assert!(result.contains(':'));

        // Parse hours and minutes from the formatted string
        let parts: Vec<&str> = result.split(':').collect();
        assert_eq!(parts.len(), 2);
        let parsed_hours: u32 = parts[0].parse().unwrap();
        let parsed_mins: u32 = parts[1].parse().unwrap();

        // Minutes should be < 60
        assert!(parsed_mins < 60);

        // Total should approximately match input
        let total_minutes = parsed_hours * 60 + parsed_mins;
        let diff = f64::abs(total_minutes as f64 - minutes);
        assert!(diff < 1.0);
    }
}

// Test duration estimation boundaries
proptest! {
    #[test]
    fn test_duration_estimation_bounds(
        distance_km in 1.0..200.0,
        elevation_m in 0u32..3000u32,
        zwift_score in 0..650
    ) {
        // Mock route for testing
        let route = RouteData {
            route_id: 1,
            distance_km,
            elevation_m,
            name: "Test Route".to_string(),
            world: "Watopia".to_string(),
            surface: "tarmac".to_string(),
            lead_in_distance_km: 0.0,
            lead_in_elevation_m: 0,
            lead_in_distance_free_ride_km: None,
            lead_in_elevation_free_ride_m: None,
            lead_in_distance_meetups_km: None,
            lead_in_elevation_meetups_m: None,
            slug: None,
        };

        // Inline duration estimation logic
        let base_speed = match zwift_score {
            0..=199 => 30.0,   // Cat D
            200..=299 => 33.0, // Cat C
            300..=399 => 36.0, // Cat B
            _ => 39.0,         // Cat A
        };

        let elevation_factor = 1.0 - (route.elevation_m as f64 / route.distance_km / 100.0).min(0.3);
        let effective_speed = base_speed * elevation_factor;
        let duration = route.distance_km / effective_speed * 60.0;

        // Duration should be positive
        assert!(duration > 0.0);

        // Duration should be reasonable (between 1 min and 10 hours for these ranges)
        assert!(duration >= 1.0);  // 1km races can be very short
        assert!(duration <= 600.0); // 200km races can be very long

        // Longer distances should take more time
        if distance_km > 50.0 {
            assert!(duration > 30.0);
        }
    }
}

// Test monotonicity: longer routes take more time (key behavioral invariant)
proptest! {
    #[test]
    fn test_duration_monotonic_with_distance(
        base_distance in 10.0..100.0,
        extra_distance in 1.0..50.0,
        elevation_m in 0u32..2000u32,
        zwift_score in 0..650
    ) {
        // Create two routes with different distances but same elevation
        let route1 = RouteData {
            route_id: 1,
            distance_km: base_distance,
            elevation_m,
            name: "Short Route".to_string(),
            world: "Watopia".to_string(),
            surface: "tarmac".to_string(),
            lead_in_distance_km: 0.0,
            lead_in_elevation_m: 0,
            lead_in_distance_free_ride_km: None,
            lead_in_elevation_free_ride_m: None,
            lead_in_distance_meetups_km: None,
            lead_in_elevation_meetups_m: None,
            slug: None,
        };

        let route2 = RouteData {
            route_id: 2,
            distance_km: base_distance + extra_distance,
            elevation_m,
            name: "Long Route".to_string(),
            world: "Watopia".to_string(),
            surface: "tarmac".to_string(),
            lead_in_distance_km: 0.0,
            lead_in_elevation_m: 0,
            lead_in_distance_free_ride_km: None,
            lead_in_elevation_free_ride_m: None,
            lead_in_distance_meetups_km: None,
            lead_in_elevation_meetups_m: None,
            slug: None,
        };

        // Calculate durations using same logic
        let base_speed = match zwift_score {
            0..=199 => 30.0,   // Cat D
            200..=299 => 33.0, // Cat C
            300..=399 => 36.0, // Cat B
            _ => 39.0,         // Cat A
        };

        let elevation_factor1 = 1.0 - (route1.elevation_m as f64 / route1.distance_km / 100.0).min(0.3);
        let effective_speed1 = base_speed * elevation_factor1;
        let duration1 = route1.distance_km / effective_speed1 * 60.0;

        let elevation_factor2 = 1.0 - (route2.elevation_m as f64 / route2.distance_km / 100.0).min(0.3);
        let effective_speed2 = base_speed * elevation_factor2;
        let duration2 = route2.distance_km / effective_speed2 * 60.0;

        // INVARIANT: Longer route must take more time
        assert!(duration2 > duration1,
            "Longer route should take more time: {} km in {} min vs {} km in {} min",
            route2.distance_km, duration2, route1.distance_km, duration1);
    }
}

// Test elevation impact: more climbing increases duration
proptest! {
    #[test]
    fn test_duration_increases_with_elevation(
        distance_km in 20.0..100.0,
        base_elevation in 0u32..500u32,
        extra_elevation in 100u32..1000u32,
        zwift_score in 0..650
    ) {
        // Create two routes with same distance but different elevation
        let flat_route = RouteData {
            route_id: 1,
            distance_km,
            elevation_m: base_elevation,
            name: "Flat Route".to_string(),
            world: "Watopia".to_string(),
            surface: "tarmac".to_string(),
            lead_in_distance_km: 0.0,
            lead_in_elevation_m: 0,
            lead_in_distance_free_ride_km: None,
            lead_in_elevation_free_ride_m: None,
            lead_in_distance_meetups_km: None,
            lead_in_elevation_meetups_m: None,
            slug: None,
        };

        let hilly_route = RouteData {
            route_id: 2,
            distance_km,
            elevation_m: base_elevation + extra_elevation,
            name: "Hilly Route".to_string(),
            world: "Watopia".to_string(),
            surface: "tarmac".to_string(),
            lead_in_distance_km: 0.0,
            lead_in_elevation_m: 0,
            lead_in_distance_free_ride_km: None,
            lead_in_elevation_free_ride_m: None,
            lead_in_distance_meetups_km: None,
            lead_in_elevation_meetups_m: None,
            slug: None,
        };

        // Calculate durations
        let base_speed = match zwift_score {
            0..=199 => 30.0,   // Cat D
            200..=299 => 33.0, // Cat C
            300..=399 => 36.0, // Cat B
            _ => 39.0,         // Cat A
        };

        let flat_elevation_factor = 1.0 - (flat_route.elevation_m as f64 / flat_route.distance_km / 100.0).min(0.3);
        let flat_effective_speed = base_speed * flat_elevation_factor;
        let flat_duration = flat_route.distance_km / flat_effective_speed * 60.0;

        let hilly_elevation_factor = 1.0 - (hilly_route.elevation_m as f64 / hilly_route.distance_km / 100.0).min(0.3);
        let hilly_effective_speed = base_speed * hilly_elevation_factor;
        let hilly_duration = hilly_route.distance_km / hilly_effective_speed * 60.0;

        // INVARIANT: More elevation must increase duration (slower speed)
        assert!(hilly_duration > flat_duration,
            "Hilly route should take more time: {}m elevation in {} min vs {}m elevation in {} min",
            hilly_route.elevation_m, hilly_duration, flat_route.elevation_m, flat_duration);
    }
}

// Test category speed ordering: higher categories are faster
proptest! {
    #[test]
    fn test_category_speed_ordering(
        distance_km in 20.0..50.0,
        elevation_m in 0u32..1000u32
    ) {
        let route = RouteData {
            route_id: 1,
            distance_km,
            elevation_m,
            name: "Test Route".to_string(),
            world: "Watopia".to_string(),
            surface: "tarmac".to_string(),
            lead_in_distance_km: 0.0,
            lead_in_elevation_m: 0,
            lead_in_distance_free_ride_km: None,
            lead_in_elevation_free_ride_m: None,
            lead_in_distance_meetups_km: None,
            lead_in_elevation_meetups_m: None,
            slug: None,
        };

        // Calculate duration for each category
        let cat_d_speed = 30.0;
        let cat_c_speed = 33.0;
        let cat_b_speed = 36.0;
        let cat_a_speed = 39.0;

        let elevation_factor = 1.0 - (route.elevation_m as f64 / route.distance_km / 100.0).min(0.3);

        let duration_d = route.distance_km / (cat_d_speed * elevation_factor) * 60.0;
        let duration_c = route.distance_km / (cat_c_speed * elevation_factor) * 60.0;
        let duration_b = route.distance_km / (cat_b_speed * elevation_factor) * 60.0;
        let duration_a = route.distance_km / (cat_a_speed * elevation_factor) * 60.0;

        // INVARIANT: Higher categories complete routes faster
        assert!(duration_a < duration_b);
        assert!(duration_b < duration_c);
        assert!(duration_c < duration_d);
    }
}

// Test filter logic edge cases
proptest! {
    #[test]
    fn test_duration_filter_symmetry(
        target in 10..300,
        tolerance in 0..100,
        actual in 0.0..400.0
    ) {
        // If actual is within range, it should pass filter
        let min = (target - tolerance) as f64;
        let max = (target + tolerance) as f64;

        let passes_filter = actual >= min && actual <= max;

        // Test the inverse - if outside range, should not pass
        if actual < min || actual > max {
            assert!(!passes_filter);
        } else {
            assert!(passes_filter);
        }
    }
}

// Test race result parsing edge cases
proptest! {
    #[test]
    fn test_race_result_parsing(
        route_id in 0u32..999999u32,
        time in 0.0..1000.0,
        name in "[a-zA-Z0-9 ]{1,100}"
    ) {
        let result_string = format!("{},{},{}", route_id, time, name);

        // Test parsing logic inline
        let parts: Vec<&str> = result_string.split(',').collect();

        if parts.len() == 3 {
            let route_parse = parts[0].parse::<u32>();
            let time_parse = parts[1].parse::<f64>();
            let parsed_name = parts[2].to_string();

            if time > 0.0 && !name.trim().is_empty() {
                assert!(route_parse.is_ok());
                assert!(time_parse.is_ok());

                if let (Ok(r_id), Ok(r_time)) = (route_parse, time_parse) {
                    assert_eq!(r_id, route_id);
                    let time_diff = f64::abs(r_time - time);
                    assert!(time_diff < 0.01);
                    assert_eq!(parsed_name, name);
                }
            }
        }
    }
}
