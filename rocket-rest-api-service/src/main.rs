#[macro_use]
extern crate rocket;
use common::Movie;
use rocket::serde::json::Json;
use std::fs;

// #[derive(Serialize)]
// #[serde(crate = "rocket::serde")]
// struct Movie {
//     title: String,
//     year: u16,
// }

#[get("/movies")]
fn movies() -> Json<Vec<Movie>> {
    let data = fs::read_to_string("movies.json").unwrap_or_else(|_| "[]".to_string());
    Json(serde_json::from_str::<Vec<Movie>>(&data).unwrap_or_default())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![movies])
}
