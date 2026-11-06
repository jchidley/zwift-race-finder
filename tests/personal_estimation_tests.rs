use chrono::{Duration, Utc};
use rusqlite::Connection;
use zwift_race_finder::{
    database::{Database, RouteData},
    event_display::{format_duration_info, prepare_event_row},
    event_filtering::{estimate_event, event_matches_duration, resolve_event, EstimateUnavailable},
    history::HistoryRecord,
    models::{EventSubGroup, ZwiftEvent},
    personal_estimation::{predict_from_database, PersonalBasis, PersonalFallback, PersonalTarget},
};

const ROUTE: u32 = 4_000_003_001;
const ALIAS: u32 = 4_000_003_002;
const OTHER: u32 = 4_000_003_003;

fn route(id: u32, elevation: Option<f64>) -> RouteData {
    serde_json::from_value(serde_json::json!({
        "route_id": id, "name": "Synthetic personal fixture", "world": "Test",
        "distance_km": 10.0, "elevation_m": elevation, "surface": "road",
        "lead_in_distance_km": 0.0, "lead_in_elevation_m": 0.0
    }))
    .unwrap()
}

fn event() -> ZwiftEvent {
    ZwiftEvent {
        id: 1,
        name: "Synthetic ordinary race".into(),
        event_start: Utc::now() + Duration::days(5),
        event_type: "RACE".into(),
        distance_in_meters: Some(20_000.0),
        duration_in_minutes: None,
        duration_in_seconds: None,
        route_id: Some(ALIAS),
        route: None,
        description: None,
        category_enforcement: false,
        event_sub_groups: vec![],
        sport: "CYCLING".into(),
        tags: vec![],
    }
}

fn record(i: i64) -> HistoryRecord {
    let start = (Utc::now() - Duration::days(i + 1))
        .date_naive()
        .and_hms_opt(12, 0, 0)
        .unwrap()
        .and_utc();
    HistoryRecord {
        route_id: ROUTE,
        event_name: format!("Synthetic result {i}"),
        actual_seconds: 3600.25,
        zwift_score: None,
        race_date: start.format("%Y-%m-%d").to_string(),
        event_start_utc: start.to_rfc3339(),
        event_distance_km: Some(20.0),
        laps: None,
        result_category: None,
        gun_time_seconds: None,
        source: "zwiftpower-profile".into(),
        source_rider_id: "1".into(),
        source_result_id: format!("synthetic-{i}"),
        source_event_id: (i + 1).to_string(),
        source_sha256: "a".repeat(64),
        source_payload_json: r#"{"f_t":"TYPE_RACE"}"#.into(),
    }
}

