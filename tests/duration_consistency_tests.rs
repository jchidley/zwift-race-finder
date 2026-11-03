//! Filtering and presentation must agree on the duration of every match.
use chrono::Utc;
use zwift_race_finder::{
    estimation::get_route_data,
    event_analysis::find_user_subgroup,
    event_display::{format_duration_info, prepare_event_row},
    event_filtering::{estimate_event, event_matches_duration, resolve_event, EstimateUnavailable},
    models::{EventSubGroup, ZwiftEvent},
    personal_estimation::{PersonalBasis, PersonalFallback},
};

// Immutable, source-qualified synthetic observations. OnceLock prevents parallel
// tests from rewriting a route while another resolver is reading its geometry.
fn personal_route() -> u32 {
    static ROUTE: std::sync::OnceLock<u32> = std::sync::OnceLock::new();
    *ROUTE.get_or_init(|| {
        let id = 4_000_001_130;
        let route = serde_json::from_value(serde_json::json!({
            "route_id": id, "name": "Synthetic duration circuit", "world": "Test",
            "distance_km": 10.0, "elevation_m": 50.0, "surface": "road",
            "lead_in_distance_km": 0.5, "lead_in_elevation_m": null
        }))
        .unwrap();
        let db = zwift_race_finder::database::Database::new().unwrap();
        db.add_route(&route).unwrap();
        install_history(&db, id, &[10.5, 18.0, 20.5, 30.0]);
        let subgroup_id = 4_000_001_120u32;
        let subgroup = serde_json::from_value(serde_json::json!({
            "route_id": subgroup_id, "name": "Synthetic subgroup circuit", "world": "Test",
            "distance_km": 6.0, "elevation_m": 60.0, "surface": "road",
            "lead_in_distance_km": 1.0, "lead_in_elevation_m": 5.0
        }))
        .unwrap();
        db.add_route(&subgroup).unwrap();
        install_history(&db, subgroup_id, &[7.0, 13.0, 19.0]);
        id
    })
}

fn install_history(db: &zwift_race_finder::database::Database, id: u32, distances: &[f64]) {
    use zwift_race_finder::history::HistoryRecord;
    let mut records = Vec::new();
    for (index, &distance) in distances.iter().enumerate() {
        for sample in 0..3 {
            let start = (Utc::now() - chrono::Duration::days(sample + 2))
                .date_naive()
                .and_hms_opt(12, 0, 0)
                .unwrap()
                .and_utc();
            records.push(HistoryRecord {
                route_id: id,
                event_name: "Synthetic ordinary race".into(),
                actual_seconds: distance * 2.0 * 60.0,
                zwift_score: None,
                race_date: start.format("%Y-%m-%d").to_string(),
                event_start_utc: start.to_rfc3339(),
                event_distance_km: Some(distance),
                laps: None,
                result_category: None,
                gun_time_seconds: None,
                source: "zwiftpower-profile".into(),
                source_rider_id: "1".into(),
                source_result_id: format!("duration-{id}-{index}-{sample}"),
                source_event_id: (u64::from(id) * 100 + index as u64 * 3 + sample as u64)
                    .to_string(),
                source_sha256: "a".repeat(64),
                source_payload_json: r#"{"f_t":"TYPE_RACE"}"#.into(),
            });
        }
    }
    db.import_history(&records).unwrap();
}

fn event() -> ZwiftEvent {
    ZwiftEvent {
        id: 1,
        name: "Example race".into(),
        event_start: Utc::now(),
        event_type: "RACE".into(),
        distance_in_meters: None,
        duration_in_minutes: None,
        duration_in_seconds: None,
        route_id: None,
        route: None,
        description: None,
        category_enforcement: false,
        event_sub_groups: vec![],
        sport: "CYCLING".into(),
        tags: vec![],
    }
}

fn subgroup() -> EventSubGroup {
    EventSubGroup {
        id: 1,
        name: "D".into(),
        route_id: None,
        distance_in_meters: None,
        duration_in_minutes: None,
        duration_in_seconds: None,
        subgroup_label: None,
        category_enforcement: None,
        range_access_label: None,
        laps: None,
    }
}

fn displayed_minutes(event: &ZwiftEvent) -> u32 {
    let row = prepare_event_row(event, 195);
    let (hours, minutes) = row
        .duration
        .split_once(':')
        .expect("match must display a duration");
    hours.parse::<u32>().unwrap() * 60 + minutes.parse::<u32>().unwrap()
}

