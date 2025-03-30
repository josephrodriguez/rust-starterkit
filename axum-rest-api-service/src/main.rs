mod models;
mod handlers;

use axum::Router;
use env_logger::{Builder, WriteStyle};
use log::{LevelFilter, info};
use std::env;
use std::error::Error;
use axum::routing::get;
use tokio::net::TcpListener;
use crate::handlers::handle_get_movies;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Builder::new()
        .filter_level(LevelFilter::Debug)
        .write_style(WriteStyle::Always)
        .init();

    info!("Starting up...");

    let port =
        env::var("RUST_STARTERKIT_AXUM_RESTAPI_PORT").unwrap_or_else(|_| "3000".to_string());
    let address = format!("0.0.0.0:{}", port);

    let app = Router::new().route("/movies", get(handle_get_movies));

    let listener = TcpListener::bind(address).await.unwrap();

    info!(
        "Server running at http://{}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

