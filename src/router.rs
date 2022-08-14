use crate::handlers;

use std::sync::Arc;
use std::time::Duration;

use axum::{
    error_handling::HandleErrorLayer,
    extract::Extension,
    handler::Handler,
    http::{header, Method, StatusCode},
    routing::{get, post},
    BoxError, Router,
};

use tower::ServiceBuilder;
use tower_http::{cors, trace::TraceLayer};

/// Builds a CorsLayer which will allow the given HTTP methods.
fn build_cors_layer<T>(methods: T) -> cors::CorsLayer
where
    T: Into<cors::AllowMethods>,
{
    cors::CorsLayer::new()
        .allow_headers([header::CONTENT_TYPE])
        .allow_methods(methods)
        .allow_origin(cors::Any)
}

pub(crate) fn build_router(app: crate::Application) -> Router {
    Router::new()
        .route(
            "/v1/healthcheck",
            ServiceBuilder::new()
                .layer(build_cors_layer([Method::GET]))
                .service(get(handlers::healthcheck::healthcheck)),
        )
        .route(
            "/v1/movies/:id",
            ServiceBuilder::new()
                .layer(build_cors_layer([Method::GET]))
                .service(get(handlers::movies::show_movie)),
        )
        .route(
            "/v1/movies",
            ServiceBuilder::new()
                .layer(build_cors_layer([Method::POST]))
                .service(post(handlers::movies::create_movie)),
        )
        .fallback(handlers::not_found.into_service())
        .layer(
            ServiceBuilder::new()
                .layer(Extension(Arc::new(app)))
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .layer(TraceLayer::new_for_http())
                .timeout(Duration::from_secs(30)),
        )
}

async fn handle_timeout_error(err: BoxError) -> (StatusCode, String) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            "Request took too long".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", err),
        )
    }
}