fn assert_consistent(event: &ZwiftEvent) {
    let minutes = displayed_minutes(event);
    let estimate = resolve_event(event, 195).unwrap();
    if !estimate.is_fixed {
        assert_eq!(
            estimate.personal.as_ref().unwrap().basis,
            PersonalBasis::SameRoute
        );
    }
    for target in [15, 30, 45, 60, 90] {
        assert_eq!(
            event_matches_duration(event, target, 15, 195),
            minutes.abs_diff(target) <= 15,
            "filter disagrees with displayed {minutes} minutes for target {target}"
        );
    }
}

#[test]
fn fixed_duration_overrides_lap_estimate_krg_regression() {
    let mut race = event();
    race.name = "KRG Thursday Night Crit".into();
    // Live KRG data declares 2700 seconds as well as subgroup laps.
    // Explicit synthetic metadata makes this independent of imported client data.
    personal_route();
    race.duration_in_seconds = Some(2700);
    race.route_id = Some(4_000_001_130);
    race.distance_in_meters = Some(0.0);
    let mut group = subgroup();
    group.laps = Some(2);
    group.distance_in_meters = Some(0.0);
    race.event_sub_groups.push(group);

    assert!(event_matches_duration(&race, 30, 15, 195));
    assert_eq!(displayed_minutes(&race), 45);
    assert!(format_duration_info(&race, 195).contains("00:45 (fixed duration)"));
    assert_consistent(&race);

    race.duration_in_seconds = Some(53 * 60);
    assert_eq!(displayed_minutes(&race), 53);
    assert!(!event_matches_duration(&race, 30, 15, 195));
}

#[test]
fn racing_score_description_geometry_is_visible_for_fixed_duration() {
    let mut race = event();
    race.name = "Stage race".into();
    race.route_id = Some(u32::MAX);
    race.distance_in_meters = Some(0.0);
    race.description = Some("Distance: 20.1 km".into());
    let mut group = subgroup();
    group.range_access_label = Some("0-199".into());
    race.event_sub_groups.push(group);

    assert!(estimate_event(&race, 195).is_none());
    race.duration_in_minutes = Some(40);
    assert!(event_matches_duration(&race, 30, 15, 195));
    assert_eq!(prepare_event_row(&race, 195).distance, "20.1 km");
    let row = prepare_event_row(&race, 195);
    let verbose = format_duration_info(&race, 195);
    assert!(verbose.contains("20.1 km"));
    assert!(verbose.contains(&format!("{} (fixed duration)", row.duration)));
    assert_consistent(&race);
}

#[test]
fn name_fallback_geometry_does_not_invent_personal_pace() {
    let mut race = event();
    race.name = "Example Crit".into();
    assert!(estimate_event(&race, 195).is_none());
    race.duration_in_minutes = Some(40);
    assert!(event_matches_duration(&race, 30, 15, 195));
    assert_eq!(prepare_event_row(&race, 195).distance, "20.0 km");
    assert_consistent(&race);
}

#[test]
fn subgroup_distance_and_fixed_duration_are_visible() {
    let mut race = event();
    race.route_id = Some(personal_route());
    let mut group = subgroup();
    group.distance_in_meters = Some(18000.0);
    race.event_sub_groups.push(group);
    assert_consistent(&race);
    assert_eq!(prepare_event_row(&race, 195).distance, "18.0 km");

    race.event_sub_groups[0].duration_in_minutes = Some(40);
    assert_eq!(displayed_minutes(&race), 40);
    assert_consistent(&race);
}

#[test]
fn known_route_distance_and_laps_use_filter_estimate() {
    personal_route();
    let mut race = event();
    race.route_id = Some(4_000_001_130);
    race.distance_in_meters = Some(30000.0);
    assert_consistent(&race);
    // Explicit event distance already includes its lead-in.
    assert_eq!(prepare_event_row(&race, 195).distance, "30.0 km");
    assert_eq!(displayed_minutes(&race), 60);

    let route = get_route_data(4_000_001_130).unwrap();
    let mut group = subgroup();
    group.laps = Some(2);
    race.event_sub_groups.push(group);
    assert_consistent(&race);
    let lap_distance = route.distance_km * 2.0 + route.lead_in_distance_km.unwrap();
    assert_eq!(
        prepare_event_row(&race, 195).distance,
        format!("{lap_distance:.1} km")
    );
    assert_eq!(
        displayed_minutes(&race),
        (lap_distance * 2.0).round() as u32
    );

    race.event_sub_groups.clear();
    race.distance_in_meters = None;
    assert_consistent(&race);
    assert_eq!(displayed_minutes(&race), 21);
}

