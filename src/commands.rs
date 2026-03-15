use anyhow::Result;
use chrono::Utc;
use colored::*;
use std::io::Write;

use crate::api::fetch_events;
use crate::database;
use crate::database::Database;
use crate::route_discovery;
use zwift_race_finder::constants::*;
use zwift_race_finder::estimation::*;
use zwift_race_finder::formatting::*;

pub fn show_unknown_routes() -> Result<()> {
    let db = Database::new()?;
    let routes = db.get_unknown_routes()?;

    if routes.is_empty() {
        println!("No unknown routes found. Great job mapping!");
    } else {
        println!("\n{}", "Unknown Routes (need mapping):".yellow().bold());
        println!("{}", "=".repeat(60));
        println!("{:<12} {:<8} {}", "Route ID", "Seen", "Event Name");
        println!("{}", "-".repeat(60));

        for (route_id, event_name, times_seen) in routes {
            println!(
                "{:<12} {:<8} {}",
                route_id.to_string().yellow(),
                times_seen,
                event_name
            );
        }

        println!(
            "\n{}: Research these routes on ZwiftHacks or Zwift Insider",
            "Tip".yellow()
        );
        println!("Then add them to the database with distance and elevation data.");
    }
    Ok(())
}

pub async fn discover_unknown_routes() -> Result<()> {
    let db = Database::new()?;
    let mut unknown = db.get_unknown_routes()?;

    if unknown.is_empty() {
        println!("No unknown routes to discover!");
        return Ok(());
    }

    // Sort by frequency (times_seen) to prioritize high-value routes
    unknown.sort_by(|a, b| b.2.cmp(&a.2));

    let total_count = unknown.len();
    println!(
        "🔍 Starting route discovery for {} unknown routes...",
        total_count
    );
    println!("📋 Prioritizing high-frequency events first\n");

    // Process in batches to avoid timeouts
    const BATCH_SIZE: usize = 20;
    const BATCH_TIMEOUT_MINS: u64 = 2;

    let discovery = route_discovery::RouteDiscovery::new()?;
    let mut total_discovered = 0;
    let mut total_failed = 0;
    let mut total_skipped = 0;

    // Process routes in batches
    for (batch_num, chunk) in unknown.chunks(BATCH_SIZE).enumerate() {
        let batch_start = std::time::Instant::now();
        println!(
            "\n📦 Batch {} of {} ({} routes):",
            batch_num + 1,
            (unknown.len() + BATCH_SIZE - 1) / BATCH_SIZE,
            chunk.len()
        );

        let mut batch_discovered = 0;
        let mut batch_failed = 0;

        for (route_id, event_name, times_seen) in chunk {
            // Check if we're approaching timeout
            if batch_start.elapsed().as_secs() > BATCH_TIMEOUT_MINS * 60 - 30 {
                println!("\n⏰ Approaching timeout limit, saving progress...");
                break;
            }

            // Check if we should attempt discovery (not tried recently)
            if !db.should_attempt_discovery(*route_id)? {
                println!("⏭️  Skipping {} (recently attempted)", event_name);
                total_skipped += 1;
                continue;
            }

            print!(
                "🔎 [{:3}x] Searching for '{}' (ID: {})... ",
                times_seen, event_name, route_id
            );
            std::io::stdout().flush()?;

            // Record the attempt
            db.record_discovery_attempt(*route_id, event_name)?;

            // Try to discover route data
            match discovery.discover_route(event_name).await {
                Ok(discovered) => {
                    // Use the discovered route_id if it's valid, otherwise use the original
                    let final_route_id = if discovered.route_id != 9999 {
                        discovered.route_id
                    } else {
                        *route_id
                    };

                    // Save to database
                    db.save_discovered_route(
                        final_route_id,
                        discovered.distance_km,
                        discovered.elevation_m,
                        &discovered.world,
                        &discovered.surface,
                        &discovered.name,
                    )?;

                    println!(
                        "✅ Found! {}km, {}m elevation, ID: {}",
                        discovered.distance_km, discovered.elevation_m, final_route_id
                    );
                    batch_discovered += 1;
                }
                Err(e) => {
                    println!("❌ Failed: {}", e);
                    batch_failed += 1;
                }
            }

            // Small delay to be polite to external services
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }

        total_discovered += batch_discovered;
        total_failed += batch_failed;

        // Batch summary
        println!(
            "\nBatch {} complete: {} found, {} failed",
            batch_num + 1,
            batch_discovered,
            batch_failed
        );

        // Check if we should continue
        if batch_start.elapsed().as_secs() > BATCH_TIMEOUT_MINS * 60 - 30 {
            println!(
                "\n⏰ Timeout reached. {} routes remaining for next run.",
                total_count - (batch_num + 1) * BATCH_SIZE
            );
            break;
        }

        // Pause between batches
        if batch_num + 1 < (unknown.len() + BATCH_SIZE - 1) / BATCH_SIZE {
            println!("\n⏸️  Pausing 5 seconds before next batch...");
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }

    println!("\n📊 Discovery Summary:");
    println!("  ✅ Successfully discovered: {}", total_discovered);
    println!("  ❌ Failed to discover: {}", total_failed);
    println!("  ⏭️  Skipped (recent attempts): {}", total_skipped);
    println!(
        "  ⏳ Remaining to process: {}",
        total_count.saturating_sub(total_discovered + total_failed + total_skipped)
    );

    if total_discovered > 0 {
        println!("\n💡 Tip: Run the tool normally to see the newly discovered routes in action!");
    }

    Ok(())
}

pub fn record_race_result(input: &str) -> Result<()> {
    // Parse format: "route_id,minutes,event_name[,zwift_score]"
    let parts: Vec<&str> = input.split(',').collect();
    if parts.len() < 3 {
        anyhow::bail!("Format: --record-result 'route_id,minutes,event_name[,zwift_score]'");
    }

    let route_id: u32 = parts[0]
        .trim()
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid route_id"))?;
    let minutes: u32 = parts[1]
        .trim()
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid minutes"))?;

    // Check if zwift_score is provided at position 3 (before event name)
    let (event_name, zwift_score_override) =
        if parts.len() >= 4 && parts[3].trim().parse::<u32>().is_ok() {
            // Format: route_id,minutes,event_name,zwift_score
            let event_name = parts[2].trim().to_string();
            let zwift_score = parts[3].trim().parse::<u32>().unwrap();
            (event_name, Some(zwift_score))
        } else {
            // Format: route_id,minutes,event_name (may contain commas)
            let event_name = parts[2..].join(",").trim().to_string();
            (event_name, None)
        };

    let db = Database::new()?;

    // Ensure route exists so race_result FK is satisfied
    if db.get_route(route_id)?.is_none() {
        println!(
            "{}: Route {} not found in database — creating stub entry",
            "Warning".yellow(),
            route_id
        );
        let stub = database::RouteData {
            route_id,
            distance_km: 0.0,
            elevation_m: 0,
            name: event_name.clone(),
            world: "Unknown".to_string(),
            surface: "road".to_string(),
            lead_in_distance_km: 0.0,
            lead_in_elevation_m: 0,
            lead_in_distance_free_ride_km: None,
            lead_in_elevation_free_ride_m: None,
            lead_in_distance_meetups_km: None,
            lead_in_elevation_meetups_m: None,
            slug: None,
        };
        db.add_route(&stub)?;
        db.record_unknown_route(route_id, &event_name, "RACE")?;
    }

    // Get zwift_score from override or default
    let zwift_score = zwift_score_override.unwrap_or(195);

    let result = database::RaceResult {
        id: None,
        route_id,
        event_name: event_name.clone(),
        actual_minutes: minutes,
        zwift_score,
        race_date: Utc::now().format("%Y-%m-%d").to_string(),
        notes: None,
    };

    db.add_race_result(&result)?;

    println!(
        "\n✅ {} recorded successfully!",
        "Race result".green().bold()
    );
    println!("  Route ID: {}", route_id);
    println!("  Event: {}", event_name);
    println!("  Time: {}", format_duration(minutes));
    println!("  Zwift Score: {}", zwift_score);

    // Show comparison with estimate if route is known
    if let Some(estimated) = estimate_duration_from_route_id(route_id, zwift_score) {
        let diff = (estimated as i32 - minutes as i32).abs();
        let accuracy = PERCENT_MULTIPLIER - (diff as f64 / minutes as f64 * PERCENT_MULTIPLIER);
        println!(
            "\n  Estimated: {} ({}% accurate)",
            format_duration(estimated),
            accuracy.round() as i32
        );
    }

    Ok(())
}

pub async fn analyze_event_descriptions() -> Result<()> {
    println!(
        "\n{}",
        "Analyzing Event Descriptions for Route Names..."
            .yellow()
            .bold()
    );

    // Fetch current events (reuse same endpoint as main event fetch)
    let events = fetch_events().await?;

    let mut route_patterns: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    let mut unknown_routes = 0;
    let mut parsed_count = 0;

    for event in &events {
        // Skip if we already know this route
        if let Some(route_id) = event.route_id {
            if get_route_data(route_id).is_some() {
                continue;
            }
        }

        unknown_routes += 1;

        // Try to parse description
        if let Some(description) = &event.description {
            if let Some(parsed) = route_discovery::parse_route_from_description(description) {
                parsed_count += 1;
                let key = format!("{} ({} laps)", parsed.route_name, parsed.laps);
                route_patterns
                    .entry(key)
                    .or_insert_with(Vec::new)
                    .push(event.name.clone());
            }
        }
    }

    println!(
        "\n{}: {} unknown routes, {} descriptions parsed",
        "Summary".bright_blue(),
        unknown_routes,
        parsed_count
    );

    if !route_patterns.is_empty() {
        println!("\n{}", "Discovered Route Patterns:".green().bold());
        println!("{}", "=".repeat(80));

        // Sort by frequency
        let mut patterns: Vec<_> = route_patterns.iter().collect();
        patterns.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

        for (route_info, events) in patterns.iter().take(20) {
            println!("\n{} ({} events)", route_info.yellow(), events.len());
            for event in events.iter().take(3) {
                println!("  - {}", event.dimmed());
            }
            if events.len() > 3 {
                println!("  ... and {} more", events.len() - 3);
            }
        }

        if patterns.len() > 20 {
            println!("\n... and {} more route patterns", patterns.len() - 20);
        }
    }

    println!(
        "\n{}: Use this information to create route mappings",
        "Next Step".yellow()
    );
    println!("Look up the actual route names on Zwift Insider or ZwiftHacks");

    Ok(())
}

pub fn mark_route_complete(route_id: u32) -> Result<()> {
    let db = Database::new()?;

    // Check if route exists
    if let Some(route) = db.get_route(route_id)? {
        // Mark as complete
        db.mark_route_complete(route_id, None, None)?;
        println!(
            "✅ Marked route {} ({}) as completed!",
            route.name, route.world
        );

        // Show updated progress
        let (completed, total) = db.get_completion_stats()?;
        println!(
            "Progress: {}/{} routes completed ({}%)",
            completed,
            total,
            (completed * PERCENT_MULTIPLIER as u32) / total
        );
    } else {
        eprintln!("Error: Route {} not found in database", route_id);
    }

    Ok(())
}

pub fn show_route_progress() -> Result<()> {
    let db = Database::new()?;

    // Overall stats
    let (completed, total) = db.get_completion_stats()?;
    let percentage = if total > 0 {
        (completed * PERCENT_MULTIPLIER as u32) / total
    } else {
        0
    };

    println!(
        "🏆 {} {}",
        "Route Completion Progress".bold(),
        format!("v0.1.0").dimmed()
    );
    println!();
    println!("Overall: {}/{} routes ({}%)", completed, total, percentage);

    // Progress bar
    let bar_width: usize = 30;
    let filled = bar_width * completed as usize / total.max(1) as usize;
    let bar = "█".repeat(filled) + &"░".repeat(bar_width - filled);
    println!("{}", bar.bright_green());
    println!();

    // World stats
    println!("By World:");
    let world_stats = db.get_world_completion_stats()?;
    for (world, world_completed, world_total) in world_stats {
        let world_percentage = if world_total > 0 {
            (world_completed * PERCENT_MULTIPLIER as u32) / world_total
        } else {
            0
        };
        let world_filled = 10 * world_completed as usize / world_total.max(1) as usize;
        let world_bar = "▓".repeat(world_filled) + &"░".repeat(10 - world_filled);
        println!(
            "  {:<15} {}/{} {} {}%",
            world, world_completed, world_total, world_bar, world_percentage
        );
    }

    Ok(())
}
