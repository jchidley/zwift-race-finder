//! Zwift Race Finder - Find Zwift races that match your target duration and racing score.
//!
//! This library provides functionality to:
//! - Fetch and filter Zwift events based on duration and fitness level
//! - Store and retrieve route data and race results
//! - Import route metadata from explicit installed-client exports
//! - Load configuration and environment-provided credentials

/// A/B testing framework for comparing implementations
pub mod ab_testing;

/// Public event API adapter.
pub mod api;

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

/// Source-qualified personal performance history.
pub mod history;

/// Score-independent live history estimation and its database adapter.
pub mod personal_estimation;

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




/// OCR constants
#[cfg(feature = "ocr")]
pub mod ocr_constants;

/// OCR configuration loading
#[cfg(feature = "ocr")]
pub mod ocr_config;

/// OCR image processing utilities
#[cfg(feature = "ocr")]
pub mod ocr_image_processing;

/// OCR regex patterns
#[cfg(feature = "ocr")]
pub mod ocr_regex;

/// Compact OCR implementation
#[cfg(feature = "ocr")]
pub mod ocr_compact;

#[cfg(feature = "ocr")]
pub mod ocr_ocrs;

/// Parallel OCR implementation for high performance
#[cfg(feature = "ocr")]
pub mod ocr_parallel;
