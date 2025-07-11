//! Development tool: Analyze event tags from saved Zwift API data
//!
//! This is a development/debugging utility, not part of the main application.
//! It reads a debug JSON file (debug_event_tags.json) containing saved API responses
//! and analyzes tag patterns to help identify useful filtering strategies.
//!
//! The main program already includes tag filtering functionality (--tags, --exclude-tags).
//! This tool helps developers understand what tags are available and their frequency.
//!
//! Usage:
//! 1. Save API response to debug_event_tags.json
//! 2. cargo run --bin debug_tags
//!
//! This tool is intentionally excluded from test coverage as it's for
//! development analysis rather than production functionality.

use serde_json::Value;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the debug JSON file
    let json_str = std::fs::read_to_string("debug_event_tags.json")?;
    let events: Vec<Value> = serde_json::from_str(&json_str)?;

    // Analyze tags
    let mut tag_frequency: HashMap<String, usize> = HashMap::new();
    let mut tag_patterns: HashMap<String, Vec<String>> = HashMap::new();

    for event in &events {
        if let Some(tags) = event["tags"].as_array() {
            for tag in tags {
                if let Some(tag_str) = tag.as_str() {
                    // Count frequency
                    *tag_frequency.entry(tag_str.to_string()).or_insert(0) += 1;

                    // Extract patterns
                    if tag_str.contains('=') {
                        let parts: Vec<&str> = tag_str.splitn(2, '=').collect();
                        if parts.len() == 2 {
                            tag_patterns
                                .entry(parts[0].to_string())
                                .or_insert_with(Vec::new)
                                .push(parts[1].to_string());
                        }
                    }
                }
            }
        }
    }

    // Print analysis
    println!("=== Most Common Tags ===");
    let mut sorted_tags: Vec<_> = tag_frequency.iter().collect();
    sorted_tags.sort_by(|a, b| b.1.cmp(a.1));

    for (tag, count) in sorted_tags.iter().take(20) {
        println!("{}: {} occurrences", tag, count);
    }

    println!("\n=== Tag Patterns ===");
    for (pattern, values) in &tag_patterns {
        println!(
            "{}: {} unique values",
            pattern,
            values
                .iter()
                .collect::<std::collections::HashSet<_>>()
                .len()
        );
        if values.len() <= 5 {
            println!("  Values: {:?}", values);
        }
    }

    // Look for filtering opportunities
    println!("\n=== Useful Filtering Tags ===");
    println!("Race series tags:");
    for (tag, _) in &tag_frequency {
        if tag.contains("zracing")
            || tag.contains("zwiftepic")
            || tag.contains("critclub")
            || tag.contains("zwifttt")
            || tag.contains("3r")
            || tag.contains("evo")
        {
            println!("  - {}", tag);
        }
    }

    println!("\nSpecial event tags:");
    for (tag, _) in &tag_frequency {
        if tag.contains("ranked")
            || tag.contains("showplacements")
            || tag.contains("jerseyunlock")
            || tag.contains("completionprize")
            || tag.contains("communityevent")
        {
            println!("  - {}", tag);
        }
    }

    Ok(())
}
