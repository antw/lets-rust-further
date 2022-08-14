pub(crate) mod healthcheck;
pub(crate) mod movies;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

/// Custom handler for 404 pages.
pub(crate) async fn not_found() -> impl IntoResponse {
    return not_found_response();
}

/// General function for creating an error response.
fn error_response(status: StatusCode, message: &str) -> (StatusCode, Json<serde_json::Value>) {
    (status, Json(json!({ "error": message })))
}

/// Creates a 400 Bad Request response.
fn bad_request_response(message: Option<&str>) -> (StatusCode, Json<serde_json::Value>) {
    error_response(StatusCode::BAD_REQUEST, message.unwrap_or("bad request"))
}

/// Creates a 404 Not Found response.
fn not_found_response() -> (StatusCode, Json<serde_json::Value>) {
    error_response(
        StatusCode::NOT_FOUND,
        "the requested resource could not be found",
    )
}

/// Creates a 500 Server Error response.
fn server_error_response() -> (StatusCode, Json<serde_json::Value>) {
    error_response(StatusCode::INTERNAL_SERVER_ERROR, "server error")
}
