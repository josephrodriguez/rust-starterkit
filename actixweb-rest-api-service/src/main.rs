use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Movie {
    title: String,
    director: String,
    rating: f32,
    rank: u32,
    synopsis: String,
    genre: String,
    release_year: u32,
    cast: String,
}

#[get("/movies")]
async fn movies() -> impl Responder {
    let data = fs::read_to_string("movies.json").unwrap_or_else(|_| "[]".to_string());
    HttpResponse::Ok().json(serde_json::from_str::<Vec<Movie>>(&data).unwrap_or_default())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(movies))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}