#[test]
fn genuinely_unestimable_event_is_not_a_match() {
    let mut race = event();
    assert_eq!(prepare_event_row(&race, 195).duration, "? min");
    assert!(!event_matches_duration(&race, 30, 15, 195));
    race.route_id = Some(u32::MAX);
    race.distance_in_meters = Some(0.0);
    assert_eq!(prepare_event_row(&race, 195).duration, "? min");
    assert!(!event_matches_duration(&race, 30, 15, 195));
}

#[test]
fn event_fixed_minutes_take_precedence_over_seconds_and_subgroup() {
    personal_route();
    let mut race = event();
    race.duration_in_minutes = Some(40);
    race.duration_in_seconds = Some(3600);
    race.route_id = Some(4_000_001_130);
    race.distance_in_meters = Some(30000.0);
    let mut group = subgroup();
    group.duration_in_minutes = Some(80);
    race.event_sub_groups.push(group);
    assert_eq!(displayed_minutes(&race), 40);
    assert_eq!(prepare_event_row(&race, 195).distance, "30.0 km");
    // The synthetic fixture deliberately has unknown lead-in climbing.
    assert_eq!(prepare_event_row(&race, 195).elevation, "?m");
    assert_consistent(&race);
}

// Both IDs exceed i32::MAX; differing lap geometry exposes accidental use of
// the event route for subgroup duration or elevation.
fn subgroup_route() -> u32 {
    personal_route(); // One initialization gate for all fixture writes.
    4_000_001_120
}

fn assert_fixed_geometry_matches_resolved(race: &ZwiftEvent) {
    let estimated = prepare_event_row(race, 195);
    assert_consistent(race);
    let mut fixed = race.clone();
    fixed.duration_in_minutes = Some(40);
    let fixed_row = prepare_event_row(&fixed, 195);
    assert_eq!(fixed_row.distance, estimated.distance);
    assert_eq!(fixed_row.elevation, estimated.elevation);
    assert_eq!(displayed_minutes(&fixed), 40);
    assert_consistent(&fixed);
    let verbose = format_duration_info(&fixed, 195);
    assert!(verbose.contains(&fixed_row.distance));
    if let Some(gain) = estimate_event(&fixed, 195).unwrap().elevation_m {
        assert!(verbose.contains(&format!("{gain} m")));
    }
}

#[test]
fn subgroup_route_controls_laps_duration_and_elevation_in_both_paths() {
    let mut race = event();
    race.route_id = Some(personal_route());
    race.distance_in_meters = Some(30000.0);
    let mut group = subgroup();
    group.route_id = Some(subgroup_route());
    group.laps = Some(2);
    race.event_sub_groups.push(group);
    let row = prepare_event_row(&race, 195);
    assert_eq!(row.distance, "13.0 km"); // 2 * 6 km + 1 km lead-in
    assert_eq!(row.elevation, "125m"); // 2 * 60 m + 5 m lead-in
    assert_eq!(displayed_minutes(&race), 26);
    assert_fixed_geometry_matches_resolved(&race);

    // An absent event route must not discard a known subgroup route.
    race.route_id = None;
    assert_eq!(prepare_event_row(&race, 195).distance, "13.0 km");
    assert_fixed_geometry_matches_resolved(&race);

    // A positive explicit subgroup total wins over laps; don't add lead-in.
    race.event_sub_groups[0].distance_in_meters = Some(19000.0);
    let row = prepare_event_row(&race, 195);
    assert_eq!(row.distance, "19.0 km");
    assert_eq!(row.elevation, "~185m");
    assert_eq!(displayed_minutes(&race), 38);
    assert_fixed_geometry_matches_resolved(&race);
}

