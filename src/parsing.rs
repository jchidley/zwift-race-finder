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

/// Estimate distance from common race name patterns
pub fn estimate_distance_from_name(name: &str) -> Option<f64> {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}