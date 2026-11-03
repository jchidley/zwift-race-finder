//! Event analysis utilities
//!
//! This module contains functions for analyzing Zwift events.

use crate::category::get_category_from_score;
use crate::models::{is_racing_score_event, EventSubGroup, ZwiftEvent};
use std::collections::HashMap;

/// Find the user's actual score band, or traditional category subgroup.
pub fn find_user_subgroup(event: &ZwiftEvent, zwift_score: u32) -> Option<&EventSubGroup> {
    if is_racing_score_event(event) {
        // API ranges are inclusive. Touching endpoints are ambiguous: retain
        // API order rather than guessing which band's geometry was intended.
        return event.event_sub_groups.iter().find(|sg| {
            sg.range_access_label
                .as_deref()
                .and_then(|label| label.split_once('-'))
                .and_then(|(low, high)| {
                    Some((
                        low.trim().parse::<u32>().ok()?,
                        high.trim().parse::<u32>().ok()?,
                    ))
                })
                .is_some_and(|(low, high)| low <= zwift_score && zwift_score <= high)
        });
    }

    let category = get_category_from_score(zwift_score);
    let matches = |sg: &&EventSubGroup, wanted: &str| {
        if let Some(label) = &sg.subgroup_label {
            return label == wanted;
        }
        // Match category tokens, never letters embedded in an event title.
        sg.name
            .split(|c: char| !c.is_ascii_alphanumeric() && c != '+' && c != '-')
            .any(|token| token == wanted)
    };
    let find = |wanted| event.event_sub_groups.iter().find(|sg| matches(sg, wanted));
    find(category)
        .or_else(|| (category == "A++").then(|| find("A")).flatten())
        // Preserve traditional D eligibility for E only after an exact D match.
        .or_else(|| (category == "D").then(|| find("E")).flatten())
}

/// Route identity for the selected subgroup, shared by estimates and metadata.
pub fn selected_route_id(event: &ZwiftEvent, zwift_score: u32) -> Option<u32> {
    let subgroup = find_user_subgroup(event, zwift_score);
    if is_racing_score_event(event) && subgroup.is_none() {
        return None;
    }
    subgroup.and_then(|sg| sg.route_id).or(event.route_id)
}

