use serde::{Deserialize, Serialize};
use super::vehicle::VehicleRoute;

/// Represents a complete routing optimization response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingResponse {
    /// Summary of the optimization result
    pub summary: RoutingSummary,
    
    /// Routes for each vehicle
    pub routes: Vec<VehicleRoute>,
    
    /// IDs of unassigned jobs
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub unassigned: Vec<u64>,
    
    /// Detailed route geometries if requested
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<Vec<String>>,
}

/// Summary of the optimization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingSummary {
    /// Total cost of the solution
    pub cost: f64,
    
    /// Total distance of all routes in meters
    pub distance: u64,
    
    /// Total duration of all routes in seconds
    pub duration: u64,
    
    /// Number of routes in the solution
    pub routes: u32,
    
    /// Number of unassigned jobs
    pub unassigned: u32,
    
    /// Computation time in milliseconds
    pub computing_time: u64,
} 