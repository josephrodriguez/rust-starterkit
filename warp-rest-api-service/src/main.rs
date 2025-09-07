use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    let host = env::var("RUST_ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("RUST_PORT").unwrap_or_else(|_| "3000".to_string());
    let address = format!("{}:{}", host, port);

    let movies_route = warp::path("movies")
        .and(warp::get())
        .map(|| {
        });

    // warp::serve(movies_route).run(([0, 0, 0, 0], 3000)).await;
}