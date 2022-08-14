use std::sync::Arc;

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use validator::Validate;

use crate::models::movie::MovieParams;

pub(crate) async fn create_movie(
    Json(params): Json<MovieParams>,
    Extension(app): Extension<Arc<crate::Application>>,
) -> impl IntoResponse {
    match params.validate() {
        Err(e) => Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"errors": e.into_errors()})),
        )),
        Ok(_) => {
            let movie = app.models.movies.insert(&params).await;
            Ok((
                StatusCode::CREATED,
                Json(movie.expect("expected movie to be created")),
            ))
        }
    }
}

pub(crate) async fn show_movie(
    Path(id): Path<i64>,
    Extension(app): Extension<Arc<crate::Application>>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if id < 1 {
        return Err(super::bad_request_response(Some("invalid id")));
    }

    match app.models.movies.get(id).await {
        Err(sqlx::Error::RowNotFound) => Err(super::not_found_response()),
        Err(_) => Err(super::server_error_response()),
        Ok(movie) => Ok(Json(serde_json::to_value(movie).unwrap())),
    }
}