#[test]
fn invalid_distances_and_zero_laps_do_not_mask_event_distance() {
    let mut race = event();
    race.route_id = Some(subgroup_route());
    race.distance_in_meters = Some(19000.0);
    let mut group = subgroup();
    group.laps = Some(0);
    race.event_sub_groups.push(group);
    for distance in [
        None,
        Some(0.0),
        Some(-100.0),
        Some(f64::NAN),
        Some(f64::INFINITY),
    ] {
        race.event_sub_groups[0].distance_in_meters = distance;
        assert_eq!(prepare_event_row(&race, 195).distance, "19.0 km");
        assert_eq!(prepare_event_row(&race, 195).elevation, "~185m");
        assert_fixed_geometry_matches_resolved(&race);
    }

    // Zero laps must also leave a positive subgroup distance intact.
    race.event_sub_groups[0].distance_in_meters = Some(13000.0);
    assert_eq!(prepare_event_row(&race, 195).distance, "13.0 km");
    assert_fixed_geometry_matches_resolved(&race);

    // Invalid event distances must not poison the known-route fallback.
    race.event_sub_groups[0].distance_in_meters = None;
    for distance in [Some(0.0), Some(-1.0), Some(f64::NAN), Some(f64::INFINITY)] {
        race.distance_in_meters = distance;
        assert_eq!(prepare_event_row(&race, 195).distance, "7.0 km");
        assert_fixed_geometry_matches_resolved(&race);
    }
}

#[test]
fn score_ranges_override_category_names_and_touching_bands_keep_api_order() {
    let mut race = event();
    race.route_id = Some(personal_route());
    race.distance_in_meters = Some(30000.0);
    let mut wrong = subgroup();
    wrong.id = 11;
    wrong.name = "D".into(); // Traditional category would wrongly pick this.
    wrong.range_access_label = Some("200-300".into());
    wrong.distance_in_meters = Some(9000.0);
    let mut actual = subgroup();
    actual.id = 12;
    actual.name = "C".into();
    actual.range_access_label = Some(" 100 - 200 ".into());
    actual.route_id = Some(subgroup_route());
    actual.laps = Some(2);
    race.event_sub_groups = vec![wrong, actual];
    assert_eq!(find_user_subgroup(&race, 195).unwrap().id, 12);
    assert_eq!(prepare_event_row(&race, 195).distance, "13.0 km");
    assert_eq!(prepare_event_row(&race, 195).elevation, "125m");
    assert_fixed_geometry_matches_resolved(&race);

    // Both inclusive ranges contain 200; first API subgroup wins.
    assert_eq!(find_user_subgroup(&race, 200).unwrap().id, 11);
    race.event_sub_groups.swap(0, 1);
    assert_eq!(find_user_subgroup(&race, 200).unwrap().id, 12);
    assert_eq!(find_user_subgroup(&race, 100).unwrap().id, 12);
    assert_eq!(find_user_subgroup(&race, 300).unwrap().id, 11);
    for score in [0, 99, 301, u32::MAX] {
        assert!(find_user_subgroup(&race, score).is_none());
        assert!(matches!(
            resolve_event(&race, score),
            Err(EstimateUnavailable::NoEligibleSubgroup)
        ));
        assert!(estimate_event(&race, score).is_none());
        assert_eq!(prepare_event_row(&race, score).duration, "? min");
        assert!(!event_matches_duration(&race, 40, 100, score));
    }
    race.duration_in_minutes = Some(40);
    assert!(estimate_event(&race, 99).is_none());
}

#[test]
fn traditional_selection_uses_exact_labels_then_category_tokens_not_title_letters() {
    let mut race = event();
    let mut title = subgroup();
    title.id = 1;
    title.name = "Daring Example Race".into();
    let mut labelled = subgroup();
    labelled.id = 2;
    labelled.name = "Race D".into();
    labelled.subgroup_label = Some("C".into());
    let mut actual = subgroup();
    actual.id = 3;
    actual.name = "Category D (Open)".into();
    race.event_sub_groups = vec![title, labelled, actual];
    assert_eq!(find_user_subgroup(&race, 195).unwrap().id, 3);
    assert_eq!(find_user_subgroup(&race, 250).unwrap().id, 2);
    race.event_sub_groups[2].subgroup_label = Some("D".into());
    race.event_sub_groups[2].name = "Uninformative race title".into();
    assert_eq!(find_user_subgroup(&race, 195).unwrap().id, 3);
    race.event_sub_groups[2].subgroup_label = Some("D+".into());
    assert!(find_user_subgroup(&race, 195).is_none());

    // Preserve D-to-E eligibility but prefer an actual D, even if E is first.
    race.event_sub_groups[0].name = "E".into();
    race.event_sub_groups[2].subgroup_label = Some("D".into());
    assert_eq!(find_user_subgroup(&race, 195).unwrap().id, 3);
    race.event_sub_groups.pop();
    assert_eq!(find_user_subgroup(&race, 195).unwrap().id, 1);
}

