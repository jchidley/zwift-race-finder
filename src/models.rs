use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ZwiftEvent {
    pub id: u64,
    pub name: String,
    #[serde(rename = "eventStart")]
    pub event_start: DateTime<Utc>,
    pub event_type: String,
    pub distance_in_meters: Option<f64>,
    pub duration_in_minutes: Option<u32>,
    #[serde(rename = "durationInSeconds")]
    pub duration_in_seconds: Option<u32>,
    pub route_id: Option<u32>,
    pub route: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub category_enforcement: bool,
    #[serde(default, rename = "eventSubgroups")]
    pub event_sub_groups: Vec<EventSubGroup>,
    #[serde(default = "default_sport")]
    pub sport: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

pub fn default_sport() -> String {
    "CYCLING".to_string()
}

pub fn is_racing_score_event(event: &ZwiftEvent) -> bool {
    event
        .event_sub_groups
        .iter()
        .any(|sg| sg.range_access_label.as_deref().is_some_and(|label| !label.trim().is_empty()))
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EventSubGroup {
    pub id: u32,
    pub name: String,
    pub route_id: Option<u32>,
    pub distance_in_meters: Option<f64>,
    pub duration_in_minutes: Option<u32>,
    pub duration_in_seconds: Option<u32>,
    pub subgroup_label: Option<String>,
    pub category_enforcement: Option<bool>,
    pub range_access_label: Option<String>,
    pub laps: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserStats {
    pub zwift_score: u32,
    pub category: String,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CachedStats {
    pub stats: UserStats,
    pub cached_at: DateTime<Utc>,
}

pub struct RouteData {
    pub distance_km: f64,
    pub elevation_m: Option<f64>,
    #[allow(dead_code)]
    pub name: &'static str,
    #[allow(dead_code)]
    pub world: &'static str,
    pub surface: &'static str,
    pub lead_in_distance_km: Option<f64>,
    pub lead_in_elevation_m: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_sport() {
        assert_eq!(default_sport(), "CYCLING");
    }

    #[test]
    fn test_is_racing_score_event() {
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
            event_sub_groups: vec![EventSubGroup {
                id: 1,
                name: "A".to_string(),
                route_id: Some(1),
                distance_in_meters: Some(40000.0),
                duration_in_minutes: None,
                duration_in_seconds: None,
                subgroup_label: None,
                category_enforcement: None,
                range_access_label: None,
                laps: None,
            }],
            sport: "CYCLING".to_string(),
            tags: vec![],
        };

        assert!(!is_racing_score_event(&traditional_event));

        let racing_score_event = ZwiftEvent {
            id: 2,
            name: "Three Village Loop Race".to_string(),
            event_start: Utc::now(),
            event_type: "RACE".to_string(),
            distance_in_meters: Some(0.0),
            duration_in_minutes: None,
            duration_in_seconds: None,
            route_id: Some(9),
            route: Some("Three Village Loop".to_string()),
            description: Some("Distance: 10.6 km".to_string()),
            category_enforcement: false,
            event_sub_groups: vec![EventSubGroup {
                id: 1,
                name: "0-199".to_string(),
                route_id: Some(9),
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

        assert!(is_racing_score_event(&racing_score_event));

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
            event_sub_groups: vec![],
            sport: "CYCLING".to_string(),
            tags: vec![],
        };

        assert!(!is_racing_score_event(&no_subgroups_event));
    }
}
