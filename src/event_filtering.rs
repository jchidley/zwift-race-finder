//! Event filtering logic extracted from main.rs
//!
//! This module contains predicates and filtering functions for Zwift events

use crate::database::Database;
use crate::models::ZwiftEvent;
use chrono::{DateTime, Utc};

/// Statistics tracking what was filtered out
#[derive(Debug, Default)]
pub struct FilterStats {
    pub sport_filtered: u32,
    pub time_filtered: u32,
    pub type_filtered: u32,
    pub duration_filtered: u32,
    pub tag_filtered: u32,
    pub unknown_routes: u32,
    pub missing_distance: u32,
}

impl FilterStats {
    pub fn total_filtered(&self) -> u32 {
        self.sport_filtered
            + self.time_filtered
            + self.type_filtered
            + self.duration_filtered
            + self.tag_filtered
    }

    pub fn duration_no_match(&self) -> u32 {
        self.duration_filtered
    }
}

/// Filter events by sport (cycling only)
pub fn filter_by_sport(events: &mut Vec<ZwiftEvent>) -> u32 {
    let pre_count = events.len();
    events.retain(|event| event.sport.to_uppercase() == "CYCLING");
    (pre_count - events.len()) as u32
}

/// Filter events by time range
pub fn filter_by_time(
    events: &mut Vec<ZwiftEvent>,
    now: DateTime<Utc>,
    max_date: DateTime<Utc>,
) -> u32 {
    let pre_count = events.len();
    events.retain(|event| event.event_start > now && event.event_start < max_date);
    (pre_count - events.len()) as u32
}

/// Filter events by type
pub fn filter_by_event_type(events: &mut Vec<ZwiftEvent>, event_type: &str) -> u32 {
    let pre_count = events.len();
    events.retain(|event| match event_type.to_lowercase().as_str() {
        "all" => true,
        "race" => event.event_type == "RACE",
        "tt" | "time_trial" => event.event_type == "TIME_TRIAL",
        "workout" => event.event_type == "GROUP_WORKOUT",
        "group" => {
            event.event_type == "GROUP_RIDE"
                && !event.name.to_lowercase().contains("fondo")
                && !event.name.to_lowercase().contains("sportive")
        }
        "fondo" => {
            event.event_type == "GROUP_RIDE"
                && (event.name.to_lowercase().contains("fondo")
                    || event.name.to_lowercase().contains("sportive")
                    || event.name.to_lowercase().contains("century"))
        }
        _ => {
            eprintln!(
                "Warning: Unknown event type '{}', showing all events",
                event_type
            );
            true
        }
    });
    (pre_count - events.len()) as u32
}

/// Filter events by tags (include)
pub fn filter_by_tags(events: &mut Vec<ZwiftEvent>, tags: &[String]) -> u32 {
    if tags.is_empty() {
        return 0;
    }
    let pre_count = events.len();
    events.retain(|event| {
        tags.iter()
            .any(|tag| event.tags.iter().any(|etag| etag.contains(tag)))
    });
    (pre_count - events.len()) as u32
}

/// Filter events by excluded tags
pub fn filter_by_excluded_tags(events: &mut Vec<ZwiftEvent>, exclude_tags: &[String]) -> u32 {
    if exclude_tags.is_empty() {
        return 0;
    }
    let pre_count = events.len();
    events.retain(|event| {
        !exclude_tags
            .iter()
            .any(|tag| event.tags.iter().any(|etag| etag.contains(tag)))
    });
    (pre_count - events.len()) as u32
}


/// The event estimate shared by duration filtering and both display modes.
#[derive(Debug, Clone)]
pub struct EventEstimate {
    pub duration_minutes: u32,
    pub distance_km: Option<f64>,
    pub elevation_m: Option<u32>,
    pub is_fixed: bool,
    pub elevation_is_estimate: bool,
}

