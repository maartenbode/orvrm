use actix_web::{web, HttpResponse, Responder};
use log::{info, error};
use crate::models::RoutingRequest;
use crate::services::RoutingService;

/// Health check endpoint
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// Process a routing optimization request
pub async fn optimize(
    request: web::Json<RoutingRequest>,
    routing_service: web::Data<RoutingService>,
) -> impl Responder {
    info!("Received optimization request with {} vehicles and {} jobs", 
        request.vehicles.len(), request.jobs.len());
    
    match routing_service.process_request(request.into_inner()).await {
        Ok(response) => {
            info!("Optimization completed successfully");
            HttpResponse::Ok().json(response)
        },
        Err(err) => {
            error!("Optimization failed: {}", err);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Optimization failed: {}", err)
            }))
        }
    }
}

/// Configure API routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(health_check))
            .route("/optimize", web::post().to(optimize))
    );
} 