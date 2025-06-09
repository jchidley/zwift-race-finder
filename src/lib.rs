//! Zwift Race Finder - Find Zwift races that match your target duration and racing score.
//!
//! This library provides functionality to:
//! - Fetch and filter Zwift events based on duration and fitness level
//! - Store and retrieve route data and race results
//! - Discover route information from external sources
//! - Manage configuration and secure credential storage

/// A/B testing framework for comparing implementations
pub mod ab_testing;

/// Compatibility tracking for behavioral preservation
pub mod compatibility;

/// Common constants used throughout the application
pub mod constants;

/// Data models and structs
pub mod models;

/// Category-related utility functions
pub mod category;

/// Parsing utilities
pub mod parsing;

/// Cache functionality
pub mod cache;

/// Configuration management for the application
pub mod config;

/// Database operations for routes and race results
pub mod database;

/// Duration estimation utilities
pub mod duration_estimation;

/// Enhanced error handling with user-friendly messages
pub mod errors;

/// Route and duration estimation functions
pub mod estimation;

/// Event analysis utilities
pub mod event_analysis;

/// Event display functionality
pub mod event_display;

/// Event filtering logic
pub mod event_filtering;

/// Formatting utilities for display
pub mod formatting;

/// Regression testing utilities
#[cfg(test)]
pub mod regression_test;

/// Route discovery from external sources
pub mod route_discovery;

/// Secure storage for OAuth tokens and credentials
pub mod secure_storage;

/// Test utilities (only available in test builds)
#[cfg(test)]
pub mod test_utils;

/// Compact OCR implementation
#[cfg(feature = "ocr")]
pub mod ocr_compact;

/// OCR implementation using ocrs library
#[cfg(feature = "ocr")]
pub mod ocr_ocrs;
