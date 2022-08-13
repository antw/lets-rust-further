#[macro_use]
extern crate lazy_static;

mod handlers;
mod router;

use std::net::SocketAddr;

use clap::Parser;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Debug, Default, Parser)]
#[clap(version = VERSION, about = "A simple REST API")]
struct Arguments {
    #[clap(short, long, default_value_t = 4000, help = "Port to listen on")]
    port: u16,

    #[clap(short, long, default_value_t = String::from("development"), help = "Server environment")]
    env: String,
}

#[derive(Clone, Debug)]
struct Application {
    config: Arguments,
}

async fn signal_shutdown() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c")
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Application {
        config: Arguments::parse(),
    };

    let local_app = app.clone();
    let addr = SocketAddr::from(([127, 0, 0, 1], local_app.config.port));

    tracing::info!(
        "{} server listening on http://localhost:{}",
        local_app.config.env,
        local_app.config.port
    );

    axum::Server::bind(&addr)
        .serve(router::build_router(app).into_make_service())
        .with_graceful_shutdown(signal_shutdown())
        .await
        .unwrap();
}
