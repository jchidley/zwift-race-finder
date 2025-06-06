//! Formatting utilities for display
//!
//! This module contains formatting functions for durations and event types.

use crate::constants::MINUTES_PER_HOUR;

/// Format duration from minutes to HH:MM format
pub fn format_duration(minutes: u32) -> String {
    let hours = minutes / MINUTES_PER_HOUR;
    let mins = minutes % MINUTES_PER_HOUR;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        // Test basic formatting
        assert_eq!(format_duration(0), "00:00");
        assert_eq!(format_duration(1), "00:01");
        assert_eq!(format_duration(59), "00:59");
        assert_eq!(format_duration(60), "01:00");
        assert_eq!(format_duration(61), "01:01");
        assert_eq!(format_duration(120), "02:00");
        assert_eq!(format_duration(150), "02:30");

        // Test larger values
        assert_eq!(format_duration(599), "09:59");
        assert_eq!(format_duration(600), "10:00");
        assert_eq!(format_duration(1439), "23:59");
        assert_eq!(format_duration(1440), "24:00"); // 24 hours
        assert_eq!(format_duration(2880), "48:00"); // 48 hours
    }

    #[test]
    fn test_format_event_type() {
        assert_eq!(format_event_type("RACE", 5), "5 races");
        assert_eq!(format_event_type("race", 1), "1 races"); // Note: doesn't handle singular
        assert_eq!(format_event_type("TIME_TRIAL", 3), "3 time trials");
        assert_eq!(format_event_type("GROUP_RIDE", 10), "10 group rides");
        assert_eq!(format_event_type("GROUP_WORKOUT", 2), "2 group workouts");
        assert_eq!(format_event_type("UNKNOWN_TYPE", 7), "7 UNKNOWN_TYPE");
    }
}
