use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

pub(crate) async fn create_movie(
    Json(params): Json<MovieParams>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match params.validate() {
        Err(e) => Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"errors": e.into_errors()})),
        )),
        Ok(_) => Ok((StatusCode::CREATED, Json(Movie::from(params)))),
    }
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
            title: "The Shawshank Redemption".to_string(),
            year: 1994,
            runtime: 142,
            genres: vec!["crime".to_string(), "drama".to_string()],
            created_at: Utc::now(),
            version: 1,
        })
        .unwrap(),
    ))
}

#[derive(Debug, Deserialize, Validate)]
pub(crate) struct MovieParams {
    #[validate(
        required(message = "cannot be blank"),
        length(min = 1, message = "cannot be blank"),
        length(max = 500, message = "cannot be longer than 500 bytes")
    )]
    title: Option<String>,

    #[validate(
        required(message = "cannot be blank"),
        range(min = 1888, message = "cannot be less than 1888")
    )]
    year: Option<u32>,

    #[validate(
        required(message = "cannot be blank"),
        range(min = 1, message = "cannot be less than 1")
    )]
    runtime: Option<u32>,

    #[validate(
        required(message = "cannot be blank"),
        length(min = 1, message = "cannot be empty"),
        length(max = 5, message = "cannot have more than 5 genres")
    )]
    genres: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
struct Movie {
    id: u64,
    title: String,
    year: u32,
    runtime: u32,
    genres: Vec<String>,
    #[serde(skip)]
    created_at: DateTime<Utc>,
    version: u32,
}

impl From<MovieParams> for Movie {
    fn from(params: MovieParams) -> Self {
        Self {
            id: 0,
            title: params.title.unwrap(),
            year: params.year.unwrap(),
            runtime: params.runtime.unwrap(),
            genres: params.genres.unwrap(),
            created_at: Utc::now(),
            version: 1,
        }
    }
}