impl EventEstimate {
    fn fixed(duration_minutes: u32, event: &ZwiftEvent, zwift_score: u32) -> Self {
        let subgroup = crate::event_analysis::find_user_subgroup(event, zwift_score);
        let distance_km = subgroup
            .and_then(|sg| sg.distance_in_meters)
            .filter(|&d| d > 0.0)
            .or(event.distance_in_meters.filter(|&d| d > 0.0))
            .map(|meters| meters / crate::constants::METERS_PER_KILOMETER)
            .or_else(|| {
                let laps = subgroup?.laps?;
                let route = crate::estimation::get_route_data(event.route_id?)?;
                Some(route.distance_km * f64::from(laps) + route.lead_in_distance_km?)
            });
        let mut estimate = distance_km.map_or(
            Self {
                duration_minutes,
                distance_km: None,
                elevation_m: None,
                is_fixed: true,
                elevation_is_estimate: false,
            },
            |distance| Self::estimated(duration_minutes, distance, event.route_id),
        );
        // Laps provide an exact total; a supplied distance only supports a
        // proportional estimate without the route's full elevation profile.
        let has_distance = subgroup
            .and_then(|sg| sg.distance_in_meters)
            .filter(|&d| d > 0.0)
            .or(event.distance_in_meters.filter(|&d| d > 0.0))
            .is_some();
        if !has_distance {
            if let (Some(route_id), Some(laps)) = (event.route_id, subgroup.and_then(|sg| sg.laps))
            {
                estimate = estimate.with_laps(route_id, laps);
            }
        }
        estimate.is_fixed = true;
        estimate
    }

    fn estimated(duration_minutes: u32, distance_km: f64, route_id: Option<u32>) -> Self {
        let elevation_m = route_id
            .and_then(crate::estimation::get_route_data)
            .and_then(|route| crate::estimation::elevation_for_distance(&route, distance_km))
            .map(|gain| gain.round() as u32);
        Self {
            duration_minutes,
            distance_km: Some(distance_km),
            elevation_m,
            is_fixed: false,
            elevation_is_estimate: true,
        }
    }
    fn with_laps(mut self, route_id: u32, laps: u32) -> Self {
        self.elevation_m = crate::estimation::get_route_data(route_id)
            .and_then(|route| crate::estimation::elevation_for_laps(&route, laps))
            .map(|gain| gain.round() as u32);
        self.elevation_is_estimate = false;
        self
    }
}

