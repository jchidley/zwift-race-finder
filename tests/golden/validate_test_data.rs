//! Validation tool to ensure test data is representative of real data
//!
//! This compares results from our focused test set against the full production
//! database to ensure we haven't introduced bias by reducing test cases.

use std::collections::HashMap;
use zwift_race_finder::{
    database::Database,
    duration_estimation::estimate_duration_for_category,
    estimation::{estimate_duration_from_route_id, get_route_data_from_db},
};

#[derive(Debug)]
struct ValidationStats {
    route_name: String,
    test_results: Vec<f64>,
    real_results: Vec<f64>,
    mean_difference: f64,
    max_difference: f64,
    correlation: f64,
}

/// Routes used in our focused test set
fn get_test_routes() -> Vec<&'static str> {
    vec![
        "Tempus Fugit",
        "Tick Tock",
        "Watopia's Waistband",
        "Two Village Loop", 
        "Downtown Dolphin",
        "Hilly Route",
        "Castle to Castle",
        "Epic KOM",
        "Road to Sky",
        "Ven-Top",
        "Four Horsemen",
    ]
}

/// Validate that our test routes are representative
#[test]
#[ignore] // Run with: cargo test validate_test_routes -- --ignored --nocapture
fn validate_test_routes() {
    println!("\n=== Validating Test Routes Against Production Database ===\n");
    
    // Try to connect to production database
    let db = match Database::new() {
        Ok(db) => db,
        Err(e) => {
            println!("❌ Cannot access production database: {}", e);
            println!("   This validation requires the production database at ~/.local/share/zwift-race-finder/races.db");
            return;
        }
    };
    
    // Get all routes from database
    let all_routes: Vec<(u32, String, f64, u32)> = db
        .conn
        .prepare("SELECT route_id, name, distance_km, elevation_m FROM routes")?
        .query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    
    println!("Found {} routes in production database", all_routes.len());
    
    // Check which test routes exist in database
    let test_routes = get_test_routes();
    let mut found_count = 0;
    let mut missing_routes = Vec::new();
    
    for test_route in &test_routes {
        if all_routes.iter().any(|(_, name, _, _)| name.contains(test_route)) {
            found_count += 1;
        } else {
            missing_routes.push(test_route);
        }
    }
    
    println!("\nTest Route Coverage:");
    println!("  ✓ Found {}/{} test routes in database", found_count, test_routes.len());
    
    if !missing_routes.is_empty() {
        println!("  ⚠ Missing routes: {:?}", missing_routes);
    }
    
    // Analyze route diversity
    analyze_route_diversity(&all_routes, &test_routes);
    
    // Compare duration estimates
    compare_duration_estimates(&all_routes);
}

/// Analyze if test routes cover the diversity of all routes
fn analyze_route_diversity(all_routes: &[(u32, String, f64, u32)], test_routes: &[&str]) {
    println!("\n📊 Route Diversity Analysis:");
    
    // Calculate statistics for all routes
    let all_distances: Vec<f64> = all_routes.iter().map(|(_, _, d, _)| *d).collect();
    let all_elevations: Vec<u32> = all_routes.iter().map(|(_, _, _, e)| *e).collect();
    
    // Calculate statistics for test routes
    let test_route_data: Vec<_> = all_routes
        .iter()
        .filter(|(_, name, _, _)| test_routes.iter().any(|tr| name.contains(tr)))
        .collect();
    
    let test_distances: Vec<f64> = test_route_data.iter().map(|(_, _, d, _)| *d).collect();
    let test_elevations: Vec<u32> = test_route_data.iter().map(|(_, _, _, e)| *e).collect();
    
    // Distance analysis
    println!("\n  Distance Coverage:");
    println!("    All routes:  min={:.1}km, max={:.1}km, avg={:.1}km",
        all_distances.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(),
        all_distances.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(),
        all_distances.iter().sum::<f64>() / all_distances.len() as f64
    );
    
    if !test_distances.is_empty() {
        println!("    Test routes: min={:.1}km, max={:.1}km, avg={:.1}km",
            test_distances.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(),
            test_distances.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(),
            test_distances.iter().sum::<f64>() / test_distances.len() as f64
        );
    }
    
    // Elevation analysis
    println!("\n  Elevation Coverage:");
    println!("    All routes:  min={}m, max={}m, avg={}m",
        all_elevations.iter().min().unwrap(),
        all_elevations.iter().max().unwrap(),
        all_elevations.iter().sum::<u32>() / all_elevations.len() as u32
    );
    
    if !test_elevations.is_empty() {
        println!("    Test routes: min={}m, max={}m, avg={}m",
            test_elevations.iter().min().unwrap(),
            test_elevations.iter().max().unwrap(),
            test_elevations.iter().sum::<u32>() / test_elevations.len() as u32
        );
    }
    
    // Elevation/distance ratio (difficulty indicator)
    println!("\n  Difficulty Distribution:");
    let all_difficulties: Vec<f64> = all_routes
        .iter()
        .map(|(_, _, d, e)| *e as f64 / *d)
        .collect();
    
    let test_difficulties: Vec<f64> = test_route_data
        .iter()
        .map(|(_, _, d, e)| *e as f64 / *d)
        .collect();
    
    println!("    All routes:  min={:.1}, max={:.1}, avg={:.1} m/km",
        all_difficulties.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(),
        all_difficulties.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(),
        all_difficulties.iter().sum::<f64>() / all_difficulties.len() as f64
    );
    
    if !test_difficulties.is_empty() {
        println!("    Test routes: min={:.1}, max={:.1}, avg={:.1} m/km",
            test_difficulties.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(),
            test_difficulties.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(),
            test_difficulties.iter().sum::<f64>() / test_difficulties.len() as f64
        );
    }
}

