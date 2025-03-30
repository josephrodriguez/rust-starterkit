mod models;

use axum::{Json, Router};
use env_logger::{Builder, WriteStyle};
use log::{LevelFilter, info};
use std::env;
use std::error::Error;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use tokio::fs;
use tokio::net::TcpListener;
use crate::models::Movie;

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

async fn handle_get_movies() -> impl IntoResponse {
    let movies_json = match fs::read_to_string("../assets/movies.json").await {
        Ok(data) => data,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let movies: Vec<Movie> = match serde_json::from_str(&movies_json) {
        Ok(movies) => movies,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    Json(movies).into_response()
}