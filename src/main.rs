//! Zwift Race Finder - Find races that match your target duration and racing score
//! 
//! This tool fetches upcoming Zwift events and filters them based on estimated
//! completion time for your specific Zwift Racing Score.

// ABOUTME: Tool to find Zwift races suitable for Cat C riders (~180 ZwiftScore) lasting ~2 hours
// Fetches events from Zwift API and filters based on race duration estimates

mod config;
mod database;
mod route_discovery;
#[cfg(test)]
mod regression_test;

use anyhow::Result;
use chrono::{DateTime, Local, Utc};
use clap::Parser;
use colored::*;
use config::{FullConfig, Secrets};
use database::{Database, RouteData as DbRouteData};
use regex::Regex;
use std::io::Write;
use zwift_race_finder::models::*;
use zwift_race_finder::category::*;
use zwift_race_finder::parsing::*;
use zwift_race_finder::cache::*;
use zwift_race_finder::errors::*;
use zwift_race_finder::event_analysis::*;
use zwift_race_finder::formatting::*;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Target Zwift racing score (auto-detected from ZwiftPower if available)
    #[arg(short = 's', long)]
    zwift_score: Option<u32>,

    /// Target race duration in minutes (default: 120)
    #[arg(short = 'd', long, default_value = "120")]
    duration: u32,

    /// Duration tolerance in minutes (default: 30 for 1.5-2.5h range)
    #[arg(short = 't', long, default_value = "30")]
    tolerance: u32,

    /// Event type filter: all, race, fondo, group, workout, tt (time trial)
    #[arg(short = 'e', long, default_value = "race")]
    event_type: String,

    /// Show next N days of events (default: 1)
    #[arg(short = 'n', long, default_value = "1")]
    days: u32,

    /// ZwiftPower username (optional, for auto-fetching stats)
    #[arg(long)]
    zwiftpower_username: Option<String>,

    /// Debug mode - show why events are filtered out
    #[arg(long)]
    debug: bool,
    
    /// Show unknown routes that need mapping
    #[arg(long)]
    show_unknown_routes: bool,
    
    /// Analyze event descriptions to find route names
    #[arg(long)]
    analyze_descriptions: bool,
    
    /// Record a race result (format: "route_id,minutes,event_name[,zwift_score]")
    #[arg(long)]
    record_result: Option<String>,
    
    /// Discover unknown routes from web sources
    #[arg(long)]
    discover_routes: bool,
    
    /// Filter by event tags (e.g., "ranked", "zracing", "jerseyunlock")
    #[arg(long, value_delimiter = ',')]
    tags: Vec<String>,
    
    /// Exclude events with specific tags
    #[arg(long, value_delimiter = ',')]
    exclude_tags: Vec<String>,
    
    /// Mark a route as completed (by route ID)
    #[arg(long)]
    mark_complete: Option<u32>,
    
    /// Show route completion progress
    #[arg(long)]
    show_progress: bool,
    
    /// Only show events with routes you haven't completed
    #[arg(long)]
    new_routes_only: bool,
    
    /// Use verbose output format (default: compact table)
    #[arg(short = 'v', long)]
    verbose: bool,
}







// Zwift route database - route_id is the primary key for all calculations
// This should be expanded with Jack's actual race data

// Get route data from database
fn get_route_data_from_db(route_id: u32) -> Option<DbRouteData> {
    if let Ok(db) = Database::new() {
        db.get_route(route_id).ok().flatten()
    } else {
        None
    }
}

// Common Zwift route data indexed by route_id (fallback for when DB is not available)
fn get_route_data(route_id: u32) -> Option<RouteData> {
    // First try database
    if let Some(db_route) = get_route_data_from_db(route_id) {
        return Some(RouteData {
            distance_km: db_route.distance_km,
            elevation_m: db_route.elevation_m,
            name: &"",  // We don't use name in calculations
            world: &"",  // We don't use world in calculations
            surface: match db_route.surface.as_str() {
                "road" => "road",
                "gravel" => "gravel",
                "mixed" => "mixed",
                _ => "road",
            },
            lead_in_distance_km: db_route.lead_in_distance_km,
        });
    }
    
    // Fallback to hardcoded data
    match route_id {
        // Women's races - typically shorter criteriums
        1258415487 => Some(RouteData {
            distance_km: 14.1,
            elevation_m: 59,
            name: "Bell Lap",
            world: "Crit City",
            surface: "road",
            lead_in_distance_km: 0.5,  // Default lead-in for crit races
        }),
        
        // Common race routes
        2143464829 => Some(RouteData {
            distance_km: 33.4,
            elevation_m: 170,
            name: "Watopia Flat Route",
            world: "Watopia",
            surface: "road",
            lead_in_distance_km: 0.3,
        }),
        
        2927651296 => Some(RouteData {
            distance_km: 67.5,
            elevation_m: 654,
            name: "Makuri Pretzel",
            world: "Makuri Islands",
            surface: "road",
            lead_in_distance_km: 2.0,
        }),
        
        3742187716 => Some(RouteData {
            distance_km: 24.5,
            elevation_m: 168,
            name: "Castle to Castle",
            world: "Makuri Islands",
            surface: "road",
            lead_in_distance_km: 2.0,
        }),
        
        // Crit Racing Club routes
        2698009951 => Some(RouteData {
            distance_km: 22.9,
            elevation_m: 80,
            name: "Downtown Dolphin",
            world: "Crit City",
            surface: "road",
            lead_in_distance_km: 0.5,
        }),
        
        // Mt. Fuji Hill Climb
        2663908549 => Some(RouteData {
            distance_km: 20.3,
            elevation_m: 1159,
            name: "Mt. Fuji",
            world: "Makuri Islands",
            surface: "road",
            lead_in_distance_km: 1.0,
        }),
        
        // Common race routes discovered from API
        3368626651 => Some(RouteData {
            distance_km: 27.4,  // Estimated from typical eRacing events
            elevation_m: 223,
            name: "eRacing Course",
            world: "Various",
            surface: "road",
            lead_in_distance_km: 0.3,
        }),
        
        1656629976 => Some(RouteData {
            distance_km: 19.8,  // Ottawa TopSpeed typically shorter
            elevation_m: 142,
            name: "Ottawa TopSpeed",
            world: "Various",
            surface: "road",
            lead_in_distance_km: 0.3,
        }),
        
        2474227587 => Some(RouteData {
            distance_km: 100.0,  // KISS Racing 100 - it's in the name!
            elevation_m: 892,
            name: "KISS 100",
            world: "Watopia",
            surface: "road",
            lead_in_distance_km: 0.5,
        }),
        
        3395698268 => Some(RouteData {
            distance_km: 60.0,  // NoPinz R3R - 60km Race
            elevation_m: 543,
            name: "R3R 60km",
            world: "Various",
            surface: "road",
            lead_in_distance_km: 0.5,
        }),
        
        // Add more routes as we discover them
        _ => None,
    }
}

// Get just the distance for backward compatibility



// Generate no results suggestions based on search criteria
fn generate_no_results_suggestions(args: &Args) -> Vec<String> {
    let mut suggestions = Vec::new();
    
    if args.event_type == "race" {
        suggestions.push("Most races are short (20-30 minutes). Try:".to_string());
        suggestions.push(format!("  • {} for short races", "cargo run -- -d 30 -t 30".cyan()));
        suggestions.push(format!("  • {} for any race duration", "cargo run -- -d 60 -t 120".cyan()));
        suggestions.push(format!("  • {} for time trials instead", "cargo run -- -e tt".cyan()));
        suggestions.push("".to_string());
        suggestions.push("Common race durations:".to_string());
        suggestions.push("  • Crit races: 15-25 minutes".to_string());
        suggestions.push("  • Short courses: 25-35 minutes".to_string());
        suggestions.push("  • Endurance races: 60-90 minutes".to_string());
    } else if args.event_type == "tt" || args.event_type == "time_trial" {
        suggestions.push("Time trials are less common. Try:".to_string());
        suggestions.push(format!("  • {} for regular races", "cargo run -- -e race -d 30 -t 30".cyan()));
        suggestions.push(format!("  • {} for all event types", "cargo run -- -e all".cyan()));
        suggestions.push("".to_string());
        suggestions.push("Note: Time trials are usually scheduled events, not always available.".to_string());
    } else if args.event_type == "group" {
        suggestions.push("Group rides vary widely in duration. Try:".to_string());
        suggestions.push(format!("  • {} for social rides", "cargo run -- -e group -d 60 -t 30".cyan()));
        suggestions.push(format!("  • {} for endurance rides", "cargo run -- -e group -d 120 -t 60".cyan()));
    } else {
        suggestions.push("No events match your duration criteria. Try:".to_string());
        suggestions.push(format!("  • {} for wider search", format!("cargo run -- -t {}", args.tolerance * 2).cyan()));
        suggestions.push(format!("  • {} for all event types", "cargo run -- -e all".cyan()));
        suggestions.push(format!("  • {} to see more days", "cargo run -- -n 3".cyan()));
    }
    
    // Add API limitation note if searching multiple days
    if args.days > 1 {
        suggestions.push("".to_string());
        suggestions.push("⚠️  Note: The Zwift API only returns ~12 hours of events regardless of days requested.".to_string());
    }
    
    suggestions
}


// Try to determine distance from event name patterns
fn estimate_distance_from_name(name: &str) -> Option<f64> {
    // First try to parse explicit distance from name
    if let Some(distance) = parse_distance_from_name(name) {
        return Some(distance);
    }
    
    let name_lower = name.to_lowercase();

    // Common race name patterns with typical distances
    if name_lower.contains("3r") && name_lower.contains("flat") {
        Some(33.4) // 3R races on flat routes
    } else if name_lower.contains("epic") && name_lower.contains("pretzel") {
        Some(67.5) // Epic races on Pretzel routes
    } else if name_lower.contains("crit") {
        Some(20.0) // Criterium races are typically short
    } else if name_lower.contains("gran fondo") {
        Some(92.6) // Gran Fondo distance
    } else if name_lower.contains("century") {
        Some(160.0) // Century rides
    } else {
        None
    }
}

// Calculate difficulty multiplier based on elevation gain per km
fn get_route_difficulty_multiplier_from_elevation(distance_km: f64, elevation_m: u32) -> f64 {
    let meters_per_km = elevation_m as f64 / distance_km;
    
    match meters_per_km {
        m if m < 5.0 => 1.1,   // Very flat (like Tempus Fugit)
        m if m < 10.0 => 1.0,  // Flat to rolling
        m if m < 20.0 => 0.9,  // Rolling hills
        m if m < 40.0 => 0.8,  // Hilly
        _ => 0.7,              // Very hilly (like Mt. Fuji or Alpe)
    }
}

// Route difficulty multipliers (some routes are hillier)
fn get_route_difficulty_multiplier(route_name: &str) -> f64 {
    let route_lower = route_name.to_lowercase();

    if route_lower.contains("alpe") || route_lower.contains("ventoux") {
        0.7 // Very hilly, slower
    } else if route_lower.contains("epic") || route_lower.contains("mountain") {
        0.8 // Hilly
    } else if route_lower.contains("flat") || route_lower.contains("tempus") {
        1.1 // Flat, faster
    } else {
        1.0 // Default
    }
}

// Primary duration estimation - uses route_id when available
fn estimate_duration_from_route_id(route_id: u32, zwift_score: u32) -> Option<u32> {
    let route_data = get_route_data(route_id)?;
    // For races, include lead-in distance
    let total_distance = route_data.distance_km + route_data.lead_in_distance_km;
    estimate_duration_with_distance(route_id, total_distance, zwift_score)
}

// Duration estimation with explicit distance (for multi-lap races)
fn estimate_duration_with_distance(route_id: u32, distance_km: f64, zwift_score: u32) -> Option<u32> {
    let route_data = get_route_data(route_id)?;
    
    // Try pack-based calculation if rider stats are available
    if let Ok(db) = Database::new() {
        if let Ok(Some(rider_stats)) = db.get_rider_stats() {
            // We don't need complex physics for pack racing
            // Power matters less than staying with the group
            
            // In Zwift races, you ride at pack speed which is fairly consistent
            // The key insight: draft is so powerful that individual power matters less
            // than staying with the group. Pack speed is determined by the strongest riders.
            
            let category = get_category_from_score(zwift_score);
            let base_pack_speed = get_category_speed(category);
            
            // Elevation still matters - packs slow on climbs, speed up on descents
            // But the effect is less dramatic than solo riding due to draft
            let elevation_factor = get_route_difficulty_multiplier_from_elevation(
                route_data.distance_km,
                route_data.elevation_m
            );
            
            // Surface penalties still apply but are reduced in a pack
            let surface_factor = match route_data.surface {
                "road" => 1.0,
                "gravel" => 0.92,  // Only 8% slower (vs 15% solo)
                "mixed" => 0.96,   // Only 4% slower (vs 8% solo)
                _ => 1.0,
            };
            
            // Weight affects climbing ability - heavier riders struggle more on hills
            let weight_factor = if route_data.elevation_m > 500 {
                // On hilly routes, lighter is better
                (75.0 / rider_stats.weight_kg).powf(0.15).min(1.1)
            } else {
                1.0  // Weight doesn't matter much on flats due to draft
            };
            
            let speed_kmh = base_pack_speed * elevation_factor * surface_factor * weight_factor;
            
            let duration_hours = distance_km / speed_kmh;
            return Some((duration_hours * 60.0) as u32);
        }
    }
    
    // Fallback to category-based estimation
    let category = get_category_from_score(zwift_score);
    let base_speed = get_category_speed(category);
    
    // Use elevation-based multiplier for more accurate estimates
    // Use base route distance for elevation calculation, not total distance
    let difficulty_multiplier = get_route_difficulty_multiplier_from_elevation(
        route_data.distance_km,
        route_data.elevation_m
    );
    
    // Apply surface penalty for non-road surfaces
    let surface_multiplier = match route_data.surface {
        "road" => 1.0,
        "gravel" => 0.85, // 15% slower on gravel
        "mixed" => 0.92,  // 8% slower on mixed surfaces
        _ => 1.0,
    };
    
    let effective_speed = base_speed * difficulty_multiplier * surface_multiplier;
    let duration_hours = distance_km / effective_speed;  // Use actual distance
    
    Some((duration_hours * 60.0) as u32)
}

// Fallback duration estimation when route_id is not available
fn estimate_duration_for_category(distance_km: f64, route_name: &str, zwift_score: u32) -> u32 {
    // Get category-based speed
    let category = get_category_from_score(zwift_score);
    let base_speed = get_category_speed(category);

    let difficulty_multiplier = get_route_difficulty_multiplier(route_name);
    let effective_speed = base_speed * difficulty_multiplier;

    let duration_hours = distance_km / effective_speed;
    (duration_hours * 60.0) as u32
}


