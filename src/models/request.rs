use serde::{Deserialize, Serialize};
use super::{vehicle::Vehicle, job::Job};

/// Represents a complete routing optimization request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingRequest {
    /// List of vehicles available for the routing problem
    pub vehicles: Vec<Vehicle>,
    
    /// List of jobs to be assigned to vehicles
    pub jobs: Vec<Job>,
    
    /// Optional routing profile to use (car, bike, foot, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profile: Option<String>,
    
    /// Optional routing options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<RoutingOptions>,
}

/// Options for the routing algorithm
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RoutingOptions {
    /// Maximum time to spend on optimization (in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_time: Option<u32>,
    
    /// Number of threads to use for optimization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threads: Option<u8>,
    
    /// Whether to explore all possible vehicle combinations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explore_all: Option<bool>,
    
    /// Whether to return detailed route geometry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<bool>,
} 