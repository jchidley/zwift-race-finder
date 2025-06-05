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

