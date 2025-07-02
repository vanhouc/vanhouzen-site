use std::{
    borrow::Cow,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use axum::{
    response::{IntoResponse, Redirect},
    routing::get,
};
use dotenvy::dotenv;
use fastrace::collector::Config;
use fastrace_axum::FastraceLayer;
use fastrace_opentelemetry::OpenTelemetryReporter;
use log::error;
use maud::{DOCTYPE, Markup, html};
use opentelemetry::{InstrumentationScope, KeyValue, trace::SpanKind};
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry_sdk::Resource;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

mod cameron;

// Entrypoint for axum application
#[tokio::main]
async fn main() {
    if let Err(error) = dotenv() {
        error!("failed to load env file, error: {error}")
    }

    // Setup logging out to the console
    logforth::stdout().apply();

    // Initialize OTLP reporter
    if let Ok(reporter) = initialize_otlp_reporter() {
        fastrace::set_reporter(reporter, Config::default());
    }

    let app = axum::Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .nest("/cameron", cameron::router())
        .route("/", get(index))
        .fallback(async || Redirect::to("/"))
        .layer(FastraceLayer);

    #[cfg(debug_assertions)]
    let app = app.layer(LiveReloadLayer::new());

    let port: u16 = std::env::var("PORT")
        .expect("PORT environment variable must be set")
        .parse()
        .expect("port must be a valid u16");

    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    fastrace::flush();
}

fn initialize_otlp_reporter() -> Result<OpenTelemetryReporter, anyhow::Error> {
    let otlp_exporter_endpoint = std::env::var("OTLP_EXPORTER_ENDPOINT")?;
    let reporter = OpenTelemetryReporter::new(
        SpanExporter::builder()
            .with_tonic()
            .with_endpoint(otlp_exporter_endpoint)
            .with_protocol(opentelemetry_otlp::Protocol::Grpc)
            .with_timeout(opentelemetry_otlp::OTEL_EXPORTER_OTLP_TIMEOUT_DEFAULT)
            .build()?,
        SpanKind::Server,
        Cow::Owned(
            Resource::builder()
                .with_attributes([KeyValue::new("service.name", "vanhouzen-site")])
                .build(),
        ),
        InstrumentationScope::builder("vanhouzen-site")
            .with_version(env!("CARGO_PKG_VERSION"))
            .build(),
    );
    Ok(reporter)
}

#[fastrace::trace]
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
