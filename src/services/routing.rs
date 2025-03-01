use anyhow::Result;
use log::{info, warn};
use std::time::Instant;
use serde::Deserialize;
use crate::models::{
    Job, VehicleRoute, RoutingRequest, RoutingResponse, RoutingSummary, RouteStep
};
use super::osrm::{OsrmService, OsrmConfig};

/// Configuration for the routing service
#[derive(Debug, Clone, Deserialize)]
pub struct RoutingConfig {
    /// OSRM service configuration
    pub osrm: OsrmConfig,
    
    /// Default maximum time for optimization in seconds
    pub default_max_time: u32,
    
    /// Default number of threads to use
    pub default_threads: u8,
}

impl Default for RoutingConfig {
    fn default() -> Self {
        Self {
            osrm: OsrmConfig::default(),
            default_max_time: 30,
            default_threads: 4,
        }
    }
}

/// Service for handling routing optimization
#[derive(Debug, Clone)]
pub struct RoutingService {
    osrm: OsrmService,
    config: RoutingConfig,
}

impl RoutingService {
    /// Create a new routing service with the given configuration
    pub fn new(config: RoutingConfig) -> Self {
        let osrm = OsrmService::new(config.osrm.clone());
        Self { osrm, config }
    }
    
    /// Process a routing request and return an optimized solution
    pub async fn process_request(&self, request: RoutingRequest) -> Result<RoutingResponse> {
        let start_time = Instant::now();
        
        // Extract options
        let max_time = request.options
            .as_ref()
            .and_then(|o| o.max_time)
            .unwrap_or(self.config.default_max_time);
            
        let threads = request.options
            .as_ref()
            .and_then(|o| o.threads)
            .unwrap_or(self.config.default_threads);
            
        let include_geometry = request.options
            .as_ref()
            .and_then(|o| o.geometry)
            .unwrap_or(false);
            
        let routing_profile = request.routing_profile
            .as_deref()
            .unwrap_or(&self.config.osrm.default_profile);
            
        info!(
            "Processing routing request with {} vehicles and {} jobs",
            request.vehicles.len(),
            request.jobs.len()
        );
        
        // Check if we have predefined routes
        let has_predefined_routes = request.vehicles.iter()
            .any(|v| v.steps.is_some() && !v.steps.as_ref().unwrap().is_empty());
            
        let routes = if has_predefined_routes {
            // Process predefined routes
            self.process_predefined_routes(&request, routing_profile, include_geometry).await?
        } else {
            // Perform optimization
            self.optimize_routes(&request, routing_profile, max_time, threads, include_geometry).await?
        };
        
        // Calculate summary
        let mut total_distance = 0;
        let mut total_duration = 0;
        let unassigned = Vec::new(); // In a real implementation, this would be populated
        
        for route in &routes {
            total_distance += route.distance as u64;
            total_duration += route.duration as u64;
        }
        
        let summary = RoutingSummary {
            cost: total_duration as f64, // Simple cost function based on duration
            distance: total_distance,
            duration: total_duration,
            routes: routes.len() as u32,
            unassigned: unassigned.len() as u32,
            computing_time: start_time.elapsed().as_millis() as u64,
        };
        
        // Build response
        let geometry = if include_geometry {
            // Extract polylines from routes if available
            let polylines = routes.iter()
                .filter_map(|route| route.polyline.clone())
                .collect::<Vec<_>>();
                
            if polylines.is_empty() {
                None
            } else {
                Some(polylines)
            }
        } else {
            None
        };
        
        let response = RoutingResponse {
            summary,
            routes,
            unassigned,
            geometry,
        };
        
        info!(
            "Routing completed in {}ms: {} routes, {} unassigned",
            response.summary.computing_time,
            response.summary.routes,
            response.summary.unassigned
        );
        
        Ok(response)
    }
    
