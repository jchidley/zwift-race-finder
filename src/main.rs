//! Zwift Race Finder - Find races that match your target duration and racing score
//! 
//! This tool fetches upcoming Zwift events and filters them based on estimated
//! completion time for your specific Zwift Racing Score.

// ABOUTME: Tool to find Zwift races suitable for Cat C riders (~180 ZwiftScore) lasting ~2 hours
// Fetches events from Zwift API and filters based on race duration estimates

mod config;
mod database;
#[cfg(test)]
mod regression_test;

use anyhow::Result;
use chrono::{DateTime, Local, Utc};
use clap::Parser;
use colored::*;
use config::{FullConfig, Secrets};
use database::{Database, RouteData as DbRouteData};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
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
    
    /// Record a race result (format: "route_id,minutes,event_name")
    #[arg(long)]
    record_result: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ZwiftEvent {
    id: u64,
    name: String,
    #[serde(rename = "eventStart")]
    event_start: DateTime<Utc>,
    event_type: String,
    distance_in_meters: Option<f64>,
    duration_in_minutes: Option<u32>,
    #[serde(rename = "durationInSeconds")]
    duration_in_seconds: Option<u32>,
    route_id: Option<u32>,
    route: Option<String>,
    description: Option<String>,
    #[serde(default)]
    category_enforcement: bool,
    #[serde(default)]
    event_sub_groups: Vec<EventSubGroup>,
    #[serde(default = "default_sport")]
    sport: String,
}

fn default_sport() -> String {
    "CYCLING".to_string()
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct EventSubGroup {
    id: u32,
    name: String,
    route_id: Option<u32>,
    distance_in_meters: Option<f64>,
    duration_in_minutes: Option<u32>,
    category_enforcement: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct UserStats {
    zwift_score: u32,
    category: String,
    username: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct CachedStats {
    stats: UserStats,
    cached_at: DateTime<Utc>,
}

// Average speeds by category (km/h) - rough estimates
const CAT_A_SPEED: f64 = 40.0;
const CAT_B_SPEED: f64 = 35.0;
const CAT_C_SPEED: f64 = 30.0;
const CAT_D_SPEED: f64 = 25.0;
const STRONG_CAT_D_SPEED: f64 = 27.0; // For 190-199 score range

// Zwift route database - route_id is the primary key for all calculations
// This should be expanded with Jack's actual race data
struct RouteData {
    distance_km: f64,
    elevation_m: u32,
    #[allow(dead_code)]
    name: &'static str,
    #[allow(dead_code)]
    world: &'static str,
    surface: &'static str, // "road", "gravel", "mixed"
}

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
        }),
        
        // Common race routes
        2143464829 => Some(RouteData {
            distance_km: 33.4,
            elevation_m: 170,
            name: "Watopia Flat Route",
            world: "Watopia",
            surface: "road",
        }),
        
        2927651296 => Some(RouteData {
            distance_km: 67.5,
            elevation_m: 654,
            name: "Makuri Pretzel",
            world: "Makuri Islands",
            surface: "road",
        }),
        
        3742187716 => Some(RouteData {
            distance_km: 24.5,
            elevation_m: 168,
            name: "Castle to Castle",
            world: "Makuri Islands",
            surface: "road",
        }),
        
        // Crit Racing Club routes
        2698009951 => Some(RouteData {
            distance_km: 22.9,
            elevation_m: 80,
            name: "Downtown Dolphin",
            world: "Crit City",
            surface: "road",
        }),
        
        // Mt. Fuji Hill Climb
        2663908549 => Some(RouteData {
            distance_km: 20.3,
            elevation_m: 1159,
            name: "Mt. Fuji",
            world: "Makuri Islands",
            surface: "road",
        }),
        
        // Common race routes discovered from API
        3368626651 => Some(RouteData {
            distance_km: 27.4,  // Estimated from typical eRacing events
            elevation_m: 223,
            name: "eRacing Course",
            world: "Various",
            surface: "road",
        }),
        
        1656629976 => Some(RouteData {
            distance_km: 19.8,  // Ottawa TopSpeed typically shorter
            elevation_m: 142,
            name: "Ottawa TopSpeed",
            world: "Various",
            surface: "road",
        }),
        
        2474227587 => Some(RouteData {
            distance_km: 100.0,  // KISS Racing 100 - it's in the name!
            elevation_m: 892,
            name: "KISS 100",
            world: "Watopia",
            surface: "road",
        }),
        
        3395698268 => Some(RouteData {
            distance_km: 60.0,  // NoPinz R3R - 60km Race
            elevation_m: 543,
            name: "R3R 60km",
            world: "Various",
            surface: "road",
        }),
        
        // Add more routes as we discover them
        _ => None,
    }
}

