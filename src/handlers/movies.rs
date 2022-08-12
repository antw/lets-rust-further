use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use chrono::{DateTime, Utc};
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
        Json(
            serde_json::to_value(Movie {
                id,
                created_at: Utc::now(),
                title: "The Shawshank Redemption".to_string(),
                year: 1994,
                runtime: 142,
                genres: vec!["crime".to_string(), "drama".to_string()],
                version: 1,
            })
            .unwrap(),
        ),
    );
}

#[derive(Debug, Serialize)]
struct Movie {
    id: u64,
    created_at: DateTime<Utc>,
    title: String,
    year: u32,
    runtime: u32,
    genres: Vec<String>,
    version: u32,
}
