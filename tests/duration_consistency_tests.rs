//! Filtering and presentation must agree on the duration of every match.
mod common;
use chrono::Utc;
use zwift_race_finder::{
    estimation::{
        estimate_duration_from_route_id, estimate_duration_with_distance, get_route_data,
    },
    event_display::{format_duration_info, prepare_event_row},
    event_filtering::event_matches_duration,
    models::{EventSubGroup, ZwiftEvent},
};

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
    common::install_route();
    race.duration_in_seconds = Some(2700);
    race.route_id = Some(4_000_001_100);
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
fn racing_score_description_estimate_is_visible() {
    let mut race = event();
    race.name = "Stage race".into();
    race.route_id = Some(u32::MAX);
    race.distance_in_meters = Some(0.0);
    race.description = Some("Distance: 20.1 km".into());
    let mut group = subgroup();
    group.range_access_label = Some("0-199".into());
    race.event_sub_groups.push(group);

    assert!(event_matches_duration(&race, 30, 15, 195));
    assert_eq!(prepare_event_row(&race, 195).distance, "20.1 km");
    let row = prepare_event_row(&race, 195);
    let verbose = format_duration_info(&race, 195);
    assert!(verbose.contains("20.1 km"));
    assert!(verbose.contains(&format!("{} (estimated", row.duration)));
    assert_consistent(&race);
}

#[test]
fn name_fallback_estimate_is_visible() {
    let mut race = event();
    race.name = "Example Crit".into();
    assert!(event_matches_duration(&race, 30, 15, 195));
    assert_eq!(prepare_event_row(&race, 195).distance, "20.0 km");
    assert_consistent(&race);
}

#[test]
fn subgroup_distance_and_fixed_duration_are_visible() {
    let mut race = event();
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
    common::install_route();
    let mut race = event();
    race.route_id = Some(4_000_001_100);
    race.distance_in_meters = Some(30000.0);
    assert_consistent(&race);
    // Explicit event distance already includes its lead-in.
    assert_eq!(prepare_event_row(&race, 195).distance, "30.0 km");
    assert_eq!(
        displayed_minutes(&race),
        estimate_duration_with_distance(4_000_001_100, 30.0, 195).unwrap()
    );

    let route = get_route_data(4_000_001_100).unwrap();
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
        estimate_duration_with_distance(4_000_001_100, lap_distance, 195).unwrap()
    );

    race.event_sub_groups.clear();
    race.distance_in_meters = None;
    assert_consistent(&race);
    assert_eq!(
        displayed_minutes(&race),
        estimate_duration_from_route_id(4_000_001_100, 195).unwrap()
    );
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
    common::install_route();
    let mut race = event();
    race.duration_in_minutes = Some(40);
    race.duration_in_seconds = Some(3600);
    race.route_id = Some(4_000_001_100);
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