    /// Process predefined routes from the request
    async fn process_predefined_routes(
        &self,
        request: &RoutingRequest,
        profile: &str,
        include_geometry: bool,
    ) -> Result<Vec<VehicleRoute>> {
        let mut routes = Vec::new();
        
        // Create a map of job IDs to jobs for quick lookup
        let job_map: std::collections::HashMap<u64, &Job> = request.jobs
            .iter()
            .map(|job| (job.id, job))
            .collect();
            
        for vehicle in &request.vehicles {
            if let Some(steps) = &vehicle.steps {
                // Extract job IDs from steps
                let mut job_ids = Vec::new();
                for step in steps {
                    if let RouteStep::Job { id, .. } = step {
                        job_ids.push(*id);
                    }
                }
                
                if job_ids.is_empty() {
                    continue;
                }
                
                // Collect coordinates for the route
                let mut coordinates = Vec::new();
                coordinates.push(vehicle.start);
                
                for job_id in &job_ids {
                    if let Some(job) = job_map.get(job_id) {
                        coordinates.push(job.location);
                    } else {
                        warn!("Job ID {} not found in job list", job_id);
                    }
                }
                
                coordinates.push(vehicle.end);
                
                // Get route from OSRM
                let osrm_response = self.osrm.route(
                    &coordinates,
                    Some(profile),
                    include_geometry
                ).await?;
                
                if osrm_response.routes.is_empty() {
                    warn!("No route found for vehicle {}", vehicle.id);
                    continue;
                }
                
                let osrm_route = &osrm_response.routes[0];
                
                // Calculate arrival and departure times
                // This is a simplified implementation
                let mut arrival_times = Vec::new();
                let mut departure_times = Vec::new();
                let mut current_time = 0;
                
                // Start time
                arrival_times.push(current_time);
                
                if let Some(RouteStep::Start { service_after, .. }) = steps.first() {
                    if let Some(time) = service_after {
                        current_time = *time;
                    }
                }
                
                departure_times.push(current_time);
                
                // Job stops
                for (i, job_id) in job_ids.iter().enumerate() {
                    let leg_duration = osrm_route.legs[i].duration as i64;
                    current_time += leg_duration;
                    arrival_times.push(current_time);
                    
                    if let Some(job) = job_map.get(job_id) {
                        current_time += job.service as i64;
                    }
                    
                    departure_times.push(current_time);
                }
                
                // Create steps for the route
                let mut route_steps = Vec::new();
                
                // Add start step
                if let Some(first_step) = steps.first() {
                    if let RouteStep::Start { service_after, .. } = first_step {
                        route_steps.push(RouteStep::Start { 
                            service_after: *service_after,
                            location: Some(vehicle.start),
                            arrival_time: Some(arrival_times[0]),
                            departure_time: Some(departure_times[0]),
                        });
                    } else {
                        route_steps.push(RouteStep::Start { 
                            service_after: None,
                            location: Some(vehicle.start),
                            arrival_time: Some(arrival_times[0]),
                            departure_time: Some(departure_times[0]),
                        });
                    }
                } else {
                    route_steps.push(RouteStep::Start { 
                        service_after: None,
                        location: Some(vehicle.start),
                        arrival_time: Some(arrival_times[0]),
                        departure_time: Some(departure_times[0]),
                    });
                }
                
                // Add job steps
                for (i, job_id) in job_ids.iter().enumerate() {
                    let job = job_map.get(job_id).cloned();
                    let location = job.map(|j| j.location);
                    let service = job.map(|j| j.service);
                    
                    route_steps.push(RouteStep::Job { 
                        id: *job_id,
                        location,
                        service,
                        arrival_time: Some(arrival_times[i + 1]),
                        departure_time: Some(departure_times[i + 1]),
                    });
                }
                
                // Add end step
                route_steps.push(RouteStep::End {
                    location: Some(vehicle.end),
                    arrival_time: Some(arrival_times.last().cloned().unwrap_or(0)),
                    departure_time: Some(departure_times.last().cloned().unwrap_or(0)),
                });
                
                // Create vehicle route
                let vehicle_route = VehicleRoute {
                    vehicle_id: vehicle.id,
                    route: job_ids,
                    steps: route_steps,
                    distance: osrm_route.distance as u32,
                    duration: osrm_route.duration as u32,
                    arrival_times,
                    departure_times,
                    load_profile: Vec::new(), // In a real implementation, this would be calculated
                    polyline: osrm_route.geometry.clone(),
                };
                
                routes.push(vehicle_route);
            }
        }
        
        Ok(routes)
    }
    
