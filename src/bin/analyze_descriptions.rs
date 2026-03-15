//! Development tool: Analyze event descriptions to find distance/elevation patterns
//!
//! This is a development/debugging utility, not part of the main application.
//! It fetches recent events and extracts patterns from descriptions to help
//! understand how Zwift formats event data, particularly for Racing Score events
//! where distance is often embedded in the description text.
//!
//! Usage: cargo run --bin analyze_descriptions
//!
//! This tool is intentionally excluded from test coverage as it's for
//! development analysis rather than production functionality.

use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zwift_race_finder::constants::METERS_PER_KILOMETER;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ZwiftEvent {
    id: u64,
    name: String,
    description: Option<String>,
    event_type: String,
    distance_in_meters: Option<f64>,
    route_id: Option<u32>,
}

async fn fetch_events() -> Result<Vec<ZwiftEvent>> {
    let url = "https://us-or-rly101.zwift.com/api/public/events/upcoming";

    let client = reqwest::Client::builder()
        .user_agent("Zwift Race Finder")
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let response = client
        .get(url)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    let events: Vec<ZwiftEvent> = response.json().await?;
    Ok(events)
}

fn extract_distance_patterns(description: &str) -> Vec<String> {
    let mut patterns = Vec::new();

    // Common patterns to look for
    let patterns_to_check = vec![
        r"Distance:\s*(\d+(?:\.\d+)?)\s*(km|miles?|mi)",
        r"(\d+(?:\.\d+)?)\s*(km|miles?|mi)\s*\(",
        r"Route:\s*([^,\n]+)",
        r"Elevation:\s*(\d+(?:\.\d+)?)\s*(m|meters?|ft|feet)",
        r"(\d+(?:\.\d+)?)\s*(m|meters?|ft|feet)\s*elevation",
        r"(\d+)\s*laps?",
        r"Length:\s*(\d+(?:\.\d+)?)\s*(km|miles?|mi)",
    ];

    for pattern in patterns_to_check {
        let re = Regex::new(pattern).unwrap();
        if let Some(captures) = re.captures(description) {
            patterns.push(format!(
                "Pattern '{}' matched: {}",
                pattern,
                captures.get(0).unwrap().as_str()
            ));
        }
    }

    patterns
}

#[tokio::main]
async fn main() -> Result<()> {
    // Simple --help support
    if std::env::args().any(|a| a == "--help" || a == "-h") {
        println!("analyze_descriptions — fetch Zwift events and extract distance/elevation patterns from descriptions");
        println!("\nUsage: analyze_descriptions");
        println!("\nDevelopment tool for understanding Zwift event description formats.");
        return Ok(());
    }

    println!("Fetching recent Zwift events to analyze descriptions...\n");

    let events = fetch_events().await?;

    // Filter for events with descriptions
    let events_with_desc: Vec<_> = events.iter().filter(|e| e.description.is_some()).collect();

    println!(
        "Found {} events, {} with descriptions\n",
        events.len(),
        events_with_desc.len()
    );

    // Analyze different event types
    let mut by_type: HashMap<String, Vec<&ZwiftEvent>> = HashMap::new();
    for event in &events_with_desc {
        by_type
            .entry(event.event_type.clone())
            .or_default()
            .push(event);
    }

    println!("Events with descriptions by type:");
    for (event_type, events) in &by_type {
        println!("  {}: {} events", event_type, events.len());
    }
    println!();

    // Sample some descriptions from each type
    for (event_type, type_events) in by_type {
        println!("\n=== {} Events ===", event_type);

        // Take up to 5 examples
        for (i, event) in type_events.iter().take(5).enumerate() {
            if let Some(desc) = &event.description {
                println!(
                    "\n{}. {} (ID: {}, Route: {:?})",
                    i + 1,
                    event.name,
                    event.id,
                    event.route_id
                );

                // Show first 200 chars of description
                let preview = if desc.len() > 200 {
                    format!("{}...", &desc[..200])
                } else {
                    desc.clone()
                };
                println!("Description preview: {}", preview.replace('\n', " "));

                // Extract patterns
                let patterns = extract_distance_patterns(desc);
                if !patterns.is_empty() {
                    println!("Found patterns:");
                    for pattern in patterns {
                        println!("  - {}", pattern);
                    }
                } else {
                    println!("No distance/elevation patterns found");
                }

                // Show if API has distance
                if let Some(dist) = event.distance_in_meters {
                    println!("API distance: {:.1} km", dist / METERS_PER_KILOMETER);
                } else {
                    println!("API distance: None (0.0)");
                }
            }
        }
    }

    // Look for specific Racing Score events
    println!("\n\n=== Racing Score Events (with 0 distance in API) ===");
    let zero_distance_events: Vec<_> = events_with_desc
        .iter()
        .filter(|e| e.distance_in_meters.unwrap_or(0.0) == 0.0)
        .take(10)
        .collect();

    for event in zero_distance_events {
        if let Some(desc) = &event.description {
            println!("\n{} (Type: {})", event.name, event.event_type);
            let patterns = extract_distance_patterns(desc);
            if !patterns.is_empty() {
                for pattern in patterns {
                    println!("  - {}", pattern);
                }
            }
        }
    }

    Ok(())
}
