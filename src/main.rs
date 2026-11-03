mod commands;
use zwift_race_finder::{api, config, database};
mod zwiftpower;

use anyhow::Result;
use chrono::Utc;
use clap::Parser;
use colored::*;
use config::FullConfig;
use zwift_race_finder::category::*;
use zwift_race_finder::errors::*;
use zwift_race_finder::estimation::*;
use zwift_race_finder::event_analysis::*;
use zwift_race_finder::event_display::{
    display_filter_stats, log_unknown_route, print_event, print_events_table,
};
use zwift_race_finder::event_filtering::*;
use zwift_race_finder::formatting::*;
use zwift_race_finder::models::*;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "Target Zwift racing score (auto-detected from ZwiftPower if available)")]
    #[arg(short = 's', long)]
    zwift_score: Option<u32>,

    #[arg(help = "Target race duration in minutes (default: 120)")]
    #[arg(short = 'd', long, default_value = "120")]
    duration: u32,

    #[arg(help = "Duration tolerance in minutes (default: 30 for 1.5-2.5h range)")]
    #[arg(short = 't', long, default_value = "30")]
    tolerance: u32,

    #[arg(help = "Event type filter: all, race, fondo, group, workout, tt (time trial)")]
    #[arg(short = 'e', long, default_value = "race")]
    event_type: String,

    #[arg(help = "Show next N days of events (default: 1)")]
    #[arg(short = 'n', long, default_value = "1")]
    days: u32,

    #[arg(help = "Debug mode - show why events are filtered out")]
    #[arg(long)]
    debug: bool,

    #[arg(help = "Show event route IDs missing from the local route catalogue")]
    #[arg(long)]
    show_unknown_routes: bool,

    #[arg(help = "Filter by event tags (e.g., \"ranked\", \"zracing\", \"jerseyunlock\")")]
    #[arg(long, value_delimiter = ',')]
    tags: Vec<String>,

    #[arg(help = "Exclude events with specific tags")]
    #[arg(long, value_delimiter = ',')]
    exclude_tags: Vec<String>,

    #[arg(help = "Use verbose output format (default: compact table)")]
    #[arg(short = 'v', long)]
    verbose: bool,
}




fn generate_no_results_suggestions(args: &Args) -> Vec<String> {
    let mut suggestions = Vec::new();

    if args.event_type == "race" {
        suggestions.push("Most races are short (20-30 minutes). Try:".to_string());
        suggestions.push(format!(
            "  • {} for short races",
            "cargo run -- -d 30 -t 30".cyan()
        ));
        suggestions.push(format!(
            "  • {} for any race duration",
            "cargo run -- -d 60 -t 120".cyan()
        ));
        suggestions.push(format!(
            "  • {} for time trials instead",
            "cargo run -- -e tt".cyan()
        ));
        suggestions.push("".to_string());
        suggestions.push("Common race durations:".to_string());
        suggestions.push("  • Crit races: 15-25 minutes".to_string());
        suggestions.push("  • Short courses: 25-35 minutes".to_string());
        suggestions.push("  • Endurance races: 60-90 minutes".to_string());
    } else if args.event_type == "tt" || args.event_type == "time_trial" {
        suggestions.push("Time trials are less common. Try:".to_string());
        suggestions.push(format!(
            "  • {} for regular races",
            "cargo run -- -e race -d 30 -t 30".cyan()
        ));
        suggestions.push(format!(
            "  • {} for all event types",
            "cargo run -- -e all".cyan()
        ));
        suggestions.push("".to_string());
        suggestions.push(
            "Note: Time trials are usually scheduled events, not always available.".to_string(),
        );
    } else if args.event_type == "group" {
        suggestions.push("Group rides vary widely in duration. Try:".to_string());
        suggestions.push(format!(
            "  • {} for social rides",
            "cargo run -- -e group -d 60 -t 30".cyan()
        ));
        suggestions.push(format!(
            "  • {} for endurance rides",
            "cargo run -- -e group -d 120 -t 60".cyan()
        ));
    } else {
        suggestions.push("No events match your duration criteria. Try:".to_string());
        suggestions.push(format!(
            "  • {} for wider search",
            format!("cargo run -- -t {}", args.tolerance * 2).cyan()
        ));
        suggestions.push(format!(
            "  • {} for all event types",
            "cargo run -- -e all".cyan()
        ));
        suggestions.push(format!(
            "  • {} to see more days",
            "cargo run -- -n 3".cyan()
        ));
    }

    if args.days > 1 {
        suggestions.push("".to_string());
        suggestions.push("⚠️  Note: The Zwift API only returns ~12 hours of events regardless of days requested.".to_string());
    }

    suggestions
}



