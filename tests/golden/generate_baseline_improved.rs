//! Improved golden baseline generation for UOM migration
//!
//! This version uses hardcoded test data instead of the production database
//! to ensure consistent, reproducible tests without external dependencies.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use zwift_race_finder::{
    category::get_category_from_score,
    duration_estimation::estimate_duration_for_category,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GoldenTest {
    function: String,
    inputs: serde_json::Value,
    output: serde_json::Value,
    context: String,
}

/// Representative test routes covering different profiles
fn get_test_routes() -> Vec<&'static str> {
    vec![
        // Flat routes
        "Tempus Fugit",      // Classic flat route
        "Tick Tock",         // Another flat option
        
        // Rolling/Mixed routes  
        "Watopia's Waistband",
        "Two Village Loop",
        "Downtown Dolphin",
        
        // Hilly routes
        "Hilly Route",
        "Castle to Castle", 
        "Epic KOM",
        
        // Mountain routes
        "Road to Sky",
        "Ven-Top",
        "Four Horsemen",
    ]
}

/// Focused set of test distances covering typical race lengths
fn get_test_distances() -> Vec<f64> {
    vec![
        // Short races
        10.0, 15.0, 20.0,
        // Medium races  
        25.0, 30.0, 40.0,
        // Long races
        50.0, 60.0, 80.0,
        // Edge cases
        0.1, 200.0,
    ]
}

/// Test scores covering category boundaries and centers
fn get_test_scores() -> Vec<u32> {
    vec![
        // Category boundaries
        99, 100,     // E/D boundary
        199, 200,    // D/C boundary  
        299, 300,    // C/B boundary
        399, 400,    // B/A boundary
        
        // Category centers
        150,  // Mid Cat D
        250,  // Mid Cat C
        350,  // Mid Cat B
        450,  // Mid Cat A
        
        // Edge cases
        0,    // Unrated
        999,  // Max score
    ]
}

#[test]
#[ignore] // Run with: cargo test generate_golden_baseline_improved -- --ignored
fn generate_golden_baseline_improved() {
    let mut golden_tests = Vec::new();
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");

    // Test estimate_duration_for_category
    // This function doesn't use database - it uses route names
    println!("Generating tests for estimate_duration_for_category...");
    for route_name in get_test_routes() {
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

    // Save the golden baseline
    let filename = format!("tests/golden/baseline_improved_{}.json", timestamp);
    let json = serde_json::to_string_pretty(&golden_tests).unwrap();
    fs::write(&filename, json).unwrap();

    println!(
        "\n✅ Generated {} golden tests in {}",
        golden_tests.len(),
        filename
    );
    
    println!("\nTest breakdown:");
    println!("  - Routes: {}", get_test_routes().len());
    println!("  - Distances: {}", get_test_distances().len());
    println!("  - Scores: {}", get_test_scores().len());
    println!("  - Total combinations: {}", 
        get_test_routes().len() * get_test_distances().len() * get_test_scores().len()
    );
    
    println!("\nThis improved version:");
    println!("  ✅ No database dependency");
    println!("  ✅ Consistent across environments");
    println!("  ✅ Focused on representative test cases");
    println!("  ✅ No cleanup required");
}

#[cfg(test)]
mod integration_with_test_db {
    use super::*;
    use std::env;
    use tempfile::TempDir;
    
    /// For tests that DO need database access, use a temporary test database
    #[test]
    #[ignore]
    fn test_with_temporary_database() {
        // Create temporary directory for test database
        let temp_dir = TempDir::new().unwrap();
        let test_db_path = temp_dir.path().join("test_races.db");
        
        // Override database path for this test
        env::set_var("ZWIFT_RACE_FINDER_DB", test_db_path.to_str().unwrap());
        
        // Run tests that need database...
        // Database will be automatically cleaned up when temp_dir drops
        
        env::remove_var("ZWIFT_RACE_FINDER_DB");
    }
}