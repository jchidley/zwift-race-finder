//! Property-based tests for zwift-race-finder
//!
//! These tests verify behavioral invariants by calling production functions
//! with randomly generated inputs. Every assertion references actual production
//! code — no reimplemented logic.

use proptest::prelude::*;
use zwift_race_finder::category::{
    get_category_from_score, get_category_speed, CAT_A_PLUS_SPEED, CAT_A_SPEED, CAT_B_SPEED,
    CAT_C_SPEED, CAT_D_SPEED, CAT_E_SPEED,
};
use zwift_race_finder::duration_estimation::{
    estimate_duration_for_category, get_route_difficulty_multiplier,
    get_route_difficulty_multiplier_from_elevation,
    get_route_difficulty_multiplier_from_elevation_and_category,
};

// ── Monotonicity: longer routes take more time ──────────────────────────────

proptest! {
    #[test]
    fn longer_distance_takes_longer(
        base_km in 5.0..100.0,
        extra_km in 1.0..50.0,
        zwift_score in 0u32..800
    ) {
        // Use a route name that gets the default multiplier (1.0)
        let short = estimate_duration_for_category(base_km, "Test Route", zwift_score);
        let long = estimate_duration_for_category(base_km + extra_km, "Test Route", zwift_score);
        prop_assert!(
            long >= short,
            "Longer route ({} km = {} min) should take >= shorter ({} km = {} min)",
            base_km + extra_km, long, base_km, short
        );
    }
}

// ── Monotonicity: more elevation slows you down ─────────────────────────────

proptest! {
    #[test]
    fn more_elevation_gives_lower_multiplier(
        distance_km in 10.0..100.0,
        base_elev in 0u32..500,
        extra_elev in 100u32..2000
    ) {
        let flat_mult = get_route_difficulty_multiplier_from_elevation(distance_km, base_elev);
        let hilly_mult = get_route_difficulty_multiplier_from_elevation(distance_km, base_elev + extra_elev);
        prop_assert!(
            hilly_mult <= flat_mult,
            "More elevation ({} m, mult={}) should give <= multiplier than ({} m, mult={})",
            base_elev + extra_elev, hilly_mult, base_elev, flat_mult
        );
    }
}

// ── Monotonicity: higher category = faster speed ────────────────────────────

proptest! {
    #[test]
    fn higher_category_completes_faster(
        distance_km in 10.0..80.0,
        zwift_score_low in 0u32..200,
        zwift_score_high in 400u32..800
    ) {
        // Higher score = higher category = faster
        let slow = estimate_duration_for_category(distance_km, "Test Route", zwift_score_low);
        let fast = estimate_duration_for_category(distance_km, "Test Route", zwift_score_high);
        prop_assert!(
            fast <= slow,
            "Higher category (score={}, {} min) should be <= lower (score={}, {} min)",
            zwift_score_high, fast, zwift_score_low, slow
        );
    }
}

// ── Category speed ordering is strictly monotonic ───────────────────────────

#[test]
fn category_speeds_are_strictly_ordered() {
    // This is an exhaustive check (6 values), not a property test — appropriate
    assert!(CAT_E_SPEED < CAT_D_SPEED);
    assert!(CAT_D_SPEED < CAT_C_SPEED);
    assert!(CAT_C_SPEED < CAT_B_SPEED);
    assert!(CAT_B_SPEED < CAT_A_SPEED);
    assert!(CAT_A_SPEED < CAT_A_PLUS_SPEED);
}

// ── Score→category→speed pipeline is monotonically non-decreasing ───────────

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

// ── Category-aware elevation: lower categories suffer more on climbs ────────

proptest! {
    #[test]
    fn lower_category_suffers_more_on_steep_climbs(
        distance_km in 10.0..50.0,
        elevation_m in 400u32..2000   // > 15 m/km to trigger category penalty
    ) {
        // Only test when meters_per_km > 15 (where category penalty applies)
        let meters_per_km = elevation_m as f64 / distance_km;
        prop_assume!(meters_per_km > 15.0);

        let mult_a = get_route_difficulty_multiplier_from_elevation_and_category(
            distance_km, elevation_m, "A",
        );
        let mult_c = get_route_difficulty_multiplier_from_elevation_and_category(
            distance_km, elevation_m, "C",
        );
        let mult_d = get_route_difficulty_multiplier_from_elevation_and_category(
            distance_km, elevation_m, "D",
        );
        let mult_e = get_route_difficulty_multiplier_from_elevation_and_category(
            distance_km, elevation_m, "E",
        );

        prop_assert!(mult_a >= mult_c, "Cat A ({}) should >= Cat C ({}) on climbs", mult_a, mult_c);
        prop_assert!(mult_c >= mult_d, "Cat C ({}) should >= Cat D ({}) on climbs", mult_c, mult_d);
        prop_assert!(mult_d >= mult_e, "Cat D ({}) should >= Cat E ({}) on climbs", mult_d, mult_e);
    }
}

// ── Difficulty multiplier is always positive ────────────────────────────────

proptest! {
    #[test]
    fn difficulty_multiplier_always_positive(
        distance_km in 1.0..200.0,
        elevation_m in 0u32..5000
    ) {
        let mult = get_route_difficulty_multiplier_from_elevation(distance_km, elevation_m);
        prop_assert!(mult > 0.0, "Multiplier should be positive, got {}", mult);
        prop_assert!(mult <= 1.1, "Multiplier should be <= 1.1 (max flat bonus), got {}", mult);
    }
}

// ── Route name-based multiplier: flat routes faster than hilly routes ───────

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

// ── Duration is always positive and bounded ─────────────────────────────────

proptest! {
    #[test]
    fn duration_positive_and_bounded(
        distance_km in 1.0..200.0,
        zwift_score in 0u32..800
    ) {
        let duration = estimate_duration_for_category(distance_km, "Test Route", zwift_score);
        // 1 km at 45 km/h A++ speed ≈ 1.3 min → rounds to 1
        // 200 km at 28 km/h E speed with 0.7 alpe multiplier ≈ 612 min
        prop_assert!(duration > 0, "Duration should be > 0, got {} for {} km", duration, distance_km);
        prop_assert!(duration < 700, "Duration should be < 700 min, got {} for {} km", duration, distance_km);
    }
}