#[test]
fn live_history_changes_filter_and_display_with_explicit_honest_fallbacks() {
    let state = tempfile::tempdir().unwrap();
    std::env::set_var("HOME", state.path());
    std::env::set_var("XDG_DATA_HOME", state.path().join("data"));
    std::env::set_var("XDG_CONFIG_HOME", state.path().join("config"));
    let db = Database::new().unwrap();
    db.add_route(&route(ROUTE, Some(0.0))).unwrap();
    db.add_route(&route(OTHER, Some(0.0))).unwrap();
    let sql = Connection::open(Database::default_path().unwrap()).unwrap();
    sql.execute(
        "INSERT INTO route_aliases(alias_route_id,canonical_route_id) VALUES (?1,?2)",
        [ALIAS, ROUTE],
    )
    .unwrap();
    assert_eq!(db.get_route(ALIAS).unwrap().unwrap().route_id, ROUTE);

    let mut race = event();
    assert!(estimate_event(&race, 195).is_none());
    assert!(matches!(
        resolve_event(&race, 195),
        Err(EstimateUnavailable::Personal(
            PersonalFallback::InsufficientHistory {
                available: 0,
                required: 12
            }
        ))
    ));
    assert!(format_duration_info(&race, 195).contains("unknown"));
    assert!(prepare_event_row(&race, 195).basis.contains("insufficient"));
    assert!(!event_matches_duration(&race, 60, u32::MAX, 195));
    assert!(!event_matches_duration(&race, 60, 0, 195));

    let records: Vec<_> = (0..12).map(record).collect();
    db.import_history(&records).unwrap();
    sql.pragma_update(None, "foreign_keys", false).unwrap();
    sql.execute(
        "UPDATE race_results SET route_id=?1 WHERE id % 2 = 0",
        [ALIAS],
    )
    .unwrap();
    assert!(db
        .performance_history()
        .unwrap()
        .iter()
        .all(|row| row.result.zwift_score.is_none()));
    let estimate = estimate_event(&race, 195).unwrap();
    let personal = estimate.personal.as_ref().unwrap();
    assert_eq!(personal.basis, PersonalBasis::SameRoute);
    assert_eq!(personal.sample_count, 12);
    assert!((personal.minutes - 3600.25 / 60.0).abs() < 1e-10);
    assert_eq!(estimate.duration_minutes, 3600.25 / 60.0);
    assert!(!event_matches_duration(&race, 60, 0, 195));
    assert!(event_matches_duration(&race, 60, 1, 195));
    assert_eq!(prepare_event_row(&race, 195).duration, "01:00");
    assert!(prepare_event_row(&race, 195)
        .basis
        .contains("History: same route (experimental)"));
    let verbose = format_duration_info(&race, 195);
    for text in [
        "01:00",
        "history: same-route",
        "n=12",
        "typical training error",
        "descriptive",
        "not a confidence interval",
    ] {
        assert!(verbose.contains(text), "{verbose}");
    }
    assert_eq!(
        estimate_event(&race, 650).unwrap().duration_minutes,
        3600.25 / 60.0
    );

    race.event_sub_groups = vec![
        EventSubGroup {
            id: 1,
            name: "lower".into(),
            route_id: Some(ALIAS),
            distance_in_meters: Some(20_000.0),
            duration_in_minutes: None,
            duration_in_seconds: None,
            subgroup_label: None,
            category_enforcement: None,
            range_access_label: Some("0-199".into()),
            laps: None,
        },
        EventSubGroup {
            id: 2,
            name: "upper".into(),
            route_id: Some(ALIAS),
            distance_in_meters: Some(25_000.0),
            duration_in_minutes: None,
            duration_in_seconds: None,
            subgroup_label: None,
            category_enforcement: None,
            range_access_label: Some("200-1000".into()),
            laps: None,
        },
    ];
    assert_eq!(
        estimate_event(&race, 195).unwrap().duration_minutes,
        3600.25 / 60.0
    );
    assert_eq!(
        estimate_event(&race, 210).unwrap().duration_minutes,
        3600.25 / 60.0 * 1.25
    );
    race.event_sub_groups.clear();

    for kind in ["TIME_TRIAL", "GROUP_RIDE", "UNKNOWN"] {
        race.event_type = kind.into();
        assert!(estimate_event(&race, 195).is_none());
        assert!(matches!(
            resolve_event(&race, 195),
            Err(EstimateUnavailable::Personal(
                PersonalFallback::UnsupportedFormat
            ))
        ));
        assert!(!event_matches_duration(&race, 60, u32::MAX, 195));
        let text = format_duration_info(&race, 195);
        assert!(text.contains("unknown") && text.contains("unsupported format"));
    }
    race.event_type = "RACE".into();

    race.route_id = Some(OTHER);
    assert_eq!(
        estimate_event(&race, 195).unwrap().personal.unwrap().basis,
        PersonalBasis::DistanceAscent
    );
    db.add_route(&route(OTHER, None)).unwrap();
    assert_eq!(
        resolve_event(&race, 195)
            .err()
            .and_then(|reason| match reason {
                EstimateUnavailable::Personal(reason) => Some(reason),
                _ => None,
            }),
        Some(PersonalFallback::MissingGeometry)
    );
    assert!(format_duration_info(&race, 195).contains("missing distance/ascent geometry"));
    db.add_route(&route(ROUTE, None)).unwrap();
    race.route_id = Some(ALIAS);
    assert_eq!(
        estimate_event(&race, 195).unwrap().personal.unwrap().basis,
        PersonalBasis::SameRoute
    );

    sql.execute(
        "UPDATE race_results SET source_rider_id='2' WHERE source_result_id='synthetic-0'",
        [],
    )
    .unwrap();
    assert_eq!(
        resolve_event(&race, 195)
            .err()
            .and_then(|reason| match reason {
                EstimateUnavailable::Personal(reason) => Some(reason),
                _ => None,
            }),
        Some(PersonalFallback::MixedRiders)
    );
    assert!(format_duration_info(&race, 195).contains("multiple riders"));
    sql.execute(
        "UPDATE race_results SET source_rider_id='1' WHERE source_result_id='synthetic-0'",
        [],
    )
    .unwrap();

    db.add_route(&route(ROUTE, Some(0.0))).unwrap();
    db.add_route(&route(OTHER, Some(0.0))).unwrap();
    race.route_id = Some(OTHER);

    let before_laps = estimate_event(&race, 195).unwrap().duration_minutes;
    sql.execute("UPDATE race_results SET laps=100", []).unwrap();
    assert_eq!(
        estimate_event(&race, 195).unwrap().duration_minutes,
        before_laps
    );
    sql.execute("UPDATE race_results SET laps=NULL", [])
        .unwrap();

    sql.execute(
        "UPDATE race_results SET source_payload_json='{}' WHERE source_result_id='synthetic-0'",
        [],
    )
    .unwrap();
    assert!(matches!(
        resolve_event(&race, 195)
            .err()
            .and_then(|reason| match reason {
                EstimateUnavailable::Personal(reason) => Some(reason),
                _ => None,
            }),
        Some(PersonalFallback::InsufficientHistory {
            available: 11,
            required: 12
        })
    ));
    sql.execute(
        "UPDATE race_results SET event_type='RACE' WHERE source_result_id='synthetic-0'",
        [],
    )
    .unwrap();
    assert!(estimate_event(&race, 195).unwrap().personal.is_some());
    sql.execute(
        "UPDATE race_results SET event_type='TIME_TRIAL' WHERE source_result_id='synthetic-0'",
        [],
    )
    .unwrap();
    assert!(estimate_event(&race, 195).is_none());
    sql.execute("UPDATE race_results SET event_type='RACE',source=NULL WHERE source_result_id='synthetic-0'", []).unwrap();
    assert!(estimate_event(&race, 195).is_none());
    sql.execute(
        "UPDATE race_results SET source='zwiftpower-profile' WHERE source_result_id='synthetic-0'",
        [],
    )
    .unwrap();

    for (column, value) in [
        ("source", "unverified"),
        ("source_sha256", "not-a-hash"),
        ("source_rider_id", "not-an-id"),
        ("source_event_id", "not-an-id"),
    ] {
        sql.execute(
            &format!("UPDATE race_results SET {column}=?1 WHERE source_result_id='synthetic-0'"),
            [value],
        )
        .unwrap();
        assert!(estimate_event(&race, 195).is_none());
        sql.execute("UPDATE race_results SET source='zwiftpower-profile',source_sha256=?1,source_rider_id='1',source_event_id='1' WHERE source_result_id='synthetic-0'", ["a".repeat(64)]).unwrap();
    }

    let today = Utc::now().date_naive();
    let original = records[0].clone();
    for day in [today, today + Duration::days(1)] {
        sql.execute("UPDATE race_results SET race_date=?1,event_start_utc=?2 WHERE source_result_id='synthetic-0'",
            [day.to_string(), day.and_hms_opt(12,0,0).unwrap().and_utc().to_rfc3339()]).unwrap();
        assert!(matches!(
            resolve_event(&race, 195)
                .err()
                .and_then(|reason| match reason {
                    EstimateUnavailable::Personal(reason) => Some(reason),
                    _ => None,
                }),
            Some(PersonalFallback::InsufficientHistory {
                available: 11,
                required: 12
            })
        ));
    }
    sql.execute("UPDATE race_results SET race_date=?1,event_start_utc=?2 WHERE source_result_id='synthetic-0'",
        [&original.race_date, &original.event_start_utc]).unwrap();

    for column in ["event_distance_km", "actual_seconds"] {
        sql.execute(
            &format!("UPDATE race_results SET {column}=NULL WHERE source_result_id='synthetic-0'"),
            [],
        )
        .unwrap();
        assert!(estimate_event(&race, 195).is_none());
        sql.execute("UPDATE race_results SET event_distance_km=20,actual_seconds=3600.25 WHERE source_result_id='synthetic-0'", []).unwrap();
    }

    sql.execute(
        "UPDATE race_results SET actual_seconds='invalid' WHERE source_result_id='synthetic-0'",
        [],
    )
    .unwrap();
    assert!(matches!(
        resolve_event(&race, 195)
            .err()
            .and_then(|reason| match reason {
                EstimateUnavailable::Personal(reason) => Some(reason),
                _ => None,
            }),
        Some(PersonalFallback::HistoryUnavailable(_))
    ));
    assert!(format_duration_info(&race, 195).contains("history database unavailable"));
    assert!(prepare_event_row(&race, 195)
        .basis
        .contains("history database unavailable"));

    race.duration_in_minutes = Some(45);
    let fixed = estimate_event(&race, 195).unwrap();
    assert!(fixed.is_fixed);
    assert_eq!(fixed.duration_minutes, 45.0);
    assert!(fixed.personal.is_none());
    assert!(format_duration_info(&race, 195).contains("00:45 (fixed duration)"));
    assert_eq!(prepare_event_row(&race, 195).duration, "00:45");
    assert_eq!(prepare_event_row(&race, 195).basis, "Fixed");
    race.duration_in_minutes = None;
    race.event_sub_groups.push(EventSubGroup {
        id: 1,
        name: "D".into(),
        route_id: None,
        distance_in_meters: None,
        duration_in_minutes: None,
        duration_in_seconds: Some(1800),
        subgroup_label: Some("D".into()),
        category_enforcement: None,
        range_access_label: None,
        laps: None,
    });
    assert_eq!(estimate_event(&race, 195).unwrap().duration_minutes, 30.0);

    let target = PersonalTarget {
        event_day: today,
        route_id: Some(ALIAS),
        distance_km: 20.0,
        ascent_m: None,
    };
    assert!(matches!(
        predict_from_database(&db, target, None, today),
        Err(PersonalFallback::HistoryUnavailable(_))
    ));

    let terrain_db = Database::open(&state.path().join("terrain.db")).unwrap();
    let terrain_target = 4_000_004_100;
    terrain_db
        .add_route(&route(terrain_target, Some(100.0)))
        .unwrap();
    let terrain_records: Vec<_> = (0..12)
        .map(|i| {
            let id = 4_000_004_200 + i as u32;
            let gain_per_lap = i as f64 * 50.0;
            terrain_db
                .add_route(&route(id, Some(gain_per_lap)))
                .unwrap();
            let mut r = record(i);
            r.route_id = id;
            let distance = 10.0 + i as f64;
            r.event_distance_km = Some(distance);
            r.laps = Some(100);
            let total_gain = gain_per_lap * distance / 10.0;
            r.actual_seconds = 60.0 * 2.0 * (distance + 8.0 * total_gain / 1000.0);
            r
        })
        .collect();
    terrain_db.import_history(&terrain_records).unwrap();
    let p = predict_from_database(
        &terrain_db,
        PersonalTarget {
            event_day: today,
            route_id: Some(terrain_target),
            distance_km: 20.0,
            ascent_m: None,
        },
        None,
        today,
    )
    .unwrap();
    assert_eq!(p.basis, PersonalBasis::DistanceAscent);
    assert!((p.minutes - 43.2).abs() < 1e-8, "{}", p.minutes);
}