/// Generate a descriptive filter summary based on active filters
fn generate_filter_description(args: &Args, min_duration: u32, max_duration: u32) -> String {
    let mut parts = Vec::new();
    
    // Always show event type (even if it's the default "race")
    let event_type_desc = match args.event_type.to_lowercase().as_str() {
        "race" => "races",
        "tt" | "time_trial" => "time trials",
        "workout" => "group workouts",
        "group" => "group rides",
        "fondo" => "fondos/sportives",
        "all" => "all events",
        _ => &args.event_type,
    };
    parts.push(event_type_desc.to_string());
    
    // Duration filter (always shown)
    parts.push(format!("{}-{} min", min_duration, max_duration));
    
    // Time range (show if not default 1 day, or always for clarity)
    if args.days == 1 {
        parts.push("next 24h".to_string());
    } else {
        parts.push(format!("next {} days", args.days));
    }
    
    // Tag filters
    if !args.tags.is_empty() {
        let tags_str = args.tags.join(", ");
        parts.push(format!("with tags: {}", tags_str));
    }
    
    // Exclude tags
    if !args.exclude_tags.is_empty() {
        let exclude_str = args.exclude_tags.join(", ");
        parts.push(format!("excluding: {}", exclude_str));
    }
    
    // New routes only
    if args.new_routes_only {
        parts.push("new routes only".to_string());
    }
    
    parts.join(" | ")
}

async fn fetch_zwiftpower_stats(secrets: &Secrets) -> Result<Option<UserStats>> {
    // Only try to fetch if we have profile ID configured
    let (profile_id, session_id) = match (&secrets.zwiftpower_profile_id, &secrets.zwiftpower_session_id) {
        (Some(pid), Some(sid)) => (pid, sid),
        (Some(pid), None) => {
            // Try without session ID (might work for public profiles)
            let url = format!("https://zwiftpower.com/profile.php?z={}", pid);
            eprintln!("Note: No session ID configured, trying public profile access...");
            return fetch_zwiftpower_public(&url).await;
        }
        _ => return Ok(None), // No profile ID configured
    };

    let url = format!("https://zwiftpower.com/profile.php?z={}&sid={}", profile_id, session_id);

    let client = reqwest::Client::builder()
        .user_agent("Zwift Race Finder")
        .build()?;

    let response = client.get(&url).send().await;

    match response {
        Ok(resp) if resp.status().is_success() => {
            let html = resp.text().await?;

            // Parse Zwift Racing Score from the HTML
            let score_regex = Regex::new(r"Zwift Racing Score.*?(\d+)").unwrap();
            let category_regex = Regex::new(r"Category:\s*([ABCD])").unwrap();

            if let (Some(score_match), Some(cat_match)) =
                (score_regex.captures(&html), category_regex.captures(&html))
            {
                let zwift_score: u32 = score_match[1].parse().unwrap_or(195);
                let category = cat_match[1].to_string();

                return Ok(Some(UserStats {
                    zwift_score,
                    category,
                    username: "User".to_string(),
                }));
            }
        }
        _ => {
            // If we can't fetch from ZwiftPower, return None to use defaults
            return Ok(None);
        }
    }

    Ok(None)
}

async fn fetch_zwiftpower_public(url: &str) -> Result<Option<UserStats>> {
    // Simplified version for public profile access
    let client = reqwest::Client::builder()
        .user_agent("Zwift Race Finder")
        .build()?;

    match client.get(url).send().await {
        Ok(resp) if resp.status().is_success() => {
            // Try to parse what we can from public page
            Ok(None) // Public pages might be limited
        }
        _ => Ok(None),
    }
}



async fn get_user_stats(config: &FullConfig) -> Result<UserStats> {
    // Try to load from cache first
    if let Ok(Some(stats)) = load_cached_stats() {
        return Ok(stats);
    }

    // Try to fetch from ZwiftPower
    if let Ok(Some(stats)) = fetch_zwiftpower_stats(&config.secrets).await {
        // Cache the fetched stats
        let _ = save_cached_stats(&stats);
        return Ok(stats);
    }

    // Use configured defaults or fallback
    let zwift_score = config.default_zwift_score().unwrap_or(195);
    Ok(UserStats {
        zwift_score,
        category: config.default_category().cloned().unwrap_or_else(|| get_category_from_score(zwift_score).to_string()),
        username: "User".to_string(),
    })
}

async fn fetch_events() -> Result<Vec<ZwiftEvent>> {
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
            format!("HTTP {}: {}", status.as_u16(), status.canonical_reason().unwrap_or("Unknown error"))
        )
        .with_suggestion("The API might be temporarily unavailable")
        .with_suggestion("Try again in a few minutes");
        error.display();
        return Err(anyhow::anyhow!("API returned status: {}", status));
    }

    let events: Vec<ZwiftEvent> = response.json().await
        .map_err(|e| {
            UserError::new(
                "Failed to parse Zwift API response",
                "The API returned data in an unexpected format"
            )
            .with_suggestion("This might indicate an API change")
            .with_suggestion(format!("Technical details: {}", e))
            .display();
            e
        })?;
    
    Ok(events)
}

fn filter_events(mut events: Vec<ZwiftEvent>, args: &Args, zwift_score: u32) -> (Vec<ZwiftEvent>, FilterStats) {
    let now = Utc::now();
    let max_date = now + chrono::Duration::days(args.days as i64);
    let mut stats = FilterStats::default();
    let _initial_count = events.len();

    if args.debug {
        eprintln!("Debug: Starting with {} events", events.len());
    }

    // Sport filter
    let pre_sport = events.len();
    events.retain(|event| event.sport.to_uppercase() == "CYCLING");
    stats.sport_filtered = (pre_sport - events.len()) as u32;
    if args.debug {
        eprintln!("Debug: {} events after sport filter", events.len());
    }

    // Time filter
    let pre_time = events.len();
    events.retain(|event| event.event_start > now && event.event_start < max_date);
    stats.time_filtered = (pre_time - events.len()) as u32;
    if args.debug {
        eprintln!("Debug: {} events after time filter", events.len());
    }

    // Event type filter
    let pre_type = events.len();
    events.retain(|event| match args.event_type.to_lowercase().as_str() {
        "all" => true,
        "race" => event.event_type == "RACE",
        "tt" | "time_trial" => event.event_type == "TIME_TRIAL",
        "workout" => event.event_type == "GROUP_WORKOUT",
        "group" => {
            event.event_type == "GROUP_RIDE"
                && !event.name.to_lowercase().contains("fondo")
                && !event.name.to_lowercase().contains("sportive")
        }
        "fondo" => {
            event.event_type == "GROUP_RIDE"
                && (event.name.to_lowercase().contains("fondo")
                    || event.name.to_lowercase().contains("sportive")
                    || event.name.to_lowercase().contains("century"))
        }
        _ => {
            eprintln!(
                "Warning: Unknown event type '{}', showing all events",
                args.event_type
            );
            true
        }
    });
    stats.type_filtered = (pre_type - events.len()) as u32;
    if args.debug {
        eprintln!("Debug: {} events after event type filter", events.len());
    }

    // Tag filtering
    if !args.tags.is_empty() {
        let pre_tag = events.len();
        events.retain(|event| {
            args.tags.iter().any(|tag| event.tags.iter().any(|etag| etag.contains(tag)))
        });
        stats.tag_filtered += (pre_tag - events.len()) as u32;
        if args.debug {
            eprintln!("Debug: {} events after tag filter", events.len());
        }
    }
    
    // Exclude tags filtering
    if !args.exclude_tags.is_empty() {
        let pre_tag = events.len();
        events.retain(|event| {
            !args.exclude_tags.iter().any(|tag| event.tags.iter().any(|etag| etag.contains(tag)))
        });
        stats.tag_filtered += (pre_tag - events.len()) as u32;
        if args.debug {
            eprintln!("Debug: {} events after exclude tag filter", events.len());
        }
    }
    
    // New routes only filter
    if args.new_routes_only {
        let db = Database::new().ok();
        if let Some(db) = db {
            let pre_routes = events.len();
            events.retain(|event| {
                if let Some(route_id) = event.route_id {
                    // Keep events with routes we haven't completed
                    !db.is_route_completed(route_id).unwrap_or(false)
                } else {
                    // Keep events without route IDs (they might be new)
                    true
                }
            });
            stats.completed_routes_filtered = (pre_routes - events.len()) as u32;
            if args.debug {
                eprintln!("Debug: {} events after new routes filter", events.len());
            }
        }
    }

    // Duration filter
    let pre_duration = events.len();
    events.retain(|event| {
        // Duration filter - prioritize route_id for accuracy
        let duration_minutes = event
            .duration_in_minutes
            .filter(|&d| d > 0) // Ignore 0-minute durations
            .or_else(|| event.duration_in_seconds.map(|s| s / 60).filter(|&d| d > 0));

        if let Some(duration) = duration_minutes {
            // Fixed duration event
            let diff = (duration as i32 - args.duration as i32).abs();
            return diff <= args.tolerance as i32;
        }
        
        // PRIMARY METHOD: Use route_id with actual distance for most accurate estimation
        if let Some(route_id) = event.route_id {
            // Check if user's subgroup has specific info (multi-lap races)
            let user_subgroup = find_user_subgroup(event, zwift_score);
            
            // First check if subgroup has laps info (for Racing Score events)
            if let Some(sg) = user_subgroup {
                if let Some(laps) = sg.laps {
                    // We have lap count - calculate based on route distance * laps + lead-in
                    if let Some(route_data) = get_route_data(route_id) {
                        let total_distance_km = route_data.lead_in_distance_km + (route_data.distance_km * laps as f64);
                        if let Some(estimated_duration) = estimate_duration_with_distance(route_id, total_distance_km, zwift_score) {
                            let diff = (estimated_duration as i32 - args.duration as i32).abs();
                            return diff <= args.tolerance as i32;
                        }
                    }
                }
            }
            
            // Try to get distance from subgroup or event
            let distance_meters = user_subgroup
                .and_then(|sg| sg.distance_in_meters)
                .or(event.distance_in_meters);
                
            if let Some(dist_m) = distance_meters.filter(|&d| d > 0.0) {
                let distance_km = dist_m / 1000.0;
                if let Some(estimated_duration) = estimate_duration_with_distance(route_id, distance_km, zwift_score) {
                    let diff = (estimated_duration as i32 - args.duration as i32).abs();
                    return diff <= args.tolerance as i32;
                } else {
                    // Unknown route but we have distance - use fallback estimation
                    let route_name = event.route.as_deref().unwrap_or(&event.name);
                    let estimated_duration = estimate_duration_for_category(distance_km, route_name, zwift_score);
                    let diff = (estimated_duration as i32 - args.duration as i32).abs();
                    return diff <= args.tolerance as i32;
                }
            } else if let Some(estimated_duration) = estimate_duration_from_route_id(route_id, zwift_score) {
                // No distance provided, but we know the route - check for multi-lap events
                let mut actual_duration = estimated_duration;
                
                // Check if this is a known multi-lap event
                if let Ok(db) = Database::new() {
                    if let Ok(Some(lap_count)) = db.get_multi_lap_info(&event.name) {
                        actual_duration = (estimated_duration as f64 * lap_count as f64) as u32;
                    }
                }
                
                let diff = (actual_duration as i32 - args.duration as i32).abs();
                return diff <= args.tolerance as i32;
            } else if is_racing_score_event(event) {
                // Racing Score event with route_id but no distance - try parsing description
                if let Some(distance_km) = parse_distance_from_description(&event.description) {
                    if let Some(estimated_duration) = estimate_duration_with_distance(route_id, distance_km, zwift_score) {
                        let diff = (estimated_duration as i32 - args.duration as i32).abs();
                        return diff <= args.tolerance as i32;
                    }
                }
            }
            // If we have a route_id but can't estimate (unknown route with no distance), continue to fallbacks
        }
        
        // FALLBACK 1: Use provided distance (but not if it's 0.0)
        if let Some(distance) = event.distance_in_meters.filter(|&d| d > 0.0) {
            let distance_km = distance / 1000.0;
            let route_name = event.route.as_deref().unwrap_or(&event.name);
            let estimated_duration =
                estimate_duration_for_category(distance_km, route_name, zwift_score);
            let diff = (estimated_duration as i32 - args.duration as i32).abs();
            return diff <= args.tolerance as i32;
        }
        
        // FALLBACK 2: For Racing Score events with distance=0, try to parse from description
        if is_racing_score_event(event) {
            if let Some(distance_km) = parse_distance_from_description(&event.description) {
                let route_name = event.route.as_deref().unwrap_or(&event.name);
                let estimated_duration =
                    estimate_duration_for_category(distance_km, route_name, zwift_score);
                let diff = (estimated_duration as i32 - args.duration as i32).abs();
                return diff <= args.tolerance as i32;
            }
        }
        
        // FALLBACK 3: Try to guess from event name
        if let Some(distance_km) = estimate_distance_from_name(&event.name) {
            let estimated_duration =
                estimate_duration_for_category(distance_km, &event.name, zwift_score);
            let diff = (estimated_duration as i32 - args.duration as i32).abs();
            return diff <= args.tolerance as i32;
        }

        // Check subgroups if main event has no distance/duration
        if !event.event_sub_groups.is_empty() {
            // Find the subgroup that matches user's category
            let user_category = get_category_from_score(zwift_score);
            
            // Check if user's category subgroup matches criteria
            event.event_sub_groups.iter().any(|subgroup| {
                // Check if this subgroup is for user's category
                let is_user_category = category_matches_subgroup(user_category, &subgroup.name);
                
                if !is_user_category {
                    return false; // Skip other categories
                }
                
                if let Some(duration) = subgroup.duration_in_minutes {
                    let diff = (duration as i32 - args.duration as i32).abs();
                    diff <= args.tolerance as i32
                } else if let Some(distance) = subgroup.distance_in_meters.filter(|&d| d > 0.0) {
                    let distance_km = distance / 1000.0;
                    let route_name = event.route.as_deref().unwrap_or(&event.name);
                    let estimated_duration =
                        estimate_duration_for_category(distance_km, route_name, zwift_score);
                    let diff = (estimated_duration as i32 - args.duration as i32).abs();
                    diff <= args.tolerance as i32
                } else {
                    false
                }
            })
        } else {
            false
        }
    });
    stats.duration_filtered = (pre_duration - events.len()) as u32;

    if args.debug {
        eprintln!("Debug: {} events after duration filter", events.len());
    }
    
    // Count unknown routes and missing data in remaining events
    for event in &events {
        if let Some(route_id) = event.route_id {
            if get_route_data(route_id).is_none() {
                stats.unknown_routes += 1;
                log_unknown_route(event);
            }
        }
        
        // Check for missing distance data
        let has_distance = event.distance_in_meters.filter(|&d| d > 0.0).is_some()
            || event.event_sub_groups.iter().any(|sg| sg.distance_in_meters.filter(|&d| d > 0.0).is_some());
        
        if !has_distance && event.route_id.is_none() {
            stats.missing_distance += 1;
        }
    }

    (events, stats)
}


