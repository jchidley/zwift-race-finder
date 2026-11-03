use colored::Colorize;

pub struct UserError {
    pub title: String,
    pub details: String,
    pub suggestions: Vec<String>,
}

impl UserError {
    pub fn new(title: impl Into<String>, details: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            details: details.into(),
            suggestions: Vec::new(),
        }
    }

    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }

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


pub fn api_connection_error(err: &anyhow::Error) -> UserError {
    UserError::new(
        "Failed to connect to Zwift API",
        "Could not fetch upcoming events from Zwift",
    )
    .with_suggestion("Check your internet connection")
    .with_suggestion("The Zwift API might be temporarily unavailable - try again in a few minutes")
    .with_suggestion(format!("Technical details: {}", err))
}


pub fn invalid_zwift_score(score: u32) -> UserError {
    UserError::new(
        format!("Invalid Zwift Racing Score: {}", score),
        "Zwift Racing Score must be between 0 and 1000",
    )
    .with_suggestion("Check your score at: https://www.zwift.com/eu/profile")
    .with_suggestion("Typical ranges: E:0-99, D:100-199, C:200-299, B:300-399, A:400-599, A+:600+")
}



pub fn api_rate_limit() -> UserError {
    UserError::new(
        "Zwift API rate limit reached",
        "Too many requests to the Zwift API",
    )
    .with_suggestion("Wait a few minutes before trying again")
    .with_suggestion("The API has a limit to prevent overload")
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
