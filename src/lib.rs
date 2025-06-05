//! Zwift Race Finder - Find Zwift races that match your target duration and racing score.
//!
//! This library provides functionality to:
//! - Fetch and filter Zwift events based on duration and fitness level
//! - Store and retrieve route data and race results
//! - Discover route information from external sources
//! - Manage configuration and secure credential storage

/// Data models and structs
pub mod models;

/// Category-related utility functions
pub mod category;

/// Configuration management for the application
pub mod config;

/// Database operations for routes and race results
pub mod database;

/// Route discovery from external sources
pub mod route_discovery;

/// Secure storage for OAuth tokens and credentials
pub mod secure_storage;