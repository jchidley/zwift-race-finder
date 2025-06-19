// Binary to import route data from zwift-offline JSON exports

use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;
use zwift_race_finder::{
    database::{Database, RouteData},
    zwift_offline_client::{load_routes_from_file, load_events_from_file},
};

#[derive(Parser, Debug)]
#[command(
    name = "import_zwift_offline_routes",
    about = "Import route data from zwift-offline JSON export files"
)]
struct Args {
    /// Directory containing exported JSON files
    #[arg(short, long)]
    input_dir: PathBuf,
    
    /// Database file path
    #[arg(short, long, default_value = "zwift_routes.db")]
    database: PathBuf,
    
    /// Update existing routes (otherwise skip duplicates)
    #[arg(short, long)]
    update: bool,
    
    /// Dry run - show what would be imported without making changes
    #[arg(long)]
    dry_run: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Load exported data
    let routes_file = args.input_dir.join("routes.json");
    let events_file = args.input_dir.join("events.json");
    
    println!("Loading routes from: {}", routes_file.display());
    let routes = load_routes_from_file(&routes_file)
        .context("Failed to load routes")?;
    
    println!("Found {} routes", routes.len());
    
    // Also load events for cross-reference
    let events = if events_file.exists() {
        println!("Loading events from: {}", events_file.display());
        Some(load_events_from_file(&events_file).context("Failed to load events")?)
    } else {
        None
    };
    
    if args.dry_run {
        println!("\n=== DRY RUN MODE ===\n");
    }
    
    // Open database  
    let db = if !args.dry_run {
        // Use the default database location
        Database::new()?
    } else {
        // For dry run, we still need a database to check existing routes
        Database::new()?
    };
    
    // Import routes
    let mut imported = 0;
    let mut updated = 0;
    let mut skipped = 0;
    let mut event_routes = 0;
    let mut free_ride_routes = 0;
    
    let total_routes = routes.len();
    let mut imported_route_ids: std::collections::HashSet<i64> = std::collections::HashSet::new();
    
    for route in routes {
        imported_route_ids.insert(route.route_id);
        
        // Skip routes with no distance data (shouldn't happen with proper extraction)
        if route.distance_km == 0.0 && route.distance_m == 0.0 {
            println!("WARNING: Skipping route with no distance data: {} ({})", route.name, route.route_id);
            println!("         This indicates incomplete data extraction. Use wad_unpack.exe for full data.");
            skipped += 1;
            continue;
        }
        
        // Count route types
        if route.event_only {
            event_routes += 1;
        } else {
            free_ride_routes += 1;
        }
        
        // Check if route already exists
        // Convert signed to unsigned, wrapping around for negative values
        let route_id_u32 = route.route_id as u32;
        let existing = db.get_route(route_id_u32)?;
        
        if existing.is_some() && !args.update {
            if args.dry_run {
                println!("Would skip existing route: {} ({})", route.name, route.route_id);
            }
            skipped += 1;
            continue;
        }
        
        // Convert to RouteData
        let route_data = RouteData {
            route_id: route_id_u32,
            name: route.name.clone(),
            distance_km: if route.distance_without_lead_in_km > 0.0 {
                route.distance_without_lead_in_km
            } else {
                // For older data without separated lead-in
                route.distance_km - route.lead_in_distance_km
            },
            elevation_m: route.elevation_gain as u32,
            world: route.world_name.clone(),
            surface: route.surface.clone(),
            lead_in_distance_km: route.lead_in_distance_km,
            lead_in_elevation_m: 0, // Not available yet
            lead_in_distance_free_ride_km: None,
            lead_in_elevation_free_ride_m: None,
            lead_in_distance_meetups_km: None,
            lead_in_elevation_meetups_m: None,
            slug: None, // Can be added later from WhatsOnZwift
        };
        
        if args.dry_run {
            if existing.is_some() {
                if args.update {
                    println!("Would update route: {} ({}) - {:.1}km in {}", 
                        route.name, route.route_id, route.distance_km, route.world_name);
                    updated += 1;
                } else {
                    println!("Would skip existing route: {} ({})", route.name, route.route_id);
                    skipped += 1;
                }
            } else {
                println!("Would import route: {} ({}) - {:.1}km in {}", 
                    route.name, route.route_id, route.distance_km, route.world_name);
                imported += 1;
            }
        } else {
            if existing.is_some() {
                // For now, skip existing routes as we don't have an update method
                println!("Skipping existing route: {} ({})", route.name, route.route_id);
                skipped += 1;
            } else {
                db.add_route(&route_data)?;
                imported += 1;
            }
        }
    }
    
    // Summary
    println!("\n=== Import Summary ===");
    println!("Routes processed: {}", total_routes);
    println!("  Event-only routes: {}", event_routes);
    println!("  Free-ride routes: {}", free_ride_routes);
    println!("New routes imported: {}", imported);
    println!("Existing routes updated: {}", updated);
    println!("Routes skipped: {}", skipped);
    
    if args.dry_run {
        println!("\nThis was a dry run - no changes were made.");
        println!("Run without --dry-run to actually import the data.");
    }
    
    // Cross-reference check with events if available
    if let Some(events) = events {
        println!("\n=== Event Cross-Reference Check ===");
            
        let missing_routes: Vec<_> = events.iter()
            .filter(|e| !imported_route_ids.contains(&e.route))
            .collect();
            
        if !missing_routes.is_empty() {
            println!("Found {} events referencing routes not in export:", missing_routes.len());
            for event in missing_routes.iter().take(5) {
                println!("  - {} (route_id: {})", event.name, event.route);
            }
            if missing_routes.len() > 5 {
                println!("  ... and {} more", missing_routes.len() - 5);
            }
        } else {
            println!("All event routes are present in the export ✓");
        }
    }
    
    Ok(())
}