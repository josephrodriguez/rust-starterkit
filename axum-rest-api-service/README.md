# Rust Axum Movies API

This is a simple REST API built with **Rust** using the **Axum** web framework. The API serves movie data from a JSON file and follows an asynchronous, non-blocking approach using **Tokio**.

## 📌 Tech Stack
- **Rust** - Safe and efficient systems programming language
- **Axum** - Web framework for Rust (built on top of `hyper`)
- **Tokio** - Asynchronous runtime for Rust
- **Serde** - Serialization/deserialization framework (used for JSON handling)
- **Tracing** - Logging framework for better observability

## 📡 API Endpoints

### **GET /movies**
#### Description:
Returns a list of movies from the embedded JSON file.

#### Request:

```http
GET /movies

[
    {
        "title": "The Shawshank Redemption",
        "director": "Frank Darabont",
        "rating": 9.3,
        "rank": 1,
        "synopsis": "Two imprisoned men bond over a number of years, finding solace and eventual redemption through acts of common decency.",
        "genre": "Drama",
        "releaseYear": 1994,
        "cast": "Tim Robbins, Morgan Freeman"
    }
]
```

## 📜 License
GNU General Public License v3.0