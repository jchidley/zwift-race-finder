#[cfg(feature = "ocr")]
mod characterization {
    use zwift_race_finder::ocr_compact::*;

    #[test]
    fn characterize_parse_time_edge_cases() {

        assert_eq!(parse_time(""), None);

        assert_eq!(parse_time("1"), None);

        assert_eq!(parse_time("12"), None);

        assert_eq!(parse_time("12345"), None);

        assert_eq!(parse_time("12 : 34"), Some("12:34".to_string()));

        assert_eq!(parse_time("  12:34  "), Some("12:34".to_string()));

        assert_eq!(parse_time("01:02"), Some("01:02".to_string()));

        assert_eq!(parse_time("99:99"), Some("99:99".to_string()));

        assert_eq!(parse_time("Time: 12:34 remaining"), Some("12:34".to_string()));

        assert_eq!(parse_time("12:34 and 56:78"), Some("12:34".to_string()));
    }

    #[test]
    fn characterize_is_likely_name_edge_cases() {
        assert!(!is_likely_name("A"));

        assert!(is_likely_name("AB"));

        assert!(is_likely_name("A".repeat(30).as_str()));

        assert!(!is_likely_name("A".repeat(31).as_str()));

        assert!(is_likely_name("john"));
        assert!(is_likely_name("JOHN"));
        assert!(is_likely_name("jOhN"));

        assert!(is_likely_name("O'Brien"));
        assert!(is_likely_name("Anne-Marie"));
        assert!(!is_likely_name("---"));

        assert!(is_likely_name("Player1"));
        assert!(is_likely_name("123ABC"));

        assert!(!is_likely_name("..."));

        assert!(is_likely_name("José"));

        assert!(!is_likely_name("  "));
        assert!(is_likely_name("  AB  "));
    }

    #[test]
    fn characterize_parse_leaderboard_data_edge_cases() {
        let mut entry = LeaderboardEntry {
            name: "Test".to_string(),
            current: false,
            delta: None,
            km: None,
            wkg: None,
        };

        parse_leaderboard_data(&mut entry, "");
        assert_eq!(entry.delta, None);
        assert_eq!(entry.km, None);
        assert_eq!(entry.wkg, None);

        entry = new_entry();
        parse_leaderboard_data(&mut entry, "+12:34");
        assert_eq!(entry.delta, Some("+12:34".to_string()));
        assert_eq!(entry.km, None);
        assert_eq!(entry.wkg, None);

        entry = new_entry();
        parse_leaderboard_data(&mut entry, "+1:23");
        assert_eq!(entry.delta, Some("+1:23".to_string()));

        entry = new_entry();
        parse_leaderboard_data(&mut entry, "+1234");
        assert_eq!(entry.delta, None);

        entry = new_entry();
        parse_leaderboard_data(&mut entry, "12 km");
        assert_eq!(entry.km, Some(12.0));

        entry = new_entry();
        parse_leaderboard_data(&mut entry, "12.5KM");
        assert_eq!(entry.km, Some(12.5));

        entry = new_entry();
        parse_leaderboard_data(&mut entry, "12.5 Km");
        assert_eq!(entry.km, Some(12.5));

        entry = new_entry();
        parse_leaderboard_data(&mut entry, "3.2 w/kg");
        assert_eq!(entry.wkg, Some(3.2));

        entry = new_entry();
        parse_leaderboard_data(&mut entry, "3.2w/kg");
        assert_eq!(entry.wkg, Some(3.2));

        entry = new_entry();
        parse_leaderboard_data(&mut entry, "text 2.5 text");
        assert_eq!(entry.wkg, Some(2.5));

        entry = new_entry();
        parse_leaderboard_data(&mut entry, "text 0.4 text");
        assert_eq!(entry.wkg, None);

        entry = new_entry();
        parse_leaderboard_data(&mut entry, "text 7.1 text");
        assert_eq!(entry.wkg, None);

        entry = new_entry();
        parse_leaderboard_data(&mut entry, "-01:23 15.5 km 4.2 w/kg");
        assert_eq!(entry.delta, Some("-01:23".to_string()));
        assert_eq!(entry.km, Some(15.5));
        assert_eq!(entry.wkg, Some(4.2));

        entry = new_entry();
        parse_leaderboard_data(&mut entry, "3.2 w/kg 12.5 KM +00:30");
        assert_eq!(entry.delta, Some("+00:30".to_string()));
        assert_eq!(entry.km, Some(12.5));
        assert_eq!(entry.wkg, Some(3.2));
    }

    fn new_entry() -> LeaderboardEntry {
        LeaderboardEntry {
            name: "Test".to_string(),
            current: false,
            delta: None,
            km: None,
            wkg: None,
        }
    }

    #[test]
    fn characterize_rider_pose_serialization() {
        assert_eq!(
            serde_json::to_string(&RiderPose::NormalTuck).unwrap(),
            r#""normal_tuck""#
        );
        assert_eq!(
            serde_json::to_string(&RiderPose::NormalNormal).unwrap(),
            r#""normal_normal""#
        );
        assert_eq!(
            serde_json::to_string(&RiderPose::ClimbingSeated).unwrap(),
            r#""climbing_seated""#
        );
        assert_eq!(
            serde_json::to_string(&RiderPose::ClimbingStanding).unwrap(),
            r#""climbing_standing""#
        );
        assert_eq!(
            serde_json::to_string(&RiderPose::Unknown).unwrap(),
            r#""unknown""#
        );
    }

    #[test]
    fn characterize_default_values() {
        let pose = RiderPose::default();
        assert_eq!(pose, RiderPose::Unknown);

        let telemetry = TelemetryData::default();
        assert_eq!(telemetry.speed, None);
        assert_eq!(telemetry.distance, None);
        assert_eq!(telemetry.altitude, None);
        assert_eq!(telemetry.race_time, None);
        assert_eq!(telemetry.power, None);
        assert_eq!(telemetry.cadence, None);
        assert_eq!(telemetry.heart_rate, None);
        assert_eq!(telemetry.gradient, None);
        assert_eq!(telemetry.distance_to_finish, None);
        assert_eq!(telemetry.leaderboard, None);
        assert_eq!(telemetry.rider_pose, None);
    }
}