fn generate_filter_description(args: &Args, min_duration: u32, max_duration: u32) -> String {
    let mut parts = Vec::new();

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

    parts.push(format!("{}-{} min", min_duration, max_duration));

    if args.days == 1 {
        parts.push("next 24h".to_string());
    } else {
        parts.push(format!("next {} days", args.days));
    }

    if !args.tags.is_empty() {
        let tags_str = args.tags.join(", ");
        parts.push(format!("with tags: {}", tags_str));
    }

    if !args.exclude_tags.is_empty() {
        let exclude_str = args.exclude_tags.join(", ");
        parts.push(format!("excluding: {}", exclude_str));
    }

    parts.join(" | ")
}

fn filter_events(
    mut events: Vec<ZwiftEvent>,
    args: &Args,
    zwift_score: u32,
) -> (Vec<ZwiftEvent>, FilterStats) {
    let now = Utc::now();
    let max_date = now + chrono::Duration::days(args.days as i64);
    let mut stats = FilterStats::default();

    if args.debug {
        eprintln!("Debug: Starting with {} events", events.len());
    }

    stats.sport_filtered = filter_by_sport(&mut events);
    if args.debug {
        eprintln!("Debug: {} events after sport filter", events.len());
    }

    stats.time_filtered = filter_by_time(&mut events, now, max_date);
    if args.debug {
        eprintln!("Debug: {} events after time filter", events.len());
    }

    stats.type_filtered = filter_by_event_type(&mut events, &args.event_type);
    if args.debug {
        eprintln!("Debug: {} events after event type filter", events.len());
    }

    stats.tag_filtered = filter_by_tags(&mut events, &args.tags);
    if args.debug && !args.tags.is_empty() {
        eprintln!("Debug: {} events after tag filter", events.len());
    }

    stats.tag_filtered += filter_by_excluded_tags(&mut events, &args.exclude_tags);
    if args.debug && !args.exclude_tags.is_empty() {
        eprintln!("Debug: {} events after exclude tag filter", events.len());
    }

    events.retain(|event| match resolve_event(event, zwift_score) {
        Ok(estimate) => {
            let matches = estimate.duration_minutes.abs_diff(args.duration) <= args.tolerance;
            if !matches {
                stats.duration_filtered += 1;
            }
            matches
        }
        Err(reason) => {
            if reason == EstimateUnavailable::NoEligibleSubgroup {
                stats.subgroup_filtered += 1;
            } else {
                stats.duration_unknown += 1;
            }
            if args.debug {
                eprintln!("Debug: duration unavailable for '{}': {reason}", event.name);
            }
            false
        }
    });

    if args.debug {
        eprintln!("Debug: {} events after duration filter", events.len());
        eprintln!(
            "Debug: {} events filtered out by duration criteria",
            stats.duration_filtered
        );
    }

    (events, stats)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!(
        "🚴 {} {}",
        "Zwift Race Finder".bold(),
        concat!("v", env!("CARGO_PKG_VERSION")).dimmed()
    );

    if args.show_unknown_routes {
        commands::show_unknown_routes()?;
        return Ok(());
    }

    if !database::Database::new()?.has_routes()? {
        eprintln!("No route metadata is installed. Export client routes with scripts/export_client_routes.ps1, then run import_client_routes --input /path/to/client-routes.json. Fixed durations remain available; personal estimates require supported measured history and course geometry. No category-speed estimates are substituted.");
    }

    let config = match FullConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!(
                "{} Failed to load configuration: {}",
                "⚠️  Warning:".yellow(),
                e
            );
            eprintln!("   Using default settings. See config.example.toml for setup instructions.");
            eprintln!();
            FullConfig::default()
        }
    };

    let duration = if args.duration == 120 && config.config.preferences.default_duration.is_some() {
        config.config.preferences.default_duration.unwrap_or(120)
    } else {
        args.duration
    };

    let tolerance = if args.tolerance == 30 && config.config.preferences.default_tolerance.is_some()
    {
        config.config.preferences.default_tolerance.unwrap_or(30)
    } else {
        args.tolerance
    };

    let days = if args.days == 1 && config.config.preferences.default_days.is_some() {
        config.config.preferences.default_days.unwrap_or(1)
    } else {
        args.days
    };

    let user_stats = zwiftpower::get_user_stats(&config).await?;
    let zwift_score = args.zwift_score.unwrap_or(user_stats.zwift_score);

    if zwift_score > 1000 {
        invalid_zwift_score(zwift_score).display();
        return Err(anyhow::anyhow!("Invalid Zwift Racing Score"));
    }

    if args.zwift_score.is_some() {
        println!("Using provided Zwift Score: {}", zwift_score);
    } else if user_stats.username == "User" {
        println!(
            "Using default Zwift Score: {} (Cat {})",
            zwift_score.to_string().yellow(),
            get_category_from_score(zwift_score)
        );
        println!(
            "{}",
            "💡 Tip: For personalized results, configure your stats in config.toml".dimmed()
        );
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

    let events = api::fetch_events().await?;

    if events.is_empty() {
        no_events_in_time_range(1).display();
        return Ok(());
    }

    println!("Fetched {} upcoming events", events.len());

    if events.len() > 250 {
        println!(
            "\n{} Zwift API returned {} events (expected ~200)",
            "🎉 Unexpected:".green(),
            events.len()
        );
        println!("   The API may have been updated to return more data!");
        println!("   Please report this at: https://github.com/anthropics/claude-code/issues");
    }

    if !events.is_empty() {
        let earliest_start = events.iter().map(|e| e.event_start).min().unwrap();
        let latest_start = events.iter().map(|e| e.event_start).max().unwrap();

        let earliest_local = earliest_start.with_timezone(&chrono::Local);
        let latest_local = latest_start.with_timezone(&chrono::Local);

        println!(
            "Events from {} to {}",
            earliest_local.format("%b %d, %l:%M %p").to_string().trim(),
            latest_local.format("%b %d, %l:%M %p").to_string().trim()
        );
    }

    if days > 1 {
        println!(
            "\n{} Zwift API only returns ~12 hours of events (200 max)",
            "⚠️  Note:".yellow()
        );
        println!("   Multi-day searches may not show all available events.");
        println!("   For best results, search specific time windows throughout the day.");
    }

    let event_counts = count_events_by_type(&events);

    if !event_counts.is_empty() {
        print!("Found: ");
        let formatted_counts: Vec<String> = event_counts
            .iter()
            .map(|(event_type, count)| format_event_type(event_type, *count))
            .collect();
        println!("{}", formatted_counts.join(", "));
    }

    if args.debug {
        let races: Vec<_> = events
            .iter()
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
                    println!(
                        "    - {}: dist={:?}m, dur={:?}min, laps={:?}, range={:?}",
                        sg.name,
                        sg.distance_in_meters,
                        sg.duration_in_minutes,
                        sg.laps,
                        sg.range_access_label
                    );
                }
            }
            println!();
        }
    }

    if args.debug {
        let race_count = events
            .iter()
            .filter(|e| e.sport.to_uppercase() == "CYCLING" && e.event_type == "RACE")
            .count();

        if race_count > 0 {
            println!("\nDebug: Found {} races, checking first few:", race_count);
            let sample_races: Vec<_> = events
                .iter()
                .filter(|e| e.sport.to_uppercase() == "CYCLING" && e.event_type == "RACE")
                .take(3)
                .collect();

            for event in sample_races {
                println!(
                    "  '{}': route_id={:?}, dist={:?}m",
                    event.name, event.route_id, event.distance_in_meters
                );

                if let Some(route_id) = event.route_id {
                    if let Some(route_data) = get_route_data(route_id) {
                        let est = estimate_event(event, zwift_score).map(|e| e.duration_minutes);
                        println!(
                            "    → Known route: {} km, would estimate {:?} min",
                            route_data.distance_km, est
                        );
                    } else {
                        println!("    → Unknown route {}", route_id);
                    }
                }
            }
        }
    }

    if args.debug {
        let race_count = events
            .iter()
            .filter(|e| e.sport == "CYCLING" && e.event_type == "RACE")
            .count();
        println!("Found {} cycling races", race_count);
    }

    for event in &events {
        if event.sport.to_uppercase() == "CYCLING" && event.event_type == "RACE" {
            log_unknown_route(event, zwift_score);
        }
    }

    let mut effective_args = args.clone();
    effective_args.duration = duration;
    effective_args.tolerance = tolerance;
    effective_args.days = days;

    let (filtered, filter_stats) = filter_events(events.clone(), &effective_args, zwift_score);

    if filter_stats.duration_unknown > 0 {
        println!(
            "Omitted {} events with unknown duration: insufficient supported personal history, course data, or format; no category-speed estimates substituted. Use --debug for reasons.",
            filter_stats.duration_unknown
        );
    }

    if filtered.is_empty() {
        println!("\n{}", "No matching events found!".red());

        if filter_stats.duration_filtered == 0 && filter_stats.duration_unknown > 0 {
            println!("No supported durations were outside the requested window. Widening tolerance cannot recover unknown durations; check supported history/course data and --debug reasons.");
            return Ok(());
        }

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
        println!(
            "  • See all available events: {}",
            "cargo run -- -e all -d 60 -t 180".cyan()
        );
        println!("  • Most races: 20-30 min | Time trials/Group rides: 60-90 min");
    } else {
        let filter_desc = generate_filter_description(&effective_args, min_duration, max_duration);
        println!(
            "\nFound {} {} matching:",
            filtered.len().to_string().green().bold(),
            filter_desc
        );

        if args.verbose {
            for event in &filtered {
                print_event(event, zwift_score);
            }
            println!("\n{}", "─".repeat(80).dimmed());
        } else {
            print_events_table(&filtered, zwift_score);
        }

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
    use zwift_race_finder::constants::METERS_PER_KILOMETER;

    impl Default for Args {
        fn default() -> Self {
            Args {
                zwift_score: Some(195),
                duration: 30,
                tolerance: 10,
                event_type: "all".to_string(),
                days: 1,
                debug: false,
                show_unknown_routes: false,
                tags: vec![],
                exclude_tags: vec![],
                verbose: false,
            }
        }
    }

    fn create_test_event(name: &str, minutes: u32, route: &str, sport: &str) -> ZwiftEvent {
        ZwiftEvent {
            id: 1,
            name: name.to_string(),
            event_start: Utc::now() + chrono::Duration::hours(2),
            event_type: "RACE".to_string(),
            distance_in_meters: Some(20.0 * METERS_PER_KILOMETER),
            duration_in_minutes: Some(minutes),
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
    fn unknown_durations_are_excluded_and_counted_separately_from_ineligible_subgroups() {
        let mut unsupported = create_test_event("Unsupported format", 45, "Test", "CYCLING");
        unsupported.event_type = "TIME_TRIAL".into();
        unsupported.duration_in_minutes = None;
        let mut missing_geometry = unsupported.clone();
        missing_geometry.name = "No geometry".into();
        missing_geometry.event_type = "RACE".into();
        missing_geometry.distance_in_meters = None;
        missing_geometry.route = None;
        let mut ineligible = create_test_event("Restricted", 45, "Test", "CYCLING");
        ineligible.event_sub_groups.push(EventSubGroup {
            id: 1,
            name: "Upper".into(),
            route_id: None,
            distance_in_meters: None,
            duration_in_minutes: None,
            duration_in_seconds: None,
            subgroup_label: None,
            category_enforcement: None,
            range_access_label: Some("500-1000".into()),
            laps: None,
        });
        let fixed = create_test_event("Fixed", 45, "Test", "CYCLING");
        let args = Args {
            duration: 45,
            tolerance: 100,
            ..Args::default()
        };
        let (matches, stats) = filter_events(
            vec![unsupported, missing_geometry, ineligible, fixed],
            &args,
            195,
        );
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].name, "Fixed");
        assert_eq!(stats.duration_unknown, 2);
        assert_eq!(stats.duration_filtered, 0);
        assert_eq!(stats.subgroup_filtered, 1);
        assert_eq!(stats.total_filtered(), 3);
    }

    #[test]
    fn test_filters_out_running_events() {
        let events = vec![
            create_test_event("Cycling Race", 116, "Watopia Flat", "CYCLING"),
            create_test_event("Running Event", 30, "May Field", "RUNNING"),
            create_test_event("Bike Race 2", 112, "Tempus Fugit", "CYCLING"),
        ];

        let args = Args {
            zwift_score: Some(195),
            duration: 120,
            tolerance: 30,
            event_type: "all".to_string(),
            days: 1,
            debug: false,
            show_unknown_routes: false,
            tags: vec![],
            exclude_tags: vec![],
            verbose: false,
        };

        let (filtered, _) = filter_events(events, &args, 195);

        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().all(|e| e.sport == "CYCLING"));
        assert!(!filtered.iter().any(|e| e.name.contains("Running")));
    }

    #[test]
    fn test_duration_filtering() {
        let events = vec![
            create_test_event("Short Race", 39, "Watopia Flat", "CYCLING"),
            create_test_event("Perfect Race", 97, "Watopia Flat", "CYCLING"),
            create_test_event("Long Race", 155, "Watopia Flat", "CYCLING"),
        ];

        let args = Args {
            zwift_score: Some(195),
            duration: 100,
            tolerance: 20,
            event_type: "all".to_string(),
            days: 1,
            debug: false,
            show_unknown_routes: false,
            tags: vec![],
            exclude_tags: vec![],
            verbose: false,
        };

        let (filtered, _) = filter_events(events, &args, 195);

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Perfect Race");
    }

    #[test]
    fn test_filter_events_duration_arithmetic() {

        let events = vec![
            create_test_event("Race 1", 29, "Short", "CYCLING"),
            create_test_event("Race 2", 31, "Target", "CYCLING"),
            create_test_event("Race 3", 33, "Long", "CYCLING"),
        ];

        let args = Args {
            duration: 31,
            tolerance: 1,
            ..Default::default()
        };

        let (filtered, _) = filter_events(events, &args, 195);

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Race 2");
    }

    #[test]
    fn test_filter_events_comparison_operators() {

        let events = vec![
            create_test_event("Exact Match", 30, "Test", "CYCLING"),
        ];

        let args = Args {
            duration: 30,
            tolerance: 0,
            ..Default::default()
        };

        let (filtered, _) = filter_events(events, &args, 195);

        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn test_match_arm_coverage() {

        assert_eq!(get_category_from_score(50), "E");
        assert_eq!(get_category_from_score(150), "D");
        assert_eq!(get_category_from_score(250), "C");
        assert_eq!(get_category_from_score(350), "B");
        assert_eq!(get_category_from_score(450), "A");
        assert_eq!(get_category_from_score(650), "A++");
    }

    #[test]
    fn test_regression_common_zwift_races() {
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
                typical_time_minutes: 58,
                tolerance: 5,
            },
            RaceData {
                name: "ZSUN Sunday Race",
                distance_km: 25.7,
                route: "London Loop",
                typical_time_minutes: 50,
                tolerance: 5,
            },
            RaceData {
                name: "DBR Monday Race",
                distance_km: 41.1,
                route: "Tempus Fugit",
                typical_time_minutes: 72,
                tolerance: 5,
            },
            RaceData {
                name: "Herd Racing",
                distance_km: 46.9,
                route: "Innsbruck",
                typical_time_minutes: 91,
                tolerance: 7,
            },
            RaceData {
                name: "ZRL Race",
                distance_km: 29.5,
                route: "Richmond",
                typical_time_minutes: 57,
                tolerance: 5,
            },
        ];

        for race in test_races {
            let estimated = zwift_race_finder::duration_estimation::estimate_duration_for_category(
                race.distance_km,
                race.route,
                195,
            );
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
        let mut race = create_test_event("3R Race", 100, "Watopia", "CYCLING");
        race.event_type = "RACE".to_string();

        let mut fondo = create_test_event("Gran Fondo", 100, "Watopia", "CYCLING");
        fondo.event_type = "GROUP_RIDE".to_string();

        let mut group = create_test_event("Sunday Ride", 100, "Watopia", "CYCLING");
        group.event_type = "GROUP_RIDE".to_string();

        let mut tt = create_test_event("Time Trial", 100, "Watopia", "CYCLING");
        tt.event_type = "TIME_TRIAL".to_string();

        let events = vec![race, fondo, group, tt];

        let mut args = Args {
            zwift_score: Some(195),
            duration: 120,
            tolerance: 60,
            event_type: "race".to_string(),
            days: 1,
            debug: false,
            show_unknown_routes: false,
            tags: vec![],
            exclude_tags: vec![],
            verbose: false,
        };

        let (filtered, _) = filter_events(events.clone(), &args, 195);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "3R Race");

        args.event_type = "fondo".to_string();
        let (filtered, _) = filter_events(events.clone(), &args, 195);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Gran Fondo");

        args.event_type = "group".to_string();
        let (filtered, _) = filter_events(events.clone(), &args, 195);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Sunday Ride");
    }

    #[tokio::test]
    #[ignore]
    async fn test_real_zwift_api_connection() {
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
                assert!(
                    resp.status().is_success(),
                    "API returned error: {}",
                    resp.status()
                );

                let body = resp.text().await.expect("Failed to read response body");
                assert!(
                    body.starts_with('['),
                    "Response doesn't look like JSON array"
                );
                assert!(body.len() > 100, "Response seems too short");
            }
            Err(e) => {
                panic!("Failed to connect to Zwift API: {}", e);
            }
        }
    }

    #[test]
    fn test_racing_score_event_filtering() {
        let racing_score_event = ZwiftEvent {
            id: 1,
            name: "Three Village Loop Race".to_string(),
            event_start: Utc::now() + chrono::Duration::hours(2),
            event_type: "RACE".to_string(),
            distance_in_meters: Some(0.0),
            duration_in_minutes: Some(20),
            duration_in_seconds: None,
            route_id: Some(3379779247),
            route: Some("Three Village Loop".to_string()),
            description: Some("Distance: 10.6 km".to_string()),
            category_enforcement: false,
            event_sub_groups: vec![EventSubGroup {
                id: 1,
                name: "0-199".to_string(),
                route_id: Some(3379779247),
                distance_in_meters: Some(0.0),
                duration_in_minutes: None,
                duration_in_seconds: None,
                subgroup_label: None,
                category_enforcement: None,
                range_access_label: Some("0-199".to_string()),
                laps: None,
            }],
            sport: "CYCLING".to_string(),
            tags: vec![],
        };

        let args = Args {
            zwift_score: Some(195),
            duration: 20,
            tolerance: 10,
            event_type: "race".to_string(),
            days: 1,
            debug: false,
            show_unknown_routes: false,
            tags: vec![],
            exclude_tags: vec![],
            verbose: false,
        };

        let events = vec![racing_score_event.clone()];
        let (filtered, _) = filter_events(events, &args, 195);

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Three Village Loop Race");
    }

    #[test]
    fn test_racing_score_event_with_zero_distance() {
        let events = vec![
            ZwiftEvent {
                id: 1,
                name: "Traditional Race".to_string(),
                event_start: Utc::now() + chrono::Duration::hours(1),
                event_type: "RACE".to_string(),
                distance_in_meters: Some(20000.0),
                duration_in_minutes: Some(40),
                duration_in_seconds: None,
                route_id: None,
                route: Some("Watopia".to_string()),
                description: None,
                category_enforcement: false,
                event_sub_groups: vec![],
                sport: "CYCLING".to_string(),
                tags: vec![],
            },
            ZwiftEvent {
                id: 2,
                name: "Racing Score Event".to_string(),
                event_start: Utc::now() + chrono::Duration::hours(2),
                event_type: "RACE".to_string(),
                distance_in_meters: Some(0.0),
                duration_in_minutes: Some(40),
                duration_in_seconds: None,
                route_id: None,
                route: Some("Test Route".to_string()),
                description: Some("Distance: 20 km".to_string()),
                category_enforcement: false,
                event_sub_groups: vec![EventSubGroup {
                    id: 1,
                    name: "0-650".to_string(),
                    route_id: None,
                    distance_in_meters: Some(0.0),
                    duration_in_minutes: None,
                    duration_in_seconds: None,
                    subgroup_label: None,
                    category_enforcement: None,
                    range_access_label: Some("0-650".to_string()),
                    laps: None,
                }],
                sport: "CYCLING".to_string(),
                tags: vec![],
            },
        ];

        let args = Args {
            zwift_score: Some(195),
            duration: 40,
            tolerance: 10,
            event_type: "race".to_string(),
            days: 1,
            debug: false,
            show_unknown_routes: false,
            tags: vec![],
            exclude_tags: vec![],
            verbose: false,
        };

        let (filtered, _) = filter_events(events, &args, 195);

        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().any(|e| e.name == "Traditional Race"));
        assert!(filtered.iter().any(|e| e.name == "Racing Score Event"));
    }

    #[test]
    fn test_generate_filter_description() {
        let args = Args {
            zwift_score: Some(195),
            duration: 30,
            tolerance: 10,
            event_type: "race".to_string(),
            days: 1,
            debug: false,
            show_unknown_routes: false,
            tags: vec![],
            exclude_tags: vec![],
            verbose: false,
        };

        let desc = generate_filter_description(&args, 20, 40);
        assert!(desc.contains("races"));
        assert!(desc.contains("20-40 min"));
        assert!(desc.contains("next 24h"));

        let args_with_tags = Args {
            tags: vec!["3R".to_string(), "flat".to_string()],
            ..args.clone()
        };
        let desc_tags = generate_filter_description(&args_with_tags, 20, 40);
        assert!(desc_tags.contains("with tags: 3R, flat"));

        let args_multi_day = Args {
            days: 7,
            ..args.clone()
        };
        let desc_days = generate_filter_description(&args_multi_day, 20, 40);
        assert!(desc_days.contains("next 7 days"));

        let args_tt = Args {
            event_type: "tt".to_string(),
            ..args.clone()
        };
        let desc_tt = generate_filter_description(&args_tt, 20, 40);
        assert!(desc_tt.contains("time trials"));
    }

    #[test]
    fn test_generate_no_results_suggestions_for_race() {
        let args = Args {
            zwift_score: Some(195),
            duration: 60,
            tolerance: 10,
            event_type: "race".to_string(),
            days: 1,
            debug: false,
            show_unknown_routes: false,
            tags: vec![],
            exclude_tags: vec![],
            verbose: false,
        };

        let suggestions = generate_no_results_suggestions(&args);

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
            debug: false,
            show_unknown_routes: false,
            tags: vec![],
            exclude_tags: vec![],
            verbose: false,
        };

        let suggestions = generate_no_results_suggestions(&args);

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
            debug: false,
            show_unknown_routes: false,
            tags: vec![],
            exclude_tags: vec![],
            verbose: false,
        };

        let suggestions = generate_no_results_suggestions(&args);

        assert!(suggestions.len() >= 3);
        assert!(suggestions[0].contains("No events match your duration"));
        assert!(suggestions[1].contains("-t 20"));
        assert!(suggestions[2].contains("-e all"));
    }

    #[test]
    fn test_generate_no_results_suggestions() {
        let race_args = Args {
            event_type: "race".to_string(),
            duration: 60,
            tolerance: 15,
            days: 3,
            zwift_score: None,
            tags: vec![],
            exclude_tags: vec![],
            show_unknown_routes: false,
            verbose: false,
            debug: false,
        };
        let suggestions = generate_no_results_suggestions(&race_args);
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("Most races are short"));
        assert!(suggestions[1].contains("cargo run -- -d 30 -t 30"));

        let tt_args = Args {
            event_type: "tt".to_string(),
            duration: 60,
            tolerance: 15,
            days: 3,
            zwift_score: None,
            tags: vec![],
            exclude_tags: vec![],
            show_unknown_routes: false,
            verbose: false,
            debug: false,
        };
        let suggestions = generate_no_results_suggestions(&tt_args);
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("Time trials are less common"));

        let generic_args = Args {
            event_type: "workout".to_string(),
            duration: 60,
            tolerance: 15,
            days: 3,
            zwift_score: None,
            tags: vec![],
            exclude_tags: vec![],
            show_unknown_routes: false,
            verbose: false,
            debug: false,
        };
        let suggestions = generate_no_results_suggestions(&generic_args);
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("No events match"));
        assert!(suggestions[1].contains("30"));
    }
}
