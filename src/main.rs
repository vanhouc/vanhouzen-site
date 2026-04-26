use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::routing::get;
use hypertext::prelude::*;

mod cameron;

// Entrypoint for axum application
#[tokio::main]
async fn main() {
    if let Err(error) = dotenvy::dotenv() {
        log::error!("failed to load env file, error: {error}")
    }

    // Setup logging out to the console
    logforth::stdout().apply();

    let app = axum::Router::new()
        .nest_service("/assets", tower_http::services::ServeDir::new("assets"))
        .nest("/cameron", cameron::router())
        .route("/", get(index))
        .fallback(async || axum::response::Redirect::to("/"));

    #[cfg(debug_assertions)]
    let app = app.layer(tower_livereload::LiveReloadLayer::new());

    let port: u16 = std::env::var("PORT")
        .expect("PORT environment variable must be set")
        .parse()
        .expect("port must be a valid u16");

    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn layout(title: &str, content: impl hypertext::Renderable) -> impl axum::response::IntoResponse {
    hypertext::maud! {
        !DOCTYPE
        html {
            head {
                title { (title) }
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                link rel="stylesheet" href="/assets/pico.min.css";
                link rel="stylesheet" href="/assets/style.css";
            }
            body {(content)}
        }
    }
}

async fn index() -> impl axum::response::IntoResponse {
    let content = hypertext::maud! {
        main {
            h1 { "Welcome to the VanHouzen Family!" }
            p { "This site houses the personal pages for the VanHouzen family and friends" }
            h2 { "VanHouzens" }
            ul {
                li { a href="/cameron" { "Cameron VanHouzen" } }
                li { a href="/courtney" { "Courtney VanHouzen" } }
                li { a href="/wade" { "Wade VanHouzen" } }
                li { a href="/janet" { "Janet VanHouzen" } }
            }
            h2 { "Friends" }
            p { "Checkout the websites of our friends!" }
            ul {
                li { a href="/schrider" { "Schriders" } }
                li { a href="/pobudas" { "Pobudas" } }
            }
        }
    };
    layout("Home", content)
}
