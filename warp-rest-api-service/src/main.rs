use common::Movie;
use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    let host = env::var("RUST_ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = env::var("RUST_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("RUST_PORT must be a valid u16");

    let address = format!("{}:{}", host, port);
    println!("🚀 Server running on http://{}", address);

    // GET /movies
    let movies_route = warp::path("movies").and(warp::get()).map(|| {
        let movies: Vec<Movie> = vec![];

        warp::reply::json(&movies)
    });

    warp::serve(movies_route)
        .run((host.parse::<std::net::IpAddr>().unwrap(), port))
        .await;
}
