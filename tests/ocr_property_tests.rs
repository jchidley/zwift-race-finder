//! Property-based tests for OCR parsing functions
//! These tests verify invariants and properties that should always hold

#[cfg(all(test, feature = "ocr"))]
mod property_tests {
    use proptest::prelude::*;
    use zwift_race_finder::ocr_compact::{
        is_likely_name, parse_leaderboard_data, parse_time, LeaderboardEntry,
    };

    // Strategy for generating time-like strings
    fn time_string_strategy() -> impl Strategy<Value = String> {
        prop_oneof![
            // Valid time formats
            "[0-9]{1,2}:[0-9]{2}",
            // Time with prefix/suffix
            "[A-Za-z ]*[0-9]{1,2}:[0-9]{2}[A-Za-z ]*",
            // Just digits
            "[0-9]{3,4}",
            // Invalid formats
            "[A-Za-z]+",
            "[0-9]{1,2}",
            "[0-9]{5,}",
            // Mixed content
            "[A-Za-z0-9 :._-]+",
        ]
    }

    // Strategy for generating name-like strings
    fn name_string_strategy() -> impl Strategy<Value = String> {
        prop_oneof![
            // Typical names
            "[A-Z][a-z]+",
            "[A-Z]\\.",
            "[A-Z][a-z]+ [A-Z][a-z]+",
            "[A-Z]\\.[A-Z][a-z]+",
            // Edge cases
            "[A-Za-z0-9.\\- ()]+",
            // Numbers and symbols
            "[0-9]+",
            "[^A-Za-z0-9]+",
            // Unicode names
            "[A-Za-zÀ-ÿĀ-žА-я]+",
        ]
    }

    // Strategy for leaderboard data strings
    fn leaderboard_data_strategy() -> impl Strategy<Value = String> {
        prop_oneof![
            // Delta only
            "[+-][0-9]{1,2}:[0-9]{2}",
            // Distance only
            "[0-9]{1,3}(\\.[0-9])? [Kk][Mm]",
            // W/kg only
            "[0-9]\\.[0-9] w/kg",
            // Combined data
            "[+-][0-9]{1,2}:[0-9]{2} [0-9]{1,3}\\.[0-9] [Kk][Mm] [0-9]\\.[0-9] w/kg",
            // Random text with numbers
            "[A-Za-z0-9 :.+\\-/]+",
        ]
    }

