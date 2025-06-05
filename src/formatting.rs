//! Formatting utilities for display
//! 
//! This module contains formatting functions for durations and event types.

/// Format duration from minutes to HH:MM format
pub fn format_duration(minutes: u32) -> String {
    let hours = minutes / 60;
    let mins = minutes % 60;
    format!("{:02}:{:02}", hours, mins)
}

/// Format event type for display with proper pluralization
pub fn format_event_type(event_type: &str, count: usize) -> String {
    let readable_type = match event_type.to_lowercase().as_str() {
        "race" => "races",
        "time_trial" => "time trials",
        "group_ride" => "group rides",
        "group_workout" => "group workouts",
        _ => event_type,
    };
    format!("{} {}", count, readable_type)
}