#[test]
fn api_subgroup_seconds_precede_calculation_but_not_event_fixed_duration() {
    let mut race = event();
    let mut group: EventSubGroup = serde_json::from_value(serde_json::json!({
        "id": 1, "name": "Misleading event title",
        "subgroupLabel": "D", "durationInSeconds": 2700,
        "routeId": subgroup_route(), "laps": 2
    }))
    .unwrap();
    assert_eq!(group.duration_in_seconds, Some(2700));
    assert_eq!(group.subgroup_label.as_deref(), Some("D"));
    group.duration_in_minutes = Some(0);
    race.route_id = Some(personal_route());
    race.event_sub_groups.push(group);
    assert_eq!(displayed_minutes(&race), 45);
    assert_eq!(prepare_event_row(&race, 195).distance, "13.0 km");
    assert_eq!(prepare_event_row(&race, 195).elevation, "125m");
    assert!(format_duration_info(&race, 195).contains("00:45 (fixed duration)"));
    assert_consistent(&race);
    race.event_sub_groups[0].duration_in_minutes = Some(35);
    assert_eq!(displayed_minutes(&race), 35);
    race.duration_in_seconds = Some(2400);
    assert_eq!(displayed_minutes(&race), 40);
    race.duration_in_minutes = Some(30);
    assert_eq!(displayed_minutes(&race), 30);

    race.duration_in_seconds = None;
    race.duration_in_minutes = None;
    race.event_sub_groups[0].duration_in_seconds = Some(0);
    race.event_sub_groups[0].duration_in_minutes = Some(0);
    assert!(!estimate_event(&race, 195).unwrap().is_fixed);
}

#[test]
fn route_description_and_name_fallback_geometry_is_shared_by_fixed_path() {
    let mut race = event();
    race.route_id = Some(subgroup_route());
    assert_eq!(prepare_event_row(&race, 195).distance, "7.0 km");
    assert_eq!(prepare_event_row(&race, 195).elevation, "65m");
    assert_fixed_geometry_matches_resolved(&race);
    race.route_id = Some(u32::MAX);
    race.duration_in_minutes = Some(40);
    race.description = Some("Distance: 20.1 km".into());
    let mut group = subgroup();
    group.range_access_label = Some("0-199".into());
    race.event_sub_groups.push(group);
    assert_eq!(prepare_event_row(&race, 195).distance, "20.1 km");
    assert_fixed_geometry_matches_resolved(&race);
    race.event_sub_groups.clear();
    race.description = None;
    race.name = "Example Crit".into();
    assert_eq!(prepare_event_row(&race, 195).distance, "20.0 km");
    assert_fixed_geometry_matches_resolved(&race);
}

#[test]
fn blank_score_labels_are_traditional_but_malformed_restrictions_are_not() {
    use zwift_race_finder::models::is_racing_score_event;
    let mut race = event();
    race.route_id = Some(subgroup_route());
    let mut group = subgroup();
    group.subgroup_label = Some("D".into());
    group.laps = Some(2);
    race.event_sub_groups.push(group);
    race.duration_in_minutes = Some(40);
    for event_type in ["RACE", "TIME_TRIAL"] {
        race.event_type = event_type.into();
        for label in ["", "  \t"] {
            race.event_sub_groups[0].range_access_label = Some(label.into());
            assert!(!is_racing_score_event(&race));
            assert_eq!(find_user_subgroup(&race, 195).unwrap().id, 1);
            assert_eq!(prepare_event_row(&race, 195).distance, "13.0 km");
            assert_fixed_geometry_matches_resolved(&race);
        }
    }
    race.event_sub_groups[0].range_access_label = Some("invalid restriction".into());
    race.duration_in_minutes = Some(40);
    assert!(is_racing_score_event(&race));
    assert!(estimate_event(&race, 195).is_none());
}

