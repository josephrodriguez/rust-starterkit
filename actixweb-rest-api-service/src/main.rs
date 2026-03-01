use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use common::Movie;
use std::fs;

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
