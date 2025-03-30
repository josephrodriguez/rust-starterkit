use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use tokio::fs;
use crate::models::Movie;

pub(crate) async fn handle_get_movies() -> impl IntoResponse {
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