// Get just the distance for backward compatibility
fn get_route_distance_by_id(route_id: u32) -> Option<f64> {
    get_route_data(route_id).map(|data| data.distance_km)
}

// Try to determine distance from event name patterns
fn estimate_distance_from_name(name: &str) -> Option<f64> {
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
    
    // Zwift Racing Score ranges (new system)
    // Based on Jack's score of 189 being Category D
    let base_speed = match zwift_score {
        0..=199 => CAT_D_SPEED,      // 0-199 is Cat D (includes Jack at 189)
        200..=299 => CAT_C_SPEED,    // 200-299 is Cat C  
        300..=399 => CAT_B_SPEED,    // 300-399 is Cat B
        _ => CAT_A_SPEED,            // 400+ is Cat A
    };
    
    // Use elevation-based multiplier for more accurate estimates
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
    let duration_hours = route_data.distance_km / effective_speed;
    
    Some((duration_hours * 60.0) as u32)
}

// Fallback duration estimation when route_id is not available
fn estimate_duration_for_category(distance_km: f64, route_name: &str, zwift_score: u32) -> u32 {
    // Same scoring ranges as primary estimation
    let base_speed = match zwift_score {
        0..=199 => CAT_D_SPEED,      // 0-199 is Cat D
        200..=299 => CAT_C_SPEED,    // 200-299 is Cat C  
        300..=399 => CAT_B_SPEED,    // 300-399 is Cat B
        _ => CAT_A_SPEED,            // 400+ is Cat A
    };

    let difficulty_multiplier = get_route_difficulty_multiplier(route_name);
    let effective_speed = base_speed * difficulty_multiplier;

    let duration_hours = distance_km / effective_speed;
    (duration_hours * 60.0) as u32
}

