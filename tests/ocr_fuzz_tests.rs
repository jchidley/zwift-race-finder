#[cfg(all(test, feature = "ocr"))]
mod fuzz_tests {
    use proptest::prelude::*;
    use zwift_race_finder::ocr_compact::{
        is_likely_name, parse_leaderboard_data, parse_time, LeaderboardEntry,
    };

    fn arbitrary_unicode_string() -> impl Strategy<Value = String> {
        prop::string::string_regex(".*").unwrap()
    }

    fn long_string() -> impl Strategy<Value = String> {
        prop::string::string_regex(".{0,10000}").unwrap()
    }

    fn special_chars_string() -> impl Strategy<Value = String> {
        prop::string::string_regex(r"[\x00-\x1F\x7F-\x9F\u{200B}-\u{200D}\u{FEFF}]*").unwrap()
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn fuzz_parse_time_arbitrary(s in arbitrary_unicode_string()) {
            let _ = parse_time(&s);
        }

        #[test]
        fn fuzz_parse_time_long(s in long_string()) {
            let _ = parse_time(&s);
        }

        #[test]
        fn fuzz_parse_time_special_chars(s in special_chars_string()) {
            let _ = parse_time(&s);
        }

        #[test]
        fn fuzz_is_likely_name_arbitrary(s in arbitrary_unicode_string()) {
            let _ = is_likely_name(&s);
        }

        #[test]
        fn fuzz_is_likely_name_empty_and_whitespace(s in prop::string::string_regex(r"\s*").unwrap()) {
            let _ = is_likely_name(&s);
        }

        #[test]
        fn fuzz_is_likely_name_unicode_names(s in prop::string::string_regex(r"[\p{L}\p{M}\p{Nd}\p{Pc}\p{Join_Control}]+").unwrap()) {
            let _ = is_likely_name(&s);
        }

        #[test]
        fn fuzz_parse_leaderboard_data_arbitrary(s in arbitrary_unicode_string()) {
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
        fn fuzz_parse_leaderboard_data_numeric_chaos(s in prop::string::string_regex(r"[\d\.\,\+\-\s]+").unwrap()) {
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
        fn fuzz_parse_leaderboard_data_mixed_units(s in prop::string::string_regex(r"(\d+\.?\d*\s*(km|KM|w/kg|W/KG|wkg|WKG)\s*)+").unwrap()) {
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
        fn fuzz_combined_chaos(s in arbitrary_unicode_string()) {
            let _ = parse_time(&s);
            let _ = is_likely_name(&s);

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
        fn fuzz_regex_performance(c in any::<char>(), count in 0usize..1000usize) {
            let s = c.to_string().repeat(count);
            let _ = parse_time(&s);
            let _ = is_likely_name(&s);

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
        fn fuzz_null_bytes(s in prop::collection::vec(any::<u8>(), 0..1000)) {
            if let Ok(text) = String::from_utf8(s) {
                let _ = parse_time(&text);
                let _ = is_likely_name(&text);
            }
        }
    }

    proptest! {
        #[test]
        fn fuzz_time_format_variations(
            hours in 0u8..100u8,
            minutes in 0u8..100u8,
            prefix in prop::string::string_regex(r"[^\d]*").unwrap(),
            suffix in prop::string::string_regex(r"[^\d]*").unwrap(),
            separator in prop::sample::select(vec![":", " : ", ".", " ", ""])
        ) {
            let time_str = format!("{}{}{}{}{}", prefix, hours, separator, minutes, suffix);
            let _ = parse_time(&time_str);
        }

        #[test]
        fn fuzz_leaderboard_numeric_combinations(
            delta_sign in prop::option::of(prop::sample::select(vec!["+", "-"])),
            delta_time in prop::option::of((0u8..99u8, 0u8..99u8)),
            distance in prop::option::of(0.0f64..999.9f64),
            power in prop::option::of(0.0f64..10.0f64),
            spaces in 0usize..10usize,
        ) {
            let mut parts = Vec::new();

            if let (Some(sign), Some((h, m))) = (delta_sign, delta_time) {
                parts.push(format!("{}{:02}:{:02}", sign, h, m));
            }

            if let Some(d) = distance {
                parts.push(format!("{:.1} km", d));
            }

            if let Some(p) = power {
                parts.push(format!("{:.1} w/kg", p));
            }

            let data = parts.join(&" ".repeat(spaces));
            let mut entry = LeaderboardEntry {
                name: "Test".to_string(),
                current: false,
                delta: None,
                km: None,
                wkg: None,
            };
            parse_leaderboard_data(&mut entry, &data);

            if let Some(d) = distance {
                if let Some(parsed_km) = entry.km {
                    if d > 0.1 {
                        prop_assert!((parsed_km - d).abs() < 0.1,
                            "Distance parsing error: expected ~{:.1}, got {:.1}", d, parsed_km);
                    }
                }
            }
        }
    }
}