use axum::extract::Query;
use axum::response::IntoResponse;
use axum::Form;
use axum::{response::Html, routing::get, Router};
use serde::Deserialize;
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
        .route("/form", get(show_form).post(accept_form))
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

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Input {
    name: String,
    email: String,
}

async fn accept_form(Form(input): Form<Input>) -> Html<&'static str> {
    tracing::debug!("form={:?}", input);

    Html("<h3>Test form</h3>")
}

async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html>
            <head></head>
            <body>
                <form action="/form" method="post">
                    <label for="name">
                        Enter your name:
                        <input type="text" id="name" name="name">
                    </label>

                    <label for="email">
                        Enter your email:
                        <input type="text" id="email" name="email">
                    </label>

                    <input type="submit" value="Submit">
                </form>
            </body>
        </html>
    "#,
    )
}