/// Display filter statistics and actionable fixes
fn display_filter_stats(stats: &FilterStats, _total_fetched: usize) {
    let total_filtered = stats.sport_filtered + stats.time_filtered + stats.type_filtered 
        + stats.tag_filtered + stats.completed_routes_filtered + stats.duration_filtered;
    
    if total_filtered == 0 && stats.unknown_routes == 0 && stats.missing_distance == 0 {
        return; // No issues to report
    }
    
    println!("\n{}", "─".repeat(80).dimmed());
    println!("{}: {} events filtered out", "Filter Summary".yellow(), total_filtered);
    
    if stats.sport_filtered > 0 {
        println!("  • {} non-cycling events", stats.sport_filtered);
    }
    
    if stats.time_filtered > 0 {
        println!("  • {} events outside time range", stats.time_filtered);
    }
    
    if stats.type_filtered > 0 {
        println!("  • {} events of wrong type", stats.type_filtered);
    }
    
    if stats.tag_filtered > 0 {
        println!("  • {} events filtered by tags", stats.tag_filtered);
    }
    
    if stats.completed_routes_filtered > 0 {
        println!("  • {} events on completed routes", stats.completed_routes_filtered);
    }
    
    if stats.duration_filtered > 0 {
        println!("  • {} events outside duration range", stats.duration_filtered);
    }
    
    // Data quality issues in shown events
    if stats.unknown_routes > 0 || stats.missing_distance > 0 {
        println!("\n{}: Some events may have inaccurate estimates", "Data Quality".yellow());
        
        if stats.unknown_routes > 0 {
            println!("  • {} events with unknown routes", stats.unknown_routes);
            println!("    {} Run {} to help map these routes", "Fix:".green(), "cargo run --bin zwift-race-finder -- --discover-routes".cyan());
            println!("    {} Check {} for manual mapping", "Or:".green(), "sql/mappings/manual_route_mappings.sql".cyan());
        }
        
        if stats.missing_distance > 0 {
            println!("  • {} events missing distance data", stats.missing_distance);
            println!("    {} These are typically new Racing Score events", "Note:".green());
            println!("    {} Distance parsing from descriptions is attempted automatically", "Info:".green());
        }
    }
    
    // Suggest actions for large numbers of filtered events
    if stats.duration_filtered > 20 {
        println!("\n{}: Many events filtered by duration", "Tip".green());
        println!("  • Try wider tolerance: {}", "--tolerance 60".cyan());
        println!("  • Or different duration: {}", "--duration 60".cyan());
    }
}

// Helper function to collect unknown route data
fn log_unknown_route(event: &ZwiftEvent) {
    if let Some(route_id) = event.route_id {
        if get_route_data(route_id).is_none() {
            // First try to parse route name from description
            if let Some(description) = &event.description {
                if let Some(parsed) = route_discovery::parse_route_from_description(description) {
                    // Log with parsed route info for manual mapping later
                    if let Ok(db) = Database::new() {
                        let event_name_with_route = format!("{} -> {} ({} laps)",
                            event.name, parsed.route_name, parsed.laps);
                        let _ = db.record_unknown_route(
                            route_id,
                            &event_name_with_route,
                            &event.event_type
                        );
                        return;
                    }
                }
            }
            
            // If description parsing failed, log normally
            if let Ok(db) = Database::new() {
                let _ = db.record_unknown_route(
                    route_id,
                    &event.name,
                    &event.event_type
                );
            }
        }
    }
}

/// Structure to hold table row data for compact display
struct EventTableRow {
    name: String,
    time: String,
    distance: String,
    elevation: String,
    duration: String,
}

/// Structure to track filtered events and reasons
#[derive(Debug, Default)]
struct FilterStats {
    sport_filtered: u32,
    time_filtered: u32,
    type_filtered: u32,
    tag_filtered: u32,
    completed_routes_filtered: u32,
    duration_filtered: u32,
    unknown_routes: u32,
    missing_distance: u32,
}

/// Print events in table format
fn print_events_table(events: &[ZwiftEvent], _args: &Args, zwift_score: u32) {
    if events.is_empty() {
        return;
    }
    
    // Check if events span multiple days
    let first_event_time: DateTime<Local> = events[0].event_start.into();
    let last_event_time: DateTime<Local> = events[events.len() - 1].event_start.into();
    let spans_multiple_days = first_event_time.date_naive() != last_event_time.date_naive();
    
    // Collect data for all events
    let mut rows: Vec<(EventTableRow, DateTime<Local>)> = Vec::new();
    
    for event in events {
        let row = prepare_event_row(event, zwift_score);
        let local_time: DateTime<Local> = event.event_start.into();
        rows.push((row, local_time));
    }
    
    // Calculate column widths
    let name_width = rows.iter().map(|(r, _)| r.name.len()).max().unwrap_or(10).max(10);
    let time_width = rows.iter().map(|(r, _)| r.time.len()).max().unwrap_or(5).max(5);
    let distance_width = rows.iter().map(|(r, _)| r.distance.len()).max().unwrap_or(8).max(8);
    let elevation_width = rows.iter().map(|(r, _)| r.elevation.len()).max().unwrap_or(6).max(6);
    let duration_width = rows.iter().map(|(r, _)| r.duration.len()).max().unwrap_or(8).max(8);
    let total_width = name_width + time_width + distance_width + elevation_width + duration_width + 17;
    
    // Print header
    println!("\n{}", "─".repeat(total_width).dimmed());
    println!(
        "{:<width1$} │ {:<width2$} │ {:<width3$} │ {:<width4$} │ {:<width5$}",
        "Event Name".bright_blue().bold(),
        "Time".bright_blue().bold(),
        "Distance".bright_blue().bold(),
        "Elev".bright_blue().bold(),
        "Duration".bright_blue().bold(),
        width1 = name_width,
        width2 = time_width,
        width3 = distance_width,
        width4 = elevation_width,
        width5 = duration_width
    );
    println!("{}", "─".repeat(total_width).dimmed());
    
    // Print rows with day separators if needed
    let mut current_date = None;
    for (row, event_time) in rows {
        let event_date = event_time.date_naive();
        
        // Insert day separator if date changes and we span multiple days
        if spans_multiple_days && current_date.is_some() && current_date != Some(event_date) {
            println!("{}", "─".repeat(total_width).dimmed());
            let day_label = event_time.format("%A, %B %d").to_string();
            println!("{:^width$}", day_label.yellow(), width = total_width);
            println!("{}", "─".repeat(total_width).dimmed());
        } else if spans_multiple_days && current_date.is_none() {
            // First day label
            let day_label = event_time.format("%A, %B %d").to_string();
            println!("{:^width$}", day_label.yellow(), width = total_width);
            println!("{}", "─".repeat(total_width).dimmed());
        }
        
        current_date = Some(event_date);
        
        println!(
            "{:<width1$} │ {:<width2$} │ {:<width3$} │ {:<width4$} │ {:<width5$}",
            row.name,
            row.time,
            row.distance,
            row.elevation,
            row.duration.green(),
            width1 = name_width,
            width2 = time_width,
            width3 = distance_width,
            width4 = elevation_width,
            width5 = duration_width
        );
    }
    
    println!("{}", "─".repeat(total_width).dimmed());
}

/// Prepare a single event row for table display
fn prepare_event_row(event: &ZwiftEvent, zwift_score: u32) -> EventTableRow {
    let local_time: DateTime<Local> = event.event_start.into();
    let time_str = local_time.format("%H:%M").to_string();
    
    // Get route data and calculate total distance and elevation
    let (distance_str, elevation_str, duration_str) = if let Some(route_id) = event.route_id {
        if let Some(route_data) = get_route_data(route_id) {
            // Calculate total distance including lead-in
            let user_subgroup = find_user_subgroup(event, zwift_score);
            let distance_meters = user_subgroup
                .and_then(|sg| sg.distance_in_meters)
                .or(event.distance_in_meters);
            
            let mut actual_distance_km = route_data.distance_km;
            let mut lap_count = 1;
            
            // Calculate actual distance including laps
            if let Some(sg) = user_subgroup {
                if let Some(laps) = sg.laps {
                    lap_count = laps;
                    actual_distance_km = route_data.distance_km * laps as f64;
                } else if let Some(dist_m) = sg.distance_in_meters.filter(|&d| d > 0.0) {
                    actual_distance_km = dist_m / 1000.0;
                    lap_count = (actual_distance_km / route_data.distance_km).round() as u32;
                }
            } else if let Some(dist_m) = distance_meters.filter(|&d| d > 0.0) {
                actual_distance_km = dist_m / 1000.0;
                lap_count = (actual_distance_km / route_data.distance_km).round() as u32;
            }
            
            // Total distance including lead-in (no lap indicator)
            let total_distance = actual_distance_km + route_data.lead_in_distance_km;
            let distance_str = format!("{:.1} km", total_distance);
            
            // Calculate total elevation (multiply by laps if multi-lap)
            let total_elevation = route_data.elevation_m * lap_count;
            let elevation_str = format!("{}m", total_elevation);
            
            // Calculate duration
            let category = get_category_from_score(zwift_score);
            let effective_speed = get_category_speed(category);
            
            let difficulty_multiplier = get_route_difficulty_multiplier_from_elevation(
                route_data.distance_km,
                route_data.elevation_m
            );
            
            let surface_multiplier = match route_data.surface {
                "road" => 1.0,
                "gravel" => 0.85,
                "mixed" => 0.92,
                _ => 1.0,
            };
            
            let adjusted_speed = effective_speed * difficulty_multiplier * surface_multiplier;
            let estimated_duration = ((total_distance / adjusted_speed) * 60.0) as u32;
            
            (distance_str, elevation_str, format_duration(estimated_duration))
        } else {
            // Unknown route
            if let Some(dist_m) = event.distance_in_meters.filter(|&d| d > 0.0) {
                let distance_km = dist_m / 1000.0;
                let route_name = event.route.as_deref().unwrap_or(&event.name);
                let estimated_duration = estimate_duration_for_category(distance_km, route_name, zwift_score);
                (format!("{:.1} km", distance_km), "?m".to_string(), format_duration(estimated_duration))
            } else {
                ("? km".to_string(), "?m".to_string(), "? min".to_string())
            }
        }
    } else {
        // No route ID
        if let Some(dist_m) = event.distance_in_meters.filter(|&d| d > 0.0) {
            let distance_km = dist_m / 1000.0;
            let route_name = event.route.as_deref().unwrap_or(&event.name);
            let estimated_duration = estimate_duration_for_category(distance_km, route_name, zwift_score);
            (format!("{:.1} km", distance_km), "?m".to_string(), format_duration(estimated_duration))
        } else {
            ("? km".to_string(), "?m".to_string(), "? min".to_string())
        }
    };
    
    EventTableRow {
        name: event.name.clone(),
        time: time_str,
        distance: distance_str,
        elevation: elevation_str,
        duration: duration_str,
    }
}