    /// Optimize routes for the given request
    async fn optimize_routes(
        &self,
        request: &RoutingRequest,
        profile: &str,
        _max_time: u32,
        _threads: u8,
        include_geometry: bool,
    ) -> Result<Vec<VehicleRoute>> {
        // In a real implementation, this would use a proper optimization algorithm
        // For now, we'll implement a simple greedy algorithm
        
        // Collect all locations
        let mut all_locations = Vec::new();
        
        // Add vehicle start/end locations
        for vehicle in &request.vehicles {
            all_locations.push(vehicle.start);
            if vehicle.start != vehicle.end {
                all_locations.push(vehicle.end);
            }
        }
        
        // Add job locations
        for job in &request.jobs {
            all_locations.push(job.location);
        }
        
        // Remove duplicates by converting to strings and back
        let mut unique_locations = Vec::new();
        let mut seen = std::collections::HashSet::new();
        
        for loc in all_locations {
            let loc_str = format!("{},{}", loc[0], loc[1]);
            if seen.insert(loc_str.clone()) {
                unique_locations.push((loc_str, loc));
            }
        }
        
        // Get distance/duration matrix from OSRM
        let matrix_response = self.osrm.table(
            &unique_locations.iter().map(|(_, loc)| *loc).collect::<Vec<_>>(),
            Some(profile),
            true
        ).await?;
        
        // Simple greedy assignment
        let mut routes = Vec::new();
        let mut assigned_jobs = std::collections::HashSet::new();
        
        for vehicle in &request.vehicles {
            // Find indices for start and end
            let start_str = format!("{},{}", vehicle.start[0], vehicle.start[1]);
            let end_str = format!("{},{}", vehicle.end[0], vehicle.end[1]);
            
            let start_idx = unique_locations.iter().position(|(s, _)| *s == start_str).unwrap();
            let _end_idx = unique_locations.iter().position(|(s, _)| *s == end_str).unwrap();
            
            // Find closest unassigned jobs
            let mut route_jobs = Vec::new();
            let mut current_idx = start_idx;
            let mut current_capacity = vehicle.capacity.clone();
            
            for _ in 0..request.jobs.len() {
                if route_jobs.len() >= 10 || assigned_jobs.len() >= request.jobs.len() {
                    break;
                }
                
                let mut best_job = None;
                let mut best_distance = f64::MAX;
                
                for job in &request.jobs {
                    if assigned_jobs.contains(&job.id) {
                        continue;
                    }
                    
                    // Check capacity constraints
                    if let Some(delivery) = &job.delivery {
                        let mut can_deliver = true;
                        for (i, amount) in delivery.iter().enumerate() {
                            if i >= current_capacity.len() || *amount as u32 > current_capacity[i] {
                                can_deliver = false;
                                break;
                            }
                        }
                        
                        if !can_deliver {
                            continue;
                        }
                    }
                    
                    let job_str = format!("{},{}", job.location[0], job.location[1]);
                    let job_idx = unique_locations.iter().position(|(s, _)| *s == job_str).unwrap();
                    let distance = matrix_response.durations[current_idx][job_idx];
                    
                    if distance < best_distance {
                        best_distance = distance;
                        best_job = Some(job);
                    }
                }
                
                if let Some(job) = best_job {
                    route_jobs.push(job.id);
                    assigned_jobs.insert(job.id);
                    
                    let job_str = format!("{},{}", job.location[0], job.location[1]);
                    current_idx = unique_locations.iter().position(|(s, _)| *s == job_str).unwrap();
                    
                    // Update capacity
                    if let Some(delivery) = &job.delivery {
                        for (i, amount) in delivery.iter().enumerate() {
                            if i < current_capacity.len() {
                                current_capacity[i] -= *amount as u32;
                            }
                        }
                    }
                } else {
                    break;
                }
            }
            
            if route_jobs.is_empty() {
                continue;
            }
            
            // Calculate route
            let mut coordinates = Vec::new();
            coordinates.push(vehicle.start);
            
            for job_id in &route_jobs {
                let job = request.jobs.iter().find(|j| j.id == *job_id).unwrap();
                coordinates.push(job.location);
            }
            
            coordinates.push(vehicle.end);
            
            // Get route from OSRM
            let osrm_response = self.osrm.route(
                &coordinates,
                Some(profile),
                include_geometry
            ).await?;
            
            if osrm_response.routes.is_empty() {
                warn!("No route found for vehicle {}", vehicle.id);
                continue;
            }
            
            let osrm_route = &osrm_response.routes[0];
            
            // Calculate arrival and departure times
            let mut arrival_times = Vec::new();
            let mut departure_times = Vec::new();
            let mut current_time = 0;
            
            // Start time
            arrival_times.push(current_time);
            
            if let Some(time_window) = vehicle.time_window {
                current_time = time_window[0];
            }
            
            departure_times.push(current_time);
            
            // Job stops
            for (i, job_id) in route_jobs.iter().enumerate() {
                let leg_duration = osrm_route.legs[i].duration as i64;
                current_time += leg_duration;
                arrival_times.push(current_time);
                
                let job = request.jobs.iter().find(|j| j.id == *job_id).unwrap();
                current_time += job.service as i64;
                
                departure_times.push(current_time);
            }
            
            // Final leg to end
            let final_leg_duration = osrm_route.legs.last().unwrap().duration as i64;
            current_time += final_leg_duration;
            arrival_times.push(current_time);
            departure_times.push(current_time);
            
            // Create steps for the route
            let mut route_steps = Vec::new();
            
            // Add start step
            let service_after = if let Some(time_window) = vehicle.time_window {
                Some(time_window[0])
            } else {
                None
            };
            route_steps.push(RouteStep::Start { 
                service_after,
                location: Some(vehicle.start),
                arrival_time: Some(arrival_times[0]),
                departure_time: Some(departure_times[0]),
            });
            
            // Add job steps
            for (i, job_id) in route_jobs.iter().enumerate() {
                let job = request.jobs.iter().find(|j| j.id == *job_id).unwrap();
                route_steps.push(RouteStep::Job { 
                    id: *job_id,
                    location: Some(job.location),
                    service: Some(job.service),
                    arrival_time: Some(arrival_times[i + 1]),
                    departure_time: Some(departure_times[i + 1]),
                });
            }
            
            // Add end step
            route_steps.push(RouteStep::End {
                location: Some(vehicle.end),
                arrival_time: Some(arrival_times.last().cloned().unwrap_or(0)),
                departure_time: Some(departure_times.last().cloned().unwrap_or(0)),
            });
            
            // Create vehicle route
            let vehicle_route = VehicleRoute {
                vehicle_id: vehicle.id,
                route: route_jobs,
                steps: route_steps,
                distance: osrm_route.distance as u32,
                duration: osrm_route.duration as u32,
                arrival_times,
                departure_times,
                load_profile: Vec::new(), // In a real implementation, this would be calculated
                polyline: osrm_route.geometry.clone(),
            };
            
            routes.push(vehicle_route);
        }
        
        Ok(routes)
    }
} 