mod handlers;
mod models;
mod router;

use std::net::SocketAddr;
use std::sync::Arc;

use clap::Parser;
use sqlx::postgres::PgPoolOptions;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Debug, Default, Parser)]
#[clap(version = VERSION, about = "A simple REST API")]
struct Arguments {
    #[clap(short, long, default_value_t = 4000, help = "Port to listen on")]
    port: u16,

    #[clap(short, long, default_value_t = String::from("development"), help = "Server environment")]
    env: String,
}

struct Application {
    config: Arguments,
    models: models::Models,
}

/// A type alias for accessing the Application instance in handlers.
type App = axum::Extension<Arc<Application>>;

async fn signal_shutdown() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c")
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://greenscreen:pa55word@localhost:5432/greenscreen?sslmode=disable")
        .await?;

    let app = Application {
        config: Arguments::parse(),
        models: models::Models::new(Arc::new(pool)),
    };

    // let local_app = app.clone();
    let addr = SocketAddr::from(([127, 0, 0, 1], app.config.port));

    tracing::info!(
        "{} server listening on http://localhost:{}",
        app.config.env,
        app.config.port
    );

    axum::Server::bind(&addr)
        .serve(router::build_router(app).into_make_service())
        .with_graceful_shutdown(signal_shutdown())
        .await
        .unwrap();

    Ok(())
}
