use anyhow::Result;
use zwift_race_finder::route_discovery::RouteDiscovery;

#[tokio::main]
async fn main() -> Result<()> {
    let discovery = RouteDiscovery::new()?;
    
    println\!("Testing world detection and discovery...\n");
    
    // Test Makuri detection
    match discovery.discover_route("STAGE 3: RACE MAKURI— Turf N Surf").await {
        Ok(route) => {
            println\!("✅ Successfully discovered route:");
            println\!("   Name: {}", route.name);
            println\!("   World: {}", route.world);
            println\!("   Distance: {} km", route.distance_km);
            println\!("   Elevation: {} m", route.elevation_m);
            println\!("   Route ID: {}", route.route_id);
        }
        Err(e) => {
            println\!("❌ Failed to discover route: {}", e);
        }
    }
    
    Ok(())
}
