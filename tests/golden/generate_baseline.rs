//! Golden baseline generation for UOM migration
//!
//! This captures the current behavior of all duration estimation functions
//! to ensure we preserve exact behavior during the migration.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use zwift_race_finder::{
    category::get_category_from_score,
    duration_estimation::estimate_duration_for_category,
    estimation::{estimate_duration_from_route_id, estimate_duration_with_distance},
    models::ZwiftEvent,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GoldenTest {
    function: String,
    inputs: serde_json::Value,
    output: serde_json::Value,
    context: String,
}

/// List of all known routes we want to test
fn get_test_routes() -> Vec<(&'static str, u32)> {
    vec![
        ("Watopia's Waistband", 1),
        ("Hilly Route", 2),
        ("Volcano Circuit", 3),
        ("Jungle Circuit", 5),
        ("Big Foot Hills", 11),
        ("Tempus Fugit", 12),
        ("The Pretzel", 14),
        ("Muir and the Mountain", 16),
        ("Three Sisters", 17),
        ("Four Horsemen", 18),
        ("The Uber Pretzel", 20),
        ("Road to Sky", 22),
        ("Two Village Loop", 25),
        ("Eastern Eight", 1001),
        ("Serpentine 8", 1002),
        ("Hilly Loop", 1003),
        ("Downtown Dolphin", 2001),
        ("Bell Lap", 2003),
        ("Castle to Castle", 3001),
        ("Rooftop Rendezvous", 5001),
        ("Sand and Sequoias", 5002),
        ("R.G.V.", 5003),
        ("Repack Ridge", 5010),
    ]
}

/// List of test distances in km
fn get_test_distances() -> Vec<f64> {
    vec![0.1, 1.0, 5.0, 10.0, 15.0, 20.0, 25.0, 30.0, 40.0, 50.0, 60.0, 80.0, 100.0, 150.0, 200.0]
}

/// List of test Zwift scores covering all categories
fn get_test_scores() -> Vec<u32> {
    vec![
        0, 50, 99,      // Cat D edge cases
        100, 150, 199,  // Cat D
        200, 250, 299,  // Cat C
        300, 350, 399,  // Cat B
        400, 450, 500,  // Cat A
        600, 999,       // Extreme cases
    ]
}

#[test]
#[ignore] // Run with: cargo test generate_golden_baseline -- --ignored
fn generate_golden_baseline() {
    let mut golden_tests = Vec::new();
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");

    // Test 1: estimate_duration_for_category with route names
    println!("Generating tests for estimate_duration_for_category...");
    for (route_name, _) in &get_test_routes() {
        for distance in &get_test_distances() {
            for score in &get_test_scores() {
                let duration = estimate_duration_for_category(*distance, route_name, *score);
                golden_tests.push(GoldenTest {
                    function: "estimate_duration_for_category".to_string(),
                    inputs: serde_json::json!({
                        "distance_km": distance,
                        "route_name": route_name,
                        "zwift_score": score
                    }),
                    output: serde_json::json!(duration),
                    context: format!(
                        "Testing {} km on {} with score {} (Cat {})",
                        distance,
                        route_name,
                        score,
                        get_category_from_score(*score)
                    ),
                });
            }
        }
    }

    // Test 2: estimate_duration_from_route_id
    println!("Generating tests for estimate_duration_from_route_id...");
    for (route_name, route_id) in &get_test_routes() {
        for score in &get_test_scores() {
            if let Some(duration) = estimate_duration_from_route_id(*route_id, *score) {
                golden_tests.push(GoldenTest {
                    function: "estimate_duration_from_route_id".to_string(),
                    inputs: serde_json::json!({
                        "route_id": route_id,
                        "zwift_score": score
                    }),
                    output: serde_json::json!(duration),
                    context: format!(
                        "Testing route {} (id: {}) with score {} (Cat {})",
                        route_name,
                        route_id,
                        score,
                        get_category_from_score(*score)
                    ),
                });
            }
        }
    }

    // Test 3: estimate_duration_with_distance
    println!("Generating tests for estimate_duration_with_distance...");
    for (route_name, route_id) in &get_test_routes() {
        for distance in &get_test_distances() {
            for score in &get_test_scores() {
                if let Some(duration) = estimate_duration_with_distance(*route_id, *distance, *score) {
                    golden_tests.push(GoldenTest {
                        function: "estimate_duration_with_distance".to_string(),
                        inputs: serde_json::json!({
                            "route_id": route_id,
                            "distance_km": distance,
                            "zwift_score": score
                        }),
                        output: serde_json::json!(duration),
                        context: format!(
                            "Testing route {} (id: {}) with {} km and score {} (Cat {})",
                            route_name,
                            route_id,
                            distance,
                            score,
                            get_category_from_score(*score)
                        ),
                    });
                }
            }
        }
    }

    // Test 4: Edge cases and special scenarios
    println!("Generating edge case tests...");
    
    // Test with unknown route IDs
    for route_id in &[9999u32, 0, u32::MAX] {
        for score in &[150u32, 250, 350] {
            let result = estimate_duration_from_route_id(*route_id, *score);
            golden_tests.push(GoldenTest {
                function: "estimate_duration_from_route_id".to_string(),
                inputs: serde_json::json!({
                    "route_id": route_id,
                    "zwift_score": score
                }),
                output: serde_json::json!(result),
                context: format!("Testing unknown route_id {} with score {}", route_id, score),
            });
        }
    }

    // Test with extreme distances
    for distance in &[0.0, 0.001, 1000.0, f64::MAX] {
        for score in &[195u32] {
            let duration = estimate_duration_for_category(*distance, "Test Route", *score);
            golden_tests.push(GoldenTest {
                function: "estimate_duration_for_category".to_string(),
                inputs: serde_json::json!({
                    "distance_km": distance,
                    "route_name": "Test Route",
                    "zwift_score": score
                }),
                output: serde_json::json!(duration),
                context: format!("Testing extreme distance {} km", distance),
            });
        }
    }

    // Save the golden baseline
    let filename = format!("tests/golden/baseline_{}.json", timestamp);
    let json = serde_json::to_string_pretty(&golden_tests).unwrap();
    fs::write(&filename, json).unwrap();

    println!(
        "\n✅ Generated {} golden tests in {}",
        golden_tests.len(),
        filename
    );
    println!("\nBaseline captured successfully! This file should be committed to git.");
    println!("Total test combinations:");
    println!("  - estimate_duration_for_category: {}", 
        get_test_routes().len() * get_test_distances().len() * get_test_scores().len()
    );
    println!("  - estimate_duration_from_route_id: {}", 
        get_test_routes().len() * get_test_scores().len()
    );
    println!("  - estimate_duration_with_distance: {}", 
        get_test_routes().len() * get_test_distances().len() * get_test_scores().len()
    );
    println!("  - Edge cases: {}", 
        3 * 3 + 4 // unknown routes + extreme distances
    );
}