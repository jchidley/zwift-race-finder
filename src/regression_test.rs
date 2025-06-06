// Regression tests comparing actual race times vs predictions

#[cfg(test)]
mod tests {
    use crate::constants::PERCENT_MULTIPLIER;
    use crate::database::Database;
    use crate::estimation::{
        estimate_duration_from_route_id, estimate_duration_with_distance, get_route_data,
    };
    use crate::parsing::parse_distance_from_name;

    #[test]
    fn test_race_predictions_accuracy() {
        // Load actual race data from database
        let db = Database::new().expect("Failed to open database");
        let results = db
            .get_all_race_results()
            .expect("Failed to get race results");

        // Cache multi-lap info to avoid repeated database queries
        let mut multi_lap_cache = std::collections::HashMap::new();

        let mut total_error = 0.0;
        let mut count = 0;
        let mut errors_by_route = std::collections::HashMap::new();

        println!("\n=== Regression Test: Actual vs Predicted Times ===\n");
        println!(
            "{:<40} {:>8} {:>8} {:>8} {:>6}",
            "Event", "Actual", "Predict", "Error", "%Err"
        );
        println!("{}", "-".repeat(80));

        // Limit to first 500 results for faster testing, excluding test races
        let test_results: Vec<_> = results.into_iter()
            .filter(|r| r.route_id != 9999 && !r.event_name.starts_with("Test Race"))
            .take(500)
            .collect();

        for result in test_results.iter() {
            // Estimate duration using route_id and actual distance
            let predicted = if let Some(distance_km) = parse_distance_from_name(&result.event_name)
            {
                // Use parsed distance for multi-lap races
                estimate_duration_with_distance(result.route_id, distance_km, result.zwift_score)
            } else {
                // Check for multi-lap events in cache or database
                let mut base_prediction =
                    estimate_duration_from_route_id(result.route_id, result.zwift_score);

                // Apply multi-lap multiplier if known
                let lap_count = multi_lap_cache
                    .entry(result.event_name.clone())
                    .or_insert_with(|| {
                        db.get_multi_lap_info(&result.event_name)
                            .unwrap_or(None)
                    });

                if let Some(laps) = lap_count {
                    base_prediction =
                        base_prediction.map(|d| (d as f64 * (*laps) as f64) as u32);
                }

                base_prediction
            };

            if let Some(predicted_minutes) = predicted {
                let actual_minutes = result.actual_minutes as f64;
                let predicted_minutes_f64 = predicted_minutes as f64;
                let error = predicted_minutes_f64 - actual_minutes;
                let percent_error = (error / actual_minutes * PERCENT_MULTIPLIER).abs();

                // Track errors by route
                let route_errors = errors_by_route.entry(result.route_id).or_insert(Vec::new());
                route_errors.push(percent_error);

                // Print details for large errors
                if percent_error > 20.0 {
                    println!(
                        "{:<40} {:>8.0} {:>8.0} {:>8.0} {:>6.1}%",
                        &result.event_name[..40.min(result.event_name.len())],
                        actual_minutes,
                        predicted_minutes_f64,
                        error,
                        percent_error
                    );
                }

                total_error += percent_error;
                count += 1;
            }
        }

        if count > 0 {
            let mean_error = total_error / count as f64;
            println!("\n{}", "=".repeat(80));
            println!("Mean Absolute Percentage Error: {:.1}%", mean_error);
            println!("Total races analyzed: {}", count);

            // Show error by route
            println!("\n=== Error Analysis by Route ===");
            println!("{:<30} {:>6} {:>10}", "Route", "Races", "Avg Error");
            println!("{}", "-".repeat(50));

            let mut route_stats: Vec<_> = errors_by_route
                .iter()
                .map(|(route_id, errors)| {
                    let avg_error = errors.iter().sum::<f64>() / errors.len() as f64;
                    let route_name = get_route_data(*route_id)
                        .map(|r| r.name.to_string())
                        .unwrap_or_else(|| format!("Route {}", route_id));
                    (route_name, errors.len(), avg_error)
                })
                .collect();

            route_stats.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

            for (route_name, count, avg_error) in route_stats.iter().take(10) {
                println!("{:<30} {:>6} {:>9.1}%", route_name, count, avg_error);
            }

            // Target: < 20% mean error
            assert!(
                mean_error < 30.0,
                "Mean error {:.1}% exceeds 30% threshold",
                mean_error
            );
        } else {
            println!("No races with mapped routes found for regression testing");
        }
    }

