//! Integration tests for OCR with real images and golden data
//! These tests verify the full OCR pipeline against known good outputs

#[cfg(all(test, feature = "ocr"))]
mod integration_tests {
    use anyhow::Result;
    use serde::{Deserialize, Serialize};
    use std::fs;
    use std::path::{Path, PathBuf};
    use zwift_race_finder::ocr_compact::{extract_telemetry, TelemetryData};

    /// Golden test data structure
    #[derive(Debug, Serialize, Deserialize)]
    struct GoldenTestCase {
        image_path: String,
        description: String,
        expected: TelemetryData,
        tolerance: ToleranceConfig,
    }

    /// Tolerance configuration for comparing OCR results
    #[derive(Debug, Serialize, Deserialize)]
    struct ToleranceConfig {
        speed_tolerance: Option<u32>,
        distance_tolerance: Option<f64>,
        altitude_tolerance: Option<u32>,
        power_tolerance: Option<u32>,
        gradient_tolerance: Option<f64>,
        allow_missing_leaderboard: bool,
        allow_missing_pose: bool,
    }

    impl Default for ToleranceConfig {
        fn default() -> Self {
            Self {
                speed_tolerance: Some(2),      // ±2 km/h
                distance_tolerance: Some(0.2), // ±0.2 km
                altitude_tolerance: Some(5),   // ±5 m
                power_tolerance: Some(10),     // ±10 watts
                gradient_tolerance: Some(1.0), // ±1%
                allow_missing_leaderboard: true,
                allow_missing_pose: true,
            }
        }
    }

    /// Helper to get test image path
    fn test_image(filename: &str) -> PathBuf {
        Path::new("docs/screenshots").join(filename)
    }

    /// Helper to get golden data path
    fn golden_data_path() -> PathBuf {
        Path::new("tests/golden/ocr_golden_data.json").to_path_buf()
    }

    /// Compare two telemetry results with tolerance
    fn compare_telemetry(
        actual: &TelemetryData,
        expected: &TelemetryData,
        tolerance: &ToleranceConfig,
    ) -> Result<()> {
        // Speed comparison
        match (actual.speed, expected.speed) {
            (Some(a), Some(e)) => {
                let diff = (a as i32 - e as i32).abs() as u32;
                let tol = tolerance.speed_tolerance.unwrap_or(0);
                if diff > tol {
                    anyhow::bail!(
                        "Speed mismatch: expected {}, got {} (tolerance: {})",
                        e,
                        a,
                        tol
                    );
                }
            }
            (None, None) => {}
            (a, e) => anyhow::bail!("Speed presence mismatch: expected {:?}, got {:?}", e, a),
        }

        // Distance comparison
        match (actual.distance, expected.distance) {
            (Some(a), Some(e)) => {
                let diff = (a - e).abs();
                let tol = tolerance.distance_tolerance.unwrap_or(0.0);
                if diff > tol {
                    anyhow::bail!(
                        "Distance mismatch: expected {}, got {} (tolerance: {})",
                        e,
                        a,
                        tol
                    );
                }
            }
            (None, None) => {}
            (a, e) => anyhow::bail!("Distance presence mismatch: expected {:?}, got {:?}", e, a),
        }

        // Altitude comparison
        match (actual.altitude, expected.altitude) {
            (Some(a), Some(e)) => {
                let diff = (a as i32 - e as i32).abs() as u32;
                let tol = tolerance.altitude_tolerance.unwrap_or(0);
                if diff > tol {
                    anyhow::bail!(
                        "Altitude mismatch: expected {}, got {} (tolerance: {})",
                        e,
                        a,
                        tol
                    );
                }
            }
            (None, None) => {}
            (a, e) => anyhow::bail!("Altitude presence mismatch: expected {:?}, got {:?}", e, a),
        }

        // Race time comparison (exact match for strings)
        if actual.race_time != expected.race_time {
            anyhow::bail!(
                "Race time mismatch: expected {:?}, got {:?}",
                expected.race_time,
                actual.race_time
            );
        }

        // Power comparison
        match (actual.power, expected.power) {
            (Some(a), Some(e)) => {
                let diff = (a as i32 - e as i32).abs() as u32;
                let tol = tolerance.power_tolerance.unwrap_or(0);
                if diff > tol {
                    anyhow::bail!(
                        "Power mismatch: expected {}, got {} (tolerance: {})",
                        e,
                        a,
                        tol
                    );
                }
            }
            (None, None) => {}
            (a, e) => anyhow::bail!("Power presence mismatch: expected {:?}, got {:?}", e, a),
        }

        // Gradient comparison
        match (actual.gradient, expected.gradient) {
            (Some(a), Some(e)) => {
                let diff = (a - e).abs();
                let tol = tolerance.gradient_tolerance.unwrap_or(0.0);
                if diff > tol {
                    anyhow::bail!(
                        "Gradient mismatch: expected {}, got {} (tolerance: {})",
                        e,
                        a,
                        tol
                    );
                }
            }
            (None, None) => {}
            (a, e) => anyhow::bail!("Gradient presence mismatch: expected {:?}, got {:?}", e, a),
        }

        // Leaderboard comparison (if not allowed to be missing)
        if !tolerance.allow_missing_leaderboard {
            match (&actual.leaderboard, &expected.leaderboard) {
                (Some(a), Some(e)) => {
                    if a.len() != e.len() {
                        anyhow::bail!(
                            "Leaderboard length mismatch: expected {}, got {}",
                            e.len(),
                            a.len()
                        );
                    }
                    // Could add more detailed comparison here
                }
                (None, None) => {}
                (a, e) => {
                    anyhow::bail!("Leaderboard presence mismatch: expected {:?}, got {:?}", e, a)
                }
            }
        }

        // Pose comparison (if not allowed to be missing)
        if !tolerance.allow_missing_pose && actual.rider_pose != expected.rider_pose {
            anyhow::bail!(
                "Rider pose mismatch: expected {:?}, got {:?}",
                expected.rider_pose,
                actual.rider_pose
            );
        }

        Ok(())
    }

