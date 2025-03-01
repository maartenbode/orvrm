use serde::{Deserialize, Serialize};

/// Represents a job (delivery, pickup, etc.) in the routing problem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    /// Unique identifier for the job
    pub id: u64,
    
    /// Location as [longitude, latitude]
    pub location: [f64; 2],
    
    /// Service time in seconds
    #[serde(default)]
    pub service: u32,
    
    /// Delivery amounts (can be multi-dimensional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<Vec<u32>>,
    
    /// Pickup amounts (can be multi-dimensional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup: Option<Vec<u32>>,
    
    /// Time windows for the job
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_windows: Option<Vec<[i64; 2]>>,
    
    /// Skills required to perform this job
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skills: Option<Vec<String>>,
    
    /// Priority of the job (higher value means higher priority)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u8>,
} 