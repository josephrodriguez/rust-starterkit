use env_logger::{Builder, WriteStyle};
use log::{info, LevelFilter};

fn main() {
    Builder::new()
        .filter_level(LevelFilter::Debug)
        .write_style(WriteStyle::Always)
        .init();

    info!("Hello, world from Rust with Axum!");
}
