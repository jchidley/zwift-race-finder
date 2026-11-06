use crate::database::Database;
use crate::models::ZwiftEvent;
use chrono::{DateTime, Utc};

#[derive(Debug, Default)]
pub struct FilterStats {
    pub sport_filtered: u32,
    pub time_filtered: u32,
    pub type_filtered: u32,
    pub duration_filtered: u32,
    pub duration_unknown: u32,
    pub subgroup_filtered: u32,
    pub tag_filtered: u32,
}

impl FilterStats {
    pub fn total_filtered(&self) -> u32 {
        self.sport_filtered
            + self.time_filtered
            + self.type_filtered
            + self.duration_filtered
            + self.duration_unknown
            + self.subgroup_filtered
            + self.tag_filtered
    }
}

pub fn filter_by_sport(events: &mut Vec<ZwiftEvent>) -> u32 {
    let pre_count = events.len();
    events.retain(|event| event.sport.to_uppercase() == "CYCLING");
    (pre_count - events.len()) as u32
}

pub fn filter_by_time(
    events: &mut Vec<ZwiftEvent>,
    now: DateTime<Utc>,
    max_date: DateTime<Utc>,
) -> u32 {
    let pre_count = events.len();
    events.retain(|event| event.event_start > now && event.event_start < max_date);
    (pre_count - events.len()) as u32
}

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

#[derive(Debug, Clone)]
pub struct EventEstimate {
    pub duration_minutes: f64,
    pub distance_km: Option<f64>,
    pub elevation_m: Option<u32>,
    pub is_fixed: bool,
    pub elevation_is_estimate: bool,
    pub personal: Option<crate::personal_estimation::PersonalEstimate>,
}

impl EventEstimate {
    #[must_use]
    pub fn matches_duration(&self, target_minutes: u32, tolerance_minutes: u32) -> bool {
        (self.duration_minutes - f64::from(target_minutes)).abs() <= f64::from(tolerance_minutes)
    }

    fn resolved(duration_minutes: f64, geometry: Option<&EventGeometry>, is_fixed: bool) -> Self {
        let mut estimate = geometry.map_or(
            Self {
                duration_minutes,
                distance_km: None,
                elevation_m: None,
                is_fixed,
                elevation_is_estimate: false,
                personal: None,
            },
            |geometry| {
                let estimate =
                    Self::estimated(duration_minutes, geometry.distance_km, geometry.route_id);
                match (geometry.route_id, geometry.laps) {
                    (Some(route_id), Some(laps)) => estimate.with_laps(route_id, laps),
                    _ => estimate,
                }
            },
        );
        estimate.is_fixed = is_fixed;
        estimate
    }

    fn estimated(duration_minutes: f64, distance_km: f64, route_id: Option<u32>) -> Self {
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
            personal: None,
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

struct EventGeometry {
    distance_km: f64,
    route_id: Option<u32>,
    laps: Option<u32>,
}

fn positive_distance(distance: Option<f64>) -> Option<f64> {
    distance.filter(|d| d.is_finite() && *d > 0.0)
}

pub(crate) fn fixed_minutes(minutes: Option<u32>, seconds: Option<u32>) -> Option<f64> {
    minutes
        .filter(|&minutes| minutes > 0)
        .map(f64::from)
        .or_else(|| {
            seconds
                .filter(|&seconds| seconds > 0)
                .map(|seconds| f64::from(seconds) / 60.0)
        })
}

fn resolve_geometry(
    event: &ZwiftEvent,
    subgroup: Option<&crate::models::EventSubGroup>,
    route_id: Option<u32>,
) -> Option<EventGeometry> {
    use crate::constants::METERS_PER_KILOMETER;
    use crate::estimation::get_route_data;
    use crate::models::is_racing_score_event;
    use crate::parsing::{estimate_distance_from_name, parse_distance_from_description};

    let geometry = |distance_km, laps| EventGeometry {
        distance_km,
        route_id,
        laps,
    };
    let lap_distance = |route: &crate::models::RouteData, laps: u32| {
        let lead = route.lead_in_distance_km?;
        if !lead.is_finite() || lead < 0.0 {
            return None;
        }
        positive_distance(Some(route.distance_km))?;
        positive_distance(Some(route.distance_km * f64::from(laps) + lead))
    };

    if let Some(distance) = positive_distance(subgroup.and_then(|sg| sg.distance_in_meters)) {
        return Some(geometry(distance / METERS_PER_KILOMETER, None));
    }
    if let Some(laps) = subgroup.and_then(|sg| sg.laps).filter(|&laps| laps > 0) {
        if let Some(distance) = route_id
            .and_then(get_route_data)
            .and_then(|route| lap_distance(&route, laps))
        {
            return Some(geometry(distance, Some(laps)));
        }
    }
    if let Some(distance) = positive_distance(event.distance_in_meters) {
        return Some(geometry(distance / METERS_PER_KILOMETER, None));
    }
    if let Some(route) = route_id.and_then(get_route_data) {
        let laps = Database::new()
            .ok()
            .and_then(|db| db.get_multi_lap_info(&event.name).ok().flatten())
            .filter(|&laps| laps > 0)
            .unwrap_or(1);
        if let Some(distance) = lap_distance(&route, laps) {
            return Some(geometry(distance, Some(laps)));
        }
    }
    if is_racing_score_event(event) {
        if let Some(distance) =
            positive_distance(parse_distance_from_description(&event.description))
        {
            return Some(geometry(distance, None));
        }
    }
    positive_distance(estimate_distance_from_name(&event.name))
        .map(|distance_km| geometry(distance_km, None))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EstimateUnavailable {
    NoEligibleSubgroup,
    MissingGeometry,
    Personal(crate::personal_estimation::PersonalFallback),
}

impl std::fmt::Display for EstimateUnavailable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoEligibleSubgroup => f.write_str("no eligible subgroup"),
            Self::MissingGeometry => f.write_str("missing event distance"),
            Self::Personal(reason) => std::fmt::Display::fmt(reason, f),
        }
    }
}

