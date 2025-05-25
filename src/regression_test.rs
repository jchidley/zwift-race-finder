// Regression tests comparing actual race times vs predictions

#[cfg(test)]
mod tests {
    use crate::database::Database;
    use crate::{estimate_duration_from_route_id, estimate_duration_with_distance, 
                parse_distance_from_name, get_route_data};
    
    #[test]
    fn test_race_predictions_accuracy() {
        // Load actual race data from database
        let db = Database::new().expect("Failed to open database");
        let results = db.get_all_race_results().expect("Failed to get race results");
        
        let mut total_error = 0.0;
        let mut count = 0;
        let mut errors_by_route = std::collections::HashMap::new();
        
        println!("\n=== Regression Test: Actual vs Predicted Times ===\n");
        println!("{:<40} {:>8} {:>8} {:>8} {:>6}", "Event", "Actual", "Predict", "Error", "%Err");
        println!("{}", "-".repeat(80));
        
        for result in results.iter() {
            // Skip placeholder routes for now
            if result.route_id == 9999 {
                continue;
            }
            
            // Estimate duration using route_id and actual distance
            let predicted = if let Some(distance_km) = parse_distance_from_name(&result.event_name) {
                // Use parsed distance for multi-lap races
                estimate_duration_with_distance(
                    result.route_id,
                    distance_km,
                    result.zwift_score,
                )
            } else {
                // Fall back to base route distance
                estimate_duration_from_route_id(
                    result.route_id,
                    result.zwift_score,
                )
            };
            
            if let Some(predicted_minutes) = predicted {
                let actual_minutes = result.actual_minutes as f64;
                let predicted_minutes_f64 = predicted_minutes as f64;
                let error = predicted_minutes_f64 - actual_minutes;
                let percent_error = (error / actual_minutes * 100.0).abs();
                    
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
            
            let mut route_stats: Vec<_> = errors_by_route.iter()
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
            assert!(mean_error < 30.0, "Mean error {:.1}% exceeds 30% threshold", mean_error);
        } else {
            println!("No races with mapped routes found for regression testing");
        }
    }
    
    #[test]
    fn test_specific_route_accuracy() {
        // Test specific routes we've mapped
        let test_cases = vec![
            // (route_id, zwift_score, expected_minutes_range)
            (3369744027, 195, (25, 35)),  // Volcano Flat for Cat D
            (1258415487, 195, (30, 40)),  // Bell Lap for Cat D
            (3742187716, 195, (50, 70)),  // Castle to Castle for Cat D
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
}