mod handlers;

use crate::handlers::handle_get_movies;
use axum::routing::get;
use axum::Router;
use std::env;
use std::error::Error;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let port = env::var("RUST_PORT").unwrap_or_else(|_| "3000".to_string());
    let address = format!("0.0.0.0:{}", port);

    let app = Router::new().route("/movies", get(handle_get_movies));

    let listener = TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
