// Regression tests using actual race data
use crate::database::Database;
use crate::{estimate_duration_from_route_id, get_route_data};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_estimation_with_actual_data() {
        // Initialize database
        let db = Database::new().expect("Failed to create database");
        
        // Test parameters - using Jack's actual Zwift Racing Score
        let zwift_score = 189; // Jack's current score (Category D)
        
        // Get actual race results for routes we've mapped
        let test_cases = vec![
            (1015, "Volcano Flat"),       // We have 3 races, avg 69 min
            (1016, "Volcano Circuit"),     // We have 3 races
            (6, "Alpe du Zwift"),         // We have 3 races
            (236, "Innsbruckring"),       // We have 4 races
        ];
        
        println!("\nRegression Test Results:");
        println!("========================");
        println!("Route                | Actual Avg | Predicted | Error");
        println!("---------------------|------------|-----------|-------");
        
        let mut total_error = 0.0;
        let mut valid_tests = 0;
        
        for (route_id, route_name) in test_cases {
            // Get actual average time from database
            let actual_avg = db.get_average_race_time(route_id, zwift_score)
                .unwrap_or(None);
            
            // Get predicted time
            let predicted = estimate_duration_from_route_id(route_id, zwift_score);
            
            if let (Some(actual), Some(pred)) = (actual_avg, predicted) {
                let error = ((pred as f64 - actual as f64) / actual as f64 * 100.0).abs();
                total_error += error;
                valid_tests += 1;
                
                println!("{:<20} | {:>10} | {:>9} | {:>5.1}%",
                    route_name,
                    format!("{} min", actual),
                    format!("{} min", pred),
                    error
                );
            } else {
                println!("{:<20} | {:>10} | {:>9} | {:>5}",
                    route_name,
                    actual_avg.map_or("No data".to_string(), |a| format!("{} min", a)),
                    predicted.map_or("No pred".to_string(), |p| format!("{} min", p)),
                    "N/A"
                );
            }
        }
        
        if valid_tests > 0 {
            let mae = total_error / valid_tests as f64;
            println!("---------------------|------------|-----------|-------");
            println!("Mean Absolute Error: {:.1}%", mae);
            
            // Assert that predictions are within reasonable range (20% error)
            assert!(mae < 20.0, "Mean absolute error {:.1}% exceeds 20% threshold", mae);
        }
    }
    
    #[test] 
    fn test_specific_routes_accuracy() {
        // Test Volcano Flat specifically since we have good data
        let zwift_score = 189; // Jack's actual score
        let route_id = 1015;
        
        let predicted = estimate_duration_from_route_id(route_id, zwift_score);
        assert!(predicted.is_some(), "Should predict duration for Volcano Flat");
        
        let duration = predicted.unwrap();
        // Based on actual data: 3 races averaging 69 minutes
        assert!(duration >= 60 && duration <= 80, 
            "Volcano Flat prediction {} should be close to actual average of 69", duration);
    }
    
    #[test]
    fn test_route_data_availability() {
        // Ensure we can get route data for mapped routes
        let routes = vec![1015, 1016, 6, 236];
        
        for route_id in routes {
            let route_data = get_route_data(route_id);
            assert!(route_data.is_some(), "Route {} should have data", route_id);
            
            let data = route_data.unwrap();
            assert!(data.distance_km > 0.0, "Route {} should have valid distance", route_id);
            assert!(data.elevation_m >= 0, "Route {} should have valid elevation", route_id);
        }
    }
}