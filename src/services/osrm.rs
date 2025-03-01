use anyhow::{Result, Context};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use log::{debug, error};

/// Configuration for the OSRM service
#[derive(Debug, Clone, Deserialize)]
pub struct OsrmConfig {
    /// Base URL for the OSRM service
    pub base_url: String,
    
    /// Default routing profile (car, bike, foot, etc.)
    pub default_profile: String,
    
    /// Timeout for OSRM requests in seconds
    pub timeout_seconds: u64,
}

impl Default for OsrmConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:5000".to_string(),
            default_profile: "car".to_string(),
            timeout_seconds: 30,
        }
    }
}

/// Service for interacting with the OSRM API
#[derive(Debug, Clone)]
pub struct OsrmService {
    client: Client,
    config: OsrmConfig,
}

/// OSRM route response
#[derive(Debug, Deserialize, Serialize)]
pub struct OsrmRouteResponse {
    pub code: String,
    pub routes: Vec<OsrmRoute>,
    pub waypoints: Vec<OsrmWaypoint>,
}

/// OSRM route
#[derive(Debug, Deserialize, Serialize)]
pub struct OsrmRoute {
    pub distance: f64,
    pub duration: f64,
    pub geometry: Option<String>,
    pub legs: Vec<OsrmRouteLeg>,
}

/// OSRM route leg
#[derive(Debug, Deserialize, Serialize)]
pub struct OsrmRouteLeg {
    pub distance: f64,
    pub duration: f64,
    pub steps: Vec<OsrmRouteStep>,
}

/// OSRM route step
#[derive(Debug, Deserialize, Serialize)]
pub struct OsrmRouteStep {
    pub distance: f64,
    pub duration: f64,
    pub geometry: Option<String>,
    pub name: String,
}

/// OSRM waypoint
#[derive(Debug, Deserialize, Serialize)]
pub struct OsrmWaypoint {
    pub hint: String,
    pub distance: f64,
    pub name: String,
    pub location: [f64; 2],
}

/// OSRM table response
#[derive(Debug, Deserialize, Serialize)]
pub struct OsrmTableResponse {
    pub code: String,
    pub durations: Vec<Vec<f64>>,
    pub distances: Option<Vec<Vec<f64>>>,
}

impl OsrmService {
    /// Create a new OSRM service with the given configuration
    pub fn new(config: OsrmConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .expect("Failed to build HTTP client");
            
        Self { client, config }
    }
    
    /// Get the route between multiple coordinates
    pub async fn route(
        &self,
        coordinates: &[[f64; 2]],
        profile: Option<&str>,
        geometry: bool,
    ) -> Result<OsrmRouteResponse> {
        let profile = profile.unwrap_or(&self.config.default_profile);
        
        // Build coordinates string
        let coords_str = coordinates
            .iter()
            .map(|coord| format!("{},{}", coord[0], coord[1]))
            .collect::<Vec<_>>()
            .join(";");
            
        // Build URL
        let url = format!(
            "{}/route/v1/{}/{}?overview={}&steps=true",
            self.config.base_url,
            profile,
            coords_str,
            if geometry { "full" } else { "false" }
        );
        
        debug!("OSRM route request: {}", url);
        
        // Make request
        let response = self.client.get(&url)
            .send()
            .await
            .context("Failed to send OSRM route request")?;
            
        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            error!("OSRM route request failed with status {}: {}", status, error_text);
            anyhow::bail!("OSRM route request failed with status {}", status);
        }
        
        let route_response = response.json::<OsrmRouteResponse>()
            .await
            .context("Failed to parse OSRM route response")?;
            
        Ok(route_response)
    }
    
    /// Get a duration/distance matrix between multiple coordinates
    pub async fn table(
        &self,
        coordinates: &[[f64; 2]],
        profile: Option<&str>,
        include_distances: bool,
    ) -> Result<OsrmTableResponse> {
        let profile = profile.unwrap_or(&self.config.default_profile);
        
        // Build coordinates string
        let coords_str = coordinates
            .iter()
            .map(|coord| format!("{},{}", coord[0], coord[1]))
            .collect::<Vec<_>>()
            .join(";");
            
        // Build URL
        let url = format!(
            "{}/table/v1/{}/{}?annotations={}",
            self.config.base_url,
            profile,
            coords_str,
            if include_distances { "duration,distance" } else { "duration" }
        );
        
        debug!("OSRM table request: {}", url);
        
        // Make request
        let response = self.client.get(&url)
            .send()
            .await
            .context("Failed to send OSRM table request")?;
            
        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            error!("OSRM table request failed with status {}: {}", status, error_text);
            anyhow::bail!("OSRM table request failed with status {}", status);
        }
        
        let table_response = response.json::<OsrmTableResponse>()
            .await
            .context("Failed to parse OSRM table response")?;
            
        Ok(table_response)
    }
} 