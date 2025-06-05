//! Parsing utilities for extracting data from event descriptions and names

use regex::Regex;

// Parse distance and elevation from event description
#[derive(Debug, Clone)]
pub struct DescriptionData {
    pub distance_km: Option<f64>,
    pub elevation_m: Option<u32>,
    pub laps: Option<u32>,
}

pub fn parse_description_data(description: &Option<String>) -> DescriptionData {
    let mut data = DescriptionData {
        distance_km: None,
        elevation_m: None,
        laps: None,
    };
    
    if let Some(desc) = description {
        // Parse distance
        data.distance_km = parse_distance_from_description(description);
        
        // Parse elevation - look for patterns like "Elevation: X m" or "X meters elevation"
        let elevation_patterns = vec![
            r"Elevation:\s*(\d+(?:\.\d+)?)\s*(m|meters?|ft|feet)",
            r"(\d+(?:\.\d+)?)\s*(m|meters?)\s*elevation",
            r"(\d+(?:\.\d+)?)\s*(m|meters?)\s*of\s*climbing",
            r"Elevation\s*Gain:\s*(\d+(?:\.\d+)?)\s*(m|meters?|ft|feet)",
        ];
        
        for pattern in elevation_patterns {
            let re = Regex::new(pattern).unwrap();
            if let Some(caps) = re.captures(desc) {
                if let (Some(value), Some(unit)) = (caps.get(1), caps.get(2)) {
                    let elevation = value.as_str().parse::<f64>().ok().unwrap_or(0.0);
                    // Convert feet to meters if necessary
                    data.elevation_m = Some(if unit.as_str().contains("ft") || unit.as_str().contains("feet") {
                        (elevation / 3.28084) as u32
                    } else {
                        elevation as u32
                    });
                    break;
                }
            }
        }
        
        // Parse lap count
        let lap_re = Regex::new(r"(\d+)\s*laps?\b").unwrap();
        if let Some(caps) = lap_re.captures(desc) {
            if let Some(value) = caps.get(1) {
                data.laps = value.as_str().parse().ok();
            }
        }
    }
    
    data
}

pub fn parse_distance_from_description(description: &Option<String>) -> Option<f64> {
    if let Some(desc) = description {
        // Look for patterns like "Distance: X km" or "Distance: X miles"
        // This is common in Racing Score events and stage events
        let distance_re = Regex::new(r"Distance:\s*(\d+(?:\.\d+)?)\s*(km|miles?)").unwrap();
        if let Some(caps) = distance_re.captures(desc) {
            if let (Some(value), Some(unit)) = (caps.get(1), caps.get(2)) {
                let distance = value.as_str().parse::<f64>().ok()?;
                // Convert miles to km if necessary
                return Some(if unit.as_str().contains("mile") {
                    distance * 1.60934
                } else {
                    distance
                });
            }
        }
        
        // Fallback to general distance parsing
        parse_distance_from_name(desc)
    } else {
        None
    }
}

pub fn parse_distance_from_name(name: &str) -> Option<f64> {
    // Try to find km distance first
    let km_re = Regex::new(r"(\d+(?:\.\d+)?)\s*km").unwrap();
    if let Some(caps) = km_re.captures(name) {
        return caps.get(1)?.as_str().parse().ok();
    }
    
    // If no km found, try miles and convert
    let mi_re = Regex::new(r"(\d+(?:\.\d+)?)\s*mi").unwrap();
    if let Some(caps) = mi_re.captures(name) {
        let miles: f64 = caps.get(1)?.as_str().parse().ok()?;
        return Some(miles * 1.60934); // Convert miles to km
    }
    
    None
}

// Parse lap count from event name (e.g., "3 Laps", "6 laps")
pub fn parse_lap_count(name: &str) -> Option<u32> {
    let re = Regex::new(r"(\d+)\s*[Ll]aps?").unwrap();
    if let Some(caps) = re.captures(name) {
        caps.get(1)?.as_str().parse().ok()
    } else {
        None
    }
}