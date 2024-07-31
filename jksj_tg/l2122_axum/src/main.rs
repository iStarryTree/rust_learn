use axum::{response::Html, routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt::init();

    let serve_dir =
        ServeDir::new("assets2").not_found_service(ServeFile::new("assets2/index.html"));

    let app = Router::new()
        .route("/foo", get(handler))
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/assets2", serve_dir.clone())
        .fallback_service(serve_dir)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::debug!("listening on 127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