    /// Load golden test data from JSON file
    fn load_golden_data() -> Result<Vec<GoldenTestCase>> {
        let path = golden_data_path();
        if !path.exists() {
            // Return empty vec if no golden data exists yet
            return Ok(vec![]);
        }
        let content = fs::read_to_string(&path)?;
        let cases: Vec<GoldenTestCase> = serde_json::from_str(&content)?;
        Ok(cases)
    }

    /// Save golden test data to JSON file
    #[allow(dead_code)]
    fn save_golden_data(cases: &[GoldenTestCase]) -> Result<()> {
        let path = golden_data_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(cases)?;
        fs::write(&path, content)?;
        Ok(())
    }

    /// Integration test for normal riding image
    #[test]
    #[ignore] // Remove ignore when test images and golden data are available
    fn test_normal_ride_integration() -> Result<()> {
        let image_path = test_image("normal_1_01_16_02_21.jpg");
        
        if !image_path.exists() {
            eprintln!("Skipping test: {} not found", image_path.display());
            return Ok(());
        }

        // Extract telemetry
        let actual = extract_telemetry(&image_path)?;
        
        // Load golden data
        let golden_cases = load_golden_data()?;
        let golden = golden_cases
            .iter()
            .find(|c| c.image_path.ends_with("normal_1_01_16_02_21.jpg"))
            .ok_or_else(|| anyhow::anyhow!("No golden data for normal ride image"))?;
        
        // Compare with tolerance
        compare_telemetry(&actual, &golden.expected, &golden.tolerance)?;
        
        Ok(())
    }

    /// Integration test for climbing image
    #[test]
    #[ignore] // Remove ignore when test images and golden data are available
    fn test_climbing_integration() -> Result<()> {
        let image_path = test_image("with_climbing_1_01_36_01_42.jpg");
        
        if !image_path.exists() {
            eprintln!("Skipping test: {} not found", image_path.display());
            return Ok(());
        }

        let actual = extract_telemetry(&image_path)?;
        
        let golden_cases = load_golden_data()?;
        let golden = golden_cases
            .iter()
            .find(|c| c.image_path.ends_with("with_climbing_1_01_36_01_42.jpg"))
            .ok_or_else(|| anyhow::anyhow!("No golden data for climbing image"))?;
        
        compare_telemetry(&actual, &golden.expected, &golden.tolerance)?;
        
        Ok(())
    }

    /// Helper to generate golden data from current extractions
    #[test]
    #[ignore] // Only run manually to generate golden data
    fn generate_golden_data() -> Result<()> {
        let test_images = vec![
            ("normal_1_01_16_02_21.jpg", "Normal riding on flat terrain"),
            ("with_climbing_1_01_36_01_42.jpg", "Climbing with elevation"),
        ];
        
        let mut cases = Vec::new();
        
        for (filename, description) in test_images {
            let image_path = test_image(filename);
            if !image_path.exists() {
                eprintln!("Skipping {}: not found", filename);
                continue;
            }
            
            match extract_telemetry(&image_path) {
                Ok(telemetry) => {
                    cases.push(GoldenTestCase {
                        image_path: filename.to_string(),
                        description: description.to_string(),
                        expected: telemetry,
                        tolerance: ToleranceConfig::default(),
                    });
                    println!("Generated golden data for {}", filename);
                }
                Err(e) => {
                    eprintln!("Failed to extract from {}: {}", filename, e);
                }
            }
        }
        
        if !cases.is_empty() {
            save_golden_data(&cases)?;
            println!("Saved {} golden test cases", cases.len());
        }
        
        Ok(())
    }

    /// Test comparing against existing telemetry JSON files
    #[test]
    fn test_against_telemetry_json() -> Result<()> {
        // Check if we have the telemetry JSON file
        let json_path = Path::new("docs/screenshots/normal_1_01_16_02_21_telemetry.json");
        if !json_path.exists() {
            eprintln!("Skipping test: telemetry JSON not found");
            return Ok(());
        }
        
        // Read the JSON file
        let json_content = fs::read_to_string(json_path)?;
        let json_data: serde_json::Value = serde_json::from_str(&json_content)?;
        
        // Print some key values to understand the format
        println!("JSON telemetry structure:");
        if let Some(speed) = json_data.get("speed") {
            println!("  Speed: {:?}", speed);
        }
        if let Some(distance) = json_data.get("distance") {
            println!("  Distance: {:?}", distance);
        }
        if let Some(altitude) = json_data.get("altitude") {
            println!("  Altitude: {:?}", altitude);
        }
        
        // Note: This JSON appears to be from the Python OCR tool
        // and has a different structure than our Rust TelemetryData
        
        Ok(())
    }
}