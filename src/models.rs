//! Data models for Zwift Race Finder

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
    // Racing Score events have range_access_label in subgroups
    event.event_sub_groups.iter().any(|sg| sg.range_access_label.is_some())
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EventSubGroup {
    pub id: u32,
    pub name: String,
    pub route_id: Option<u32>,
    pub distance_in_meters: Option<f64>,
    pub duration_in_minutes: Option<u32>,
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
    pub elevation_m: u32,
    #[allow(dead_code)]
    pub name: &'static str,
    #[allow(dead_code)]
    pub world: &'static str,
    pub surface: &'static str, // "road", "gravel", "mixed"
    pub lead_in_distance_km: f64,
}