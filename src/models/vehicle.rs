use serde::{Deserialize, Serialize};

/// Represents a step in a vehicle's route
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RouteStep {
    #[serde(rename = "start")]
    Start {
        #[serde(skip_serializing_if = "Option::is_none")]
        service_after: Option<i64>,
        
        /// Location coordinates [longitude, latitude]
        #[serde(skip_serializing_if = "Option::is_none")]
        location: Option<[f64; 2]>,
        
        /// Arrival time at this step
        #[serde(skip_serializing_if = "Option::is_none")]
        arrival_time: Option<i64>,
        
        /// Departure time from this step
        #[serde(skip_serializing_if = "Option::is_none")]
        departure_time: Option<i64>,
    },
    #[serde(rename = "job")]
    Job {
        /// Job ID
        id: u64,
        
        /// Location coordinates [longitude, latitude]
        #[serde(skip_serializing_if = "Option::is_none")]
        location: Option<[f64; 2]>,
        
        /// Service time in seconds
        #[serde(skip_serializing_if = "Option::is_none")]
        service: Option<u32>,
        
        /// Arrival time at this step
        #[serde(skip_serializing_if = "Option::is_none")]
        arrival_time: Option<i64>,
        
        /// Departure time from this step
        #[serde(skip_serializing_if = "Option::is_none")]
        departure_time: Option<i64>,
    },
    #[serde(rename = "end")]
    End {
        /// Location coordinates [longitude, latitude]
        #[serde(skip_serializing_if = "Option::is_none")]
        location: Option<[f64; 2]>,
        
        /// Arrival time at this step
        #[serde(skip_serializing_if = "Option::is_none")]
        arrival_time: Option<i64>,
        
        /// Departure time from this step
        #[serde(skip_serializing_if = "Option::is_none")]
        departure_time: Option<i64>,
    },
}

/// Represents a vehicle in the routing problem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
    /// Unique identifier for the vehicle
    pub id: u64,
    
    /// Starting location as [longitude, latitude]
    pub start: [f64; 2],
    
    /// Ending location as [longitude, latitude]
    pub end: [f64; 2],
    
    /// Vehicle capacity (can be multi-dimensional)
    #[serde(default)]
    pub capacity: Vec<u32>,
    
    /// Time window for the vehicle's operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_window: Option<[i64; 2]>,
    
    /// Predefined steps for the vehicle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<RouteStep>>,
    
    /// Skills that the vehicle possesses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skills: Option<Vec<String>>,
}

/// Represents a vehicle with its assigned route in the solution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleRoute {
    /// Reference to the vehicle
    pub vehicle_id: u64,
    
    /// Ordered list of jobs in the route
    pub route: Vec<u64>,
    
    /// Sequence of steps in the route including start, jobs, and end
    pub steps: Vec<RouteStep>,
    
    /// Total distance of the route in meters
    pub distance: u32,
    
    /// Total duration of the route in seconds
    pub duration: u32,
    
    /// Estimated arrival times at each stop
    pub arrival_times: Vec<i64>,
    
    /// Estimated departure times from each stop
    pub departure_times: Vec<i64>,
    
    /// Load of the vehicle after each stop
    pub load_profile: Vec<Vec<i32>>,
    
    /// Polyline representation of the route geometry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polyline: Option<String>,
} 