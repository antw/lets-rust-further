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

pub(crate) async fn show_movie(
    Path(id): Path<i64>,
    Extension(app): App,
) -> Result<impl IntoResponse, HandlerError> {
    Ok(Json(app.models.movies.get(id).await?))
}

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

pub(crate) async fn update_movie(
    Path(id): Path<i64>,
    Json(params): Json<MovieParams>,
    Extension(app): App,
) -> Result<impl IntoResponse, HandlerError> {
    params.validate()?;

    let mut movie = app.models.movies.get(id).await?;

    movie.update(params);
    app.models.movies.update(&mut movie).await?;

    Ok((StatusCode::OK, Json(movie)))
}

pub(crate) async fn delete_movie(
    Path(id): Path<i64>,
    Extension(app): App,
) -> Result<impl IntoResponse, HandlerError> {
    app.models.movies.delete(id).await?;
    Ok((StatusCode::NO_CONTENT, Json(())))
}