/// Count events by type for display
pub fn count_events_by_type(events: &[ZwiftEvent]) -> Vec<(String, usize)> {
    let mut event_counts = HashMap::new();
    for event in events {
        if event.sport.to_uppercase() == "CYCLING" {
            *event_counts.entry(event.event_type.clone()).or_insert(0) += 1;
        }
    }

    let mut counts: Vec<_> = event_counts.into_iter().collect();
    counts.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
    counts
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_count_events_by_type() {
        let events = vec![
            ZwiftEvent {
                id: 1,
                name: "Race 1".to_string(),
                event_start: Utc::now(),
                event_type: "RACE".to_string(),
                distance_in_meters: Some(20000.0),
                duration_in_minutes: None,
                duration_in_seconds: None,
                route_id: None,
                route: None,
                description: None,
                category_enforcement: false,
                event_sub_groups: vec![],
                sport: "CYCLING".to_string(),
                tags: vec![],
            },
            ZwiftEvent {
                id: 2,
                name: "Race 2".to_string(),
                event_start: Utc::now(),
                event_type: "RACE".to_string(),
                distance_in_meters: Some(20000.0),
                duration_in_minutes: None,
                duration_in_seconds: None,
                route_id: None,
                route: None,
                description: None,
                category_enforcement: false,
                event_sub_groups: vec![],
                sport: "CYCLING".to_string(),
                tags: vec![],
            },
            ZwiftEvent {
                id: 3,
                name: "Group Ride".to_string(),
                event_start: Utc::now(),
                event_type: "GROUP_RIDE".to_string(),
                distance_in_meters: Some(50000.0),
                duration_in_minutes: None,
                duration_in_seconds: None,
                route_id: None,
                route: None,
                description: None,
                category_enforcement: false,
                event_sub_groups: vec![],
                sport: "CYCLING".to_string(),
                tags: vec![],
            },
            ZwiftEvent {
                id: 4,
                name: "Running Event".to_string(),
                event_start: Utc::now(),
                event_type: "RACE".to_string(),
                distance_in_meters: Some(10000.0),
                duration_in_minutes: None,
                duration_in_seconds: None,
                route_id: None,
                route: None,
                description: None,
                category_enforcement: false,
                event_sub_groups: vec![],
                sport: "RUNNING".to_string(),
                tags: vec![],
            },
        ];

        let counts = count_events_by_type(&events);

        // Should have 2 races and 1 group ride (running excluded)
        assert_eq!(counts.len(), 2);
        assert_eq!(counts[0], ("RACE".to_string(), 2));
        assert_eq!(counts[1], ("GROUP_RIDE".to_string(), 1));
    }

    #[test]
    fn test_find_user_subgroup() {
        // Create test event with subgroups
        let mut event = ZwiftEvent {
            id: 1,
            name: "Test Race".to_string(),
            description: None,
            event_start: chrono::Utc::now(),
            event_type: "RACE".to_string(),
            distance_in_meters: Some(20000.0),
            duration_in_minutes: None,
            duration_in_seconds: None,
            route_id: Some(1),
            route: None,
            category_enforcement: false,
            sport: "CYCLING".to_string(),
            tags: vec![],
            event_sub_groups: vec![
                EventSubGroup {
                    id: 1,
                    name: "A".to_string(),
                    route_id: None,
                    distance_in_meters: Some(40000.0),
                    duration_in_minutes: None,
                    duration_in_seconds: None,
                    subgroup_label: None,
                    category_enforcement: None,
                    range_access_label: None,
                    laps: None,
                },
                EventSubGroup {
                    id: 2,
                    name: "B".to_string(),
                    route_id: None,
                    distance_in_meters: Some(30000.0),
                    duration_in_minutes: None,
                    duration_in_seconds: None,
                    subgroup_label: None,
                    category_enforcement: None,
                    range_access_label: None,
                    laps: None,
                },
                EventSubGroup {
                    id: 3,
                    name: "C".to_string(),
                    route_id: None,
                    distance_in_meters: Some(20000.0),
                    duration_in_minutes: None,
                    duration_in_seconds: None,
                    subgroup_label: None,
                    category_enforcement: None,
                    range_access_label: None,
                    laps: None,
                },
                EventSubGroup {
                    id: 4,
                    name: "D".to_string(),
                    route_id: None,
                    distance_in_meters: Some(15000.0),
                    duration_in_minutes: None,
                    duration_in_seconds: None,
                    subgroup_label: None,
                    category_enforcement: None,
                    range_access_label: None,
                    laps: None,
                },
                EventSubGroup {
                    id: 5,
                    name: "E".to_string(),
                    route_id: None,
                    distance_in_meters: Some(10000.0),
                    duration_in_minutes: None,
                    duration_in_seconds: None,
                    subgroup_label: None,
                    category_enforcement: None,
                    range_access_label: None,
                    laps: None,
                },
            ],
        };

        // Test Cat D rider
        let subgroup = find_user_subgroup(&event, 195).unwrap();
        assert_eq!(subgroup.name, "D");
        assert_eq!(subgroup.distance_in_meters, Some(15000.0));

        // Test Cat C rider
        let subgroup = find_user_subgroup(&event, 250).unwrap();
        assert_eq!(subgroup.name, "C");
        assert_eq!(subgroup.distance_in_meters, Some(20000.0));

        // Test Cat B rider
        let subgroup = find_user_subgroup(&event, 350).unwrap();
        assert_eq!(subgroup.name, "B");
        assert_eq!(subgroup.distance_in_meters, Some(30000.0));

        // Test Cat A rider
        let subgroup = find_user_subgroup(&event, 450).unwrap();
        assert_eq!(subgroup.name, "A");
        assert_eq!(subgroup.distance_in_meters, Some(40000.0));

        // Test Cat E rider
        let subgroup = find_user_subgroup(&event, 50).unwrap();
        assert_eq!(subgroup.name, "E");
        assert_eq!(subgroup.distance_in_meters, Some(10000.0));

        // Test event with no subgroups
        event.event_sub_groups.clear();
        assert!(find_user_subgroup(&event, 195).is_none());

        // Test category boundaries
        event.event_sub_groups = vec![
            EventSubGroup {
                id: 1,
                name: "D".to_string(),
                route_id: None,
                distance_in_meters: Some(15000.0),
                duration_in_minutes: None,
                duration_in_seconds: None,
                subgroup_label: None,
                category_enforcement: None,
                range_access_label: None,
                laps: None,
            },
            EventSubGroup {
                id: 2,
                name: "C".to_string(),
                route_id: None,
                distance_in_meters: Some(20000.0),
                duration_in_minutes: None,
                duration_in_seconds: None,
                subgroup_label: None,
                category_enforcement: None,
                range_access_label: None,
                laps: None,
            },
        ];

        // 199 should be D (boundary)
        let subgroup = find_user_subgroup(&event, 199).unwrap();
        assert_eq!(subgroup.name, "D");

        // 200 should be C (boundary)
        let subgroup = find_user_subgroup(&event, 200).unwrap();
        assert_eq!(subgroup.name, "C");
    }
}