    #[test]
    #[ignore = "Run with --ignored for full regression test on all 7500+ races"]
    fn test_all_race_predictions_accuracy() {
        // Load actual race data from database
        let db = Database::new().expect("Failed to open database");
        let results = db
            .get_all_race_results()
            .expect("Failed to get race results");

        // Cache multi-lap info to avoid repeated database queries
        let mut multi_lap_cache = std::collections::HashMap::new();

        let mut total_error = 0.0;
        let mut count = 0;
        let mut errors_by_route = std::collections::HashMap::new();

        println!("\n=== Full Regression Test: Actual vs Predicted Times ===\n");
        println!(
            "{:<40} {:>8} {:>8} {:>8} {:>6}",
            "Event", "Actual", "Predict", "Error", "%Err"
        );
        println!("{}", "-".repeat(80));

        for result in results.iter() {
            // Skip placeholder routes for now
            if result.route_id == 9999 {
                continue;
            }

            // Estimate duration using route_id and actual distance
            let predicted = if let Some(distance_km) = parse_distance_from_name(&result.event_name)
            {
                // Use parsed distance for multi-lap races
                estimate_duration_with_distance(result.route_id, distance_km, result.zwift_score)
            } else {
                // Check for multi-lap events in cache or database
                let mut base_prediction =
                    estimate_duration_from_route_id(result.route_id, result.zwift_score);

                // Apply multi-lap multiplier if known
                let lap_count = multi_lap_cache
                    .entry(result.event_name.clone())
                    .or_insert_with(|| {
                        db.get_multi_lap_info(&result.event_name)
                            .unwrap_or(None)
                    });

                if let Some(laps) = lap_count {
                    base_prediction =
                        base_prediction.map(|d| (d as f64 * (*laps) as f64) as u32);
                }

                base_prediction
            };

            if let Some(predicted_minutes) = predicted {
                let actual_minutes = result.actual_minutes as f64;
                let predicted_minutes_f64 = predicted_minutes as f64;
                let error = predicted_minutes_f64 - actual_minutes;
                let percent_error = (error / actual_minutes * PERCENT_MULTIPLIER).abs();

                // Track errors by route
                let route_errors = errors_by_route.entry(result.route_id).or_insert(Vec::new());
                route_errors.push(percent_error);

                // Print details for large errors
                if percent_error > 20.0 && count < 50 {  // Limit output
                    println!(
                        "{:<40} {:>8.0} {:>8.0} {:>8.0} {:>6.1}%",
                        &result.event_name[..40.min(result.event_name.len())],
                        actual_minutes,
                        predicted_minutes_f64,
                        error,
                        percent_error
                    );
                }

                total_error += percent_error;
                count += 1;
            }
        }

        if count > 0 {
            let mean_error = total_error / count as f64;
            println!("\n{}", "=".repeat(80));
            println!("Mean Absolute Percentage Error: {:.1}%", mean_error);
            println!("Total races analyzed: {}", count);

            // Show error by route
            println!("\n=== Error Analysis by Route ===");
            println!("{:<30} {:>6} {:>10}", "Route", "Races", "Avg Error");
            println!("{}", "-".repeat(50));

            let mut route_stats: Vec<_> = errors_by_route
                .iter()
                .map(|(route_id, errors)| {
                    let avg_error = errors.iter().sum::<f64>() / errors.len() as f64;
                    let route_name = get_route_data(*route_id)
                        .map(|r| r.name.to_string())
                        .unwrap_or_else(|| format!("Route {}", route_id));
                    (route_name, errors.len(), avg_error)
                })
                .collect();

            route_stats.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

            for (route_name, count, avg_error) in route_stats.iter().take(10) {
                println!("{:<30} {:>6} {:>9.1}%", route_name, count, avg_error);
            }

            // Target: < 20% mean error
            assert!(
                mean_error < 30.0,
                "Mean error {:.1}% exceeds 30% threshold",
                mean_error
            );
        } else {
            println!("No races with mapped routes found for regression testing");
        }
    }

    #[test]
    fn test_specific_route_accuracy() {
        // Test specific routes we've mapped
        let test_cases = vec![
            // (route_id, zwift_score, expected_minutes_range)
            (3369744027, 195, (20, 25)), // Volcano Flat for Cat D (12.3km, 45m elevation)
            (1258415487, 195, (22, 28)), // Bell Lap for Cat D (14.1km flat)
            (3742187716, 195, (45, 60)), // Castle to Castle for Cat D
        ];

        for (route_id, zwift_score, (min_expected, max_expected)) in test_cases {
            let predicted = estimate_duration_from_route_id(route_id, zwift_score);

            if let Some(predicted_minutes) = predicted {
                let route_name = get_route_data(route_id)
                    .map(|r| r.name.to_string())
                    .unwrap_or_else(|| format!("Route {}", route_id));
                assert!(
                    predicted_minutes >= min_expected && predicted_minutes <= max_expected,
                    "Route {} prediction {} minutes outside expected range {}-{} minutes",
                    route_name,
                    predicted_minutes,
                    min_expected,
                    max_expected
                );
            }
        }
    }

    #[test]
    fn test_route_mapping_consistency() {
        // Ensure mapped routes have reasonable race times
        let db = Database::new().expect("Failed to open database");

        let results = db
            .get_all_race_results()
            .expect("Failed to get race results");

        // Group by route and check for outliers
        let mut route_times: std::collections::HashMap<u32, Vec<u32>> =
            std::collections::HashMap::new();

        for result in results
            .iter()
            .filter(|r| r.route_id != 9999 && !r.event_name.starts_with("Test Race"))
        {
            route_times
                .entry(result.route_id)
                .or_insert_with(Vec::new)
                .push(result.actual_minutes);
        }

        // Check each route for suspicious variance
        for (route_id, times) in route_times {
            if times.len() < 2 {
                continue;
            }

            let min = *times.iter().min().unwrap() as f64;
            let max = *times.iter().max().unwrap() as f64;
            let avg = times.iter().sum::<u32>() as f64 / times.len() as f64;

            // If max is more than 2x min, likely a mapping error
            if max > min * 2.0 {
                if let Some(route_data) = get_route_data(route_id) {
                    // Calculate expected speed range
                    let distance_km = route_data.distance_km;
                    let min_speed = distance_km * 60.0 / max; // km/h
                    let max_speed = distance_km * 60.0 / min; // km/h

                    println!("WARNING: Route {} shows high variance", route_data.name);
                    println!("  Times: {}-{} min (avg: {:.0})", min, max, avg);
                    println!("  Speeds: {:.1}-{:.1} km/h", min_speed, max_speed);

                    // Fail if speeds are unreasonably low (< 15 km/h for races)
                    assert!(
                        min_speed > 15.0,
                        "Route {} has suspiciously slow times - likely wrong distance or multi-lap",
                        route_data.name
                    );
                }
            }
        }
    }
}
