use thiserror::Error;
use actix_web::{HttpResponse, ResponseError};
use serde_json::json;

/// Application error types
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("OSRM service error: {0}")]
    #[allow(dead_code)]
    OsrmError(String),
    
    #[error("Routing error: {0}")]
    #[allow(dead_code)]
    RoutingError(String),
    
    #[error("Validation error: {0}")]
    #[allow(dead_code)]
    ValidationError(String),
    
    #[error("Internal server error: {0}")]
    #[allow(dead_code)]
    InternalError(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::ValidationError(msg) => {
                HttpResponse::BadRequest().json(json!({
                    "error": "Validation Error",
                    "message": msg
                }))
            },
            AppError::OsrmError(msg) => {
                HttpResponse::ServiceUnavailable().json(json!({
                    "error": "OSRM Service Error",
                    "message": msg
                }))
            },
            _ => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "Internal Server Error",
                    "message": self.to_string()
                }))
            }
        }
    }
} 