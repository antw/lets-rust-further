pub(crate) mod healthcheck;
pub(crate) mod movies;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub(crate) async fn root() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Json(json!({"message": "not found"})))
}

/// Creates a 400 Bad Request response.
fn bad_request(message: Option<&str>) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::BAD_REQUEST,
        Json(json!({ "message": message.unwrap_or("bad request") })),
    )
}
