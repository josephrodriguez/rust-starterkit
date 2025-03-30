use axum::Router;
use env_logger::{Builder, WriteStyle};
use log::{LevelFilter, info};
use std::env;
use std::error::Error;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Builder::new()
        .filter_level(LevelFilter::Debug)
        .write_style(WriteStyle::Always)
        .init();

    info!("Starting up...");

    let port =
        env::var("RUST_STARTER_KIT_AXUM_RESTAPI_PORT").unwrap_or_else(|_| "3000".to_string());
    let address = format!("0.0.0.0:{}", port);

    let app = Router::new();
    let listener = TcpListener::bind(address).await.unwrap();

    info!(
        "Server running at http://{}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
