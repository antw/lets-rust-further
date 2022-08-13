use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub(crate) async fn create_movie(Json(params): Json<MovieParams>) -> impl IntoResponse {
    tracing::info!("received {:?}", params);

    (
        StatusCode::CREATED,
        Json(json!({"message": "movie created"})),
    )
}

pub(crate) async fn show_movie(
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if id < 1 {
        return Err(super::bad_request_response(Some("invalid id")));
    }

    Ok(Json(
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
    ))
}

#[derive(Debug, Deserialize)]
pub(crate) struct MovieParams {
    title: Option<String>,
    year: Option<u32>,
    runtime: Option<u32>,
    genres: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
struct Movie {
    id: u64,
    title: String,
    year: u32,
    runtime: u32,
    genres: Vec<String>,
    created_at: DateTime<Utc>,
    version: u32,
}
