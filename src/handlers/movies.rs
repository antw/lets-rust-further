use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use serde_json::json;

pub(crate) async fn create_movie() -> impl IntoResponse {
    (
        StatusCode::CREATED,
        Json(json!({"message": "movie created"})),
    )
}

pub(crate) async fn show_movie(Path(id): Path<u64>) -> impl IntoResponse {
    if id < 1 {
        return super::bad_request(Some("invalid id"));
    }

    return (
        StatusCode::OK,
        Json(serde_json::to_value(Movie { id }).unwrap()),
    );
}

#[derive(Serialize)]
struct Movie {
    id: u64,
}
