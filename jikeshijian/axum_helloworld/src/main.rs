use axum::{response::Html, routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let serve_dir =
        ServeDir::new("assert2").not_found_service(ServeFile::new("assert2/index.html"));

    let app = Router::new()
        .route("/foo", get(handler))
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/assert2", serve_dir.clone())
        .fallback_service(serve_dir);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World</h1>")
}
