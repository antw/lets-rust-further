pub(crate) mod healthcheck;
pub(crate) mod movies;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

/// Custom handler for 404 pages.
pub(crate) async fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(json!({"error": "the requested resource could not be not found"})),
    )
}

/// General function for creating an error response.
fn error_response(status: StatusCode, message: &str) -> (StatusCode, Json<serde_json::Value>) {
    (status, Json(json!({ "error": message })))
}

/// Creates a 400 Bad Request response.
fn bad_request_response(message: Option<&str>) -> (StatusCode, Json<serde_json::Value>) {
    error_response(StatusCode::BAD_REQUEST, message.unwrap_or("bad request"))
}
