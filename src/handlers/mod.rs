pub(crate) mod errors;
pub(crate) mod healthcheck;
pub(crate) mod movies;

use axum::response::IntoResponse;

/// Custom handler for 404 pages.
pub(crate) async fn not_found() -> impl IntoResponse {
    return errors::HandlerError::NotFound;
}
