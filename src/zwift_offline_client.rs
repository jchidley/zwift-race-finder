// Client for importing route data from zwift-offline API
// Maintains license boundary through HTTP API calls

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportedRoute {
    pub route_id: i64,
    pub name: String,
    pub distance_m: f64,
    pub distance_km: f64,
    #[serde(default)]
    pub distance_without_lead_in_km: f64,
    #[serde(default)]
    pub lead_in_distance_km: f64,
    pub course_id: Option<u32>,
    pub world_id: Option<u32>,
    pub world_name: String,
    pub sport: u32, // 0=cycling, 1=running
    #[serde(default = "default_true")]
    pub event_only: bool,
    #[serde(default = "default_surface")]
    pub surface: String,
    #[serde(default)]
    pub elevation_gain: f64,
}

fn default_true() -> bool {
    true
}

fn default_surface() -> String {
    "road".to_string()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportedStartLine {
    pub hash: String,
    pub name: String,
    pub road: u32,
    pub time: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportedEvent {
    pub name: String,
    pub route: i64,
    pub distance: f64,
    pub course: u32,
    pub sport: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteExportResponse {
    pub routes: Vec<ExportedRoute>,
    pub count: usize,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartLineExportResponse {
    pub start_lines: Vec<ExportedStartLine>,
    pub count: usize,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventExportResponse {
    pub events: Vec<ExportedEvent>,
    pub count: usize,
    pub source: String,
}

/// Client for zwift-offline route export API
pub struct ZwiftOfflineClient {
    base_url: String,
    client: reqwest::Client,
}

impl ZwiftOfflineClient {
    pub fn new(base_url: &str) -> Result<Self> {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true) // For local self-signed certs
            .build()
            .context("Failed to create HTTP client")?;
            
        Ok(Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client,
        })
    }
    
    /// Fetch routes from zwift-offline
    pub async fn fetch_routes(&self) -> Result<Vec<ExportedRoute>> {
        let url = format!("{}/api/export/routes", self.base_url);
        let response: RouteExportResponse = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch routes")?
            .json()
            .await
            .context("Failed to parse routes response")?;
            
        Ok(response.routes)
    }
    
    /// Fetch start lines from zwift-offline
    pub async fn fetch_start_lines(&self) -> Result<Vec<ExportedStartLine>> {
        let url = format!("{}/api/export/start_lines", self.base_url);
        let response: StartLineExportResponse = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch start lines")?
            .json()
            .await
            .context("Failed to parse start lines response")?;
            
        Ok(response.start_lines)
    }
    
    /// Fetch events from zwift-offline
    pub async fn fetch_events(&self) -> Result<Vec<ExportedEvent>> {
        let url = format!("{}/api/export/events", self.base_url);
        let response: EventExportResponse = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch events")?
            .json()
            .await
            .context("Failed to parse events response")?;
            
        Ok(response.events)
    }
}

/// Load exported routes from JSON file
pub fn load_routes_from_file(path: &Path) -> Result<Vec<ExportedRoute>> {
    let data = std::fs::read_to_string(path)
        .context("Failed to read routes file")?;
    let response: RouteExportResponse = serde_json::from_str(&data)
        .context("Failed to parse routes JSON")?;
    Ok(response.routes)
}

/// Load exported start lines from JSON file
pub fn load_start_lines_from_file(path: &Path) -> Result<Vec<ExportedStartLine>> {
    let data = std::fs::read_to_string(path)
        .context("Failed to read start lines file")?;
    let response: StartLineExportResponse = serde_json::from_str(&data)
        .context("Failed to parse start lines JSON")?;
    Ok(response.start_lines)
}

/// Load exported events from JSON file  
pub fn load_events_from_file(path: &Path) -> Result<Vec<ExportedEvent>> {
    let data = std::fs::read_to_string(path)
        .context("Failed to read events file")?;
    let response: EventExportResponse = serde_json::from_str(&data)
        .context("Failed to parse events JSON")?;
    Ok(response.events)
}