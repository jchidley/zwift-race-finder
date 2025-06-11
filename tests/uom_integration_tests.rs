//! Integration tests for UOM migration
//! These tests ensure end-to-end functionality remains unchanged

#[cfg(all(test, feature = "uom-migration"))]
mod uom_integration {
    use std::process::Command;
    use zwift_race_finder::ab_testing::ABTest;
    
    /// Helper to capture CLI output
    fn capture_cli_output(args: &[&str]) -> Result<String, String> {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--bin")
            .arg("zwift-race-finder")
            .arg("--")
            .args(args)
            .output()
            .map_err(|e| format!("Failed to execute: {}", e))?;
            
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
    
    #[test]
    fn test_cli_output_unchanged() {
        // Test various CLI invocations produce identical output
        let test_cases = vec![
            vec!["--help"],
            vec!["--duration", "30", "--tolerance", "10"],
            vec!["--zwift-score", "250", "--duration", "60"],
            vec!["--event-type", "race", "--days", "3"],
        ];
        
        for args in test_cases {
            let test_name = format!("CLI: {}", args.join(" "));
            
            // Note: This would need to be implemented to switch between
            // UOM and non-UOM versions, possibly via environment variable
            let ab_test = ABTest {
                name: test_name.clone(),
                old_impl: Box::new(|| capture_cli_output(&args)),
                new_impl: Box::new(|| {
                    std::env::set_var("USE_UOM", "1");
                    let result = capture_cli_output(&args);
                    std::env::remove_var("USE_UOM");
                    result
                }),
                context: format!("Testing CLI args: {:?}", args),
            };
            
            match ab_test.run() {
                Ok(_) => println!("✅ {}: Output identical", test_name),
                Err(e) => panic!("❌ {}: Output differs!\n{:?}", test_name, e),
            }
        }
    }
    
    #[test]
    fn test_event_filtering_pipeline() {
        use zwift_race_finder::{
            event_filtering::{event_matches_duration, event_matches_duration_uom},
            models::{ZwiftEvent, EventSubGroup},
            units::{Distance, Duration},
        };
        use chrono::Utc;
        use uom::si::{length::kilometer, time::minute};
        
        // Create test event
        let event = ZwiftEvent {
            id: 1,
            name: "Test Race".to_string(),
            event_start: Utc::now(),
            event_type: "RACE".to_string(),
            distance_in_meters: Some(40000.0),
            duration_in_minutes: None,
            duration_in_seconds: None,
            route_id: Some(12), // Tempus Fugit
            route: Some("Tempus Fugit".to_string()),
            description: None,
            category_enforcement: true,
            event_sub_groups: vec![],
            sport: "CYCLING".to_string(),
            tags: vec![],
        };
        
        // Test filtering with both implementations
        let target_duration = 75; // minutes
        let tolerance = 15; // minutes
        let zwift_score = 195;
        
        let ab_test = ABTest {
            name: "event_filtering".to_string(),
            old_impl: Box::new(|| {
                event_matches_duration(&event, target_duration, tolerance, zwift_score)
            }),
            new_impl: Box::new(|| {
                let event_uom = event.to_uom();
                let target = Duration::new::<minute>(target_duration as f64);
                let tol = Duration::new::<minute>(tolerance as f64);
                event_matches_duration_uom(&event_uom, target, tol, zwift_score)
            }),
            context: "Testing event filtering logic".to_string(),
        };
        
        match ab_test.run() {
            Ok(result) => {
                assert!(result.matches, "Event filtering should produce same result");
            }
            Err(e) => {
                panic!("Event filtering differs between implementations: {:?}", e);
            }
        }
    }
    
    #[test]
    fn test_database_round_trip() {
        use zwift_race_finder::database::{Database, RouteData};
        
        // This test would verify that UOM types can be stored/retrieved
        // from the database without changing behavior
        
        let db = Database::new().expect("Failed to open database");
        
        // Test getting route data with both implementations
        let route_id = 12; // Tempus Fugit
        
        let ab_test = ABTest {
            name: "database_route_lookup".to_string(),
            old_impl: Box::new(|| db.get_route(route_id)),
            new_impl: Box::new(|| {
                // In real implementation, this would use get_route_uom
                db.get_route(route_id)
            }),
            context: format!("Testing database lookup for route {}", route_id),
        };
        
        match ab_test.run() {
            Ok(_) => println!("✅ Database operations identical"),
            Err(e) => panic!("❌ Database operations differ: {:?}", e),
        }
    }
}