use axum::extract::Query;
use axum::response::IntoResponse;
use serde::Deserialize;
use axum::{response::Html, routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let serve_dir =
        ServeDir::new("assert2").not_found_service(ServeFile::new("assert2/index.html"));

    let app = Router::new()
        .route("/foo", get(handler))
        .route("/query", get(query))
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/assert2", serve_dir.clone())
        .fallback_service(serve_dir)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    // println!("listening on {}", listener.local_addr().unwrap());
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World</h1>")
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct InputParams {
    foo: i32,
    bar: String,
    third: Option<i32>,
}

async fn query(Query(params): Query<InputParams>) -> impl IntoResponse {
    tracing::debug!("params={:?}", params);
    Html("<h3>Test query</h3>")
}
