use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::validator;

pub(crate) async fn create_movie(
    Json(params): Json<MovieParams>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    tracing::info!("received {:?}", params);

    let mut validator = validator::Validator::new();
    validate_movie(&mut validator, &params);

    if !validator.is_valid() {
        return Err(validator);
    }

    Ok((
        StatusCode::CREATED,
        Json(Movie {
            id: 0,
            title: params.title.unwrap(),
            year: params.year.unwrap(),
            runtime: params.runtime.unwrap(),
            genres: params.genres.unwrap(),
            created_at: Utc::now(),
            version: 1,
        }),
    ))
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
    #[serde(skip)]
    created_at: DateTime<Utc>,
    version: u32,
}

fn validate_movie(validator: &mut validator::Validator, params: &MovieParams) {
    validator.check(params.title.is_some(), "title", "cannot be blank");
    validator.check(params.year.is_some(), "year", "cannot be blank");
    validator.check(params.runtime.is_some(), "runtime", "cannot be blank");
    validator.check(params.genres.is_some(), "genres", "cannot be blank");

    validator.check(
        params.title.is_none() || !params.title.as_ref().unwrap().is_empty(),
        "title",
        "cannot be blank",
    );

    validator.check(
        params.title.is_none() || params.title.as_ref().unwrap().len() <= 500,
        "title",
        "cannot be more than 500 bytes long",
    );

    validator.check(
        params.year.is_none() || params.year.as_ref().unwrap() > &1888,
        "year",
        "cannot be less than 1888",
    );

    validator.check(
        params.runtime.is_none() || params.runtime.as_ref().unwrap() > &0,
        "runtime",
        "cannot be zero",
    );

    validator.check(
        params.genres.is_none() || params.genres.as_ref().unwrap().len() >= 1,
        "genres",
        "cannot be blank",
    );

    validator.check(
        params.genres.is_none() || params.genres.as_ref().unwrap().len() <= 5,
        "genres",
        "cannot contain more than 5 genres",
    );

    validator.check(
        params.genres.is_none() || validator::unique(params.genres.as_ref().unwrap()),
        "genres",
        "cannot contain duplicate values",
    );
}
