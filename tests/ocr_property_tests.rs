#[cfg(all(test, feature = "ocr"))]
mod property_tests {
    use proptest::prelude::*;
    use zwift_race_finder::ocr_compact::{
        is_likely_name, parse_leaderboard_data, parse_time, LeaderboardEntry,
    };

    fn time_string_strategy() -> impl Strategy<Value = String> {
        prop_oneof![
            "[0-9]{1,2}:[0-9]{2}",
            "[A-Za-z ]*[0-9]{1,2}:[0-9]{2}[A-Za-z ]*",
            "[0-9]{3,4}",
            "[A-Za-z]+",
            "[0-9]{1,2}",
            "[0-9]{5,}",
            "[A-Za-z0-9 :._-]+",
        ]
    }

    fn name_string_strategy() -> impl Strategy<Value = String> {
        prop_oneof![
            "[A-Z][a-z]+",
            "[A-Z]\\.",
            "[A-Z][a-z]+ [A-Z][a-z]+",
            "[A-Z]\\.[A-Z][a-z]+",
            "[A-Za-z0-9.\\- ()]+",
            "[0-9]+",
            "[^A-Za-z0-9]+",
            "[A-Za-zÀ-ÿĀ-žА-я]+",
        ]
    }

    fn leaderboard_data_strategy() -> impl Strategy<Value = String> {
        prop_oneof![
            "[+-][0-9]{1,2}:[0-9]{2}",
            "[0-9]{1,3}(\\.[0-9])? [Kk][Mm]",
            "[0-9]\\.[0-9] w/kg",
            "[+-][0-9]{1,2}:[0-9]{2} [0-9]{1,3}\\.[0-9] [Kk][Mm] [0-9]\\.[0-9] w/kg",
            "[A-Za-z0-9 :.+\\-/]+",
        ]
    }

    proptest! {
        #[test]
        fn parse_time_never_panics(s in ".*") {
            let _ = parse_time(&s);
        }

        #[test]
        fn parse_time_preserves_digits(s in time_string_strategy()) {
            if let Some(parsed) = parse_time(&s) {
                let input_digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
                let output_digits: String = parsed.chars().filter(|c| c.is_ascii_digit()).collect();

                if input_digits.len() >= 3 && input_digits.len() <= 4 {
                    prop_assert_eq!(input_digits, output_digits);
                }
            }
        }

        #[test]
        fn parse_time_valid_format(s in "[0-9]{1,2}:[0-9]{2}") {
            let result = parse_time(&s);
            prop_assert!(result.is_some());
            prop_assert_eq!(result.unwrap(), s);
        }

        #[test]
        fn parse_time_idempotent(s in ".*") {
            if let Some(parsed1) = parse_time(&s) {
                let parsed2 = parse_time(&parsed1);
                prop_assert_eq!(Some(parsed1), parsed2);
            }
        }

        #[test]
        fn is_likely_name_never_panics(s in ".*") {
            let _ = is_likely_name(&s);
        }

        #[test]
        fn is_likely_name_length_bounds(s in name_string_strategy()) {
            let trimmed = s.trim();
            let result = is_likely_name(&s);

            if trimmed.len() < 2 || trimmed.len() > 30 {
                prop_assert!(!result, "String '{}' with length {} should not be a name", s, trimmed.len());
            }
        }

        #[test]
        fn is_likely_name_requires_letters(s in "[^A-Za-z]+") {
            let result = is_likely_name(&s);

            let result2 = is_likely_name(&s);
            prop_assert_eq!(result, result2);
        }

        #[test]
        fn is_likely_name_consistent(s in ".*") {
            let result1 = is_likely_name(&s);
            let result2 = is_likely_name(&s);
            prop_assert_eq!(result1, result2);
        }

        #[test]
        fn parse_leaderboard_data_never_panics(s in ".*") {
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
            let data = format!("{:.1} w/kg", wkg);
            let mut entry = LeaderboardEntry {
                name: "Test".to_string(),
                current: false,
                delta: None,
                km: None,
                wkg: None,
            };

            parse_leaderboard_data(&mut entry, &data);

            prop_assert!(entry.wkg.is_some(), "W/kg {} should be extracted", wkg);
            let expected = (wkg * 10.0).round() / 10.0;
            prop_assert!((entry.wkg.unwrap() - expected).abs() < 0.05,
                "Expected {:.1}, got {}", expected, entry.wkg.unwrap());
        }

        #[test]
        fn parse_leaderboard_data_wkg_standalone_decimal(wkg in 0.0f64..10.0f64) {
            let data = format!("some text {:.1} more text", wkg);
            let mut entry = LeaderboardEntry {
                name: "Test".to_string(),
                current: false,
                delta: None,
                km: None,
                wkg: None,
            };

            parse_leaderboard_data(&mut entry, &data);

            let rounded_wkg = (wkg * 10.0).round() / 10.0;

            if rounded_wkg >= 0.5 && rounded_wkg <= 7.0 {
                prop_assert!(entry.wkg.is_some(), "Standalone decimal {} (rounds to {:.1}) in w/kg range should be extracted", wkg, rounded_wkg);
                prop_assert!((entry.wkg.unwrap() - rounded_wkg).abs() < 0.05,
                    "Expected {:.1}, got {}", rounded_wkg, entry.wkg.unwrap());
            } else {
                prop_assert!(entry.wkg.is_none(), "Standalone decimal {} (rounds to {:.1}) outside w/kg range should not be extracted", wkg, rounded_wkg);
            }
        }

        #[test]
        fn parse_leaderboard_data_km_non_negative(km in 0.0f64..200.0f64) {
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
                prop_assert!((extracted_km - km).abs() < 0.1,
                    "Expected {}, got {}", km, extracted_km);
            }
        }

        #[test]
        fn parse_leaderboard_data_delta_format(sign in prop::sample::select(&['+', '-']), mins in 0u32..60u32, secs in 0u32..60u32) {
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
            let mut entry = LeaderboardEntry {
                name: "Test".to_string(),
                current: false,
                delta: Some("+01:00".to_string()),
                km: Some(10.0),
                wkg: Some(3.0),
            };
            let original = entry.clone();

            parse_leaderboard_data(&mut entry, &s);

            if !s.contains(char::is_numeric) {
                prop_assert_eq!(entry.delta, original.delta);
                prop_assert_eq!(entry.km, original.km);
                prop_assert_eq!(entry.wkg, original.wkg);
            }
        }
    }

    proptest! {
        #[test]
        fn combined_parsing_consistency(
            time_str in time_string_strategy(),
            name_str in name_string_strategy(),
            data_str in leaderboard_data_strategy()
        ) {
            let time_result1 = parse_time(&time_str);
            let name_result1 = is_likely_name(&name_str);

            let name_result2 = is_likely_name(&name_str);
            let time_result2 = parse_time(&time_str);

            prop_assert_eq!(time_result1, time_result2);
            prop_assert_eq!(name_result1, name_result2);

            let mut entry1 = LeaderboardEntry {
                name: name_str.clone(),
                current: false,
                delta: None,
                km: None,
                wkg: None,
            };
            let mut entry2 = entry1.clone();

            parse_leaderboard_data(&mut entry1, &data_str);
            let _ = parse_time(&time_str);
            let _ = is_likely_name(&name_str);
            parse_leaderboard_data(&mut entry2, &data_str);

            prop_assert_eq!(entry1, entry2);
        }
    }
}