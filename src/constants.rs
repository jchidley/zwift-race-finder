//! Common constants used throughout the application
//!
//! This module centralizes magic numbers and constants to improve code clarity
//! and maintainability.

// Distance conversions
/// Meters in one kilometer
pub const METERS_PER_KILOMETER: f64 = 1000.0;

// Time conversions
/// Minutes in one hour
pub const MINUTES_PER_HOUR: u32 = 60;
/// Seconds in one minute
pub const SECONDS_PER_MINUTE: u32 = 60;

// Percentage calculations
/// Multiplier to convert decimal to percentage (e.g., 0.25 -> 25%)
pub const PERCENT_MULTIPLIER: f64 = 100.0;