fn print_event(event: &ZwiftEvent, _args: &Args, zwift_score: u32) {
    let local_time: DateTime<Local> = event.event_start.into();

    println!("\n{}", "─".repeat(80).dimmed());
    println!("{}: {}", "Event".bright_blue(), event.name.bold());
    println!(
        "{}: {}",
        "Type".bright_blue(),
        event.event_type.replace("_", " ")
    );
    println!(
        "{}: {}",
        "Time".bright_blue(),
        local_time.format("%a %b %d at %I:%M %p")
    );

    if let Some(route) = &event.route {
        println!("{}: {}", "Route".bright_blue(), route);
    }
    
    // Show route ID for debugging and data collection
    if let Some(route_id) = event.route_id {
        // Check completion status
        let completion_marker = if let Ok(db) = Database::new() {
            if db.is_route_completed(route_id).unwrap_or(false) {
                " ✓".green().to_string()
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        };
        
        if let Some(route_data) = get_route_data(route_id) {
            println!(
                "{}: {} ({}m elevation){}",
                "Route ID".bright_blue().dimmed(),
                route_id.to_string().dimmed(),
                route_data.elevation_m,
                completion_marker
            );
        } else {
            println!(
                "{}: {} (unknown route - automatically logged)",
                "Route ID".bright_blue().dimmed(),
                route_id.to_string().yellow()
            );
            log_unknown_route(event);
        }
    }

    // Duration info - check subgroups first for per-category data
    let user_subgroup = find_user_subgroup(event, zwift_score);
    
    // Try to get duration/distance from user's category subgroup first
    let duration_minutes = user_subgroup
        .and_then(|sg| sg.duration_in_minutes)
        .or(event.duration_in_minutes)
        .or_else(|| event.duration_in_seconds.map(|s| s / 60));
    
    let distance_meters = user_subgroup
        .and_then(|sg| sg.distance_in_meters)
        .or(event.distance_in_meters);

    if let Some(duration) = duration_minutes.filter(|&d| d > 0) {
        println!(
            "{}: {} (fixed duration)",
            "Duration".bright_blue(),
            format_duration(duration)
        );
    } else if let Some(route_id) = event.route_id {
        // PRIMARY: Use route_id with actual race distance
        if let Some(route_data) = get_route_data(route_id) {
            let actual_distance_km;
            let mut lap_count = 1;
            
            // Check if subgroup has lap information (for Racing Score events)
            if let Some(sg) = user_subgroup {
                if let Some(laps) = sg.laps {
                    lap_count = laps;
                    actual_distance_km = route_data.distance_km * laps as f64;
                } else if let Some(dist_m) = sg.distance_in_meters.filter(|&d| d > 0.0) {
                    // Subgroup has explicit distance
                    actual_distance_km = dist_m / 1000.0;
                    lap_count = (actual_distance_km / route_data.distance_km).round() as u32;
                } else {
                    // No lap info in subgroup, use event distance or route distance
                    actual_distance_km = distance_meters.map(|d| d / 1000.0)
                        .unwrap_or(route_data.distance_km);
                }
            } else {
                // No subgroup - use event distance or check database for multi-lap info
                if let Some(dist_m) = distance_meters.filter(|&d| d > 0.0) {
                    actual_distance_km = dist_m / 1000.0;
                    lap_count = (actual_distance_km / route_data.distance_km).round() as u32;
                } else {
                    // Check if this is a known multi-lap event
                    if let Ok(db) = Database::new() {
                        if let Ok(Some(laps)) = db.get_multi_lap_info(&event.name) {
                            lap_count = laps;
                        }
                    }
                    actual_distance_km = route_data.distance_km * lap_count as f64;
                }
            }
            
            // Display distance and laps
            if lap_count > 1 {
                println!("{}: {:.1} km ({} laps of {:.1} km)", 
                         "Distance".bright_blue(), 
                         actual_distance_km, 
                         lap_count, 
                         route_data.distance_km);
            } else {
                println!("{}: {:.1} km", "Distance".bright_blue(), actual_distance_km);
            }
            
            // Show lead-in distance if significant
            if route_data.lead_in_distance_km > 0.1 {
                println!("{}: {:.1} km", "Lead-in".bright_blue(), route_data.lead_in_distance_km);
            }
            
            // Use actual distance for estimation, not base route distance
            let category = get_category_from_score(zwift_score);
            let effective_speed = get_category_speed(category);
            
            let difficulty_multiplier = get_route_difficulty_multiplier_from_elevation(
                route_data.distance_km,  // Use base route for elevation calculation
                route_data.elevation_m
            );
            
            let surface_multiplier = match route_data.surface {
                "road" => 1.0,
                "gravel" => 0.85,
                "mixed" => 0.92,
                _ => 1.0,
            };
            
            let adjusted_speed = effective_speed * difficulty_multiplier * surface_multiplier;
            // Include lead-in distance in total duration calculation
            let total_distance = actual_distance_km + route_data.lead_in_distance_km;
            let estimated_duration = ((total_distance / adjusted_speed) * 60.0) as u32;
            
            let cat_string = get_detailed_category_from_score(zwift_score);
            println!(
                "{}: {} (estimated for Cat {} rider)",
                "Duration".bright_blue(),
                format_duration(estimated_duration).green(),
                cat_string
            );
        } else {
            // Unknown route_id - try to estimate using fallback methods
            let user_subgroup = find_user_subgroup(event, zwift_score);
            let distance_meters = user_subgroup
                .and_then(|sg| sg.distance_in_meters)
                .or(event.distance_in_meters)
                .filter(|&d| d > 0.0);  // Ignore 0.0 distances
                
            if let Some(dist_m) = distance_meters {
                let distance_km = dist_m / 1000.0;
                let route_name = event.route.as_deref().unwrap_or(&event.name);
                let estimated_duration = estimate_duration_for_category(distance_km, route_name, zwift_score);
                
                println!("{}: {:.1} km", "Distance".bright_blue(), distance_km);
                let cat_string = get_detailed_category_from_score(zwift_score);
                println!(
                    "{}: {} (estimated for Cat {} rider, unknown route)",
                    "Duration".bright_blue(),
                    format_duration(estimated_duration).green(),
                    cat_string
                );
            } else {
                println!(
                    "{}: Route ID {} needs mapping",
                    "Info".yellow(),
                    route_id.to_string().yellow()
                );
            }
        }
    } else if let Some(distance) = distance_meters.filter(|&d| d > 0.0) {
        // FALLBACK: Use provided distance (from subgroup or event)
        let distance_km = distance / 1000.0;
        let route_name = event.route.as_deref().unwrap_or(&event.name);
        let estimated_duration =
            estimate_duration_for_category(distance_km, route_name, zwift_score);

        println!("{}: {:.1} km", "Distance".bright_blue(), distance_km);
        let cat_string = get_detailed_category_from_score(zwift_score);
        println!(
            "{}: {} (estimated for Cat {} rider)",
            "Duration".bright_blue(),
            format_duration(estimated_duration).green(),
            cat_string
        );
    } else {
        // LAST RESORT: Try to estimate from name
        let estimated_distance = estimate_distance_from_name(&event.name);

        if let Some(distance_km) = estimated_distance {
            let route_name = event.route.as_deref().unwrap_or(&event.name);
            let estimated_duration =
                estimate_duration_for_category(distance_km, route_name, zwift_score);

            println!(
                "{}: ~{:.1} km (estimated from route)",
                "Distance".bright_blue(),
                distance_km
            );
            let cat_string = get_detailed_category_from_score(zwift_score);
            println!(
                "{}: {} (estimated for Cat {} rider)",
                "Duration".bright_blue(),
                format_duration(estimated_duration).green(),
                cat_string
            );
        }
    }

    if event.category_enforcement {
        println!("{}: {}", "Category".bright_blue(), "Enforced ✓".green());
    }

    // Show subgroups if any
    if !event.event_sub_groups.is_empty() {
        println!("{}: ", "Categories".bright_blue());
        
        // Find the subgroup that matches user's category
        let user_category = get_category_from_score(zwift_score);
        
        for group in &event.event_sub_groups {
            let is_user_category = group.name.contains(user_category) || 
                                   (user_category == "D" && group.name.contains("E"));
            
            print!("  • {}", group.name);
            
            // Show distance and calculate laps if possible
            if let Some(dist) = group.distance_in_meters {
                let dist_km = dist / 1000.0;
                print!(" ({:.1} km", dist_km);
                
                // Try to calculate laps based on base route distance
                if let Some(route_id) = event.route_id {
                    if let Some(route_data) = get_route_data(route_id) {
                        let base_distance = route_data.distance_km;
                        if base_distance > 0.0 {
                            let laps = (dist_km / base_distance).round() as u32;
                            if laps > 1 {
                                print!(" - {} laps", laps);
                            }
                        }
                    }
                }
                print!(")");
                
                // Show estimated duration for user's category
                if is_user_category {
                    let route_name = event.route.as_deref().unwrap_or(&event.name);
                    let estimated_duration = estimate_duration_for_category(dist_km, route_name, zwift_score);
                    print!(" → {} estimated", format_duration(estimated_duration).green());
                }
            }
            
            if let Some(dur) = group.duration_in_minutes {
                print!(" ({})", format_duration(dur));
            }
            
            println!();
        }
    }

    // Parse and display description data
    if event.description.is_some() {
        let desc_data = parse_description_data(&event.description);
        
        // Show parsed distance/elevation from description
        let mut parsed_info = Vec::new();
        if let Some(dist) = desc_data.distance_km {
            parsed_info.push(format!("{:.1} km", dist));
        }
        if let Some(elev) = desc_data.elevation_m {
            parsed_info.push(format!("{} m elevation", elev));
        }
        if let Some(laps) = desc_data.laps {
            parsed_info.push(format!("{} laps", laps));
        }
        
        if !parsed_info.is_empty() {
            println!("{}: {}", "From description".bright_blue(), parsed_info.join(", ").cyan());
        }
        
        // Show first 2 lines of description as before
        if let Some(desc) = &event.description {
            let cleaned_desc = desc
                .lines()
                .take(2)
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();
            if !cleaned_desc.is_empty() {
                println!("{}: {}", "Info".bright_blue(), cleaned_desc.dimmed());
            }
        }
    }
    
    // Show external URL if we have route slug
    if let Some(route_id) = event.route_id {
        if let Some(db_route) = get_route_data_from_db(route_id) {
            if let Some(slug) = db_route.slug {
                let world_slug = match db_route.world.as_str() {
                    "Watopia" => "watopia",
                    "London" => "london",
                    "New York" => "new-york",
                    "Innsbruck" => "innsbruck",
                    "Yorkshire" => "yorkshire",
                    "France" => "france",
                    "Paris" => "paris",
                    "Makuri Islands" => "makuri-islands",
                    "Scotland" => "scotland",
                    "Bologna" => "bologna",
                    "Crit City" => "crit-city",
                    _ => "watopia", // Default
                };
                let url = format!("https://whatsonzwift.com/world/{}/route/{}", world_slug, slug);
                println!("{}: {}", "Route Info".bright_blue(), url.dimmed());
            }
        }
    }
}

fn show_unknown_routes() -> Result<()> {
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
        
        println!("\n{}: Research these routes on ZwiftHacks or Zwift Insider", "Tip".yellow());
        println!("Then add them to the database with distance and elevation data.");
    }
    Ok(())
}