/// Compare duration estimates between test and all routes
fn compare_duration_estimates(all_routes: &[(u32, String, f64, u32)]) {
    println!("\n⏱️  Duration Estimate Comparison:");
    
    // Test parameters
    let test_distances = vec![20.0, 40.0, 60.0];
    let test_scores = vec![150, 250, 350]; // Cat D, C, B
    
    let mut all_durations = Vec::new();
    let mut test_durations = Vec::new();
    
    // Calculate durations for all routes
    for (_, name, _, _) in all_routes {
        for &distance in &test_distances {
            for &score in &test_scores {
                let duration = estimate_duration_for_category(distance, name, score);
                all_durations.push(duration as f64);
            }
        }
    }
    
    // Calculate durations for test routes only
    let test_routes = get_test_routes();
    for route in test_routes {
        for &distance in &test_distances {
            for &score in &test_scores {
                let duration = estimate_duration_for_category(distance, route, score);
                test_durations.push(duration as f64);
            }
        }
    }
    
    // Statistical comparison
    let all_mean = all_durations.iter().sum::<f64>() / all_durations.len() as f64;
    let test_mean = test_durations.iter().sum::<f64>() / test_durations.len() as f64;
    
    let all_std = calculate_std_dev(&all_durations, all_mean);
    let test_std = calculate_std_dev(&test_durations, test_mean);
    
    println!("\n  Statistical Comparison:");
    println!("    All routes:  mean={:.1} min, std={:.1}", all_mean, all_std);
    println!("    Test routes: mean={:.1} min, std={:.1}", test_mean, test_std);
    println!("    Difference:  {:.1}% mean, {:.1}% std dev",
        ((test_mean - all_mean) / all_mean * 100.0).abs(),
        ((test_std - all_std) / all_std * 100.0).abs()
    );
    
    // Distribution comparison
    println!("\n  Duration Distribution:");
    print_distribution("All routes", &all_durations);
    print_distribution("Test routes", &test_durations);
}

fn calculate_std_dev(values: &[f64], mean: f64) -> f64 {
    let variance = values.iter()
        .map(|v| (v - mean).powi(2))
        .sum::<f64>() / values.len() as f64;
    variance.sqrt()
}

fn print_distribution(label: &str, durations: &[f64]) {
    let mut sorted = durations.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let p10 = sorted[(sorted.len() as f64 * 0.1) as usize];
    let p50 = sorted[(sorted.len() as f64 * 0.5) as usize];
    let p90 = sorted[(sorted.len() as f64 * 0.9) as usize];
    
    println!("    {}: P10={:.0}min, P50={:.0}min, P90={:.0}min", 
        label, p10, p50, p90);
}

/// Validate against Jack's actual race results
#[test]
#[ignore] // Run manually: cargo test validate_against_race_history -- --ignored --nocapture
fn validate_against_race_history() {
    println!("\n=== Validating Test Data Against Race History ===\n");
    
    let db = match Database::new() {
        Ok(db) => db,
        Err(e) => {
            println!("❌ Cannot access database: {}", e);
            return;
        }
    };
    
    // Get Jack's race results
    let results: Vec<(u32, String, u32, u32)> = db
        .conn
        .prepare("SELECT route_id, event_name, actual_minutes, zwift_score FROM race_results WHERE route_id != 9999")?
        .query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    
    println!("Found {} race results to validate against", results.len());
    
    // Group by route
    let mut routes_with_results: HashMap<u32, Vec<(String, u32, u32)>> = HashMap::new();
    for (route_id, event_name, actual_minutes, score) in results {
        routes_with_results
            .entry(route_id)
            .or_insert_with(Vec::new)
            .push((event_name, actual_minutes, score));
    }
    
    println!("\nRoutes with race history: {}", routes_with_results.len());
    
    // Check which test routes have actual race data
    let test_routes = get_test_routes();
    let mut validated_count = 0;
    
    for (route_id, races) in &routes_with_results {
        if let Some(route_data) = get_route_data_from_db(*route_id) {
            if test_routes.iter().any(|tr| route_data.name.contains(tr)) {
                validated_count += 1;
                println!("\n  ✓ {} ({} races)", route_data.name, races.len());
                
                // Compare predictions
                let mut errors = Vec::new();
                for (event_name, actual, score) in races {
                    if let Some(predicted) = estimate_duration_from_route_id(*route_id, *score) {
                        let error = ((predicted as f64 - *actual as f64) / *actual as f64 * 100.0).abs();
                        errors.push(error);
                    }
                }
                
                if !errors.is_empty() {
                    let mean_error = errors.iter().sum::<f64>() / errors.len() as f64;
                    println!("    Mean prediction error: {:.1}%", mean_error);
                }
            }
        }
    }
    
    println!("\n📋 Summary:");
    println!("  Test routes with race history: {}/{}", validated_count, test_routes.len());
    
    if validated_count < test_routes.len() / 2 {
        println!("  ⚠️  Warning: Less than half of test routes have race history for validation");
    } else {
        println!("  ✅ Good coverage: Most test routes have been validated with real race data");
    }
}