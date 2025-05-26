// Route discovery module for automatically finding route data from external sources
// Searches whatsonzwift.com and zwiftinsider.com for unknown routes

use anyhow::{anyhow, Result};
use regex::Regex;
use reqwest;
use std::time::Duration;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

// No need for re-export since it's already a public function

#[derive(Debug, Clone)]
pub struct DiscoveredRoute {
    pub route_id: u32,
    pub name: String,
    pub distance_km: f64,
    pub elevation_m: u32,
    pub world: String,
    pub surface: String,
}

pub struct RouteDiscovery {
    client: reqwest::Client,
    cache: Arc<Mutex<HashMap<String, Option<DiscoveredRoute>>>>,
}

impl RouteDiscovery {
    pub fn new() -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .build()?;
        
        Ok(Self { 
            client,
            cache: Arc::new(Mutex::new(HashMap::new())),
        })
    }
    
    /// Search for route information on external sites
    pub async fn discover_route(&self, event_name: &str) -> Result<DiscoveredRoute> {
        // Check cache first
        {
            let cache = self.cache.lock().await;
            if let Some(cached_result) = cache.get(event_name) {
                if let Some(route) = cached_result {
                    return Ok(route.clone());
                } else {
                    return Err(anyhow!("Route already searched but not found: {}", event_name));
                }
            }
        }
        
        // Try whatsonzwift.com first
        if let Ok(route) = self.search_whatsonzwift(event_name).await {
            // Cache successful result
            let mut cache = self.cache.lock().await;
            cache.insert(event_name.to_string(), Some(route.clone()));
            return Ok(route);
        }
        
        // Fallback to zwiftinsider.com
        if let Ok(route) = self.search_zwiftinsider(event_name).await {
            // Cache successful result
            let mut cache = self.cache.lock().await;
            cache.insert(event_name.to_string(), Some(route.clone()));
            return Ok(route);
        }
        
        // Cache failure to avoid repeated searches
        let mut cache = self.cache.lock().await;
        cache.insert(event_name.to_string(), None);
        
        Err(anyhow!("Could not find route information for: {}", event_name))
    }
    
    /// Search whatsonzwift.com for route information
    async fn search_whatsonzwift(&self, event_name: &str) -> Result<DiscoveredRoute> {
        // Try to construct direct URLs based on common patterns
        // Extract the route name from event name (remove prefixes like "Stage X:", suffixes like "|| Advanced")
        let cleaned_name = self.extract_route_name(event_name);
        
        // Prioritize common worlds first
        let worlds = ["watopia", "makuri-islands", "london", "new-york", "france"];
        
        eprintln!("  Searching whatsonzwift.com for '{}'...", cleaned_name);
        
        for world in &worlds {
            let route_slug = cleaned_name.to_lowercase()
                .replace(" ", "-")
                .replace("'", "")
                .replace("(", "")
                .replace(")", "");
            
            let route_url = format!("https://whatsonzwift.com/world/{}/route/{}", world, route_slug);
            
            // Add small delay to be respectful
            tokio::time::sleep(Duration::from_millis(500)).await;
            
            match self.client.get(&route_url).send().await {
                Ok(response) if response.status().is_success() => {
                    let route_html = response.text().await?;
                    
                    // Check if this is actually a route page (not 404 or redirect)
                    if route_html.contains("Distance:") && route_html.contains("Elevation:") {
                        // Parse route data from the page
                        if let Ok(mut route) = self.parse_whatsonzwift_route(&route_html, event_name) {
                            // Set the correct world
                            route.world = self.format_world_name(world);
                            return Ok(route);
                        }
                    }
                }
                _ => continue,
            }
        }
        
        Err(anyhow!("Could not find route on whatsonzwift.com: {}", event_name))
    }
    
    /// Extract the actual route name from event names
    fn extract_route_name(&self, event_name: &str) -> String {
        let name = event_name.to_string();
        
        // Remove common prefixes
        let name = if let Some(idx) = name.find(':') {
            name[idx+1..].trim().to_string()
        } else {
            name
        };
        
        // Remove suffixes after || or |
        let name = if let Some(idx) = name.find("||") {
            name[..idx].trim().to_string()
        } else if let Some(idx) = name.find('|') {
            name[..idx].trim().to_string()
        } else {
            name
        };
        
        name
    }
    
    /// Format world name for display
    fn format_world_name(&self, world_slug: &str) -> String {
        match world_slug {
            "makuri-islands" => "Makuri Islands".to_string(),
            "new-york" => "New York".to_string(),
            _ => {
                // Capitalize first letter of each word
                world_slug.split('-')
                    .map(|word| {
                        let mut chars = word.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(first) => first.to_uppercase().chain(chars).collect(),
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            }
        }
    }
    
    /// Parse route data from whatsonzwift.com HTML
    fn parse_whatsonzwift_route(&self, html: &str, event_name: &str) -> Result<DiscoveredRoute> {
        // Extract distance (e.g., "Distance: 10.6km")
        let distance_regex = Regex::new(r#"Distance:\s*([0-9.]+)\s*km"#)?;
        let distance_km = distance_regex.captures(html)
            .and_then(|cap| cap.get(1))
            .and_then(|m| m.as_str().parse::<f64>().ok())
            .ok_or_else(|| anyhow!("Could not parse distance"))?;
        
        // Extract elevation (e.g., "Elevation: 145m")
        let elevation_regex = Regex::new(r#"Elevation:\s*([0-9,]+)\s*m"#)?;
        let elevation_str = elevation_regex.captures(html)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().replace(",", ""))
            .ok_or_else(|| anyhow!("Could not parse elevation"))?;
        let elevation_m = elevation_str.parse::<u32>()?;
        
        // Extract world (e.g., "World: Makuri Islands")
        let world_regex = Regex::new(r#"World:\s*([^<]+)"#)?;
        let world = world_regex.captures(html)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        
        // Extract route name from title or heading
        let name_regex = Regex::new(r#"<h1[^>]*>([^<]+)</h1>"#)?;
        let name = name_regex.captures(html)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_else(|| event_name.to_string());
        
        // Determine surface type (default to road, check for gravel/mixed indicators)
        let surface = if html.contains("gravel") || html.contains("Gravel") {
            "gravel".to_string()
        } else if html.contains("mixed surface") || html.contains("Mixed") {
            "mixed".to_string()
        } else {
            "road".to_string()
        };
        
        // For now, use a placeholder route_id - will need to be updated
        // In production, we'd need to extract this from the URL or page
        Ok(DiscoveredRoute {
            route_id: 9999, // Placeholder, needs proper extraction
            name,
            distance_km,
            elevation_m,
            world,
            surface,
        })
    }
    
    /// Search zwiftinsider.com for route information
    async fn search_zwiftinsider(&self, event_name: &str) -> Result<DiscoveredRoute> {
        eprintln!("  Searching zwiftinsider.com...");
        
        // Try direct URL construction based on route name
        let cleaned_name = self.extract_route_name(event_name);
        let route_slug = cleaned_name.to_lowercase()
            .replace(" ", "-")
            .replace("'", "")
            .replace("(", "")
            .replace(")", "");
        
        // ZwiftInsider uses simple /route/ROUTE-NAME/ format
        let route_url = format!("https://zwiftinsider.com/route/{}/", route_slug);
        
        // Add small delay to be respectful
        tokio::time::sleep(Duration::from_millis(500)).await;
        
        match self.client.get(&route_url).send().await {
            Ok(response) if response.status().is_success() => {
                let route_html = response.text().await?;
                
                // Check if this is actually a route page
                if route_html.contains("Length") || route_html.contains("Distance") {
                    return self.parse_zwiftinsider_route(&route_html, event_name);
                }
            }
            _ => {}
        }
        
        // If direct URL didn't work, try fetching the routes listing and searching
        let routes_url = "https://zwiftinsider.com/routes/";
        let response = self.client.get(routes_url).send().await?;
        let html = response.text().await?;
        
        // Look for route links that might match
        let route_words: Vec<&str> = cleaned_name.split_whitespace()
            .filter(|w| w.len() > 3)
            .collect();
        
        for word in route_words {
            let pattern = format!(r#"href="(/route/[^"]*{}[^"]*)"#, 
                regex::escape(&word.to_lowercase()));
            if let Ok(regex) = Regex::new(&pattern) {
                if let Some(captures) = regex.captures(&html) {
                    let route_path = captures.get(1).unwrap().as_str();
                    let full_url = format!("https://zwiftinsider.com{}", route_path);
                    
                    if let Ok(resp) = self.client.get(&full_url).send().await {
                        if resp.status().is_success() {
                            let route_html = resp.text().await?;
                            return self.parse_zwiftinsider_route(&route_html, event_name);
                        }
                    }
                }
            }
        }
        
        Err(anyhow!("Could not find route on zwiftinsider.com: {}", event_name))
    }
    
    /// Parse route data from zwiftinsider.com HTML
    fn parse_zwiftinsider_route(&self, html: &str, event_name: &str) -> Result<DiscoveredRoute> {
        // Zwift Insider uses slightly different format
        // Look for route stats table or info box
        
        // Extract distance
        let distance_regex = Regex::new(r#"(?:Length|Distance)[:\s]*([0-9.]+)\s*km"#)?;
        let distance_km = distance_regex.captures(html)
            .and_then(|cap| cap.get(1))
            .and_then(|m| m.as_str().parse::<f64>().ok())
            .ok_or_else(|| anyhow!("Could not parse distance"))?;
        
        // Extract elevation
        let elevation_regex = Regex::new(r#"(?:Elevation|Gain)[:\s]*([0-9,]+)\s*m"#)?;
        let elevation_str = elevation_regex.captures(html)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().replace(",", ""))
            .ok_or_else(|| anyhow!("Could not parse elevation"))?;
        let elevation_m = elevation_str.parse::<u32>()?;
        
        // Extract world
        let world_regex = Regex::new(r#"(?:World|Location)[:\s]*([^<\n]+)"#)?;
        let world = world_regex.captures(html)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        
        // Extract route name
        let name_regex = Regex::new(r#"<h1[^>]*>([^<]+)</h1>"#)?;
        let name = name_regex.captures(html)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_else(|| event_name.to_string());
        
        // Determine surface type
        let surface = if html.contains("gravel") || html.contains("dirt") {
            "gravel".to_string()
        } else if html.contains("mixed") {
            "mixed".to_string()
        } else {
            "road".to_string()
        };
        
        Ok(DiscoveredRoute {
            route_id: 9999, // Placeholder
            name,
            distance_km,
            elevation_m,
            world,
            surface,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ParsedEventDescription {
    pub route_name: String,
    pub laps: u32,
}

/// Parse event descriptions to extract route names and lap counts
pub fn parse_route_from_description(description: &str) -> Option<ParsedEventDescription> {
    // Common patterns in event descriptions:
    // "3 laps of Volcano Circuit"
    // "Volcano Circuit x 3"
    // "Stage 4: Three Village Loop"
    // "Makuri Three Village Loop (3 laps)"
    // "2x Mountain Route"
    
    // Pattern 1: "X laps of Route Name"
    let laps_of_regex = Regex::new(r"(\d+)\s*laps?\s+of\s+([^,\(\)]+)").ok()?;
    if let Some(captures) = laps_of_regex.captures(description) {
        if let (Some(laps_str), Some(route_name)) = (captures.get(1), captures.get(2)) {
            if let Ok(laps) = laps_str.as_str().parse::<u32>() {
                return Some(ParsedEventDescription {
                    route_name: route_name.as_str().trim().to_string(),
                    laps,
                });
            }
        }
    }
    
    // Pattern 2: "Route Name x N" or "Route Name xN"
    let route_x_regex = Regex::new(r"([^,\(\)]+?)\s*x\s*(\d+)").ok()?;
    if let Some(captures) = route_x_regex.captures(description) {
        if let (Some(route_name), Some(laps_str)) = (captures.get(1), captures.get(2)) {
            if let Ok(laps) = laps_str.as_str().parse::<u32>() {
                // Clean up route name - remove trailing words like "route" if present
                let mut name = route_name.as_str().trim().to_string();
                if name.to_lowercase().ends_with(" route") {
                    name = name[..name.len()-6].trim().to_string();
                }
                return Some(ParsedEventDescription {
                    route_name: name,
                    laps,
                });
            }
        }
    }
    
    // Pattern 3: "Nx Route Name"
    let n_x_route_regex = Regex::new(r"(\d+)\s*x\s+([^,\(\)]+)").ok()?;
    if let Some(captures) = n_x_route_regex.captures(description) {
        if let (Some(laps_str), Some(route_name)) = (captures.get(1), captures.get(2)) {
            if let Ok(laps) = laps_str.as_str().parse::<u32>() {
                // Clean up route name
                let mut name = route_name.as_str().trim().to_string();
                if name.to_lowercase().ends_with(" route") {
                    name = name[..name.len()-6].trim().to_string();
                }
                return Some(ParsedEventDescription {
                    route_name: name,
                    laps,
                });
            }
        }
    }
    
    // Pattern 4: "Route Name (N laps)"
    let route_laps_paren_regex = Regex::new(r"([^,\(\)]+?)\s*\((\d+)\s*laps?\)").ok()?;
    if let Some(captures) = route_laps_paren_regex.captures(description) {
        if let (Some(route_name), Some(laps_str)) = (captures.get(1), captures.get(2)) {
            if let Ok(laps) = laps_str.as_str().parse::<u32>() {
                return Some(ParsedEventDescription {
                    route_name: route_name.as_str().trim().to_string(),
                    laps,
                });
            }
        }
    }
    
    // Pattern 5: "Stage X: Route Name" (assume 1 lap for stages)
    let stage_regex = Regex::new(r"Stage\s+\d+:\s+([^,\(\)]+)").ok()?;
    if let Some(captures) = stage_regex.captures(description) {
        if let Some(route_name) = captures.get(1) {
            return Some(ParsedEventDescription {
                route_name: route_name.as_str().trim().to_string(),
                laps: 1,
            });
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_route_discovery_initialization() {
        let discovery = RouteDiscovery::new();
        assert!(discovery.is_ok());
    }
    
    #[test]
    fn test_parse_route_from_description() {
        // Test "X laps of Route" pattern
        let result = parse_route_from_description("3 laps of Volcano Circuit");
        assert!(result.is_some());
        let parsed = result.unwrap();
        assert_eq!(parsed.route_name, "Volcano Circuit");
        assert_eq!(parsed.laps, 3);
        
        // Test "Route x N" pattern
        let result = parse_route_from_description("Mountain Route x 2");
        assert!(result.is_some());
        let parsed = result.unwrap();
        assert_eq!(parsed.route_name, "Mountain");
        assert_eq!(parsed.laps, 2);
        
        // Test "Nx Route" pattern
        let result = parse_route_from_description("2x Bell Lap");
        assert!(result.is_some());
        let parsed = result.unwrap();
        assert_eq!(parsed.route_name, "Bell Lap");
        assert_eq!(parsed.laps, 2);
        
        // Test "Route (N laps)" pattern
        let result = parse_route_from_description("Three Village Loop (3 laps)");
        assert!(result.is_some());
        let parsed = result.unwrap();
        assert_eq!(parsed.route_name, "Three Village Loop");
        assert_eq!(parsed.laps, 3);
        
        // Test "Stage X: Route" pattern
        let result = parse_route_from_description("Stage 4: Makuri May");
        assert!(result.is_some());
        let parsed = result.unwrap();
        assert_eq!(parsed.route_name, "Makuri May");
        assert_eq!(parsed.laps, 1);
        
        // Test no match
        let result = parse_route_from_description("Just a regular race");
        assert!(result.is_none());
    }
}