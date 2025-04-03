#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/movies").get(movies);
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}