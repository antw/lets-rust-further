use crate::handlers;

use std::sync::Arc;

use axum::{
    extract::Extension,
    handler::Handler,
    http::{header, Method},
    routing::{get, post},
    Router,
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
        .layer(Extension(Arc::new(app)))
        .layer(TraceLayer::new_for_http())
}