/// Resolve a duration using the existing filter's ordered fallbacks.
/// No estimate means the event cannot be claimed as a duration match.
pub fn estimate_event(event: &ZwiftEvent, zwift_score: u32) -> Option<EventEstimate> {
    use crate::constants::METERS_PER_KILOMETER;
    use crate::duration_estimation::estimate_duration_for_category;
    use crate::estimation::{
        estimate_duration_from_route_id, estimate_duration_with_distance, get_route_data,
    };
    use crate::event_analysis::find_user_subgroup;
    use crate::models::is_racing_score_event;
    use crate::parsing::{estimate_distance_from_name, parse_distance_from_description};

    let duration_minutes = event
        .duration_in_minutes
        .filter(|&d| d > 0)
        .or_else(|| event.duration_in_seconds.map(|s| s / 60).filter(|&d| d > 0));
    if let Some(duration) = duration_minutes {
        return Some(EventEstimate::fixed(duration, event, zwift_score));
    }

    let user_subgroup = find_user_subgroup(event, zwift_score);
    if let Some(route_id) = event.route_id {
        if let Some(laps) = user_subgroup.and_then(|sg| sg.laps) {
            if let Some((route, lead_in)) = get_route_data(route_id)
                .and_then(|route| route.lead_in_distance_km.map(|lead| (route, lead)))
            {
                let distance = lead_in + route.distance_km * f64::from(laps);
                if let Some(minutes) =
                    estimate_duration_with_distance(route_id, distance, zwift_score)
                {
                    return Some(
                        EventEstimate::estimated(minutes, distance, Some(route_id))
                            .with_laps(route_id, laps),
                    );
                }
            }
        }

        let distance_meters = user_subgroup
            .and_then(|sg| sg.distance_in_meters)
            .or(event.distance_in_meters);
        if let Some(distance) = distance_meters.filter(|&d| d > 0.0) {
            let distance = distance / METERS_PER_KILOMETER;
            let minutes = estimate_duration_with_distance(route_id, distance, zwift_score)
                .unwrap_or_else(|| {
                    estimate_duration_for_category(
                        distance,
                        event.route.as_deref().unwrap_or(&event.name),
                        zwift_score,
                    )
                });
            return Some(EventEstimate::estimated(minutes, distance, Some(route_id)));
        }
        if let Some(minutes) = estimate_duration_from_route_id(route_id, zwift_score) {
            // Count the lead-in once even for historical multi-lap metadata.
            let laps = Database::new()
                .ok()
                .and_then(|db| db.get_multi_lap_info(&event.name).ok().flatten());

            let route = get_route_data(route_id)?;
            let lap_count = laps.unwrap_or(1);
            let distance = route.distance_km * f64::from(lap_count) + route.lead_in_distance_km?;
            let minutes = if laps.is_some() {
                estimate_duration_with_distance(route_id, distance, zwift_score)?
            } else {
                minutes
            };
            return Some(
                EventEstimate::estimated(minutes, distance, Some(route_id))
                    .with_laps(route_id, lap_count),
            );
        }
    }

    // Unknown/no route: retain the filter's distance, description, and name fallbacks.
    let route_name = event.route.as_deref().unwrap_or(&event.name);
    if let Some(distance) = event.distance_in_meters.filter(|&d| d > 0.0) {
        let distance = distance / METERS_PER_KILOMETER;
        return Some(EventEstimate::estimated(
            estimate_duration_for_category(distance, route_name, zwift_score),
            distance,
            None,
        ));
    }
    if is_racing_score_event(event) {
        if let Some(distance) = parse_distance_from_description(&event.description) {
            return Some(EventEstimate::estimated(
                estimate_duration_for_category(distance, route_name, zwift_score),
                distance,
                None,
            ));
        }
    }
    if let Some(distance) = estimate_distance_from_name(&event.name) {
        return Some(EventEstimate::estimated(
            estimate_duration_for_category(distance, &event.name, zwift_score),
            distance,
            None,
        ));
    }

    let subgroup = user_subgroup?;
    if let Some(minutes) = subgroup.duration_in_minutes {
        return Some(EventEstimate::fixed(minutes, event, zwift_score));
    }
    subgroup
        .distance_in_meters
        .filter(|&d| d > 0.0)
        .map(|distance| {
            let distance = distance / METERS_PER_KILOMETER;
            EventEstimate::estimated(
                estimate_duration_for_category(distance, route_name, zwift_score),
                distance,
                None,
            )
        })
}

