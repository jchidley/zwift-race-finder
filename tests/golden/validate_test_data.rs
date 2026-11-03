use std::collections::HashMap;
use zwift_race_finder::{
    database::Database,
    duration_estimation::estimate_duration_for_category,
    estimation::{estimate_duration_from_route_id, get_route_data_from_db},
};


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

#[test]
#[ignore]
fn validate_test_routes() {
    println!("\n=== Validating Test Routes Against Production Database ===\n");

    let db = match Database::new() {
        Ok(db) => db,
        Err(e) => {
            println!("❌ Cannot access production database: {}", e);
            println!("   This validation requires the production database at ~/.local/share/zwift-race-finder/races.db");
            return;
        }
    };

    let all_routes = db.get_all_routes_basic().unwrap_or_else(|e| {
        println!("Error getting routes: {}", e);
        Vec::new()
    });

    println!("Found {} routes in production database", all_routes.len());

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

    analyze_route_diversity(&all_routes, &test_routes);

    compare_duration_estimates(&all_routes);
}

fn analyze_route_diversity(all_routes: &[(u32, String, f64, u32)], test_routes: &[&str]) {
    println!("\n📊 Route Diversity Analysis:");

    let all_distances: Vec<f64> = all_routes.iter().map(|(_, _, d, _)| *d).collect();
    let all_elevations: Vec<u32> = all_routes.iter().map(|(_, _, _, e)| *e).collect();

    let test_route_data: Vec<_> = all_routes
        .iter()
        .filter(|(_, name, _, _)| test_routes.iter().any(|tr| name.contains(tr)))
        .collect();

    let test_distances: Vec<f64> = test_route_data.iter().map(|(_, _, d, _)| *d).collect();
    let test_elevations: Vec<u32> = test_route_data.iter().map(|(_, _, _, e)| *e).collect();

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

fn compare_duration_estimates(all_routes: &[(u32, String, f64, u32)]) {
    println!("\n⏱️  Duration Estimate Comparison:");

    let test_distances = vec![20.0, 40.0, 60.0];
    let test_scores = vec![150, 250, 350];

    let mut all_durations = Vec::new();
    let mut test_durations = Vec::new();

    for (_, name, _, _) in all_routes {
        for &distance in &test_distances {
            for &score in &test_scores {
                let duration = estimate_duration_for_category(distance, name, score);
                all_durations.push(duration as f64);
            }
        }
    }

    let test_routes = get_test_routes();
    for route in test_routes {
        for &distance in &test_distances {
            for &score in &test_scores {
                let duration = estimate_duration_for_category(distance, route, score);
                test_durations.push(duration as f64);
            }
        }
    }

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

#[test]
#[ignore]
fn validate_against_race_history() {
    println!("\n=== Validating Test Data Against Race History ===\n");

    let db = match Database::new() {
        Ok(db) => db,
        Err(e) => {
            println!("❌ Cannot access database: {}", e);
            return;
        }
    };

    let results = db.get_race_results_for_validation().unwrap_or_else(|e| {
        println!("Error getting race results: {}", e);
        Vec::new()
    });

    println!("Found {} race results to validate against", results.len());

    let mut routes_with_results: HashMap<u32, Vec<(String, u32, u32)>> = HashMap::new();
    for (route_id, event_name, actual_minutes, score) in results {
        routes_with_results
            .entry(route_id)
            .or_insert_with(Vec::new)
            .push((event_name, actual_minutes, score));
    }

    println!("\nRoutes with race history: {}", routes_with_results.len());

    let test_routes = get_test_routes();
    let mut validated_count = 0;

    for (route_id, races) in &routes_with_results {
        if let Some(route_data) = get_route_data_from_db(*route_id) {
            if test_routes.iter().any(|tr| route_data.name.contains(tr)) {
                validated_count += 1;
                println!("\n  ✓ {} ({} races)", route_data.name, races.len());

                let mut errors = Vec::new();
                for (_event_name, actual, score) in races {
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