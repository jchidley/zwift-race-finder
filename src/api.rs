use anyhow::Result;

use zwift_race_finder::errors::*;
use zwift_race_finder::models::*;

pub async fn fetch_events() -> Result<Vec<ZwiftEvent>> {
    // API has a hard limit of 200 events (about 12 hours worth)
    // The API ignores pagination parameters (limit/offset) and date filters
    // This is a Zwift API limitation, not a bug in this tool
    let url = "https://us-or-rly101.zwift.com/api/public/events/upcoming";

    let client = reqwest::Client::builder()
        .user_agent("Zwift Race Finder")
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let response = match client
        .get(url)
        .header("Content-Type", "application/json")
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            let err = anyhow::Error::from(e);
            api_connection_error(&err).display();
            return Err(err);
        }
    };

    // Check for rate limiting
    if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
        api_rate_limit().display();
        return Err(anyhow::anyhow!("API rate limit exceeded"));
    }

    // Check for other HTTP errors
    if !response.status().is_success() {
        let status = response.status();
        let error = UserError::new(
            format!("Zwift API returned error: {}", status),
            format!(
                "HTTP {}: {}",
                status.as_u16(),
                status.canonical_reason().unwrap_or("Unknown error")
            ),
        )
        .with_suggestion("The API might be temporarily unavailable")
        .with_suggestion("Try again in a few minutes");
        error.display();
        return Err(anyhow::anyhow!("API returned status: {}", status));
    }

    let events: Vec<ZwiftEvent> = response.json().await.map_err(|e| {
        UserError::new(
            "Failed to parse Zwift API response",
            "The API returned data in an unexpected format",
        )
        .with_suggestion("This might indicate an API change")
        .with_suggestion(format!("Technical details: {}", e))
        .display();
        e
    })?;

    Ok(events)
}
