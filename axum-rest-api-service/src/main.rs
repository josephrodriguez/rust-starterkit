use std::env;
use env_logger::{Builder, WriteStyle};
use log::{info, LevelFilter};

fn main() {
    Builder::new()
        .filter_level(LevelFilter::Debug)
        .write_style(WriteStyle::Always)
        .init();

    let port = env::var("RUST_STARTER_KIT_AXUM_RESTAPI_PORT").unwrap_or_else(|_| "3000".to_string());
    let address = format!("0.0.0.0:{}", port);

    info!("Hello, world from Rust with Axum!");
}