#[test]
fn route_metadata_url_and_unknown_logging_follow_selected_subgroup() {
    use zwift_race_finder::event_analysis::selected_route_id;
    use zwift_race_finder::event_display::{
        external_route_url, format_route_info, log_unknown_route,
    };
    let mut race = event();
    race.route_id = Some(personal_route());
    let template = subgroup_route();
    let chosen = 4_000_001_121u32;
    let db = zwift_race_finder::database::Database::new().unwrap();
    let mut route = db.get_route(template).unwrap().unwrap();
    route.route_id = chosen;
    route.world = "Watopia".into();
    route.slug = Some("synthetic-subgroup-circuit".into());
    db.add_route(&route).unwrap();
    let mut group = subgroup();
    group.route_id = Some(chosen);
    group.laps = Some(2);
    race.event_sub_groups.push(group);
    assert_eq!(selected_route_id(&race, 195), Some(chosen));
    let info = format_route_info(&race, 195).unwrap();
    assert!(info.contains(&chosen.to_string()));
    assert!(info.contains("60.0m"));
    assert!(!info.contains(&race.route_id.unwrap().to_string()));
    assert_eq!(
        external_route_url(&race, 195).as_deref(),
        Some("https://whatsonzwift.com/world/watopia/route/synthetic-subgroup-circuit")
    );

    let unknown = 4_000_001_199u32;
    race.event_sub_groups[0].route_id = Some(unknown);
    assert!(external_route_url(&race, 195).is_none());
    assert!(format_route_info(&race, 195)
        .unwrap()
        .contains(&unknown.to_string()));
    log_unknown_route(&race, 195);
    assert!(db
        .get_unknown_routes()
        .unwrap()
        .iter()
        .any(|(id, _, _)| *id == unknown));
}

#[test]
fn subminute_fixed_seconds_do_not_mask_a_calculated_duration() {
    let mut race = event();
    race.route_id = Some(subgroup_route());
    race.distance_in_meters = Some(19000.0);
    let mut group = subgroup();
    group.duration_in_seconds = Some(59);
    race.event_sub_groups.push(group);
    race.duration_in_seconds = Some(59);
    let estimate = estimate_event(&race, 195).unwrap();
    assert!(!estimate.is_fixed);
    assert!(estimate.duration_minutes > 0);
    race.event_sub_groups[0].duration_in_seconds = Some(60);
    assert_eq!(displayed_minutes(&race), 1);
    race.duration_in_seconds = Some(120);
    assert_eq!(displayed_minutes(&race), 2);
}

#[test]
fn rounded_prediction_is_shared_by_filter_compact_and_verbose_output() {
    let mut race = event();
    // Same-route measured pace is 2 min/km: 45.75 rounds once to 46.
    race.route_id = Some(personal_route());
    race.distance_in_meters = Some(22_875.0);
    assert_eq!(displayed_minutes(&race), 46);
    assert!(event_matches_duration(&race, 46, 0, 195));
    assert!(!event_matches_duration(&race, 45, 0, 195));
    assert!(format_duration_info(&race, 195).contains("00:46"));
    assert_consistent(&race);
}

#[test]
fn unavailable_history_and_unsupported_formats_never_match_without_fixed_time() {
    let mut race = event();
    race.route_id = Some(personal_route());
    race.distance_in_meters = Some(20_500.0);
    // A cutoff before every synthetic observation deterministically models an
    // empty eligible history even while other tests install shared fixtures.
    race.event_start = chrono::DateTime::parse_from_rfc3339("1970-01-01T12:00:00Z")
        .unwrap()
        .with_timezone(&Utc);
    assert!(matches!(
        resolve_event(&race, 195),
        Err(EstimateUnavailable::Personal(
            PersonalFallback::InsufficientHistory {
                available: 0,
                required: 12
            }
        ))
    ));
    assert!(estimate_event(&race, 195).is_none());
    assert!(!event_matches_duration(&race, 40, 100, 195));
    assert_eq!(prepare_event_row(&race, 195).duration, "? min");
    race.duration_in_minutes = Some(40);
    assert_eq!(resolve_event(&race, 195).unwrap().duration_minutes, 40);

    race.event_start = Utc::now();
    for kind in ["TIME_TRIAL", "GROUP_RIDE", "UNKNOWN"] {
        race.event_type = kind.into();
        race.duration_in_minutes = None;
        assert!(matches!(
            resolve_event(&race, 195),
            Err(EstimateUnavailable::Personal(
                PersonalFallback::UnsupportedFormat
            ))
        ));
        assert!(!event_matches_duration(&race, 40, 100, 195));
        race.duration_in_minutes = Some(40);
        assert!(resolve_event(&race, 195).unwrap().is_fixed);
    }
}

#[test]
fn missing_geometry_has_a_distinct_unavailable_reason_but_fixed_time_still_works() {
    let mut race = event();
    assert!(matches!(
        resolve_event(&race, 195),
        Err(EstimateUnavailable::MissingGeometry)
    ));
    race.duration_in_seconds = Some(2700);
    let estimate = resolve_event(&race, 195).unwrap();
    assert_eq!(estimate.duration_minutes, 45);
    assert!(estimate.is_fixed);
    assert!(estimate.distance_km.is_none());
    assert!(event_matches_duration(&race, 45, 0, 195));
}
