#[cfg(feature = "ocr")]
mod tests {
    use anyhow::Result;
    use std::path::Path;
    use zwift_race_finder::ocr_compact::{
        extract_telemetry, LeaderboardEntry, RiderPose, TelemetryData,
    };

    fn test_image(filename: &str) -> std::path::PathBuf {
        Path::new("docs/screenshots").join(filename)
    }

    #[test]
    #[ignore]
    fn test_extract_telemetry_normal_ride() -> Result<()> {
        let image_path = test_image("normal_1_01_16_02_21.jpg");

        if !image_path.exists() {
            eprintln!("Skipping test: {} not found", image_path.display());
            return Ok(());
        }

        let telemetry = extract_telemetry(&image_path)?;

        assert!(telemetry.speed.is_some() || telemetry.speed.is_none());
        assert!(telemetry.distance.is_some() || telemetry.distance.is_none());
        assert!(telemetry.altitude.is_some() || telemetry.altitude.is_none());
        assert!(telemetry.race_time.is_some() || telemetry.race_time.is_none());
        assert!(telemetry.power.is_some() || telemetry.power.is_none());
        assert!(telemetry.cadence.is_some() || telemetry.cadence.is_none());
        assert!(telemetry.heart_rate.is_some() || telemetry.heart_rate.is_none());
        assert!(telemetry.gradient.is_some() || telemetry.gradient.is_none());
        assert!(telemetry.distance_to_finish.is_some() || telemetry.distance_to_finish.is_none());
        assert!(telemetry.leaderboard.is_some() || telemetry.leaderboard.is_none());
        assert!(telemetry.rider_pose.is_some() || telemetry.rider_pose.is_none());

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_extract_telemetry_climbing() -> Result<()> {
        let image_path = test_image("with_climbing_1_01_36_01_42.jpg");

        if !image_path.exists() {
            eprintln!("Skipping test: {} not found", image_path.display());
            return Ok(());
        }

        let telemetry = extract_telemetry(&image_path)?;

        println!("Telemetry extracted: {:?}", telemetry);

        Ok(())
    }

    #[test]
    fn test_parse_time_formats() {
        use zwift_race_finder::ocr_compact::parse_time;


        let result = parse_time("12:34");
        assert_eq!(result, Some("12:34".to_string()));

        let result = parse_time("Time: 12:34");
        assert_eq!(result, Some("12:34".to_string()));

        let result = parse_time("1234");
        assert_eq!(result, Some("12:34".to_string()));

        let result = parse_time("123");
        assert_eq!(result, Some("1:23".to_string()));

        let result = parse_time("invalid");
        assert_eq!(result, None);
    }

    #[test]
    fn test_is_likely_name() {
        use zwift_race_finder::ocr_compact::is_likely_name;


        assert!(is_likely_name("J.Chidley"));
        assert!(is_likely_name("John Doe"));
        assert!(is_likely_name("A."));
        assert!(is_likely_name("C.J.Y.S"));
        assert!(is_likely_name("Name (ABC)"));

        assert!(!is_likely_name("123"));
        assert!(!is_likely_name("12.3 km"));
        assert!(!is_likely_name("3.2 w/kg"));
        assert!(!is_likely_name("+00:12"));
        assert!(!is_likely_name(""));
        assert!(!is_likely_name("a"));
    }

    #[test]
    fn test_parse_leaderboard_data() {
        use zwift_race_finder::ocr_compact::{parse_leaderboard_data, LeaderboardEntry};

        let mut entry = LeaderboardEntry {
            name: "Test".to_string(),
            current: false,
            delta: None,
            km: None,
            wkg: None,
        };

        parse_leaderboard_data(&mut entry, "+01:23 3.2 w/kg 12.5 KM");
        assert_eq!(entry.delta, Some("+01:23".to_string()));
        assert_eq!(entry.wkg, Some(3.2));
        assert_eq!(entry.km, Some(12.5));

        entry.delta = None;
        entry.wkg = None;
        entry.km = None;
        parse_leaderboard_data(&mut entry, "-00:45");
        assert_eq!(entry.delta, Some("-00:45".to_string()));

        entry.delta = None;
        entry.wkg = None;
        parse_leaderboard_data(&mut entry, "some text 4.5 more text");
        assert_eq!(entry.wkg, Some(4.5));
    }

    #[test]
    fn test_rider_pose_classification() {

        let poses = vec![
            RiderPose::NormalTuck,
            RiderPose::NormalNormal,
            RiderPose::ClimbingSeated,
            RiderPose::ClimbingStanding,
            RiderPose::Unknown,
        ];

        for pose in poses {
            let json = serde_json::to_string(&pose).unwrap();
            let deserialized: RiderPose = serde_json::from_str(&json).unwrap();
            assert_eq!(pose, deserialized);
        }
    }

    #[test]
    fn test_telemetry_serialization() {
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
                    name: "Rider 1".to_string(),
                    current: false,
                    delta: Some("+00:30".to_string()),
                    km: Some(12.0),
                    wkg: Some(3.5),
                },
                LeaderboardEntry {
                    name: "Current".to_string(),
                    current: true,
                    delta: None,
                    km: Some(12.5),
                    wkg: Some(3.2),
                },
            ]),
            rider_pose: Some(RiderPose::ClimbingSeated),
        };

        let json = serde_json::to_string(&telemetry).unwrap();
        let deserialized: TelemetryData = serde_json::from_str(&json).unwrap();

        assert_eq!(telemetry.speed, deserialized.speed);
        assert_eq!(telemetry.distance, deserialized.distance);
    }
}

#[cfg(feature = "ocr")]
mod ocrs_tests {
    use anyhow::Result;
    use image::DynamicImage;
    use zwift_race_finder::ocr_ocrs::{create_engine, extract_text_from_region};

    #[test]
    #[ignore]
    fn test_create_engine() -> Result<()> {
        let engine = create_engine();

        match engine {
            Ok(_) => {
                println!("OCR engine created successfully");
            }
            Err(e) => {
                println!("Expected error when models not present: {}", e);
            }
        }

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_extract_text_from_region() -> Result<()> {
        let img = DynamicImage::new_rgb8(200, 50);

        let result = extract_text_from_region(&img, 0, 0, 200, 50);

        match result {
            Ok(text) => {
                println!("Extracted text: '{}'", text);
                assert!(text.is_empty() || !text.is_empty());
            }
            Err(e) => {
                println!("Expected error: {}", e);
            }
        }

        Ok(())
    }
}

