use orvrm::services::osrm::OsrmConfig;
use orvrm::services::routing::{RoutingConfig, RoutingService};

#[tokio::test]
async fn test_routing_service_initialization() {
    let osrm_config = OsrmConfig {
        base_url: "http://localhost:5000".to_string(),
        default_profile: "car".to_string(),
        timeout_seconds: 30,
    };

    let routing_config = RoutingConfig {
        osrm: osrm_config,
        default_max_time: 300,
        default_threads: 4,
    };

    // Create the service and verify it doesn't panic
    let _routing_service = RoutingService::new(routing_config);

    // Just test that the service can be created without errors
    assert!(true); // Simple assertion to verify the service was created
}

// Additional tests would mock the OSRM service responses and test the routing logic
// For example:
// #[tokio::test]
// async fn test_process_request() {
//     // Setup mock OSRM service
//     // Create test request
//     // Verify response
// }
