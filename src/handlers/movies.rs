use crate::handlers::errors::HandlerError;
use crate::models::movie::MovieParams;
use crate::App;

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use validator::Validate;

pub(crate) async fn create_movie(
    Json(params): Json<MovieParams>,
    Extension(app): App,
) -> Result<impl IntoResponse, HandlerError> {
    params.validate()?;

    Ok((
        StatusCode::CREATED,
        Json(app.models.movies.insert(&params).await?),
    ))
}

pub(crate) async fn show_movie(
    Path(id): Path<i64>,
    Extension(app): App,
) -> Result<impl IntoResponse, HandlerError> {
    Ok(Json(app.models.movies.get(id).await?))
}
