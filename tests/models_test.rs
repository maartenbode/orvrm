use orvrm::models::job::Job;
use orvrm::models::request::RoutingRequest;
use orvrm::models::vehicle::Vehicle;
use serde_json;

#[test]
fn test_job_serialization() {
    let job = Job {
        id: 1,
        location: [4.8945, 52.3667], // Amsterdam [longitude, latitude]
        service: 300,
        delivery: Some(vec![10]),
        pickup: None,
        time_windows: None,
        skills: Some(vec!["delivery".to_string()]),
        priority: Some(1),
    };

    let serialized = serde_json::to_string(&job).unwrap();
    let deserialized: Job = serde_json::from_str(&serialized).unwrap();

    assert_eq!(job.id, deserialized.id);
    assert_eq!(job.location, deserialized.location);
    assert_eq!(job.service, deserialized.service);
    assert_eq!(job.skills, deserialized.skills);
    assert_eq!(job.priority, deserialized.priority);
}

#[test]
fn test_vehicle_serialization() {
    let vehicle = Vehicle {
        id: 1,
        start: [4.8945, 52.3667], // Amsterdam [longitude, latitude]
        end: [4.8945, 52.3667],   // Amsterdam [longitude, latitude]
        capacity: vec![100],
        time_window: None,
        steps: None,
        skills: Some(vec!["delivery".to_string()]),
    };

    let serialized = serde_json::to_string(&vehicle).unwrap();
    let deserialized: Vehicle = serde_json::from_str(&serialized).unwrap();

    assert_eq!(vehicle.id, deserialized.id);
    assert_eq!(vehicle.start, deserialized.start);
    assert_eq!(vehicle.end, deserialized.end);
    assert_eq!(vehicle.capacity, deserialized.capacity);
    assert_eq!(vehicle.skills, deserialized.skills);
}

#[test]
fn test_routing_request_serialization() {
    let job = Job {
        id: 1,
        location: [4.8945, 52.3667], // Amsterdam [longitude, latitude]
        service: 300,
        delivery: Some(vec![10]),
        pickup: None,
        time_windows: None,
        skills: Some(vec!["delivery".to_string()]),
        priority: Some(1),
    };

    let vehicle = Vehicle {
        id: 1,
        start: [4.8945, 52.3667], // Amsterdam [longitude, latitude]
        end: [4.8945, 52.3667],   // Amsterdam [longitude, latitude]
        capacity: vec![100],
        time_window: None,
        steps: None,
        skills: Some(vec!["delivery".to_string()]),
    };

    let request = RoutingRequest {
        vehicles: vec![vehicle],
        jobs: vec![job],
        routing_profile: Some("car".to_string()),
        options: None,
    };

    let serialized = serde_json::to_string(&request).unwrap();
    let deserialized: RoutingRequest = serde_json::from_str(&serialized).unwrap();

    assert_eq!(request.vehicles.len(), deserialized.vehicles.len());
    assert_eq!(request.jobs.len(), deserialized.jobs.len());
    assert_eq!(request.routing_profile, deserialized.routing_profile);
}
