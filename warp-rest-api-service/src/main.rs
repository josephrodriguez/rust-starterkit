use tokio::fs;
use warp::Filter;

#[tokio::main]
async fn main() {
    let movies_route = warp::path("movies")
        .and(warp::get())
        .map(|| {
            let data = fs::read_to_string("movies.json").unwrap_or_else(|_| "[]".to_string());
            warp::reply::json(&serde_json::from_str::<Vec<Movie>>(&data).unwrap_or_default())
        });

    warp::serve(movies_route).run(([0, 0, 0, 0], 3000)).await;
}