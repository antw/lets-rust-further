use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("resource not found")]
    NotFound,

    #[error("validation errors: {0}")]
    Validation(#[from] validator::ValidationErrors),

    #[error("sqlx error: {0}")]
    SqlError(#[from] sqlx::Error),
}

impl<'a> IntoResponse for HandlerError {
    fn into_response(self) -> axum::response::Response {
        let resp = match self {
            HandlerError::Validation(messages) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({ "error": "validation error", "errors": messages.into_errors() })),
            ),
            HandlerError::NotFound => not_found(),
            HandlerError::SqlError(err) => match err {
                sqlx::Error::RowNotFound => not_found(),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "internal server error" })),
                ),
            },
        };

        resp.into_response()
    }
}

/// Creates a 404 Not Found response.
fn not_found() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": "the requested resource could not be found" })),
    )
}
