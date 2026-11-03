use serde_json::Value;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().any(|a| a == "--help" || a == "-h") {
        println!("debug_tags — analyze event tag frequency from saved Zwift API data");
        println!("\nUsage: debug_tags");
        println!("\nReads debug_event_tags.json from the current directory.");
        println!("To create this file, save the output of the Zwift events API.");
        return Ok(());
    }

    let path = "debug_event_tags.json";
    if !std::path::Path::new(path).exists() {
        eprintln!("Error: {path} not found in current directory.");
        eprintln!("This tool reads saved Zwift API event data.");
        eprintln!("Run from the project root, or save API output to {path} first.");
        std::process::exit(1);
    }

    let json_str = std::fs::read_to_string(path)?;
    let events: Vec<Value> = serde_json::from_str(&json_str)?;

    let mut tag_frequency: HashMap<String, usize> = HashMap::new();
    let mut tag_patterns: HashMap<String, Vec<String>> = HashMap::new();

    for event in &events {
        if let Some(tags) = event["tags"].as_array() {
            for tag in tags {
                if let Some(tag_str) = tag.as_str() {
                    *tag_frequency.entry(tag_str.to_string()).or_insert(0) += 1;

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
