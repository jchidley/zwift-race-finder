//! Snapshot tests for OCR telemetry extraction
//! These tests capture the current output and alert on changes

#[cfg(all(test, feature = "ocr"))]
mod snapshot_tests {
    use anyhow::Result;
    use insta::{assert_yaml_snapshot, with_settings};
    use std::path::Path;
    use zwift_race_finder::ocr_compact::{
        extract_telemetry, is_likely_name, parse_leaderboard_data, parse_time, LeaderboardEntry,
        TelemetryData,
    };

    /// Test parse_time with various inputs
    #[test]
    fn snapshot_parse_time_variations() {
        let test_cases = vec![
            ("12:34", "standard format"),
            ("1:23", "single digit hour"),
            ("Time: 45:67", "with prefix"),
            ("1234", "just digits"),
            ("123", "three digits"),
            ("12 : 34", "spaces inside"),
            ("  12:34  ", "spaces outside"),
            ("invalid", "non-time string"),
            ("", "empty string"),
            ("12:34:56", "with seconds"),
            ("+12:34", "with plus sign"),
            ("-12:34", "with minus sign"),
        ];

        let results: Vec<(&str, &str, Option<String>)> = test_cases
            .into_iter()
            .map(|(input, desc)| (input, desc, parse_time(input)))
            .collect();

        assert_yaml_snapshot!(results);
    }

    /// Test is_likely_name with various inputs
    #[test]
    fn snapshot_is_likely_name_variations() {
        let thirty_chars = "A".repeat(30);
        let thirty_one_chars = "A".repeat(31);
        
        let test_cases = vec![
            ("J.Chidley", "typical format"),
            ("John Doe", "full name"),
            ("A.", "single initial"),
            ("C.J.Y.S", "multiple initials"),
            ("Name (ABC)", "with suffix"),
            ("123", "just numbers"),
            ("12.3 km", "distance data"),
            ("3.2 w/kg", "power data"),
            ("+00:12", "time delta"),
            ("", "empty string"),
            ("a", "single char"),
            ("A" , "single uppercase"),
            ("Player1", "with number"),
            ("!!!", "just symbols"),
            ("(!", "parenthesis symbol"),
            ("José", "unicode name"),
            ("O'Brien", "with apostrophe"),
            ("Anne-Marie", "hyphenated"),
            ("  AB  ", "with spaces"),
            (thirty_chars.as_str(), "30 chars"),
            (thirty_one_chars.as_str(), "31 chars"),
        ];

        let results: Vec<(&str, &str, bool)> = test_cases
            .into_iter()
            .map(|(input, desc)| (input, desc, is_likely_name(input)))
            .collect();

        assert_yaml_snapshot!(results);
    }

    /// Test parse_leaderboard_data with various inputs
    #[test]
    fn snapshot_parse_leaderboard_data_variations() {
        let test_cases = vec![
            ("+01:23 3.2 w/kg 12.5 KM", "full data"),
            ("-00:45", "just negative delta"),
            ("12.5 km", "just distance"),
            ("3.2 w/kg", "just power"),
            ("text 4.5 text", "decimal in text"),
            ("0.0 w/kg", "zero power"),
            ("0.4 w/kg", "below range power"),
            ("7.1 w/kg", "above range power"),
            ("184.8 km", "large distance"),
            ("", "empty string"),
            ("+1:23", "single digit minute"),
            ("12.5KM", "no space km"),
            ("3.2w/kg", "no space wkg"),
            ("invalid data", "no numbers"),
            ("123", "just number"),
        ];

        let results: Vec<(String, LeaderboardEntry)> = test_cases
            .into_iter()
            .map(|(input, desc)| {
                let mut entry = LeaderboardEntry {
                    name: "Test".to_string(),
                    current: false,
                    delta: None,
                    km: None,
                    wkg: None,
                };
                parse_leaderboard_data(&mut entry, input);
                (format!("{} - {}", desc, input), entry)
            })
            .collect();

        assert_yaml_snapshot!(results);
    }

    /// Helper to get test image path
    fn test_image(filename: &str) -> std::path::PathBuf {
        Path::new("docs/screenshots").join(filename)
    }

    /// Test full telemetry extraction from normal riding image
    #[test]
    #[ignore] // Remove ignore when test images are available
    fn snapshot_telemetry_normal_ride() -> Result<()> {
        let image_path = test_image("normal_1_01_16_02_21.jpg");
        
        if !image_path.exists() {
            eprintln!("Skipping test: {} not found", image_path.display());
            return Ok(());
        }

        let telemetry = extract_telemetry(&image_path)?;
        
        // Snapshot with settings to handle potential variations
        with_settings!({
            description => "Normal riding telemetry extraction",
            omit_expression => true
        }, {
            assert_yaml_snapshot!(telemetry);
        });
        
        Ok(())
    }

