//! Enhanced error handling with user-friendly messages
//!
//! This module provides better error messages and guidance for common failure scenarios.

use colored::Colorize;

/// User-friendly error messages for common scenarios
pub struct UserError {
    pub title: String,
    pub details: String,
    pub suggestions: Vec<String>,
}

impl UserError {
    /// Create a new user error with suggestions
    pub fn new(title: impl Into<String>, details: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            details: details.into(),
            suggestions: Vec::new(),
        }
    }

    /// Add a suggestion to help the user
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }

    /// Display the error with formatting
    pub fn display(&self) {
        eprintln!("\n{} {}", "❌ Error:".red().bold(), self.title.red());
        if !self.details.is_empty() {
            eprintln!("   {}", self.details);
        }

        if !self.suggestions.is_empty() {
            eprintln!("\n{}", "💡 Suggestions:".yellow());
            for suggestion in &self.suggestions {
                eprintln!("   • {}", suggestion);
            }
        }
        eprintln!();
    }
}

/// Common error scenarios with helpful messages
pub fn database_connection_error(path: &str, err: &anyhow::Error) -> UserError {
    UserError::new(
        "Failed to connect to race database",
        format!("Could not open database at: {}", path),
    )
    .with_suggestion("Check if the database file exists and is readable")
    .with_suggestion(format!(
        "Try creating the directory: mkdir -p $(dirname {})",
        path
    ))
    .with_suggestion(format!("Technical details: {}", err))
}

pub fn api_connection_error(err: &anyhow::Error) -> UserError {
    UserError::new(
        "Failed to connect to Zwift API",
        "Could not fetch upcoming events from Zwift",
    )
    .with_suggestion("Check your internet connection")
    .with_suggestion("The Zwift API might be temporarily unavailable - try again in a few minutes")
    .with_suggestion(format!("Technical details: {}", err))
}

pub fn no_stats_available() -> UserError {
    UserError::new(
        "No Zwift Racing Score configured",
        "Unable to determine your Zwift Racing Score automatically",
    )
    .with_suggestion("Provide your score manually: cargo run -- --zwift-score YOUR_SCORE")
    .with_suggestion("Configure ZwiftPower credentials in config.toml for automatic detection")
    .with_suggestion("See config.example.toml for setup instructions")
}

pub fn invalid_zwift_score(score: u32) -> UserError {
    UserError::new(
        format!("Invalid Zwift Racing Score: {}", score),
        "Zwift Racing Score must be between 0 and 1000",
    )
    .with_suggestion("Check your score at: https://www.zwift.com/eu/profile")
    .with_suggestion("Typical ranges: E:0-99, D:100-199, C:200-299, B:300-399, A:400-599, A+:600+")
}

pub fn route_not_found(route_id: u32) -> UserError {
    UserError::new(
        format!("Unknown route ID: {}", route_id),
        "This route hasn't been mapped in our database yet",
    )
    .with_suggestion("This might be a new or special event route")
    .with_suggestion("The tool will estimate duration based on event name/description")
    .with_suggestion(format!(
        "Report this route at: https://github.com/jchidley/zwift-race-finder/issues"
    ))
}

pub fn config_file_error(path: &str, err: &anyhow::Error) -> UserError {
    UserError::new(
        "Failed to load configuration file",
        format!("Could not read config from: {}", path),
    )
    .with_suggestion("Copy config.example.toml to config.toml and edit it")
    .with_suggestion("Or use command line arguments to override settings")
    .with_suggestion(format!("Technical details: {}", err))
}

pub fn api_rate_limit() -> UserError {
    UserError::new(
        "Zwift API rate limit reached",
        "Too many requests to the Zwift API",
    )
    .with_suggestion("Wait a few minutes before trying again")
    .with_suggestion("The API has a limit to prevent overload")
}

pub fn zwiftpower_auth_error() -> UserError {
    UserError::new(
        "ZwiftPower authentication failed",
        "Could not log in to ZwiftPower to fetch your stats",
    )
    .with_suggestion("Check your ZwiftPower credentials in config.toml")
    .with_suggestion("Make sure your session ID is still valid")
    .with_suggestion(
        "You can get a new session ID by logging into zwiftpower.com and checking cookies",
    )
}

pub fn no_events_in_time_range(days: u8) -> UserError {
    UserError::new(
        format!("No events found in the next {} day(s)", days),
        "The Zwift API returned no upcoming events",
    )
    .with_suggestion("This is unusual - the API normally returns ~200 events")
    .with_suggestion("Try again in a few minutes as this might be temporary")
    .with_suggestion("Check if Zwift is undergoing maintenance")
}

pub fn parse_record_result_error() -> UserError {
    UserError::new(
        "Invalid race result format",
        "Could not parse the provided race result",
    )
    .with_suggestion("Use format: --record-result 'route_id,minutes,event_name'")
    .with_suggestion("Example: --record-result '9999,32,3R Volcano Flat Race'")
    .with_suggestion("Optional: add your zwift score as 4th parameter")
}

/// Helper to wrap anyhow errors with context
pub fn with_user_context<T>(result: anyhow::Result<T>, context: UserError) -> anyhow::Result<T> {
    result.map_err(|e| {
        context.display();
        e
    })
}

/// Format error chains nicely
pub fn format_error_chain(err: &anyhow::Error) -> String {
    let mut chain = Vec::new();
    chain.push(err.to_string());

    let mut current = err.source();
    while let Some(cause) = current {
        chain.push(format!("  ← {}", cause));
        current = cause.source();
    }

    chain.join("\n")
}
