//! Event filtering logic extracted from main.rs
//! 
//! This module contains predicates and filtering functions for Zwift events

use chrono::{DateTime, Utc};
use crate::models::ZwiftEvent;
use crate::database::Database;

/// Statistics tracking what was filtered out
#[derive(Debug, Default)]
pub struct FilterStats {
    pub sport_filtered: u32,
    pub time_filtered: u32,
    pub type_filtered: u32,
    pub duration_filtered: u32,
    pub tag_filtered: u32,
    pub completed_routes_filtered: u32,
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
            + self.completed_routes_filtered
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
pub fn filter_by_time(events: &mut Vec<ZwiftEvent>, now: DateTime<Utc>, max_date: DateTime<Utc>) -> u32 {
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
        tags.iter().any(|tag| event.tags.iter().any(|etag| etag.contains(tag)))
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
        !exclude_tags.iter().any(|tag| event.tags.iter().any(|etag| etag.contains(tag)))
    });
    (pre_count - events.len()) as u32
}

/// Filter to show only new/uncompleted routes
pub fn filter_new_routes_only(events: &mut Vec<ZwiftEvent>) -> u32 {
    let db = Database::new().ok();
    if let Some(db) = db {
        let pre_count = events.len();
        events.retain(|event| {
            if let Some(route_id) = event.route_id {
                // Keep events with routes we haven't completed
                !db.is_route_completed(route_id).unwrap_or(false)
            } else {
                // Keep events without route IDs (they might be new)
                true
            }
        });
        (pre_count - events.len()) as u32
    } else {
        0
    }
}

/// Check if an event matches the duration criteria
/// NOTE: This is a simplified version. The full implementation with route-based
/// estimation is still in main.rs and will be migrated in a future refactoring.
pub fn event_matches_duration(event: &ZwiftEvent, target_duration: u32, tolerance: u32, _zwift_score: u32) -> bool {
    // Fixed duration event
    let duration_minutes = event
        .duration_in_minutes
        .filter(|&d| d > 0)
        .or_else(|| event.duration_in_seconds.map(|s| s / 60).filter(|&d| d > 0));

    if let Some(duration) = duration_minutes {
        let diff = (duration as i32 - target_duration as i32).abs();
        return diff <= tolerance as i32;
    }
    
    // For now, return false for events without fixed duration
    // The full route-based estimation logic remains in main.rs
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use crate::models::ZwiftEvent;

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
        assert!(events.iter().all(|e| 
            e.name.to_lowercase().contains("fondo") ||
            e.name.to_lowercase().contains("sportive") ||
            e.name.to_lowercase().contains("century")
        ));
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
        let mut events = vec![
            create_test_event("Race", "CYCLING", "RACE"),
        ];
        
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
        assert!(events.iter().all(|e| 
            e.tags.iter().any(|t| t.contains("climbing"))
        ));
    }

    #[test]
    fn test_filter_by_tags_empty() {
        let mut events = vec![
            create_test_event("Race 1", "CYCLING", "RACE"),
        ];
        
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
        assert!(events.iter().all(|e| 
            !e.tags.iter().any(|t| t.contains("climbing"))
        ));
    }

    #[test]
    fn test_filter_by_excluded_tags_empty() {
        let mut events = vec![
            create_test_event("Race 1", "CYCLING", "RACE"),
        ];
        
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
    fn test_event_matches_duration_no_fixed() {
        let event = create_test_event("Race", "CYCLING", "RACE");
        
        // Without fixed duration, simplified version returns false
        assert!(!event_matches_duration(&event, 60, 10, 200));
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
}