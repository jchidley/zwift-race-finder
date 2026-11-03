use proptest::prelude::*;
use zwift_race_finder::category::{
    get_category_from_score, get_category_speed, CAT_A_PLUS_SPEED, CAT_A_SPEED, CAT_B_SPEED,
    CAT_C_SPEED, CAT_D_SPEED, CAT_E_SPEED,
};
use zwift_race_finder::duration_estimation::{
    estimate_duration_for_category, get_route_difficulty_multiplier,
    get_route_difficulty_multiplier_from_gain_and_category,
};
use zwift_race_finder::parsing::{parse_distance_from_name, parse_description_data};


proptest! {
    #[test]
    fn longer_distance_takes_longer(
        base_km in 5.0..100.0,
        extra_km in 1.0..50.0,
        zwift_score in 0u32..800
    ) {
        let short = estimate_duration_for_category(base_km, "Test Route", zwift_score);
        let long = estimate_duration_for_category(base_km + extra_km, "Test Route", zwift_score);
        prop_assert!(
            long >= short,
            "Longer route ({} km = {} min) should take >= shorter ({} km = {} min)",
            base_km + extra_km, long, base_km, short
        );
    }
}


proptest! {
    #[test]
    fn more_elevation_gives_lower_multiplier(
        distance_km in 10.0..100.0,
        base_elev in 0u32..500,
        extra_elev in 100u32..2000
    ) {
        let flat_mult = get_route_difficulty_multiplier_from_gain_and_category(distance_km, f64::from(base_elev), "C");
        let hilly_mult = get_route_difficulty_multiplier_from_gain_and_category(distance_km, f64::from(base_elev + extra_elev), "C");
        prop_assert!(
            hilly_mult <= flat_mult,
            "More elevation ({} m, mult={}) should give <= multiplier than ({} m, mult={})",
            base_elev + extra_elev, hilly_mult, base_elev, flat_mult
        );
    }
}


proptest! {
    #[test]
    fn higher_category_completes_faster(
        distance_km in 10.0..80.0,
        zwift_score_low in 0u32..200,
        zwift_score_high in 400u32..800
    ) {
        let slow = estimate_duration_for_category(distance_km, "Test Route", zwift_score_low);
        let fast = estimate_duration_for_category(distance_km, "Test Route", zwift_score_high);
        prop_assert!(
            fast <= slow,
            "Higher category (score={}, {} min) should be <= lower (score={}, {} min)",
            zwift_score_high, fast, zwift_score_low, slow
        );
    }
}


#[test]
fn category_speeds_are_strictly_ordered() {
    assert!(CAT_E_SPEED < CAT_D_SPEED);
    assert!(CAT_D_SPEED < CAT_C_SPEED);
    assert!(CAT_C_SPEED < CAT_B_SPEED);
    assert!(CAT_B_SPEED < CAT_A_SPEED);
    assert!(CAT_A_SPEED < CAT_A_PLUS_SPEED);
}


proptest! {
    #[test]
    fn higher_score_means_equal_or_faster_speed(
        score_low in 0u32..650,
        delta in 1u32..200
    ) {
        let score_high = score_low.saturating_add(delta).min(999);
        let cat_low = get_category_from_score(score_low);
        let cat_high = get_category_from_score(score_high);
        let speed_low = get_category_speed(cat_low);
        let speed_high = get_category_speed(cat_high);
        prop_assert!(
            speed_high >= speed_low,
            "Score {} (cat {}, {} km/h) should be >= score {} (cat {}, {} km/h)",
            score_high, cat_high, speed_high, score_low, cat_low, speed_low
        );
    }
}


proptest! {
    #[test]
    fn lower_category_suffers_more_on_steep_climbs(
        distance_km in 10.0..50.0,
        elevation_m in 400u32..2000
    ) {
        let meters_per_km = elevation_m as f64 / distance_km;
        prop_assume!(meters_per_km > 15.0);

        let mult_a = get_route_difficulty_multiplier_from_gain_and_category(distance_km, f64::from(elevation_m), "A");
        let mult_c = get_route_difficulty_multiplier_from_gain_and_category(distance_km, f64::from(elevation_m), "C");
        let mult_d = get_route_difficulty_multiplier_from_gain_and_category(distance_km, f64::from(elevation_m), "D");
        let mult_e = get_route_difficulty_multiplier_from_gain_and_category(distance_km, f64::from(elevation_m), "E");

        prop_assert!(mult_a >= mult_c, "Cat A ({}) should >= Cat C ({}) on climbs", mult_a, mult_c);
        prop_assert!(mult_c >= mult_d, "Cat C ({}) should >= Cat D ({}) on climbs", mult_c, mult_d);
        prop_assert!(mult_d >= mult_e, "Cat D ({}) should >= Cat E ({}) on climbs", mult_d, mult_e);
    }
}


proptest! {
    #[test]
    fn difficulty_multiplier_always_positive(
        distance_km in 1.0..200.0,
        elevation_m in 0u32..5000
    ) {
        let mult = get_route_difficulty_multiplier_from_gain_and_category(distance_km, f64::from(elevation_m), "C");
        prop_assert!(mult > 0.0, "Multiplier should be positive, got {}", mult);
        prop_assert!(mult <= 1.1, "Multiplier should be <= 1.1 (max flat bonus), got {}", mult);
    }
}


#[test]
fn named_route_multiplier_ordering() {
    let flat = get_route_difficulty_multiplier("Tempus Fugit");
    let default = get_route_difficulty_multiplier("Some Random Route");
    let mountain = get_route_difficulty_multiplier("Epic Mountain");
    let alpe = get_route_difficulty_multiplier("Alpe du Zwift");

    assert!(flat > default, "Flat ({}) > default ({})", flat, default);
    assert!(default > mountain, "Default ({}) > mountain ({})", default, mountain);
    assert!(mountain > alpe, "Mountain ({}) > alpe ({})", mountain, alpe);
}


proptest! {
    #[test]
    fn duration_positive_and_bounded(
        distance_km in 1.0..200.0,
        zwift_score in 0u32..800
    ) {
        let duration = estimate_duration_for_category(distance_km, "Test Route", zwift_score);
        prop_assert!(duration > 0, "Duration should be > 0, got {} for {} km", duration, distance_km);
        prop_assert!(duration < 700, "Duration should be < 700 min, got {} for {} km", duration, distance_km);
    }
}


proptest! {
    #[test]
    fn km_distance_roundtrip(
        distance in 1.0f64..999.0
    ) {
        let formatted = format!("{:.1} km race", distance);
        let parsed = parse_distance_from_name(&formatted);
        prop_assert!(
            parsed.is_some(),
            "Should parse distance from '{}'", formatted
        );
        let parsed_val = parsed.unwrap();
        let expected = (distance * 10.0).round() / 10.0;
        prop_assert!(
            (parsed_val - expected).abs() < 0.01,
            "Parsed {} from '{}', expected {}", parsed_val, formatted, expected
        );
    }
}


proptest! {
    #[test]
    fn lap_count_roundtrip(
        laps in 1u32..100
    ) {
        let formatted = format!("{} laps of Watopia", laps);
        let parsed = parse_description_data(&Some(formatted.clone())).laps;
        prop_assert_eq!(
            parsed, Some(laps),
            "Should parse {} laps from '{}'", laps, formatted
        );
    }
}