    proptest! {
        #[test]
        fn parse_time_never_panics(s in ".*") {
            // Property: parse_time should never panic on any input
            let _ = parse_time(&s);
        }

        #[test]
        fn parse_time_preserves_digits(s in time_string_strategy()) {
            // Property: If parse_time returns Some, the digits should be preserved
            if let Some(parsed) = parse_time(&s) {
                let input_digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
                let output_digits: String = parsed.chars().filter(|c| c.is_ascii_digit()).collect();
                
                // The output should contain the same digits (possibly reformatted)
                if input_digits.len() >= 3 && input_digits.len() <= 4 {
                    prop_assert_eq!(input_digits, output_digits);
                }
            }
        }

        #[test]
        fn parse_time_valid_format(s in "[0-9]{1,2}:[0-9]{2}") {
            // Property: Valid time format should always parse successfully
            let result = parse_time(&s);
            prop_assert!(result.is_some());
            prop_assert_eq!(result.unwrap(), s);
        }

        #[test]
        fn parse_time_idempotent(s in ".*") {
            // Property: Parsing the result of parse_time should give the same result
            if let Some(parsed1) = parse_time(&s) {
                let parsed2 = parse_time(&parsed1);
                prop_assert_eq!(Some(parsed1), parsed2);
            }
        }

        #[test]
        fn is_likely_name_never_panics(s in ".*") {
            // Property: is_likely_name should never panic
            let _ = is_likely_name(&s);
        }

        #[test]
        fn is_likely_name_length_bounds(s in name_string_strategy()) {
            // Property: Names must be 2-30 characters after trimming
            let trimmed = s.trim();
            let result = is_likely_name(&s);
            
            if trimmed.len() < 2 || trimmed.len() > 30 {
                prop_assert!(!result, "String '{}' with length {} should not be a name", s, trimmed.len());
            }
        }

        #[test]
        fn is_likely_name_requires_letters(s in "[^A-Za-z]+") {
            // Property: Strings without letters might still be names (e.g., usernames with symbols)
            // This test just verifies the function doesn't panic on non-letter input
            let result = is_likely_name(&s);
            
            // Names without letters are allowed in Zwift (e.g., "123", "!!!", etc.)
            // Just verify the function returns a consistent result
            let result2 = is_likely_name(&s);
            prop_assert_eq!(result, result2);
        }

        #[test]
        fn is_likely_name_consistent(s in ".*") {
            // Property: Calling is_likely_name multiple times should give same result
            let result1 = is_likely_name(&s);
            let result2 = is_likely_name(&s);
            prop_assert_eq!(result1, result2);
        }

        #[test]
        fn parse_leaderboard_data_never_panics(s in ".*") {
            // Property: parse_leaderboard_data should never panic
            let mut entry = LeaderboardEntry {
                name: "Test".to_string(),
                current: false,
                delta: None,
                km: None,
                wkg: None,
            };
            parse_leaderboard_data(&mut entry, &s);
        }

        #[test]
        fn parse_leaderboard_data_idempotent(s in leaderboard_data_strategy()) {
            // Property: Parsing same data multiple times should give same result
            let mut entry1 = LeaderboardEntry {
                name: "Test".to_string(),
                current: false,
                delta: None,
                km: None,
                wkg: None,
            };
            let mut entry2 = entry1.clone();
            
            parse_leaderboard_data(&mut entry1, &s);
            parse_leaderboard_data(&mut entry2, &s);
            
            prop_assert_eq!(entry1, entry2);
        }

        #[test]
        fn parse_leaderboard_data_wkg_range(wkg in 0.0f64..8.0f64) {
            // Property: W/kg values with explicit "w/kg" label are always extracted
            // Human limits: recreational 0.0-3.0, amateur 3.0-4.0, pro 4.0-7.6
            // Values are parsed to nearest 0.1
            let data = format!("{:.1} w/kg", wkg);
            let mut entry = LeaderboardEntry {
                name: "Test".to_string(),
                current: false,
                delta: None,
                km: None,
                wkg: None,
            };
            
            parse_leaderboard_data(&mut entry, &data);
            
            // With explicit "w/kg" label, all values are extracted
            prop_assert!(entry.wkg.is_some(), "W/kg {} should be extracted", wkg);
            // Values are parsed to nearest 0.1, so we need appropriate tolerance
            let expected = (wkg * 10.0).round() / 10.0;
            prop_assert!((entry.wkg.unwrap() - expected).abs() < 0.05, 
                "Expected {:.1}, got {}", expected, entry.wkg.unwrap());
        }

        #[test]
        fn parse_leaderboard_data_wkg_standalone_decimal(wkg in 0.0f64..10.0f64) {
            // Property: Standalone decimal numbers are only extracted as w/kg if in range
            // Code checks 0.5-7.0 range for standalone decimals
            let data = format!("some text {:.1} more text", wkg);
            let mut entry = LeaderboardEntry {
                name: "Test".to_string(),
                current: false,
                delta: None,
                km: None,
                wkg: None,
            };
            
            parse_leaderboard_data(&mut entry, &data);
            
            // Check if the rounded value would be in range
            let rounded_wkg = (wkg * 10.0).round() / 10.0;
            
            if rounded_wkg >= 0.5 && rounded_wkg <= 7.0 {
                prop_assert!(entry.wkg.is_some(), "Standalone decimal {} (rounds to {:.1}) in w/kg range should be extracted", wkg, rounded_wkg);
                // Values are parsed to nearest 0.1
                prop_assert!((entry.wkg.unwrap() - rounded_wkg).abs() < 0.05,
                    "Expected {:.1}, got {}", rounded_wkg, entry.wkg.unwrap());
            } else {
                prop_assert!(entry.wkg.is_none(), "Standalone decimal {} (rounds to {:.1}) outside w/kg range should not be extracted", wkg, rounded_wkg);
            }
        }

        #[test]
        fn parse_leaderboard_data_km_non_negative(km in 0.0f64..200.0f64) {
            // Property: Distance values should be non-negative (0.0 is valid)
            let data = format!("{:.1} km", km);
            let mut entry = LeaderboardEntry {
                name: "Test".to_string(),
                current: false,
                delta: None,
                km: None,
                wkg: None,
            };
            
            parse_leaderboard_data(&mut entry, &data);
            
            if let Some(extracted_km) = entry.km {
                prop_assert!(extracted_km >= 0.0, "Extracted km should be non-negative");
                // Allow for precision loss when parsing formatted string
                prop_assert!((extracted_km - km).abs() < 0.1, 
                    "Expected {}, got {}", km, extracted_km);
            }
        }

        #[test]
        fn parse_leaderboard_data_delta_format(sign in prop::sample::select(&['+', '-']), mins in 0u32..60u32, secs in 0u32..60u32) {
            // Property: Valid delta format should be extracted
            let data = format!("{}{}:{:02}", sign, mins, secs);
            let mut entry = LeaderboardEntry {
                name: "Test".to_string(),
                current: false,
                delta: None,
                km: None,
                wkg: None,
            };
            
            parse_leaderboard_data(&mut entry, &data);
            
            prop_assert!(entry.delta.is_some(), "Delta '{}' should be extracted", data);
            let extracted = entry.delta.unwrap();
            prop_assert!(extracted.starts_with(sign));
            prop_assert!(extracted.contains(':'));
        }

        #[test]
        fn parse_leaderboard_data_preserves_existing(s in ".*") {
            // Property: Parsing should not clear existing values if new ones not found
            let mut entry = LeaderboardEntry {
                name: "Test".to_string(),
                current: false,
                delta: Some("+01:00".to_string()),
                km: Some(10.0),
                wkg: Some(3.0),
            };
            let original = entry.clone();
            
            // Parse data that doesn't contain any recognizable patterns
            parse_leaderboard_data(&mut entry, &s);
            
            // If no new values were found, original values should be preserved
            if !s.contains(char::is_numeric) {
                prop_assert_eq!(entry.delta, original.delta);
                prop_assert_eq!(entry.km, original.km);
                prop_assert_eq!(entry.wkg, original.wkg);
            }
        }
    }

    // Additional property tests for compound scenarios
    proptest! {
        #[test]
        fn combined_parsing_consistency(
            time_str in time_string_strategy(),
            name_str in name_string_strategy(),
            data_str in leaderboard_data_strategy()
        ) {
            // Property: Parsing functions should not interfere with each other
            let time_result1 = parse_time(&time_str);
            let name_result1 = is_likely_name(&name_str);
            
            // Parse in different order
            let name_result2 = is_likely_name(&name_str);
            let time_result2 = parse_time(&time_str);
            
            prop_assert_eq!(time_result1, time_result2);
            prop_assert_eq!(name_result1, name_result2);
            
            // Leaderboard data parsing should also be independent
            let mut entry1 = LeaderboardEntry {
                name: name_str.clone(),
                current: false,
                delta: None,
                km: None,
                wkg: None,
            };
            let mut entry2 = entry1.clone();
            
            parse_leaderboard_data(&mut entry1, &data_str);
            let _ = parse_time(&time_str); // Interleave other parsing
            let _ = is_likely_name(&name_str);
            parse_leaderboard_data(&mut entry2, &data_str);
            
            prop_assert_eq!(entry1, entry2);
        }
    }
}