pub fn resolve_event(
    event: &ZwiftEvent,
    zwift_score: u32,
) -> Result<EventEstimate, EstimateUnavailable> {
    use crate::event_analysis::find_user_subgroup;
    use crate::models::is_racing_score_event;
    use crate::personal_estimation::{predict_from_database, PersonalFallback, PersonalTarget};

    let subgroup = find_user_subgroup(event, zwift_score);
    if is_racing_score_event(event) && subgroup.is_none() {
        return Err(EstimateUnavailable::NoEligibleSubgroup);
    }
    let geometry = resolve_geometry(
        event,
        subgroup,
        crate::event_analysis::selected_route_id(event, zwift_score),
    );
    let fixed = fixed_minutes(event.duration_in_minutes, event.duration_in_seconds).or_else(|| {
        let sg = subgroup?;
        fixed_minutes(sg.duration_in_minutes, sg.duration_in_seconds)
    });
    if let Some(minutes) = fixed {
        return Ok(EventEstimate::resolved(minutes, geometry.as_ref(), true));
    }
    let geometry = geometry.ok_or(EstimateUnavailable::MissingGeometry)?;
    if event.event_type != "RACE" {
        return Err(EstimateUnavailable::Personal(
            PersonalFallback::UnsupportedFormat,
        ));
    }
    let personal = Database::new()
        .map_err(|error| PersonalFallback::HistoryUnavailable(error.to_string()))
        .and_then(|db| {
            predict_from_database(
                &db,
                PersonalTarget {
                    event_day: event.event_start.date_naive(),
                    route_id: geometry.route_id,
                    distance_km: geometry.distance_km,
                    ascent_m: None,
                },
                geometry.laps,
                Utc::now().date_naive(),
            )
        })
        .map_err(EstimateUnavailable::Personal)?;
    let mut estimate = EventEstimate::resolved(personal.minutes, Some(&geometry), false);
    estimate.personal = Some(personal);
    Ok(estimate)
}

pub fn estimate_event(event: &ZwiftEvent, zwift_score: u32) -> Option<EventEstimate> {
    resolve_event(event, zwift_score).ok()
}

