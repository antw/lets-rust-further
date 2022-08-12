mod handlers;

use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use clap::Parser;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Default, Parser)]
#[clap(version = VERSION, about = "A simple REST API")]
struct Arguments {
    #[clap(short, long, default_value_t = 4000, help = "Port to listen on")]
    port: u16,

    #[clap(short, long, default_value_t = String::from("development"), help = "Server environment")]
    env: String,
}

#[derive(Debug)]
struct Application {
    config: Arguments,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Arc::new(Application {
        config: Arguments::parse(),
    });

    let local_app = app.clone();

    let router = Router::new()
        .route("/", get(handlers::root))
        .route("/v1/healthcheck", get(handlers::healthcheck::healthcheck))
        // Movies
        .route("/v1/movies", post(handlers::movies::create_movie))
        .route("/v1/movies/:id", get(handlers::movies::show_movie))
        .layer(Extension(app));

    let addr = SocketAddr::from(([127, 0, 0, 1], local_app.config.port));

    tracing::info!(
        "{} server listening on http://localhost:{}",
        local_app.config.env,
        local_app.config.port
    );

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