/// Check the same duration that the table and verbose output display.
pub fn event_matches_duration(
    event: &ZwiftEvent,
    target_duration: u32,
    tolerance: u32,
    zwift_score: u32,
) -> bool {
    estimate_event(event, zwift_score)
        .is_some_and(|estimate| estimate.duration_minutes.abs_diff(target_duration) <= tolerance)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ZwiftEvent;
    use chrono::Duration;

    fn create_test_event(name: &str, sport: &str, event_type: &str) -> ZwiftEvent {
        ZwiftEvent {
            id: 1,
            name: name.to_string(),
            event_start: Utc::now() + Duration::hours(1),
            event_type: event_type.to_string(),
            distance_in_meters: Some(40000.0),
            duration_in_minutes: None,
            duration_in_seconds: None,
            route_id: Some(1),
            route: Some("Watopia".to_string()),
            description: None,
            category_enforcement: true,
            event_sub_groups: vec![],
            sport: sport.to_string(),
            tags: vec![],
        }
    }

    #[test]
    fn test_filter_stats_total() {
        let mut stats = FilterStats::default();
        stats.sport_filtered = 5;
        stats.time_filtered = 3;
        stats.type_filtered = 2;
        stats.duration_filtered = 1;

        assert_eq!(stats.total_filtered(), 11);
        assert_eq!(stats.duration_no_match(), 1);
    }

    #[test]
    fn test_filter_by_sport() {
        let mut events = vec![
            create_test_event("Cycling Race", "CYCLING", "RACE"),
            create_test_event("Running Event", "RUNNING", "RACE"),
            create_test_event("Cycling TT", "cycling", "TIME_TRIAL"), // lowercase
            create_test_event("Run TT", "RUN", "TIME_TRIAL"),
        ];

        let filtered = filter_by_sport(&mut events);
        assert_eq!(filtered, 2);
        assert_eq!(events.len(), 2);
        assert!(events.iter().all(|e| e.sport.to_uppercase() == "CYCLING"));
    }

    #[test]
    fn test_filter_by_time() {
        let now = Utc::now();
        let mut events = vec![
            {
                let mut e = create_test_event("Past Event", "CYCLING", "RACE");
                e.event_start = now - Duration::hours(1);
                e
            },
            {
                let mut e = create_test_event("Future Event 1h", "CYCLING", "RACE");
                e.event_start = now + Duration::hours(1);
                e
            },
            {
                let mut e = create_test_event("Future Event 25h", "CYCLING", "RACE");
                e.event_start = now + Duration::hours(25);
                e
            },
        ];

        let max_date = now + Duration::days(1);
        let filtered = filter_by_time(&mut events, now, max_date);

        assert_eq!(filtered, 2); // Past and too far future
        assert_eq!(events.len(), 1);
        assert!(events[0].name == "Future Event 1h");
    }

    #[test]
    fn test_filter_by_event_type_race() {
        let mut events = vec![
            create_test_event("Race 1", "CYCLING", "RACE"),
            create_test_event("Group Ride", "CYCLING", "GROUP_RIDE"),
            create_test_event("Time Trial", "CYCLING", "TIME_TRIAL"),
            create_test_event("Workout", "CYCLING", "GROUP_WORKOUT"),
        ];

        let filtered = filter_by_event_type(&mut events, "race");
        assert_eq!(filtered, 3);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, "RACE");
    }

    #[test]
    fn test_filter_by_event_type_tt() {
        let mut events = vec![
            create_test_event("Race", "CYCLING", "RACE"),
            create_test_event("TT", "CYCLING", "TIME_TRIAL"),
        ];

        let filtered = filter_by_event_type(&mut events, "tt");
        assert_eq!(filtered, 1);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, "TIME_TRIAL");

        // Test alias
        let mut events2 = vec![
            create_test_event("Race", "CYCLING", "RACE"),
            create_test_event("TT", "CYCLING", "TIME_TRIAL"),
        ];
        let filtered2 = filter_by_event_type(&mut events2, "time_trial");
        assert_eq!(filtered2, 1);
        assert_eq!(events2.len(), 1);
    }

    #[test]
    fn test_filter_by_event_type_group() {
        let mut events = vec![
            create_test_event("Group Ride", "CYCLING", "GROUP_RIDE"),
            create_test_event("Fondo Event", "CYCLING", "GROUP_RIDE"),
            create_test_event("Sportive", "CYCLING", "GROUP_RIDE"),
            create_test_event("Race", "CYCLING", "RACE"),
        ];

        let filtered = filter_by_event_type(&mut events, "group");
        assert_eq!(filtered, 3); // Excludes fondo, sportive, and race
        assert_eq!(events.len(), 1);
        assert!(events[0].name == "Group Ride");
    }

    #[test]
    fn test_filter_by_event_type_fondo() {
        let mut events = vec![
            create_test_event("Group Ride", "CYCLING", "GROUP_RIDE"),
            create_test_event("Gran Fondo", "CYCLING", "GROUP_RIDE"),
            create_test_event("Spring Sportive", "CYCLING", "GROUP_RIDE"),
            create_test_event("Century Ride", "CYCLING", "GROUP_RIDE"),
            create_test_event("Race", "CYCLING", "RACE"),
        ];

        let filtered = filter_by_event_type(&mut events, "fondo");
        assert_eq!(filtered, 2); // Only fondo/sportive/century remain
        assert_eq!(events.len(), 3);
        assert!(events
            .iter()
            .all(|e| e.name.to_lowercase().contains("fondo")
                || e.name.to_lowercase().contains("sportive")
                || e.name.to_lowercase().contains("century")));
    }

    #[test]
    fn test_filter_by_event_type_all() {
        let mut events = vec![
            create_test_event("Race", "CYCLING", "RACE"),
            create_test_event("Group", "CYCLING", "GROUP_RIDE"),
            create_test_event("TT", "CYCLING", "TIME_TRIAL"),
        ];

        let filtered = filter_by_event_type(&mut events, "all");
        assert_eq!(filtered, 0);
        assert_eq!(events.len(), 3);
    }

    #[test]
    fn test_filter_by_event_type_unknown() {
        let mut events = vec![create_test_event("Race", "CYCLING", "RACE")];

        // Unknown type should keep all events
        let filtered = filter_by_event_type(&mut events, "unknown_type");
        assert_eq!(filtered, 0);
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_filter_by_tags() {
        let mut events = vec![
            {
                let mut e = create_test_event("Race 1", "CYCLING", "RACE");
                e.tags = vec!["climbing".to_string(), "hilly".to_string()];
                e
            },
            {
                let mut e = create_test_event("Race 2", "CYCLING", "RACE");
                e.tags = vec!["flat".to_string(), "sprint".to_string()];
                e
            },
            {
                let mut e = create_test_event("Race 3", "CYCLING", "RACE");
                e.tags = vec!["climbing".to_string(), "mountain".to_string()];
                e
            },
        ];

        let tags = vec!["climbing".to_string()];
        let filtered = filter_by_tags(&mut events, &tags);

        assert_eq!(filtered, 1); // One without climbing tag
        assert_eq!(events.len(), 2);
        assert!(events
            .iter()
            .all(|e| e.tags.iter().any(|t| t.contains("climbing"))));
    }

    #[test]
    fn test_filter_by_tags_empty() {
        let mut events = vec![create_test_event("Race 1", "CYCLING", "RACE")];

        let tags: Vec<String> = vec![];
        let filtered = filter_by_tags(&mut events, &tags);

        assert_eq!(filtered, 0);
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_filter_by_excluded_tags() {
        let mut events = vec![
            {
                let mut e = create_test_event("Race 1", "CYCLING", "RACE");
                e.tags = vec!["climbing".to_string(), "hilly".to_string()];
                e
            },
            {
                let mut e = create_test_event("Race 2", "CYCLING", "RACE");
                e.tags = vec!["flat".to_string(), "sprint".to_string()];
                e
            },
            {
                let mut e = create_test_event("Race 3", "CYCLING", "RACE");
                e.tags = vec!["climbing".to_string(), "mountain".to_string()];
                e
            },
        ];

        let exclude_tags = vec!["climbing".to_string()];
        let filtered = filter_by_excluded_tags(&mut events, &exclude_tags);

        assert_eq!(filtered, 2); // Two with climbing tag
        assert_eq!(events.len(), 1);
        assert!(events
            .iter()
            .all(|e| !e.tags.iter().any(|t| t.contains("climbing"))));
    }

    #[test]
    fn test_filter_by_excluded_tags_empty() {
        let mut events = vec![create_test_event("Race 1", "CYCLING", "RACE")];

        let exclude_tags: Vec<String> = vec![];
        let filtered = filter_by_excluded_tags(&mut events, &exclude_tags);

        assert_eq!(filtered, 0);
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_event_matches_duration_fixed_minutes() {
        let mut event = create_test_event("Race", "CYCLING", "RACE");
        event.duration_in_minutes = Some(60);

        assert!(event_matches_duration(&event, 60, 10, 200));
        assert!(event_matches_duration(&event, 55, 10, 200));
        assert!(event_matches_duration(&event, 65, 10, 200));
        assert!(!event_matches_duration(&event, 75, 10, 200));
        assert!(!event_matches_duration(&event, 45, 10, 200));
    }

    #[test]
    fn test_event_matches_duration_fixed_seconds() {
        let mut event = create_test_event("Race", "CYCLING", "RACE");
        event.duration_in_seconds = Some(3600); // 60 minutes

        assert!(event_matches_duration(&event, 60, 10, 200));
        assert!(event_matches_duration(&event, 55, 10, 200));
        assert!(event_matches_duration(&event, 65, 10, 200));
        assert!(!event_matches_duration(&event, 75, 10, 200));
    }

    #[test]
    fn test_event_matches_duration_with_route() {
        let mut event = create_test_event("Race", "CYCLING", "RACE");
        event.route_id = Some(1258415487); // Bell Lap

        // With route_id, it should estimate duration
        // This may fail if database is not available, so we just check it doesn't panic
        let _ = event_matches_duration(&event, 30, 10, 200);
    }

    #[test]
    fn test_event_matches_duration_zero_duration() {
        let mut event = create_test_event("Race", "CYCLING", "RACE");
        event.duration_in_minutes = Some(0);
        event.duration_in_seconds = Some(0);

        // Zero durations are filtered out
        assert!(!event_matches_duration(&event, 60, 10, 200));
    }

    #[test]
    fn test_event_matches_duration_minutes_takes_precedence() {
        let mut event = create_test_event("Race", "CYCLING", "RACE");
        event.duration_in_minutes = Some(60);
        event.duration_in_seconds = Some(7200); // 120 minutes

        // Minutes takes precedence over seconds
        assert!(event_matches_duration(&event, 60, 10, 200));
        assert!(!event_matches_duration(&event, 120, 10, 200));
    }


    #[test]
    fn test_event_matches_duration_comparison_operators() {
        // Test <= comparison in event_matches_duration
        // Mutations: replace <= with <, <=  with >, <= with ==

        // Test edge case: duration exactly at tolerance boundary
        let mut event = create_test_event("Edge Case Race", "CYCLING", "RACE");
        event.distance_in_meters = Some(15450.0); // Exactly 30 min for Cat D

        // With tolerance 0, only exact matches should pass
        assert!(event_matches_duration(&event, 30, 0, 195));

        // Test that <= includes the boundary
        assert!(event_matches_duration(&event, 30, 5, 195)); // Within tolerance
        assert!(event_matches_duration(&event, 25, 5, 195)); // Exactly at upper bound
        assert!(event_matches_duration(&event, 35, 5, 195)); // Exactly at lower bound

        // If <= became <, exact boundary would fail
        // If <= became >, opposite results
        // If <= became ==, only exact matches would pass
    }

    #[test]
    fn test_filter_stats_arithmetic_operations() {
        // Test addition in FilterStats::total_filtered
        // Mutation: replace + with -
        let stats = FilterStats {
            sport_filtered: 10,
            time_filtered: 5,
            type_filtered: 3,
            duration_filtered: 8,
            tag_filtered: 2,
            unknown_routes: 0,
            missing_distance: 0,
        };

        let total = stats.total_filtered();
        assert_eq!(total, 28); // 10 + 5 + 3 + 8 + 2 = 28

        // If + became -, we'd get negative or very different values
        assert!(total > 20);
        assert!(total < 40);
    }

    #[test]
    fn test_duration_no_match_arithmetic() {
        // Test addition in FilterStats::duration_no_match
        // Mutation: replace + with -
        let stats = FilterStats {
            sport_filtered: 0,
            time_filtered: 0,
            type_filtered: 0,
            duration_filtered: 8,
            tag_filtered: 0,
            unknown_routes: 4,
            missing_distance: 2,
        };

        let no_match = stats.duration_no_match();
        assert_eq!(no_match, 8); // duration_no_match only returns duration_filtered

        // Test that it equals duration_filtered
        assert_eq!(no_match, stats.duration_filtered);
    }

    #[test]
    fn test_is_racing_score_event_conditions() {
        use crate::models::{is_racing_score_event, EventSubGroup};

        // Test various conditions that identify racing score events
        let mut event = create_test_event("Test Race", "CYCLING", "RACE");

        // Not a racing score event by default
        assert!(!is_racing_score_event(&event));

        // Add racing score subgroup
        event.event_sub_groups = vec![EventSubGroup {
            id: 1,
            name: "0-199".to_string(),
            route_id: None,
            distance_in_meters: Some(0.0),
            duration_in_minutes: None,
            category_enforcement: None,
            range_access_label: Some("0-199".to_string()),
            laps: None,
        }];

        assert!(is_racing_score_event(&event));

        // Test with different range labels
        event.event_sub_groups[0].range_access_label = Some("200-299".to_string());
        assert!(is_racing_score_event(&event));

        event.event_sub_groups[0].range_access_label = None;
        assert!(!is_racing_score_event(&event));
    }

    #[test]
    fn test_filter_by_tags_inclusion() {
        // Test tag filtering logic
        let mut events = vec![
            {
                let mut e = create_test_event("Tagged Race", "CYCLING", "RACE");
                e.tags = vec!["3R".to_string(), "flat".to_string()];
                e
            },
            {
                let mut e = create_test_event("Untagged Race", "CYCLING", "RACE");
                e.tags = vec![];
                e
            },
            {
                let mut e = create_test_event("Different Tags", "CYCLING", "RACE");
                e.tags = vec!["hilly".to_string(), "ranked".to_string()];
                e
            },
        ];

        let tags = vec!["3R".to_string()];
        let filtered = filter_by_tags(&mut events, &tags);

        assert_eq!(filtered, 2); // Should filter out 2 events
        assert_eq!(events.len(), 1); // Only one event with "3R" tag remains
        assert_eq!(events[0].name, "Tagged Race");
    }

    #[test]
    fn test_filter_by_excluded_tags_comprehensive() {
        // Test exclude tag filtering
        let mut events = vec![
            {
                let mut e = create_test_event("Women's Race", "CYCLING", "RACE");
                e.tags = vec!["women_only".to_string()];
                e
            },
            {
                let mut e = create_test_event("Open Race", "CYCLING", "RACE");
                e.tags = vec!["3R".to_string()];
                e
            },
            {
                let mut e = create_test_event("Mixed Tags", "CYCLING", "RACE");
                e.tags = vec!["3R".to_string(), "women_only".to_string()];
                e
            },
        ];

        let exclude = vec!["women_only".to_string()];
        let filtered = filter_by_excluded_tags(&mut events, &exclude);

        assert_eq!(filtered, 2); // Should filter out 2 events with "women_only"
        assert_eq!(events.len(), 1); // Only "Open Race" remains
        assert_eq!(events[0].name, "Open Race");
    }

    #[test]
    fn test_event_matches_duration_exact_tolerance_boundary() {
        // Exercises the <= boundary in tolerance check.
        // With duration=60, target=50, tolerance=10: diff=10, should match (<=10).
        // With duration=60, target=49, tolerance=10: diff=11, should NOT match (>10).
        let mut event = create_test_event("Boundary Race", "CYCLING", "RACE");
        event.duration_in_minutes = Some(60);

        // Diff exactly equal to tolerance — must match (<=)
        assert!(
            event_matches_duration(&event, 50, 10, 200),
            "diff=10, tolerance=10: should match (<=)"
        );
        assert!(
            event_matches_duration(&event, 70, 10, 200),
            "diff=10, tolerance=10: should match (<=)"
        );

        // Diff one beyond tolerance — must NOT match
        assert!(
            !event_matches_duration(&event, 49, 10, 200),
            "diff=11, tolerance=10: should not match"
        );
        assert!(
            !event_matches_duration(&event, 71, 10, 200),
            "diff=11, tolerance=10: should not match"
        );

        // Zero tolerance — only exact match
        assert!(
            event_matches_duration(&event, 60, 0, 200),
            "diff=0, tolerance=0: exact match"
        );
        assert!(
            !event_matches_duration(&event, 61, 0, 200),
            "diff=1, tolerance=0: should not match"
        );
    }
}
