use actix_web::{http::StatusCode, ResponseError};
use orvrm::utils::error::AppError;

#[test]
fn test_app_error_response() {
    // Test validation error
    let validation_error = AppError::ValidationError("Invalid input".to_string());
    let response = validation_error.error_response();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // Test OSRM error
    let osrm_error = AppError::OsrmError("OSRM service unavailable".to_string());
    let response = osrm_error.error_response();
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);

    // Test internal error
    let internal_error = AppError::InternalError("Something went wrong".to_string());
    let response = internal_error.error_response();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
