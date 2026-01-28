use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use common::Movie;
use log::info;
use tokio::fs;

pub(crate) async fn handle_get_movies() -> impl IntoResponse {
    info!("Handle GET /movies endpoint request");

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
