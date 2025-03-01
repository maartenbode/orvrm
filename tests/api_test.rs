use actix_web::{test, web, App};
use orvrm::api::routes::{configure_routes, health_check};
use orvrm::services::osrm::OsrmConfig;
use orvrm::services::routing::{RoutingConfig, RoutingService};

#[actix_web::test]
async fn test_health_check() {
    let app = test::init_service(App::new().route("/health", web::get().to(health_check))).await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(response["status"], "ok");
    assert!(response["version"].is_string());
}

#[actix_web::test]
async fn test_api_routes_configuration() {
    // Create a mock routing service
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

    let routing_service = RoutingService::new(routing_config);

    // Initialize the test service with our route configuration
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(routing_service))
            .configure(configure_routes),
    )
    .await;

    // Test that the health endpoint is configured correctly
    let req = test::TestRequest::get().uri("/api/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}
