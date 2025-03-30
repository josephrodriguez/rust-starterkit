use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Movie {
    title: String,
    director: String,
    rating: f32,
    rank: u32,
    synopsis: String,
    genre: String,
    release_year: u32,
    cast: String,
}