use axum::{response::IntoResponse, routing::get};
use dotenvy::dotenv;
use rinja::Template;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

// Entrypoint for axum application
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    tracing_subscriber::fmt::init();

    let app = axum::Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .fallback(get(index))
        .layer(LiveReloadLayer::new());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[tracing::instrument]
async fn index() -> impl IntoResponse {
    IndexTemplate {}
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}