    /// Test full telemetry extraction from climbing image
    #[test]
    #[ignore] // Remove ignore when test images are available
    fn snapshot_telemetry_climbing() -> Result<()> {
        let image_path = test_image("with_climbing_1_01_36_01_42.jpg");
        
        if !image_path.exists() {
            eprintln!("Skipping test: {} not found", image_path.display());
            return Ok(());
        }

        let telemetry = extract_telemetry(&image_path)?;
        
        with_settings!({
            description => "Climbing telemetry extraction",
            omit_expression => true
        }, {
            assert_yaml_snapshot!(telemetry);
        });
        
        Ok(())
    }

    /// Test telemetry structure with mock data
    #[test]
    fn snapshot_telemetry_structure() {
        let telemetry = TelemetryData {
            speed: Some(30),
            distance: Some(12.5),
            altitude: Some(150),
            race_time: Some("15:30".to_string()),
            power: Some(250),
            cadence: Some(85),
            heart_rate: Some(165),
            gradient: Some(5.5),
            distance_to_finish: Some(3.2),
            leaderboard: Some(vec![
                LeaderboardEntry {
                    name: "Leader".to_string(),
                    current: false,
                    delta: Some("-01:23".to_string()),
                    km: Some(13.5),
                    wkg: Some(4.2),
                },
                LeaderboardEntry {
                    name: "You".to_string(),
                    current: true,
                    delta: None,
                    km: Some(12.5),
                    wkg: Some(3.2),
                },
                LeaderboardEntry {
                    name: "Behind".to_string(),
                    current: false,
                    delta: Some("+00:45".to_string()),
                    km: Some(11.8),
                    wkg: Some(2.9),
                },
            ]),
            rider_pose: Some(zwift_race_finder::ocr_compact::RiderPose::ClimbingSeated),
        };

        with_settings!({
            description => "Complete telemetry data structure",
            omit_expression => true
        }, {
            assert_yaml_snapshot!(telemetry);
        });
    }

    /// Test edge cases in telemetry
    #[test]
    fn snapshot_telemetry_edge_cases() {
        // All None values
        let empty_telemetry = TelemetryData::default();
        
        // Mix of Some and None
        let partial_telemetry = TelemetryData {
            speed: Some(0),
            distance: Some(0.0),
            altitude: None,
            race_time: Some("00:00".to_string()),
            power: Some(0),
            cadence: None,
            heart_rate: None,
            gradient: Some(0.0),
            distance_to_finish: Some(0.0),
            leaderboard: Some(vec![]),
            rider_pose: Some(zwift_race_finder::ocr_compact::RiderPose::Unknown),
        };
        
        // Maximum realistic values
        let max_telemetry = TelemetryData {
            speed: Some(99),
            distance: Some(999.9),
            altitude: Some(9999),
            race_time: Some("99:99".to_string()),
            power: Some(2000),
            cadence: Some(200),
            heart_rate: Some(220),
            gradient: Some(25.0),
            distance_to_finish: Some(100.0),
            leaderboard: None,
            rider_pose: Some(zwift_race_finder::ocr_compact::RiderPose::NormalTuck),
        };

        let edge_cases = vec![
            ("empty", empty_telemetry),
            ("partial", partial_telemetry),
            ("maximum", max_telemetry),
        ];

        assert_yaml_snapshot!(edge_cases);
    }

    /// Test leaderboard extraction patterns
    #[test]
    fn snapshot_leaderboard_patterns() {
        let test_entries = vec![
            // Standard entry
            LeaderboardEntry {
                name: "J.Rider".to_string(),
                current: false,
                delta: Some("+01:23".to_string()),
                km: Some(42.5),
                wkg: Some(3.8),
            },
            // Current rider (no delta)
            LeaderboardEntry {
                name: "Current".to_string(),
                current: true,
                delta: None,
                km: Some(41.2),
                wkg: Some(3.5),
            },
            // Minimal data
            LeaderboardEntry {
                name: "MinData".to_string(),
                current: false,
                delta: Some("-00:05".to_string()),
                km: None,
                wkg: None,
            },
            // Edge case values
            LeaderboardEntry {
                name: "Edge".to_string(),
                current: false,
                delta: Some("+99:99".to_string()),
                km: Some(0.0),
                wkg: Some(0.0),
            },
            // Unicode name
            LeaderboardEntry {
                name: "José García".to_string(),
                current: false,
                delta: Some("-12:34".to_string()),
                km: Some(55.5),
                wkg: Some(4.5),
            },
        ];

        assert_yaml_snapshot!(test_entries);
    }
}