pub fn event_matches_duration(
    event: &ZwiftEvent,
    target_duration: u32,
    tolerance: u32,
    zwift_score: u32,
) -> bool {
    estimate_event(event, zwift_score)
        .is_some_and(|estimate| estimate.matches_duration(target_duration, tolerance))
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
    }

    #[test]
    fn test_filter_by_sport() {
        let mut events = vec![
            create_test_event("Cycling Race", "CYCLING", "RACE"),
            create_test_event("Running Event", "RUNNING", "RACE"),
            create_test_event("Cycling TT", "cycling", "TIME_TRIAL"),
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

        assert_eq!(filtered, 2);
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
        assert_eq!(filtered, 3);
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
        assert_eq!(filtered, 2);
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

        assert_eq!(filtered, 1);
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

        assert_eq!(filtered, 2);
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
        event.duration_in_seconds = Some(3600);

        assert!(event_matches_duration(&event, 60, 10, 200));
        assert!(event_matches_duration(&event, 55, 10, 200));
        assert!(event_matches_duration(&event, 65, 10, 200));
        assert!(!event_matches_duration(&event, 75, 10, 200));
    }

    #[test]
    fn test_event_matches_duration_with_route() {
        let mut event = create_test_event("Race", "CYCLING", "RACE");
        event.route_id = Some(1258415487);

        let _ = event_matches_duration(&event, 30, 10, 200);
    }

    #[test]
    fn test_event_matches_duration_zero_duration() {
        let mut event = create_test_event("Race", "CYCLING", "RACE");
        event.duration_in_minutes = Some(0);
        event.duration_in_seconds = Some(0);

        assert!(!event_matches_duration(&event, 60, 10, 200));
    }

    #[test]
    fn test_event_matches_duration_minutes_takes_precedence() {
        let mut event = create_test_event("Race", "CYCLING", "RACE");
        event.duration_in_minutes = Some(60);
        event.duration_in_seconds = Some(7200);

        assert!(event_matches_duration(&event, 60, 10, 200));
        assert!(!event_matches_duration(&event, 120, 10, 200));
    }

    #[test]
    fn test_event_matches_duration_comparison_operators() {

        let mut event = create_test_event("Edge Case Race", "CYCLING", "RACE");
        event.duration_in_minutes = Some(30);

        assert!(event_matches_duration(&event, 30, 0, 195));

        assert!(event_matches_duration(&event, 30, 5, 195));
        assert!(event_matches_duration(&event, 25, 5, 195));
        assert!(event_matches_duration(&event, 35, 5, 195));

    }

    #[test]
    fn test_filter_stats_arithmetic_operations() {
        let stats = FilterStats {
            sport_filtered: 10,
            time_filtered: 5,
            type_filtered: 3,
            duration_filtered: 8,
            duration_unknown: 0,
            subgroup_filtered: 0,
            tag_filtered: 2,
        };

        let total = stats.total_filtered();
        assert_eq!(total, 28);

        assert!(total > 20);
        assert!(total < 40);
    }

    #[test]
    fn test_is_racing_score_event_conditions() {
        use crate::models::{is_racing_score_event, EventSubGroup};

        let mut event = create_test_event("Test Race", "CYCLING", "RACE");

        assert!(!is_racing_score_event(&event));

        event.event_sub_groups = vec![EventSubGroup {
            id: 1,
            name: "0-199".to_string(),
            route_id: None,
            distance_in_meters: Some(0.0),
            duration_in_minutes: None,
            duration_in_seconds: None,
            subgroup_label: None,
            category_enforcement: None,
            range_access_label: Some("0-199".to_string()),
            laps: None,
        }];

        assert!(is_racing_score_event(&event));

        event.event_sub_groups[0].range_access_label = Some("200-299".to_string());
        assert!(is_racing_score_event(&event));

        event.event_sub_groups[0].range_access_label = None;
        assert!(!is_racing_score_event(&event));
    }

    #[test]
    fn test_filter_by_tags_inclusion() {
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

        assert_eq!(filtered, 2);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name, "Tagged Race");
    }

    #[test]
    fn test_filter_by_excluded_tags_comprehensive() {
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

        assert_eq!(filtered, 2);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name, "Open Race");
    }

    #[test]
    fn test_event_matches_duration_exact_tolerance_boundary() {
        let mut event = create_test_event("Boundary Race", "CYCLING", "RACE");
        event.duration_in_minutes = Some(60);

        assert!(
            event_matches_duration(&event, 50, 10, 200),
            "diff=10, tolerance=10: should match (<=)"
        );
        assert!(
            event_matches_duration(&event, 70, 10, 200),
            "diff=10, tolerance=10: should match (<=)"
        );

        assert!(
            !event_matches_duration(&event, 49, 10, 200),
            "diff=11, tolerance=10: should not match"
        );
        assert!(
            !event_matches_duration(&event, 71, 10, 200),
            "diff=11, tolerance=10: should not match"
        );

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
