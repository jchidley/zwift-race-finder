//! Test utilities module for unit testing

use crate::models::{EventSubGroup, ZwiftEvent};
use chrono::{Duration, Utc};

/// Helper to create a test event
pub fn create_test_event(
    name: &str,
    event_type: &str,
    distance: Option<f64>,
    route_id: Option<u32>,
) -> ZwiftEvent {
    ZwiftEvent {
        id: 1,
        name: name.to_string(),
        event_start: Utc::now() + Duration::hours(1),
        event_type: event_type.to_string(),
        distance_in_meters: distance,
        duration_in_minutes: None,
        duration_in_seconds: None,
        route_id,
        route: None,
        description: None,
        category_enforcement: false,
        event_sub_groups: vec![],
        sport: "CYCLING".to_string(),
        tags: vec![],
    }
}

/// Helper to create a racing score event
pub fn create_racing_score_event(name: &str, description: &str) -> ZwiftEvent {
    let mut event = create_test_event(name, "RACE", Some(0.0), Some(1_234_567_890));
    event.description = Some(description.to_string());
    event.event_sub_groups = vec![EventSubGroup {
        id: 1,
        name: "Score 0-650".to_string(),
        route_id: event.route_id,
        distance_in_meters: Some(0.0),
        duration_in_minutes: None,
        category_enforcement: Some(true),
        range_access_label: Some("0-650".to_string()),
        laps: None,
    }];
    event
}