async fn discover_unknown_routes() -> Result<()> {
    let db = Database::new()?;
    let mut unknown = db.get_unknown_routes()?;
    
    if unknown.is_empty() {
        println!("No unknown routes to discover!");
        return Ok(());
    }
    
    // Sort by frequency (times_seen) to prioritize high-value routes
    unknown.sort_by(|a, b| b.2.cmp(&a.2));
    
    let total_count = unknown.len();
    println!("🔍 Starting route discovery for {} unknown routes...", total_count);
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
        println!("\n📦 Batch {} of {} ({} routes):", 
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
            
            print!("🔎 [{:3}x] Searching for '{}' (ID: {})... ", 
                times_seen, event_name, route_id);
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
                    
                    println!("✅ Found! {}km, {}m elevation, ID: {}", 
                        discovered.distance_km, discovered.elevation_m, final_route_id);
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
        println!("\nBatch {} complete: {} found, {} failed", 
            batch_num + 1, batch_discovered, batch_failed);
        
        // Check if we should continue
        if batch_start.elapsed().as_secs() > BATCH_TIMEOUT_MINS * 60 - 30 {
            println!("\n⏰ Timeout reached. {} routes remaining for next run.", 
                total_count - (batch_num + 1) * BATCH_SIZE);
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
    println!("  ⏳ Remaining to process: {}", 
        total_count.saturating_sub(total_discovered + total_failed + total_skipped));
    
    if total_discovered > 0 {
        println!("\n💡 Tip: Run the tool normally to see the newly discovered routes in action!");
    }
    
    Ok(())
}

fn record_race_result(input: &str) -> Result<()> {
    // Parse format: "route_id,minutes,event_name[,zwift_score]"
    let parts: Vec<&str> = input.split(',').collect();
    if parts.len() < 3 {
        anyhow::bail!("Format: --record-result 'route_id,minutes,event_name[,zwift_score]'");
    }
    
    let route_id: u32 = parts[0].trim().parse()
        .map_err(|_| anyhow::anyhow!("Invalid route_id"))?;
    let minutes: u32 = parts[1].trim().parse()
        .map_err(|_| anyhow::anyhow!("Invalid minutes"))?;
    
    // Check if zwift_score is provided at position 3 (before event name)
    let (event_name, zwift_score_override) = if parts.len() >= 4 && parts[3].trim().parse::<u32>().is_ok() {
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
    
    // Check if route exists
    if db.get_route(route_id)?.is_none() {
        println!("{}: Route {} not found in database", "Warning".yellow(), route_id);
        println!("Recording as unknown route for future mapping.");
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
    
    println!("\n✅ {} recorded successfully!", "Race result".green().bold());
    println!("  Route ID: {}", route_id);
    println!("  Event: {}", event_name);
    println!("  Time: {}", format_duration(minutes));
    println!("  Zwift Score: {}", zwift_score);
    
    // Show comparison with estimate if route is known
    if let Some(estimated) = estimate_duration_from_route_id(route_id, zwift_score) {
        let diff = (estimated as i32 - minutes as i32).abs();
        let accuracy = 100.0 - (diff as f64 / minutes as f64 * 100.0);
        println!("\n  Estimated: {} ({}% accurate)", 
            format_duration(estimated),
            accuracy.round() as i32
        );
    }
    
    Ok(())
}

async fn analyze_event_descriptions() -> Result<()> {
    println!("\n{}", "Analyzing Event Descriptions for Route Names...".yellow().bold());
    
    // Fetch current events
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.zwift.com/api/public/events")
        .send()
        .await?;
    
    let events: Vec<ZwiftEvent> = response.json().await?;
    
    let mut route_patterns: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
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
    
    println!("\n{}: {} unknown routes, {} descriptions parsed", 
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
    
    println!("\n{}: Use this information to create route mappings", "Next Step".yellow());
    println!("Look up the actual route names on Zwift Insider or ZwiftHacks");
    
    Ok(())
}

fn mark_route_complete(route_id: u32) -> Result<()> {
    let db = Database::new()?;
    
    // Check if route exists
    if let Some(route) = db.get_route(route_id)? {
        // Mark as complete
        db.mark_route_complete(route_id, None, None)?;
        println!("✅ Marked route {} ({}) as completed!", route.name, route.world);
        
        // Show updated progress
        let (completed, total) = db.get_completion_stats()?;
        println!("Progress: {}/{} routes completed ({}%)", 
            completed, total, (completed * 100) / total);
    } else {
        eprintln!("Error: Route {} not found in database", route_id);
    }
    
    Ok(())
}

fn show_route_progress() -> Result<()> {
    let db = Database::new()?;
    
    // Overall stats
    let (completed, total) = db.get_completion_stats()?;
    let percentage = if total > 0 { (completed * 100) / total } else { 0 };
    
    println!("🏆 {} {}", "Route Completion Progress".bold(), format!("v0.1.0").dimmed());
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
            (world_completed * 100) / world_total 
        } else { 
            0 
        };
        let world_filled = 10 * world_completed as usize / world_total.max(1) as usize;
        let world_bar = "▓".repeat(world_filled) + &"░".repeat(10 - world_filled);
        println!("  {:<15} {}/{} {} {}%", 
            world, world_completed, world_total, world_bar, world_percentage);
    }
    
    Ok(())
}


#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("🚴 {} {}", "Zwift Race Finder".bold(), "v0.1.0".dimmed());
    
    // Handle special commands first
    if args.show_unknown_routes {
        show_unknown_routes()?;
        return Ok(());
    }
    
    if args.analyze_descriptions {
        analyze_event_descriptions().await?;
        return Ok(());
    }
    
    if let Some(result_str) = args.record_result {
        record_race_result(&result_str)?;
        return Ok(());
    }
    
    if args.discover_routes {
        discover_unknown_routes().await?;
        return Ok(());
    }
    
    if let Some(route_id) = args.mark_complete {
        mark_route_complete(route_id)?;
        return Ok(());
    }
    
    if args.show_progress {
        show_route_progress()?;
        return Ok(());
    }

    // Load configuration
    let config = match FullConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            // Show warning but continue with defaults
            eprintln!("{} Failed to load configuration: {}", "⚠️  Warning:".yellow(), e);
            eprintln!("   Using default settings. See config.example.toml for setup instructions.");
            eprintln!();
            FullConfig::default()
        }
    };
    
    // Apply config defaults to args where not specified
    // For duration and tolerance, we need to check if they were explicitly set
    // Since clap sets defaults, we'll use Option types in Args instead
    let duration = if args.duration == 120 && config.config.preferences.default_duration.is_some() {
        // If using clap's default and config has a preference, use config
        config.config.preferences.default_duration.unwrap_or(120)
    } else {
        args.duration
    };
    
    let tolerance = if args.tolerance == 30 && config.config.preferences.default_tolerance.is_some() {
        // If using clap's default and config has a preference, use config
        config.config.preferences.default_tolerance.unwrap_or(30)
    } else {
        args.tolerance
    };
    
    let days = if args.days == 1 && config.config.preferences.default_days.is_some() {
        // If using clap's default and config has a preference, use config
        config.config.preferences.default_days.unwrap_or(1)
    } else {
        args.days
    };
    
    // Get user stats (auto-detected or from command line)
    let user_stats = get_user_stats(&config).await?;
    let zwift_score = args.zwift_score.unwrap_or(user_stats.zwift_score);

    // Validate Zwift score
    if zwift_score > 1000 {
        invalid_zwift_score(zwift_score).display();
        return Err(anyhow::anyhow!("Invalid Zwift Racing Score"));
    }

    // Show what stats we're using
    if args.zwift_score.is_some() {
        println!("Using provided Zwift Score: {}", zwift_score);
    } else if user_stats.username == "User" {
        // Using defaults - provide guidance
        println!("Using default Zwift Score: {} (Cat {})", 
            zwift_score.to_string().yellow(),
            get_category_from_score(zwift_score)
        );
        println!("{}", "💡 Tip: For personalized results, configure your stats in config.toml".dimmed());
    } else {
        println!(
            "Using {} stats: Zwift Score {} (Cat {})",
            user_stats.username.green(),
            zwift_score.to_string().yellow(),
            user_stats.category
        );
    }

    let min_duration = duration.saturating_sub(tolerance);
    let max_duration = duration + tolerance;

    println!(
        "Looking for events {} to {}...\n",
        format_duration(min_duration).yellow(),
        format_duration(max_duration).yellow()
    );

    let events = fetch_events().await?;
    
    if events.is_empty() {
        no_events_in_time_range(1).display();
        return Ok(());
    }
    
    println!("Fetched {} upcoming events", events.len());
    
    // Notify if API returns unexpected number of events
    if events.len() > 250 {
        println!("\n{} Zwift API returned {} events (expected ~200)", "🎉 Unexpected:".green(), events.len());
        println!("   The API may have been updated to return more data!");
        println!("   Please report this at: https://github.com/anthropics/claude-code/issues");
    }
    
    // Display the actual time range covered by the fetched events
    if !events.is_empty() {
        let earliest_start = events.iter()
            .map(|e| e.event_start)
            .min()
            .unwrap();
        let latest_start = events.iter()
            .map(|e| e.event_start)
            .max()
            .unwrap();
        
        // Format the time range in user's local timezone
        let earliest_local = earliest_start.with_timezone(&chrono::Local);
        let latest_local = latest_start.with_timezone(&chrono::Local);
        
        println!("Events from {} to {}", 
            earliest_local.format("%b %d, %l:%M %p").to_string().trim(),
            latest_local.format("%b %d, %l:%M %p").to_string().trim()
        );
    }
    
    // Warn about API limitation when requesting multiple days
    if days > 1 {
        println!("\n{} Zwift API only returns ~12 hours of events (200 max)", "⚠️  Note:".yellow());
        println!("   Multi-day searches may not show all available events.");
        println!("   For best results, search specific time windows throughout the day.");
    }

    // Count events by type for informative output
    let event_counts = count_events_by_type(&events);
    
    // Display event type summary
    if !event_counts.is_empty() {
        print!("Found: ");
        let formatted_counts: Vec<String> = event_counts.iter()
            .map(|(event_type, count)| format_event_type(event_type, *count))
            .collect();
        println!("{}", formatted_counts.join(", "));
    }

    // Debug: show race data
    if args.debug {
        let races: Vec<_> = events.iter()
            .filter(|e| e.sport.to_uppercase() == "CYCLING" && e.event_type == "RACE")
            .take(5)
            .collect();
        
        println!("\nDebug: First 5 races:");
        for event in races {
            println!("  Name: {}", event.name);
            println!("  Route ID: {:?}", event.route_id);
            println!("  Distance: {:?} meters", event.distance_in_meters);
            println!("  Duration: {:?} minutes", event.duration_in_minutes);
            println!("  Subgroups: {} groups", event.event_sub_groups.len());
            if !event.event_sub_groups.is_empty() {
                for sg in &event.event_sub_groups {
                    println!("    - {}: dist={:?}m, dur={:?}min, laps={:?}, range={:?}", 
                        sg.name, sg.distance_in_meters, sg.duration_in_minutes, sg.laps, sg.range_access_label);
                }
            }
            println!();
        }
    }
    
    // Show debug info only when --debug flag is used
    if args.debug {
        let race_count = events.iter()
            .filter(|e| e.sport.to_uppercase() == "CYCLING" && e.event_type == "RACE")
            .count();
        
        if race_count > 0 {
            println!("\nDebug: Found {} races, checking first few:", race_count);
            let sample_races: Vec<_> = events.iter()
                .filter(|e| e.sport.to_uppercase() == "CYCLING" && e.event_type == "RACE")
                .take(3)
                .collect();
                
            for event in sample_races {
                println!("  '{}': route_id={:?}, dist={:?}m", 
                    event.name, 
                    event.route_id, 
                    event.distance_in_meters
                );
                
                // Show what duration we would estimate
                if let Some(route_id) = event.route_id {
                    if let Some(route_data) = get_route_data(route_id) {
                        let est = estimate_duration_from_route_id(route_id, zwift_score);
                        println!("    → Known route: {} km, would estimate {:?} min", 
                            route_data.distance_km, est);
                    } else {
                        println!("    → Unknown route {}", route_id);
                    }
                }
            }
        }
    }

    // Debug: show race count and raw event data
    if args.debug {
        let race_count = events
            .iter()
            .filter(|e| e.sport == "CYCLING" && e.event_type == "RACE")
            .count();
        println!("Found {} cycling races", race_count);
        
    }

    // Log unknown routes (discovery will be done separately)
    for event in &events {
        if event.sport.to_uppercase() == "CYCLING" && event.event_type == "RACE" {
            log_unknown_route(event);
        }
    }

    // Create modified args with config-based defaults
    let mut effective_args = args.clone();
    effective_args.duration = duration;
    effective_args.tolerance = tolerance;
    effective_args.days = days;
    
    let (filtered, filter_stats) = filter_events(events.clone(), &effective_args, zwift_score);

    if filtered.is_empty() {
        println!("\n{}", "No matching events found!".red());
        
        // Provide specific suggestions based on what was searched for
        let suggestions = generate_no_results_suggestions(&args);
        for (i, suggestion) in suggestions.iter().enumerate() {
            if i == 0 {
                println!("\n{}", suggestion);
            } else if suggestion.starts_with("  •") {
                println!("{}", suggestion.replace("cargo run", &"cargo run".yellow()));
            } else {
                println!("{}", suggestion);
            }
        }
        
        println!("\nGeneral tips:");
        println!("  • Look further ahead: {} (next 3 days)", "-n 3".cyan());
        println!("  • See all available events: {}", "cargo run -- -e all -d 60 -t 180".cyan());
        println!("  • Most races: 20-30 min | Time trials/Group rides: 60-90 min");
    } else {
        let filter_desc = generate_filter_description(&effective_args, min_duration, max_duration);
        println!(
            "\nFound {} {} matching:",
            filtered.len().to_string().green().bold(),
            filter_desc
        );

        if args.verbose {
            // Use verbose output format
            for event in &filtered {
                print_event(event, &args, zwift_score);
            }
            println!("\n{}", "─".repeat(80).dimmed());
        } else {
            // Use table format by default
            print_events_table(&filtered, &args, zwift_score);
        }
        
        // Display filter statistics
        display_filter_stats(&filter_stats, events.len());

        println!(
            "\n💡 {} Join events via Zwift Companion app or zwift.com/events",
            "Tip:".yellow()
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_event(name: &str, distance: f64, route: &str, sport: &str) -> ZwiftEvent {
        ZwiftEvent {
            id: 1,
            name: name.to_string(),
            event_start: Utc::now() + chrono::Duration::hours(2),
            event_type: "RACE".to_string(),
            distance_in_meters: Some(distance * 1000.0),
            duration_in_minutes: None,
            duration_in_seconds: None,
            route_id: None,
            route: Some(route.to_string()),
            description: None,
            category_enforcement: false,
            event_sub_groups: vec![],
            sport: sport.to_string(),
            tags: vec![],
        }
    }

    #[test]
    fn test_filters_out_running_events() {
        let events = vec![
            create_test_event("Cycling Race", 60.0, "Watopia Flat", "CYCLING"), // ~116 min
            create_test_event("Running Event", 10.0, "May Field", "RUNNING"),
            create_test_event("Bike Race 2", 62.0, "Tempus Fugit", "CYCLING"), // ~112 min
        ];

        let args = Args {
            zwift_score: Some(195),
            duration: 120,
            tolerance: 30, // Accept 90-150 min
            event_type: "all".to_string(),
            days: 1,
            zwiftpower_username: None,
            debug: false,
            show_unknown_routes: false,
            analyze_descriptions: false,
            record_result: None,
            discover_routes: false,
            tags: vec![],
            exclude_tags: vec![],
            mark_complete: None,
            show_progress: false,
            new_routes_only: false,
            verbose: false,
        };

        let (filtered, _) = filter_events(events, &args, 195);

        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().all(|e| e.sport == "CYCLING"));
        assert!(!filtered.iter().any(|e| e.name.contains("Running")));
    }

    #[test]
    fn test_duration_estimation_for_cat_d() {
        // Test known distances and expected durations for Cat D (195 score)
        // Base speed for 195 score is 30.9 km/h

        // Watopia: 40km at 30.9km/h * 1.0 multiplier = 77.7 ≈ 77 min
        let watopia_time = estimate_duration_for_category(40.0, "Watopia", 195);
        assert_eq!(watopia_time, 77);

        // Alpe du Zwift: 30km at 30.9km/h * 0.7 multiplier = 83.1 ≈ 83 min
        let alpe_time = estimate_duration_for_category(30.0, "Alpe du Zwift", 195);
        assert_eq!(alpe_time, 83);

        // Tempus Fugit: 35km at 30.9km/h * 1.1 multiplier = 61.8 ≈ 61 min
        let tempus_time = estimate_duration_for_category(35.0, "Tempus Fugit", 195);
        assert_eq!(tempus_time, 61);
    }

    #[test]
    fn test_duration_filtering() {
        let events = vec![
            create_test_event("Short Race", 20.0, "Watopia Flat", "CYCLING"), // ~39 min
            create_test_event("Perfect Race", 50.0, "Watopia Flat", "CYCLING"), // ~97 min
            create_test_event("Long Race", 80.0, "Watopia Flat", "CYCLING"),  // ~155 min
        ];

        let args = Args {
            zwift_score: Some(195),
            duration: 100, // ~1h 40m
            tolerance: 20, // ±20 minutes
            event_type: "all".to_string(),
            days: 1,
            zwiftpower_username: None,
            debug: false,
            show_unknown_routes: false,
            analyze_descriptions: false,
            record_result: None,
            discover_routes: false,
            tags: vec![],
            exclude_tags: vec![],
            mark_complete: None,
            show_progress: false,
            new_routes_only: false,
            verbose: false,
        };

        let (filtered, _) = filter_events(events, &args, 195);

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Perfect Race");
    }

    #[test]
    fn test_regression_common_zwift_races() {
        // Based on typical Cat D (195 score) performance in common Zwift races
        struct RaceData {
            name: &'static str,
            distance_km: f64,
            route: &'static str,
            typical_time_minutes: u32,
            tolerance: u32,
        }

        let test_races = vec![
            RaceData {
                name: "3R Watopia Flat Route Race",
                distance_km: 33.4,
                route: "Watopia Flat Route",
                typical_time_minutes: 58, // Calculated: 33.4km / (30.9km/h * 1.1) = 58 min
                tolerance: 5,
            },
            RaceData {
                name: "ZSUN Sunday Race",
                distance_km: 25.7,
                route: "London Loop",
                typical_time_minutes: 50, // 25.7km / 30.9km/h = 50 min
                tolerance: 5,
            },
            RaceData {
                name: "DBR Monday Race",
                distance_km: 41.1,
                route: "Tempus Fugit",
                typical_time_minutes: 72, // 41.1km / (30.9km/h * 1.1) = 72 min
                tolerance: 5,
            },
            RaceData {
                name: "Herd Racing",
                distance_km: 46.9,
                route: "Innsbruck",
                typical_time_minutes: 91, // 46.9km / 30.9km/h = 91 min
                tolerance: 7,
            },
            RaceData {
                name: "ZRL Race",
                distance_km: 29.5,
                route: "Richmond",
                typical_time_minutes: 57, // 29.5km / 30.9km/h = 57 min
                tolerance: 5,
            },
        ];

        for race in test_races {
            let estimated = estimate_duration_for_category(race.distance_km, race.route, 195);
            let diff = (estimated as i32 - race.typical_time_minutes as i32).abs();

            assert!(
                diff <= race.tolerance as i32,
                "For {}: estimated {} min but typical is {} min (diff: {}, tolerance: {})",
                race.name,
                estimated,
                race.typical_time_minutes,
                diff,
                race.tolerance
            );
        }
    }

    #[test]
    fn test_multi_lap_race_detection() {
        // Test that we can detect multi-lap races from event names
        assert_eq!(parse_lap_count("3R Volcano Flat Race - 3 Laps"), Some(3));
        assert_eq!(parse_lap_count("Race - 5 laps"), Some(5));
        assert_eq!(parse_lap_count("2 Lap Race"), Some(2));
        assert_eq!(parse_lap_count("Regular Race"), None);
        
        // Test distance calculation for multi-lap races
        if let Some(route_data) = get_route_data(123) { // Volcano Flat
            let base_distance = route_data.distance_km;
            
            // Single lap
            assert_eq!(
                get_multi_lap_distance("Regular Race", base_distance),
                base_distance
            );
            
            // 3 laps
            assert_eq!(
                get_multi_lap_distance("3 Lap Race", base_distance),
                base_distance * 3.0
            );
        }
    }

    fn get_multi_lap_distance(event_name: &str, base_distance: f64) -> f64 {
        if let Some(laps) = parse_lap_count(event_name) {
            base_distance * laps as f64
        } else {
            base_distance
        }
    }

    #[test]
    fn test_event_type_filtering() {
        // Create different event types
        let mut race = create_test_event("3R Race", 50.0, "Watopia", "CYCLING");
        race.event_type = "RACE".to_string();

        let mut fondo = create_test_event("Gran Fondo", 60.0, "Watopia", "CYCLING");
        fondo.event_type = "GROUP_RIDE".to_string();

        let mut group = create_test_event("Sunday Ride", 50.0, "Watopia", "CYCLING");
        group.event_type = "GROUP_RIDE".to_string();

        let mut tt = create_test_event("Time Trial", 40.0, "Watopia", "CYCLING");
        tt.event_type = "TIME_TRIAL".to_string();

        let events = vec![race, fondo, group, tt];

        // Test race filter
        let mut args = Args {
            zwift_score: Some(195),
            duration: 120,
            tolerance: 60, // Wide tolerance to catch all
            event_type: "race".to_string(),
            days: 1,
            zwiftpower_username: None,
            debug: false,
            show_unknown_routes: false,
            analyze_descriptions: false,
            record_result: None,
            discover_routes: false,
            tags: vec![],
            exclude_tags: vec![],
            mark_complete: None,
            show_progress: false,
            new_routes_only: false,
            verbose: false,
        };

        let (filtered, _) = filter_events(events.clone(), &args, 195);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "3R Race");

        // Test fondo filter
        args.event_type = "fondo".to_string();
        let (filtered, _) = filter_events(events.clone(), &args, 195);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Gran Fondo");

        // Test group filter (excludes fondos)
        args.event_type = "group".to_string();
        let (filtered, _) = filter_events(events.clone(), &args, 195);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Sunday Ride");
    }

    #[test]
    fn test_specific_route_multipliers() {
        // Test that route difficulty multipliers work correctly
        let flat_distance = 40.0;
        let zwift_score = 195;

        // Same distance, different routes should give different times
        let tempus = estimate_duration_for_category(flat_distance, "Tempus Fugit", zwift_score);
        let alpe = estimate_duration_for_category(flat_distance, "Alpe du Zwift", zwift_score);
        let normal = estimate_duration_for_category(flat_distance, "Regular Route", zwift_score); // No special keywords

        // Tempus has 1.1x multiplier (faster), normal has 1.0x, Alpe has 0.7x (slower)
        assert!(
            tempus < normal,
            "Tempus Fugit should be faster than normal: {} vs {}",
            tempus,
            normal
        );
        assert!(
            alpe > normal,
            "Alpe du Zwift should be slower than normal: {} vs {}",
            alpe,
            normal
        );
        assert!(
            alpe > tempus + 10,
            "Alpe should be significantly slower than Tempus: {} vs {}",
            alpe,
            tempus
        );
    }
    
    #[test]
    fn test_route_id_regression_with_actual_results() {
        // This test will use Jack's actual race results once provided
        // For now, we test the route_id infrastructure
        
        // Test that our known routes exist
        let known_routes = vec![
            (1258415487, "Bell Lap"),
            (2143464829, "Watopia Flat Route"),
            (2927651296, "Makuri Pretzel"),
            (3742187716, "Castle to Castle"),
            (2698009951, "Downtown Dolphin"),
            (2663908549, "Mt. Fuji"),
        ];
        
        for (route_id, name) in known_routes {
            assert!(
                get_route_data(route_id).is_some(),
                "Route {} ({}) should exist in database",
                route_id, name
            );
        }
        
        // Test duration estimates are reasonable for Cat D (195 score)
        struct RouteExpectation {
            route_id: u32,
            name: &'static str,
            min_minutes: u32,
            max_minutes: u32,
        }
        
        let expectations = vec![
            RouteExpectation {
                route_id: 1258415487,  // Bell Lap (14.1km, 59m elevation)
                name: "Bell Lap",
                min_minutes: 22,  // 14.1km at ~38 km/h (flat boost)
                max_minutes: 28,
            },
            RouteExpectation {
                route_id: 2698009951,  // Downtown Dolphin (22.9km, 80m elevation)
                name: "Downtown Dolphin",
                min_minutes: 40,  // 22.9km at ~34 km/h
                max_minutes: 48,
            },
            RouteExpectation {
                route_id: 2663908549,  // Mt. Fuji (20.3km, 1159m elevation)
                name: "Mt. Fuji",
                min_minutes: 52,  // Very hilly route, 20.3km at ~23 km/h
                max_minutes: 70,
            },
        ];
        
        for exp in expectations {
            if let Some(duration) = estimate_duration_from_route_id(exp.route_id, 195) {
                assert!(
                    duration >= exp.min_minutes && duration <= exp.max_minutes,
                    "Route {} ({}) estimate {} should be {}-{} min for Cat D",
                    exp.route_id, exp.name, duration, exp.min_minutes, exp.max_minutes
                );
            } else {
                panic!("Route {} ({}) should have duration estimate", exp.route_id, exp.name);
            }
        }
        
        // TODO: Add Jack's actual race results here
        // Example format:
        // struct ActualResult {
        //     route_id: u32,
        //     actual_minutes: u32,
        //     date: &'static str,
        // }
        // 
        // let jacks_results = vec![
        //     ActualResult { route_id: 2698009951, actual_minutes: 52, date: "2025-01" },
        // ];
    }

    #[test]
    fn test_edge_case_estimations() {
        // Test very short race (sprint)
        let sprint_duration = estimate_duration_for_category(5.0, "Sprint Route", 195);
        assert!(sprint_duration >= 8 && sprint_duration <= 12, 
            "Sprint (5km) should take 8-12 minutes, got {}", sprint_duration);
        
        // Test very long race (gran fondo)
        let fondo_duration = estimate_duration_for_category(100.0, "Epic Route", 195);
        assert!(fondo_duration >= 180 && fondo_duration <= 250,
            "Gran Fondo (100km) should take 3-4.2 hours, got {} min", fondo_duration);
        
        // Test extreme elevation (Alpe du Zwift)
        let alpe_duration = estimate_duration_for_category(12.2, "Alpe du Zwift", 195);
        assert!(alpe_duration >= 33 && alpe_duration <= 45,
            "Alpe (12.2km with 1035m elevation) should take 33-45 min, got {}", alpe_duration);
    }

    #[test]
    fn test_database_route_validation() {
        // Test that all routes in database have valid data
        if let Ok(db) = Database::new() {
            if let Ok(routes) = db.get_all_routes() {
                for route in routes {
                    // Distance should be reasonable
                    assert!(route.distance_km > 0.0 && route.distance_km < 200.0,
                        "Route {} has unrealistic distance: {} km", 
                        route.name, route.distance_km);
                    
                    // Elevation gain per km should be reasonable
                    let elevation_per_km = route.elevation_m as f64 / route.distance_km;
                    assert!(elevation_per_km < 100.0,
                        "Route {} has unrealistic elevation: {} m/km",
                        route.name, elevation_per_km);
                    
                    // Surface should be valid
                    assert!(matches!(route.surface.as_str(), "road" | "gravel" | "mixed"),
                        "Route {} has invalid surface: {}",
                        route.name, route.surface);
                }
            }
        }
    }

    #[tokio::test]
    #[ignore] // Run with: cargo test -- --ignored
    async fn test_real_zwift_api_connection() {
        // Simple test to verify API is accessible
        let client = reqwest::Client::new();
        let url = "https://us-or-rly101.zwift.com/api/public/events";
        
        let response = client
            .get(url)
            .header("Accept", "application/json")
            .header("User-Agent", "zwift-race-finder/1.0")
            .send()
            .await;
        
        match response {
            Ok(resp) => {
                assert!(resp.status().is_success(), "API returned error: {}", resp.status());
                
                // Try to parse as JSON array
                let body = resp.text().await.expect("Failed to read response body");
                assert!(body.starts_with('['), "Response doesn't look like JSON array");
                assert!(body.len() > 100, "Response seems too short");
            }
            Err(e) => {
                panic!("Failed to connect to Zwift API: {}", e);
            }
        }
    }

    #[test]
    fn test_is_racing_score_event() {
        // Test event without Racing Score (traditional event)
        let traditional_event = ZwiftEvent {
            id: 1,
            name: "Traditional Race".to_string(),
            event_start: Utc::now(),
            event_type: "RACE".to_string(),
            distance_in_meters: Some(40000.0),
            duration_in_minutes: None,
            duration_in_seconds: None,
            route_id: Some(1),
            route: Some("Watopia".to_string()),
            description: None,
            category_enforcement: false,
            event_sub_groups: vec![
                EventSubGroup {
                    id: 1,
                    name: "A".to_string(),
                    route_id: Some(1),
                    distance_in_meters: Some(40000.0),
                    duration_in_minutes: None,
                    category_enforcement: None,
                    range_access_label: None, // No range label for traditional events
                    laps: None,
                },
            ],
            sport: "CYCLING".to_string(),
            tags: vec![],
        };
        
        assert!(!is_racing_score_event(&traditional_event));
        
        // Test Racing Score event
        let racing_score_event = ZwiftEvent {
            id: 2,
            name: "Three Village Loop Race".to_string(),
            event_start: Utc::now(),
            event_type: "RACE".to_string(),
            distance_in_meters: Some(0.0), // Racing Score events have 0 distance
            duration_in_minutes: None,
            duration_in_seconds: None,
            route_id: Some(9),
            route: Some("Three Village Loop".to_string()),
            description: Some("Distance: 10.6 km".to_string()),
            category_enforcement: false,
            event_sub_groups: vec![
                EventSubGroup {
                    id: 1,
                    name: "0-199".to_string(),
                    route_id: Some(9),
                    distance_in_meters: Some(0.0),
                    duration_in_minutes: None,
                    category_enforcement: None,
                    range_access_label: Some("0-199".to_string()), // This indicates Racing Score
                    laps: None,
                },
            ],
            sport: "CYCLING".to_string(),
            tags: vec![],
        };
        
        assert!(is_racing_score_event(&racing_score_event));
        
        // Test event with no subgroups
        let no_subgroups_event = ZwiftEvent {
            id: 3,
            name: "Solo Event".to_string(),
            event_start: Utc::now(),
            event_type: "RACE".to_string(),
            distance_in_meters: Some(20000.0),
            duration_in_minutes: None,
            duration_in_seconds: None,
            route_id: Some(1),
            route: Some("Watopia".to_string()),
            description: None,
            category_enforcement: false,
            event_sub_groups: vec![], // No subgroups at all
            sport: "CYCLING".to_string(),
            tags: vec![],
        };
        
        assert!(!is_racing_score_event(&no_subgroups_event));
    }

    #[test]
    fn test_parse_distance_from_description() {
        // Test with no description
        assert_eq!(parse_distance_from_description(&None), None);
        
        // Test with empty description
        assert_eq!(parse_distance_from_description(&Some("".to_string())), None);
        
        // Test with km distance
        assert_eq!(
            parse_distance_from_description(&Some("Distance: 10.6 km".to_string())),
            Some(10.6)
        );
        
        // Test with miles distance (should convert to km)
        let miles_result = parse_distance_from_description(&Some("Distance: 14.6 miles".to_string()));
        assert!(miles_result.is_some());
        let km_value = miles_result.unwrap();
        assert!((km_value - 23.496364).abs() < 0.001, "Expected ~23.496, got {}", km_value);
        
        // Test with decimal km
        assert_eq!(
            parse_distance_from_description(&Some("This race is 23.5 km long".to_string())),
            Some(23.5)
        );
        
        // Test with integer km
        assert_eq!(
            parse_distance_from_description(&Some("Distance: 40 km".to_string())),
            Some(40.0)
        );
        
        // Test with no distance information
        assert_eq!(
            parse_distance_from_description(&Some("A fun race in Watopia".to_string())),
            None
        );
        
        // Test with multiple distances (should find first)
        assert_eq!(
            parse_distance_from_description(&Some("First lap: 10 km, Total: 30 km".to_string())),
            Some(10.0)
        );
    }
    
    #[test]
    fn test_parse_description_data() {
        // Test comprehensive parsing
        let desc1 = Some("Distance: 23.5 km\nElevation: 450 m\n3 laps race".to_string());
        let data1 = parse_description_data(&desc1);
        assert_eq!(data1.distance_km, Some(23.5));
        assert_eq!(data1.elevation_m, Some(450));
        assert_eq!(data1.laps, Some(3));
        
        // Test with feet elevation
        let desc2 = Some("Distance: 10 miles, Elevation: 1000 feet".to_string());
        let data2 = parse_description_data(&desc2);
        // Use approximate comparison for floating point
        assert!(data2.distance_km.is_some());
        let dist_km = data2.distance_km.unwrap();
        assert!((dist_km - 16.0934).abs() < 0.001, "Expected ~16.093, got {}", dist_km);
        assert_eq!(data2.elevation_m, Some(304)); // 1000 / 3.28084 rounds to 304
        assert_eq!(data2.laps, None);
        
        // Test elevation gain pattern
        let desc3 = Some("Elevation Gain: 250 m".to_string());
        let data3 = parse_description_data(&desc3);
        assert_eq!(data3.elevation_m, Some(250));
        
        // Test "meters of climbing" pattern
        let desc4 = Some("This route has 350 meters of climbing".to_string());
        let data4 = parse_description_data(&desc4);
        assert_eq!(data4.elevation_m, Some(350));
        
        // Test no data
        let desc5 = Some("A fun race in Watopia!".to_string());
        let data5 = parse_description_data(&desc5);
        assert_eq!(data5.distance_km, None);
        assert_eq!(data5.elevation_m, None);
        assert_eq!(data5.laps, None);
    }

    #[test]
    fn test_racing_score_event_filtering() {
        // Create a Racing Score event with distance in description
        let racing_score_event = ZwiftEvent {
            id: 1,
            name: "Three Village Loop Race".to_string(),
            event_start: Utc::now() + chrono::Duration::hours(2),
            event_type: "RACE".to_string(),
            distance_in_meters: Some(0.0), // Racing Score events have 0
            duration_in_minutes: None,
            duration_in_seconds: None,
            route_id: Some(3379779247),
            route: Some("Three Village Loop".to_string()),
            description: Some("Distance: 10.6 km".to_string()),
            category_enforcement: false,
            event_sub_groups: vec![
                EventSubGroup {
                    id: 1,
                    name: "0-199".to_string(),
                    route_id: Some(3379779247),
                    distance_in_meters: Some(0.0),
                    duration_in_minutes: None,
                    category_enforcement: None,
                    range_access_label: Some("0-199".to_string()),
                    laps: None,
                },
            ],
            sport: "CYCLING".to_string(),
            tags: vec![],
        };
        
        let args = Args {
            zwift_score: Some(195),
            duration: 20, // ~20 minutes
            tolerance: 10, // ±10 minutes
            event_type: "race".to_string(),
            days: 1,
            zwiftpower_username: None,
            debug: false,
            show_unknown_routes: false,
            analyze_descriptions: false,
            record_result: None,
            discover_routes: false,
            tags: vec![],
            exclude_tags: vec![],
            mark_complete: None,
            show_progress: false,
            new_routes_only: false,
            verbose: false,
        };
        
        let events = vec![racing_score_event.clone()];
        let (filtered, _) = filter_events(events, &args, 195);
        
        // Event should be included if distance parsing works
        // 10.6 km at Cat D speed (30.9 km/h) = ~20.6 minutes
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Three Village Loop Race");
    }

    #[test]
    fn test_racing_score_event_with_zero_distance() {
        // Test that events with distanceInMeters: 0 are not filtered out early
        let events = vec![
            // Traditional event with distance
            ZwiftEvent {
                id: 1,
                name: "Traditional Race".to_string(),
                event_start: Utc::now() + chrono::Duration::hours(1),
                event_type: "RACE".to_string(),
                distance_in_meters: Some(20000.0), // 20km
                duration_in_minutes: None,
                duration_in_seconds: None,
                route_id: None,
                route: Some("Watopia".to_string()),
                description: None,
                category_enforcement: false,
                event_sub_groups: vec![],
                sport: "CYCLING".to_string(),
                tags: vec![],
            },
            // Racing Score event with 0 distance
            ZwiftEvent {
                id: 2,
                name: "Racing Score Event".to_string(),
                event_start: Utc::now() + chrono::Duration::hours(2),
                event_type: "RACE".to_string(),
                distance_in_meters: Some(0.0), // 0 distance!
                duration_in_minutes: None,
                duration_in_seconds: None,
                route_id: None,
                route: Some("Test Route".to_string()),
                description: Some("Distance: 20 km".to_string()),
                category_enforcement: false,
                event_sub_groups: vec![
                    EventSubGroup {
                        id: 1,
                        name: "0-650".to_string(),
                        route_id: None,
                        distance_in_meters: Some(0.0),
                        duration_in_minutes: None,
                        category_enforcement: None,
                        range_access_label: Some("0-650".to_string()),
                        laps: None,
                    },
                ],
                sport: "CYCLING".to_string(),
                tags: vec![],
            },
        ];
        
        let args = Args {
            zwift_score: Some(195),
            duration: 40, // ~40 minutes
            tolerance: 10, // ±10 minutes
            event_type: "race".to_string(),
            days: 1,
            zwiftpower_username: None,
            debug: false,
            show_unknown_routes: false,
            analyze_descriptions: false,
            record_result: None,
            discover_routes: false,
            tags: vec![],
            exclude_tags: vec![],
            mark_complete: None,
            show_progress: false,
            new_routes_only: false,
            verbose: false,
        };
        
        let (filtered, _) = filter_events(events, &args, 195);
        
        // Both events should be included (20km at ~30.9km/h = ~38.8 min)
        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().any(|e| e.name == "Traditional Race"));
        assert!(filtered.iter().any(|e| e.name == "Racing Score Event"));
    }

    #[test]
    fn test_format_duration() {
        // Test basic formatting
        assert_eq!(format_duration(0), "00:00");
        assert_eq!(format_duration(1), "00:01");
        assert_eq!(format_duration(59), "00:59");
        assert_eq!(format_duration(60), "01:00");
        assert_eq!(format_duration(61), "01:01");
        assert_eq!(format_duration(120), "02:00");
        assert_eq!(format_duration(150), "02:30");
        
        // Test larger values
        assert_eq!(format_duration(599), "09:59");
        assert_eq!(format_duration(600), "10:00");
        assert_eq!(format_duration(1439), "23:59");
        assert_eq!(format_duration(1440), "24:00"); // 24 hours
        assert_eq!(format_duration(2880), "48:00"); // 48 hours
    }

    #[test]
    fn test_estimate_distance_from_name() {
        // Test explicit distance parsing first
        assert_eq!(estimate_distance_from_name("10km Race"), Some(10.0));
        assert_eq!(estimate_distance_from_name("42.2km Marathon"), Some(42.2));
        
        // Test pattern-based estimates
        assert_eq!(estimate_distance_from_name("3R Flat Route Race"), Some(33.4));
        assert_eq!(estimate_distance_from_name("Epic Pretzel Challenge"), Some(67.5));
        assert_eq!(estimate_distance_from_name("Monday Night Crit Series"), Some(20.0));
        assert_eq!(estimate_distance_from_name("Gran Fondo Saturday"), Some(92.6));
        assert_eq!(estimate_distance_from_name("Century Ride Event"), Some(160.0));
        
        // Test case insensitivity
        assert_eq!(estimate_distance_from_name("3r FLAT race"), Some(33.4));
        assert_eq!(estimate_distance_from_name("EPIC PRETZEL"), Some(67.5));
        
        // Test no match
        assert_eq!(estimate_distance_from_name("Random Race Name"), None);
        assert_eq!(estimate_distance_from_name(""), None);
    }

    #[test]
    fn test_default_sport() {
        // Simple test for the default sport function
        assert_eq!(default_sport(), "CYCLING");
    }

    #[test]
    fn test_get_multi_lap_distance() {
        // Test single lap (no lap count)
        assert_eq!(get_multi_lap_distance("Regular Race", 10.0), 10.0);
        assert_eq!(get_multi_lap_distance("Sprint Race", 5.5), 5.5);
        
        // Test multi-lap races
        assert_eq!(get_multi_lap_distance("2 Lap Race", 10.0), 20.0);
        assert_eq!(get_multi_lap_distance("3 Lap Sprint", 5.0), 15.0);
        assert_eq!(get_multi_lap_distance("4 laps of Volcano", 12.5), 50.0);
        assert_eq!(get_multi_lap_distance("5 Laps Challenge", 8.0), 40.0);
        assert_eq!(get_multi_lap_distance("10 lap time trial", 2.5), 25.0);
        
        // Test edge cases
        assert_eq!(get_multi_lap_distance("", 10.0), 10.0);
        assert_eq!(get_multi_lap_distance("Race with laps in name", 10.0), 10.0); // "laps" without number
    }

    #[test]
    fn test_prepare_event_row() {
        // Create a test event with route data
        let event = ZwiftEvent {
            id: 1,
            name: "Test Race".to_string(),
            event_start: Utc::now() + chrono::Duration::hours(2),
            event_type: "RACE".to_string(),
            distance_in_meters: Some(20000.0),
            duration_in_minutes: None,
            duration_in_seconds: None,
            route_id: Some(1), // Watopia's Ocean Boulevard
            route: Some("Ocean Boulevard".to_string()),
            description: None,
            category_enforcement: false,
            event_sub_groups: vec![],
            sport: "CYCLING".to_string(),
            tags: vec![],
        };
        
        let row = prepare_event_row(&event, 195); // Cat D rider
        
        // Verify the row has expected format
        assert_eq!(row.name, "Test Race");
        assert!(row.time.contains(":")); // Should be HH:MM format
        assert!(row.distance.contains("km")); // Should include km
        assert!(row.elevation.contains("m")); // Should include m
        assert!(row.duration.contains(":")); // Should be MM:SS or HH:MM format
    }

    #[test]
    fn test_generate_filter_description() {
        // Test basic race filter
        let args = Args {
            zwift_score: Some(195),
            duration: 30,
            tolerance: 10,
            event_type: "race".to_string(),
            days: 1,
            zwiftpower_username: None,
            debug: false,
            show_unknown_routes: false,
            analyze_descriptions: false,
            record_result: None,
            discover_routes: false,
            tags: vec![],
            exclude_tags: vec![],
            mark_complete: None,
            show_progress: false,
            new_routes_only: false,
            verbose: false,
        };
        
        let desc = generate_filter_description(&args, 20, 40);
        assert!(desc.contains("races"));
        assert!(desc.contains("20-40 min"));
        assert!(desc.contains("next 24h"));
        
        // Test with tags
        let args_with_tags = Args {
            tags: vec!["3R".to_string(), "flat".to_string()],
            ..args.clone()
        };
        let desc_tags = generate_filter_description(&args_with_tags, 20, 40);
        assert!(desc_tags.contains("with tags: 3R, flat"));
        
        // Test with multiple days
        let args_multi_day = Args {
            days: 7,
            ..args.clone()
        };
        let desc_days = generate_filter_description(&args_multi_day, 20, 40);
        assert!(desc_days.contains("next 7 days"));
        
        // Test time trial
        let args_tt = Args {
            event_type: "tt".to_string(),
            ..args.clone()
        };
        let desc_tt = generate_filter_description(&args_tt, 20, 40);
        assert!(desc_tt.contains("time trials"));
    }

    #[test]
    fn test_estimate_duration_for_category() {
        // Test Cat D rider on flat route
        let duration_d_flat = estimate_duration_for_category(20.0, "Watopia Flat Route", 195);
        // 20km at 30.9 km/h = ~38.8 minutes
        assert!(duration_d_flat >= 35 && duration_d_flat <= 42, "Expected ~39 min, got {}", duration_d_flat);
        
        // Test Cat C rider on flat route
        let duration_c_flat = estimate_duration_for_category(20.0, "Watopia Flat Route", 250);
        // 20km at 33.8 km/h = ~35.5 minutes
        assert!(duration_c_flat >= 32 && duration_c_flat <= 38, "Expected ~35 min, got {}", duration_c_flat);
        
        // Test with hilly route (should be slower)
        let duration_d_hilly = estimate_duration_for_category(20.0, "Epic KOM", 195);
        assert!(duration_d_hilly > duration_d_flat, "Hilly route should take longer");
        
        // Test with Alpe du Zwift (0.7 multiplier = 30% slower)
        let duration_alpe = estimate_duration_for_category(12.2, "Alpe du Zwift", 195);
        // 12.2km / (30.9 km/h * 0.7) * 60 = ~33.8 minutes
        assert!(duration_alpe >= 30 && duration_alpe <= 36, "Expected ~34 min for Alpe, got {}", duration_alpe);
    }

    #[test]
    fn test_get_cache_file() {
        let cache_file = get_cache_file().unwrap();
        assert!(cache_file.to_string_lossy().contains("zwift-race-finder"));
        assert!(cache_file.to_string_lossy().contains("user_stats.json"));
    }

    #[test]
    fn test_load_and_save_cached_stats() {
        use std::fs;
        use tempfile::TempDir;
        
        // Create a temporary directory for testing
        let temp_dir = TempDir::new().unwrap();
        let cache_file = temp_dir.path().join("zwift-race-finder").join("user_stats.json");
        
        // Override the cache directory for testing
        std::env::set_var("XDG_CACHE_HOME", temp_dir.path());
        
        // Test loading when cache doesn't exist
        let result = load_cached_stats().unwrap();
        assert!(result.is_none(), "Should return None when cache doesn't exist");
        
        // Create test stats
        let test_stats = UserStats {
            zwift_score: 250,
            category: "C".to_string(),
            username: "TestUser".to_string(),
        };
        
        // Save stats
        save_cached_stats(&test_stats).unwrap();
        assert!(cache_file.exists(), "Cache file should be created");
        
        // Load stats back
        let loaded = load_cached_stats().unwrap();
        assert!(loaded.is_some(), "Should load cached stats");
        let loaded_stats = loaded.unwrap();
        assert_eq!(loaded_stats.zwift_score, 250);
        assert_eq!(loaded_stats.category, "C");
        assert_eq!(loaded_stats.username, "TestUser");
        
        // Test cache expiration by modifying the file
        let content = fs::read_to_string(&cache_file).unwrap();
        let mut cached: CachedStats = serde_json::from_str(&content).unwrap();
        cached.cached_at = Utc::now() - chrono::Duration::hours(25); // Make it 25 hours old
        let expired_content = serde_json::to_string(&cached).unwrap();
        fs::write(&cache_file, expired_content).unwrap();
        
        // Should return None for expired cache
        let expired_result = load_cached_stats().unwrap();
        assert!(expired_result.is_none(), "Should return None for expired cache");
        
        // Clean up
        std::env::remove_var("XDG_CACHE_HOME");
    }

    #[test]
    fn test_display_filter_stats() {
        
        // Test with no filtering
        let empty_stats = FilterStats::default();
        display_filter_stats(&empty_stats, 100); // Should not print anything
        
        // Test with some filtering
        let stats = FilterStats {
            sport_filtered: 5,
            time_filtered: 3,
            type_filtered: 2,
            tag_filtered: 1,
            completed_routes_filtered: 0,
            duration_filtered: 4,
            unknown_routes: 2,
            missing_distance: 1,
        };
        
        // We can't easily capture println! output, but we can verify the function runs
        display_filter_stats(&stats, 100); // Should print statistics
    }

    #[test]
    fn test_log_unknown_route() {
        
        // Create a test event
        let event = ZwiftEvent {
            id: 999,
            name: "Unknown Route Race".to_string(),
            event_start: Utc::now(),
            event_type: "RACE".to_string(),
            distance_in_meters: Some(25000.0),
            duration_in_minutes: None,
            duration_in_seconds: None,
            route_id: Some(9999), // Unknown route ID
            route: Some("Mystery Route".to_string()),
            description: Some("A mysterious 25km race".to_string()),
            category_enforcement: false,
            event_sub_groups: vec![],
            sport: "CYCLING".to_string(),
            tags: vec![],
        };
        
        // This should not panic
        log_unknown_route(&event);
    }

    #[test]
    fn test_count_events_by_type() {
        let events = vec![
            ZwiftEvent {
                id: 1,
                name: "Race 1".to_string(),
                event_start: Utc::now(),
                event_type: "RACE".to_string(),
                distance_in_meters: Some(20000.0),
                duration_in_minutes: None,
                duration_in_seconds: None,
                route_id: None,
                route: None,
                description: None,
                category_enforcement: false,
                event_sub_groups: vec![],
                sport: "CYCLING".to_string(),
                tags: vec![],
            },
            ZwiftEvent {
                id: 2,
                name: "Race 2".to_string(),
                event_start: Utc::now(),
                event_type: "RACE".to_string(),
                distance_in_meters: Some(20000.0),
                duration_in_minutes: None,
                duration_in_seconds: None,
                route_id: None,
                route: None,
                description: None,
                category_enforcement: false,
                event_sub_groups: vec![],
                sport: "CYCLING".to_string(),
                tags: vec![],
            },
            ZwiftEvent {
                id: 3,
                name: "Group Ride".to_string(),
                event_start: Utc::now(),
                event_type: "GROUP_RIDE".to_string(),
                distance_in_meters: Some(50000.0),
                duration_in_minutes: None,
                duration_in_seconds: None,
                route_id: None,
                route: None,
                description: None,
                category_enforcement: false,
                event_sub_groups: vec![],
                sport: "CYCLING".to_string(),
                tags: vec![],
            },
            ZwiftEvent {
                id: 4,
                name: "Running Event".to_string(),
                event_start: Utc::now(),
                event_type: "RACE".to_string(),
                distance_in_meters: Some(10000.0),
                duration_in_minutes: None,
                duration_in_seconds: None,
                route_id: None,
                route: None,
                description: None,
                category_enforcement: false,
                event_sub_groups: vec![],
                sport: "RUNNING".to_string(),
                tags: vec![],
            },
        ];
        
        let counts = count_events_by_type(&events);
        
        // Should have 2 races and 1 group ride (running excluded)
        assert_eq!(counts.len(), 2);
        assert_eq!(counts[0], ("RACE".to_string(), 2));
        assert_eq!(counts[1], ("GROUP_RIDE".to_string(), 1));
    }

    #[test]
    fn test_format_event_type() {
        assert_eq!(format_event_type("RACE", 5), "5 races");
        assert_eq!(format_event_type("race", 1), "1 races"); // Note: doesn't handle singular
        assert_eq!(format_event_type("TIME_TRIAL", 3), "3 time trials");
        assert_eq!(format_event_type("GROUP_RIDE", 10), "10 group rides");
        assert_eq!(format_event_type("GROUP_WORKOUT", 2), "2 group workouts");
        assert_eq!(format_event_type("UNKNOWN_TYPE", 7), "7 UNKNOWN_TYPE");
    }

    #[test]
    fn test_generate_no_results_suggestions_for_race() {
        let args = Args {
            zwift_score: Some(195),
            duration: 60,
            tolerance: 10,
            event_type: "race".to_string(),
            days: 1,
            zwiftpower_username: None,
            debug: false,
            show_unknown_routes: false,
            analyze_descriptions: false,
            record_result: None,
            discover_routes: false,
            tags: vec![],
            exclude_tags: vec![],
            mark_complete: None,
            show_progress: false,
            new_routes_only: false,
            verbose: false,
        };
        
        let suggestions = generate_no_results_suggestions(&args);
        
        // We now have more suggestions with race duration info
        assert!(suggestions.len() >= 4);
        assert!(suggestions[0].contains("Most races are short"));
        assert!(suggestions[1].contains("-d 30 -t 30"));
        assert!(suggestions[2].contains("-d 60 -t 120"));
        assert!(suggestions[3].contains("-e tt"));
    }

    #[test]
    fn test_generate_no_results_suggestions_for_tt() {
        let args = Args {
            zwift_score: Some(195),
            duration: 60,
            tolerance: 10,
            event_type: "tt".to_string(),
            days: 1,
            zwiftpower_username: None,
            debug: false,
            show_unknown_routes: false,
            analyze_descriptions: false,
            record_result: None,
            discover_routes: false,
            tags: vec![],
            exclude_tags: vec![],
            mark_complete: None,
            show_progress: false,
            new_routes_only: false,
            verbose: false,
        };
        
        let suggestions = generate_no_results_suggestions(&args);
        
        // We now have more suggestions including a note
        assert!(suggestions.len() >= 3);
        assert!(suggestions[0].contains("Time trials are less common"));
        assert!(suggestions[1].contains("-e race"));
        assert!(suggestions[2].contains("-e all"));
    }

    #[test]
    fn test_generate_no_results_suggestions_for_other() {
        let args = Args {
            zwift_score: Some(195),
            duration: 60,
            tolerance: 10,
            event_type: "all".to_string(),
            days: 1,
            zwiftpower_username: None,
            debug: false,
            show_unknown_routes: false,
            analyze_descriptions: false,
            record_result: None,
            discover_routes: false,
            tags: vec![],
            exclude_tags: vec![],
            mark_complete: None,
            show_progress: false,
            new_routes_only: false,
            verbose: false,
        };
        
        let suggestions = generate_no_results_suggestions(&args);
        
        // We now have more suggestions including "-n 3"
        assert!(suggestions.len() >= 3);
        assert!(suggestions[0].contains("No events match your duration"));
        assert!(suggestions[1].contains("-t 20")); // tolerance * 2
        assert!(suggestions[2].contains("-e all"));
    }

    #[test]
    fn test_parse_lap_count() {
        // Test basic lap parsing
        assert_eq!(parse_lap_count("3 Laps of Watopia"), Some(3));
        assert_eq!(parse_lap_count("5 laps"), Some(5));
        assert_eq!(parse_lap_count("Single lap race"), None);
        
        // Test case variations (only handles Laps or laps, not LAPS)
        assert_eq!(parse_lap_count("2 Laps"), Some(2));
        assert_eq!(parse_lap_count("4 Lap race"), Some(4));
        
        // Test with surrounding text
        assert_eq!(parse_lap_count("Race: 10 laps around Richmond"), Some(10));
        assert_eq!(parse_lap_count("Watopia Flat Route - 3 laps"), Some(3));
        
        // Test edge cases
        assert_eq!(parse_lap_count("No laps mentioned"), None);
        assert_eq!(parse_lap_count(""), None);
        assert_eq!(parse_lap_count("0 laps"), Some(0));
        assert_eq!(parse_lap_count("100 laps ultra"), Some(100));
    }

    #[test]
    fn test_parse_distance_from_name() {
        // Test km parsing
        assert_eq!(parse_distance_from_name("10km race"), Some(10.0));
        assert_eq!(parse_distance_from_name("Marathon 42.2 km"), Some(42.2));
        assert_eq!(parse_distance_from_name("5.5km time trial"), Some(5.5));
        
        // Test miles parsing with conversion
        let miles_10 = parse_distance_from_name("10mi race").unwrap();
        assert!((miles_10 - 16.0934).abs() < 0.001, "Expected ~16.093, got {}", miles_10);
        
        let miles_26_2 = parse_distance_from_name("Marathon 26.2 miles").unwrap();
        assert!((miles_26_2 - 42.164708).abs() < 0.001, "Expected ~42.165, got {}", miles_26_2);
        
        // Test with spaces
        assert_eq!(parse_distance_from_name("25 km criterium"), Some(25.0));
        assert_eq!(parse_distance_from_name("Distance: 15 km"), Some(15.0));
        
        // Test no distance
        assert_eq!(parse_distance_from_name("Watopia Flat Route"), None);
        assert_eq!(parse_distance_from_name(""), None);
        
        // Test that km is preferred over miles
        assert_eq!(parse_distance_from_name("10km or 6.2mi"), Some(10.0));
    }

    #[test]
    fn test_find_user_subgroup() {
        // Create test event with subgroups
        let mut event = ZwiftEvent {
            id: 1,
            name: "Test Race".to_string(),
            description: None,
            event_start: chrono::Utc::now(),
            event_type: "RACE".to_string(),
            distance_in_meters: Some(20000.0),
            duration_in_minutes: None,
            duration_in_seconds: None,
            route_id: Some(1),
            route: None,
            category_enforcement: false,
            sport: "CYCLING".to_string(),
            tags: vec![],
            event_sub_groups: vec![
                EventSubGroup {
                    id: 1,
                    name: "A".to_string(),
                    route_id: None,
                    distance_in_meters: Some(20000.0),
                    duration_in_minutes: None,
                    category_enforcement: None,
                    range_access_label: None,
                    laps: None,
                },
                EventSubGroup {
                    id: 2,
                    name: "B".to_string(),
                    route_id: None,
                    distance_in_meters: Some(20000.0),
                    duration_in_minutes: None,
                    category_enforcement: None,
                    range_access_label: None,
                    laps: None,
                },
                EventSubGroup {
                    id: 3,
                    name: "C".to_string(),
                    route_id: None,
                    distance_in_meters: Some(15000.0),
                    duration_in_minutes: None,
                    category_enforcement: None,
                    range_access_label: None,
                    laps: None,
                },
                EventSubGroup {
                    id: 4,
                    name: "D".to_string(),
                    route_id: None,
                    distance_in_meters: Some(15000.0),
                    duration_in_minutes: None,
                    category_enforcement: None,
                    range_access_label: None,
                    laps: None,
                },
                EventSubGroup {
                    id: 5,
                    name: "E".to_string(),
                    route_id: None,
                    distance_in_meters: Some(10000.0),
                    duration_in_minutes: None,
                    category_enforcement: None,
                    range_access_label: None,
                    laps: None,
                },
            ],
        };

        // Test category E (0-99)
        let subgroup = find_user_subgroup(&event, 50).unwrap();
        assert_eq!(subgroup.name, "E");
        
        // Test category D (100-199)
        let subgroup = find_user_subgroup(&event, 150).unwrap();
        assert_eq!(subgroup.name, "D");
        
        // Test category C (200-299)
        let subgroup = find_user_subgroup(&event, 250).unwrap();
        assert_eq!(subgroup.name, "C");
        
        // Test category B (300-399)
        let subgroup = find_user_subgroup(&event, 350).unwrap();
        assert_eq!(subgroup.name, "B");
        
        // Test category A (400-599)
        let subgroup = find_user_subgroup(&event, 450).unwrap();
        assert_eq!(subgroup.name, "A");
        
        // Test edge cases
        assert_eq!(find_user_subgroup(&event, 0).unwrap().name, "E");
        assert_eq!(find_user_subgroup(&event, 99).unwrap().name, "E");
        assert_eq!(find_user_subgroup(&event, 100).unwrap().name, "D");
        assert_eq!(find_user_subgroup(&event, 199).unwrap().name, "D");
        assert_eq!(find_user_subgroup(&event, 200).unwrap().name, "C");
        assert_eq!(find_user_subgroup(&event, 299).unwrap().name, "C");
        assert_eq!(find_user_subgroup(&event, 300).unwrap().name, "B");
        assert_eq!(find_user_subgroup(&event, 399).unwrap().name, "B");
        assert_eq!(find_user_subgroup(&event, 400).unwrap().name, "A");
        assert_eq!(find_user_subgroup(&event, 599).unwrap().name, "A");
        
        // Test A+ category (600+)
        // Note: Since we don't have an A+ subgroup in the test, A+ riders would match A
        let subgroup = find_user_subgroup(&event, 700);
        assert!(subgroup.is_none() || subgroup.unwrap().name == "A");
        
        // Test that D riders can join E events
        let subgroup = find_user_subgroup(&event, 150).unwrap();
        assert_eq!(subgroup.name, "D");
        // Also verify that a D rider (150) could join an E event if only E was available
        event.event_sub_groups.retain(|sg| sg.name == "E");
        let subgroup = find_user_subgroup(&event, 150).unwrap();
        assert_eq!(subgroup.name, "E");
        
        // Test empty subgroups
        event.event_sub_groups.clear();
        assert!(find_user_subgroup(&event, 250).is_none());
    }

    #[test]
    fn test_get_route_difficulty_multiplier_from_elevation() {
        // Test very flat routes (< 5m/km)
        assert_eq!(get_route_difficulty_multiplier_from_elevation(20.0, 50), 1.1);  // 2.5m/km
        assert_eq!(get_route_difficulty_multiplier_from_elevation(10.0, 40), 1.1);  // 4m/km
        
        // Test flat to rolling (5-10m/km)
        assert_eq!(get_route_difficulty_multiplier_from_elevation(20.0, 150), 1.0); // 7.5m/km
        assert_eq!(get_route_difficulty_multiplier_from_elevation(30.0, 270), 1.0); // 9m/km
        
        // Test rolling hills (10-20m/km)
        assert_eq!(get_route_difficulty_multiplier_from_elevation(20.0, 300), 0.9); // 15m/km
        assert_eq!(get_route_difficulty_multiplier_from_elevation(40.0, 760), 0.9); // 19m/km
        
        // Test hilly (20-40m/km)
        assert_eq!(get_route_difficulty_multiplier_from_elevation(20.0, 500), 0.8); // 25m/km
        assert_eq!(get_route_difficulty_multiplier_from_elevation(30.0, 1100), 0.8); // 36.7m/km
        
        // Test very hilly (> 40m/km)
        assert_eq!(get_route_difficulty_multiplier_from_elevation(20.0, 1000), 0.7); // 50m/km
        assert_eq!(get_route_difficulty_multiplier_from_elevation(12.0, 1035), 0.7); // 86.25m/km (Alpe du Zwift)
        
        // Test edge cases
        assert_eq!(get_route_difficulty_multiplier_from_elevation(100.0, 0), 1.1);   // 0m/km - perfectly flat
        assert_eq!(get_route_difficulty_multiplier_from_elevation(5.0, 250), 0.7);   // 50m/km - extreme climb
    }

    #[test]
    fn test_get_route_difficulty_multiplier() {
        // Test very hilly routes
        assert_eq!(get_route_difficulty_multiplier("Alpe du Zwift"), 0.7);
        assert_eq!(get_route_difficulty_multiplier("Road to Sky"), 1.0);  // doesn't contain special keywords
        assert_eq!(get_route_difficulty_multiplier("Ven-Top"), 1.0);     // "ventoux" not "ven"
        assert_eq!(get_route_difficulty_multiplier("Mont Ventoux"), 0.7); // contains "ventoux"
        assert_eq!(get_route_difficulty_multiplier("ALPE DU ZWIFT"), 0.7); // case insensitive
        
        // Test hilly routes
        assert_eq!(get_route_difficulty_multiplier("Epic KOM"), 0.8);
        assert_eq!(get_route_difficulty_multiplier("Mountain Route"), 0.8);
        assert_eq!(get_route_difficulty_multiplier("The Mega Pretzel"), 1.0); // doesn't contain "epic" or "mountain"
        
        // Test flat routes
        assert_eq!(get_route_difficulty_multiplier("Tempus Fugit"), 1.1);
        assert_eq!(get_route_difficulty_multiplier("Watopia Flat Route"), 1.1);
        assert_eq!(get_route_difficulty_multiplier("Tick Tock"), 1.0); // Doesn't contain "flat" or "tempus"
        
        // Test default routes
        assert_eq!(get_route_difficulty_multiplier("Watopia Hilly Route"), 1.0);
        assert_eq!(get_route_difficulty_multiplier("Richmond UCI Worlds"), 1.0);
        assert_eq!(get_route_difficulty_multiplier("London Loop"), 1.0);
        
        // Test mixed case and partial matches
        assert_eq!(get_route_difficulty_multiplier("the EPIC kom reverse"), 0.8);
        assert_eq!(get_route_difficulty_multiplier("Flat is Fast"), 1.1);
    }

    #[test]
    fn test_generate_no_results_suggestions() {
        // Test race suggestions
        let race_args = Args {
            event_type: "race".to_string(),
            duration: 60,
            tolerance: 15,
            days: 3,
            zwift_score: None,
            zwiftpower_username: None,
            tags: vec![],
            exclude_tags: vec![],
            show_unknown_routes: false,
            analyze_descriptions: false,
            record_result: None,
            discover_routes: false,
            mark_complete: None,
            show_progress: false,
            new_routes_only: false,
            verbose: false,
            debug: false,
        };
        let suggestions = generate_no_results_suggestions(&race_args);
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("Most races are short"));
        assert!(suggestions[1].contains("cargo run -- -d 30 -t 30"));
        
        // Test time trial suggestions
        let tt_args = Args {
            event_type: "tt".to_string(),
            duration: 60,
            tolerance: 15,
            days: 3,
            zwift_score: None,
            zwiftpower_username: None,
            tags: vec![],
            exclude_tags: vec![],
            show_unknown_routes: false,
            analyze_descriptions: false,
            record_result: None,
            discover_routes: false,
            mark_complete: None,
            show_progress: false,
            new_routes_only: false,
            verbose: false,
            debug: false,
        };
        let suggestions = generate_no_results_suggestions(&tt_args);
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("Time trials are less common"));
        
        // Test generic suggestions
        let generic_args = Args {
            event_type: "workout".to_string(),
            duration: 60,
            tolerance: 15,
            days: 3,
            zwift_score: None,
            zwiftpower_username: None,
            tags: vec![],
            exclude_tags: vec![],
            show_unknown_routes: false,
            analyze_descriptions: false,
            record_result: None,
            discover_routes: false,
            mark_complete: None,
            show_progress: false,
            new_routes_only: false,
            verbose: false,
            debug: false,
        };
        let suggestions = generate_no_results_suggestions(&generic_args);
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("No events match"));
        assert!(suggestions[1].contains("30")); // Should suggest wider tolerance (15*2)
    }


}

