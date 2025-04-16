use axum::{
    response::{IntoResponse, Redirect},
    routing::get,
};
use dotenvy::dotenv;
use fastrace::collector::{Config, ConsoleReporter};
use fastrace_axum::FastraceLayer;
use maud::{DOCTYPE, Markup, html};
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

// Entrypoint for axum application
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    fastrace::set_reporter(ConsoleReporter, Config::default());

    let app = axum::Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(index))
        .fallback(async || Redirect::temporary("/"))
        .layer(FastraceLayer);

    #[cfg(debug_assertions)]
    let app = app.layer(LiveReloadLayer::new());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    fastrace::flush();
}

fn layout(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
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

#[fastrace::trace]
async fn index() -> impl IntoResponse {
    let content = html! {
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