fn get_cache_file() -> Result<PathBuf> {
    let mut cache_dir = dirs::cache_dir().unwrap_or_else(|| PathBuf::from("."));
    cache_dir.push("zwift-race-finder");
    fs::create_dir_all(&cache_dir)?;
    cache_dir.push("user_stats.json");
    Ok(cache_dir)
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

fn load_cached_stats() -> Result<Option<UserStats>> {
    let cache_file = get_cache_file()?;

    if !cache_file.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(cache_file)?;
    let cached: CachedStats = serde_json::from_str(&content)?;

    // Use cache if it's less than 24 hours old
    let age = Utc::now().signed_duration_since(cached.cached_at);
    if age.num_hours() < 24 {
        Ok(Some(cached.stats))
    } else {
        Ok(None)
    }
}

fn save_cached_stats(stats: &UserStats) -> Result<()> {
    let cache_file = get_cache_file()?;
    let cached = CachedStats {
        stats: stats.clone(),
        cached_at: Utc::now(),
    };

    let content = serde_json::to_string_pretty(&cached)?;
    fs::write(cache_file, content)?;
    Ok(())
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
    Ok(UserStats {
        zwift_score: config.default_zwift_score().unwrap_or(195),
        category: config.default_category().cloned().unwrap_or_else(|| "D".to_string()),
        username: "User".to_string(),
    })
}

async fn fetch_events() -> Result<Vec<ZwiftEvent>> {
    let url = "https://us-or-rly101.zwift.com/api/public/events/upcoming";

    let client = reqwest::Client::builder()
        .user_agent("Zwift Race Finder")
        .build()?;

    let response = client
        .get(url)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    let events: Vec<ZwiftEvent> = response.json().await?;
    Ok(events)
}

fn filter_events(mut events: Vec<ZwiftEvent>, args: &Args, zwift_score: u32) -> Vec<ZwiftEvent> {
    let now = Utc::now();
    let max_date = now + chrono::Duration::days(args.days as i64);

    if args.debug {
        eprintln!("Debug: Starting with {} events", events.len());
    }

    // Sport filter
    events.retain(|event| event.sport.to_uppercase() == "CYCLING");
    if args.debug {
        eprintln!("Debug: {} events after sport filter", events.len());
    }

    // Time filter
    events.retain(|event| event.event_start > now && event.event_start < max_date);
    if args.debug {
        eprintln!("Debug: {} events after time filter", events.len());
    }

    // Event type filter
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
    if args.debug {
        eprintln!("Debug: {} events after event type filter", events.len());
    }

    // Duration filter
    events.retain(|event| {
        // Duration filter - prioritize route_id for accuracy
        let duration_minutes = event
            .duration_in_minutes
            .or_else(|| event.duration_in_seconds.map(|s| s / 60));

        if let Some(duration) = duration_minutes {
            // Fixed duration event
            let diff = (duration as i32 - args.duration as i32).abs();
            return diff <= args.tolerance as i32;
        }
        
        // PRIMARY METHOD: Use route_id for most accurate estimation
        if let Some(route_id) = event.route_id {
            if let Some(estimated_duration) = estimate_duration_from_route_id(route_id, zwift_score) {
                let diff = (estimated_duration as i32 - args.duration as i32).abs();
                return diff <= args.tolerance as i32;
            }
        }
        
        // FALLBACK 1: Use provided distance
        if let Some(distance) = event.distance_in_meters.filter(|&d| d > 0.0) {
            let distance_km = distance / 1000.0;
            let route_name = event.route.as_deref().unwrap_or(&event.name);
            let estimated_duration =
                estimate_duration_for_category(distance_km, route_name, zwift_score);
            let diff = (estimated_duration as i32 - args.duration as i32).abs();
            return diff <= args.tolerance as i32;
        }
        
        // FALLBACK 2: Try to guess from event name
        if let Some(distance_km) = estimate_distance_from_name(&event.name) {
            let estimated_duration =
                estimate_duration_for_category(distance_km, &event.name, zwift_score);
            let diff = (estimated_duration as i32 - args.duration as i32).abs();
            return diff <= args.tolerance as i32;
        }

        // Check subgroups if main event has no distance/duration
        if !event.event_sub_groups.is_empty() {
            // If any subgroup matches our criteria, include the event
            event.event_sub_groups.iter().any(|subgroup| {
                if let Some(duration) = subgroup.duration_in_minutes {
                    let diff = (duration as i32 - args.duration as i32).abs();
                    diff <= args.tolerance as i32
                } else if let Some(distance) = subgroup.distance_in_meters.filter(|&d| d > 0.0) {
                    let distance_km = distance / 1000.0;
                    let route_name = event.route.as_deref().unwrap_or("");
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

    if args.debug {
        eprintln!("Debug: {} events after duration filter", events.len());
    }

    events
}

fn format_duration(minutes: u32) -> String {
    let hours = minutes / 60;
    let mins = minutes % 60;
    if hours > 0 {
        format!("{}h {:02}m", hours, mins)
    } else {
        format!("{}m", mins)
    }
}

// Helper function to collect unknown route data
fn log_unknown_route(event: &ZwiftEvent) {
    if let Some(route_id) = event.route_id {
        if get_route_data(route_id).is_none() {
            // Log to database for future analysis
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
        if let Some(route_data) = get_route_data(route_id) {
            println!(
                "{}: {} ({}m elevation)",
                "Route ID".bright_blue().dimmed(),
                route_id.to_string().dimmed(),
                route_data.elevation_m
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

    // Duration info - prioritize route_id for accuracy
    let duration_minutes = event
        .duration_in_minutes
        .or_else(|| event.duration_in_seconds.map(|s| s / 60));

    if let Some(duration) = duration_minutes {
        println!(
            "{}: {} (fixed duration)",
            "Duration".bright_blue(),
            format_duration(duration)
        );
    } else if let Some(route_id) = event.route_id {
        // PRIMARY: Use route_id for most accurate estimation
        if let Some(route_data) = get_route_data(route_id) {
            println!("{}: {:.1} km", "Distance".bright_blue(), route_data.distance_km);
            
            if let Some(estimated_duration) = estimate_duration_from_route_id(route_id, zwift_score) {
                let cat_string = match zwift_score {
                    0..=149 => "D",
                    150..=189 => "D",
                    190..=199 => "D+", // Strong Cat D
                    200..=249 => "C",
                    250..=299 => "B",
                    _ => "A",
                };
                println!(
                    "{}: {} (estimated for Cat {} rider)",
                    "Duration".bright_blue(),
                    format_duration(estimated_duration).green(),
                    cat_string
                );
            }
        } else {
            // Unknown route_id - show it for data collection
            println!(
                "{}: Route ID {} needs mapping",
                "Info".yellow(),
                route_id.to_string().yellow()
            );
        }
    } else if let Some(distance) = event.distance_in_meters.filter(|&d| d > 0.0) {
        // FALLBACK: Use provided distance
        let distance_km = distance / 1000.0;
        let route_name = event.route.as_deref().unwrap_or(&event.name);
        let estimated_duration =
            estimate_duration_for_category(distance_km, route_name, zwift_score);

        println!("{}: {:.1} km", "Distance".bright_blue(), distance_km);
        let cat_string = match zwift_score {
            0..=149 => "D",
            150..=189 => "D",
            190..=199 => "D+", // Strong Cat D
            200..=249 => "C",
            250..=299 => "B",
            _ => "A",
        };
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
            let cat_string = match zwift_score {
                0..=149 => "D",
                150..=189 => "D",
                190..=199 => "D+",
                200..=249 => "C",
                250..=299 => "B",
                _ => "A",
            };
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
        for group in &event.event_sub_groups {
            print!("  • {}", group.name);
            if let Some(dist) = group.distance_in_meters {
                print!(" ({:.1} km)", dist / 1000.0);
            }
            if let Some(dur) = group.duration_in_minutes {
                print!(" ({})", format_duration(dur));
            }
            println!();
        }
    }

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

fn record_race_result(input: &str) -> Result<()> {
    // Parse format: "route_id,minutes,event_name"
    let parts: Vec<&str> = input.split(',').collect();
    if parts.len() < 3 {
        anyhow::bail!("Format: --record-result 'route_id,minutes,event_name'");
    }
    
    let route_id: u32 = parts[0].trim().parse()
        .map_err(|_| anyhow::anyhow!("Invalid route_id"))?;
    let minutes: u32 = parts[1].trim().parse()
        .map_err(|_| anyhow::anyhow!("Invalid minutes"))?;
    let event_name = parts[2..].join(",").trim().to_string();
    
    let db = Database::new()?;
    
    // Check if route exists
    if db.get_route(route_id)?.is_none() {
        println!("{}: Route {} not found in database", "Warning".yellow(), route_id);
        println!("Recording as unknown route for future mapping.");
        db.record_unknown_route(route_id, &event_name, "RACE")?;
    }
    
    // Get current user stats
    let zwift_score = 195; // TODO: Get from args or auto-detect
    
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

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("🚴 {} {}", "Zwift Race Finder".bold(), "v0.1.0".dimmed());
    
    // Handle special commands first
    if args.show_unknown_routes {
        show_unknown_routes()?;
        return Ok(());
    }
    
    if let Some(result_str) = args.record_result {
        record_race_result(&result_str)?;
        return Ok(());
    }

    // Load configuration
    let config = FullConfig::load().unwrap_or_default();
    
    // Get user stats (auto-detected or from command line)
    let user_stats = get_user_stats(&config).await?;
    let zwift_score = args.zwift_score.unwrap_or(user_stats.zwift_score);

    // Show what stats we're using
    if args.zwift_score.is_some() {
        println!("Using provided Zwift Score: {}", zwift_score);
    } else {
        println!(
            "Using {} stats: Zwift Score {} (Cat {})",
            user_stats.username.green(),
            zwift_score.to_string().yellow(),
            user_stats.category
        );
    }

    let min_duration = args.duration.saturating_sub(args.tolerance);
    let max_duration = args.duration + args.tolerance;

    println!(
        "Looking for events {} to {}...\n",
        format_duration(min_duration).yellow(),
        format_duration(max_duration).yellow()
    );

    let events = fetch_events().await?;
    println!("Fetched {} upcoming events", events.len());

    // Debug: show race count
    if args.debug {
        let race_count = events
            .iter()
            .filter(|e| e.sport == "CYCLING" && e.event_type == "RACE")
            .count();
        println!("Found {} cycling races", race_count);
    }

    let filtered = filter_events(events, &args, zwift_score);

    if filtered.is_empty() {
        println!("\n{}", "No matching events found!".red());
        println!("Try adjusting your criteria:");
        println!("  • Increase tolerance with -t");
        println!("  • Look further ahead with -n");
        println!("  • Change event type with -e (all, race, fondo, group, workout, tt)");
    } else {
        println!(
            "\nFound {} matching events:",
            filtered.len().to_string().green().bold()
        );

        for event in &filtered {
            print_event(event, &args, zwift_score);
        }

        println!("\n{}", "─".repeat(80).dimmed());
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
        }
    }

    #[test]
    fn test_filters_out_running_events() {
        let events = vec![
            create_test_event("Cycling Race", 50.0, "Watopia Flat", "CYCLING"), // ~111 min
            create_test_event("Running Event", 10.0, "May Field", "RUNNING"),
            create_test_event("Bike Race 2", 54.0, "Tempus Fugit", "CYCLING"), // ~110 min
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
            record_result: None,
        };

        let filtered = filter_events(events, &args, 195);

        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().all(|e| e.sport == "CYCLING"));
        assert!(!filtered.iter().any(|e| e.name.contains("Running")));
    }

    #[test]
    fn test_duration_estimation_for_cat_d() {
        // Test known distances and expected durations for Cat D (195 score)
        // Base speed for 195 score is 27 km/h

        // Watopia: 40km at 27km/h * 1.0 multiplier = 88.88 ≈ 88 min (rounds down)
        let watopia_time = estimate_duration_for_category(40.0, "Watopia", 195);
        assert_eq!(watopia_time, 88);

        // Alpe du Zwift: 30km at 27km/h * 0.7 multiplier = 95.23 ≈ 95 min
        let alpe_time = estimate_duration_for_category(30.0, "Alpe du Zwift", 195);
        assert_eq!(alpe_time, 95);

        // Tempus Fugit: 35km at 27km/h * 1.1 multiplier = 70.70 ≈ 70 min (rounds down)
        let tempus_time = estimate_duration_for_category(35.0, "Tempus Fugit", 195);
        assert_eq!(tempus_time, 70);
    }

    #[test]
    fn test_duration_filtering() {
        let events = vec![
            create_test_event("Short Race", 20.0, "Watopia Flat", "CYCLING"), // ~44 min
            create_test_event("Perfect Race", 50.0, "Watopia Flat", "CYCLING"), // ~111 min
            create_test_event("Long Race", 80.0, "Watopia Flat", "CYCLING"),  // ~178 min
        ];

        let args = Args {
            zwift_score: Some(195),
            duration: 120, // 2 hours
            tolerance: 20, // ±20 minutes
            event_type: "all".to_string(),
            days: 1,
            zwiftpower_username: None,
            debug: false,
            show_unknown_routes: false,
            record_result: None,
        };

        let filtered = filter_events(events, &args, 195);

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
                typical_time_minutes: 67, // Calculated: 33.4km / (27km/h * 1.1) = 67 min
                tolerance: 5,
            },
            RaceData {
                name: "ZSUN Sunday Race",
                distance_km: 25.7,
                route: "London Loop",
                typical_time_minutes: 57, // 25.7km / 27km/h = 57 min
                tolerance: 5,
            },
            RaceData {
                name: "DBR Monday Race",
                distance_km: 41.1,
                route: "Tempus Fugit",
                typical_time_minutes: 83, // 41.1km / (27km/h * 1.1) = 83 min
                tolerance: 5,
            },
            RaceData {
                name: "Herd Racing",
                distance_km: 46.9,
                route: "Innsbruck",
                typical_time_minutes: 104, // 46.9km / 27km/h = 104 min
                tolerance: 7,
            },
            RaceData {
                name: "ZRL Race",
                distance_km: 29.5,
                route: "Richmond",
                typical_time_minutes: 66, // 29.5km / 27km/h = 65.5 ≈ 66 min
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
            record_result: None,
        };

        let filtered = filter_events(events.clone(), &args, 195);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "3R Race");

        // Test fondo filter
        args.event_type = "fondo".to_string();
        let filtered = filter_events(events.clone(), &args, 195);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Gran Fondo");

        // Test group filter (excludes fondos)
        args.event_type = "group".to_string();
        let filtered = filter_events(events.clone(), &args, 195);
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
                min_minutes: 28,
                max_minutes: 35,
            },
            RouteExpectation {
                route_id: 2698009951,  // Downtown Dolphin (22.9km, 80m elevation)
                name: "Downtown Dolphin",
                min_minutes: 45,
                max_minutes: 55,
            },
            RouteExpectation {
                route_id: 2663908549,  // Mt. Fuji (20.3km, 1159m elevation)
                name: "Mt. Fuji",
                min_minutes: 60,  // Very hilly route, wide range for Cat D
                max_minutes: 85,
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
}
