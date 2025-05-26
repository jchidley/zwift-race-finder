// ABOUTME: Quick test script to verify route discovery functionality
// Tests discovery for a specific event to debug any issues

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Import the necessary types
    use zwift_race_finder::route_discovery::RouteDiscovery;
    
    println!("Testing route discovery...");
    
    let discovery = RouteDiscovery::new()?;
    
    // Test with a known event
    let test_event = "Stage 4: Makuri May: Three Village Loop";
    
    match discovery.discover_route(test_event).await {
        Ok(route) => {
            println!("✓ Successfully discovered route!");
            println!("  Name: {}", route.name);
            println!("  Distance: {} km", route.distance_km);
            println!("  Elevation: {} m", route.elevation_m);
            println!("  World: {}", route.world);
            println!("  Surface: {}", route.surface);
        }
        Err(e) => {
            println!("✗ Discovery failed: {}", e);
        }
    }
    
    Ok(())
}