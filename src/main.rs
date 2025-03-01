use actix_web::{web, App, HttpServer, middleware::Logger};
use env_logger::Env;
use log::{info, error};
use std::io;

mod api;
mod models;
mod services;
mod config;
mod utils;

use config::AppConfig;
use services::{RoutingService, RoutingConfig};

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    // Load configuration
    let config = match AppConfig::load() {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            return Err(io::Error::new(io::ErrorKind::Other, e));
        }
    };
    
    info!("Starting ORVRM server on {}:{}", config.server.host, config.server.port);
    
    // Create routing service
    let routing_config = RoutingConfig {
        osrm: config.osrm.clone(),
        default_max_time: config.routing.default_max_time,
        default_threads: config.routing.default_threads,
    };
    
    let routing_service = RoutingService::new(routing_config);
    
    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(routing_service.clone()))
            .configure(api::configure_routes)
    })
    .bind((config.server.host.clone(), config.server.port))?
    .workers(config.server.workers)
    .run()
    .await
}
