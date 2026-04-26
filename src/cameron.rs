use axum::{response::IntoResponse, routing::get};
use hypertext::prelude::*;

pub fn router() -> axum::Router {
    axum::Router::new().route("/", get(cameron))
}

async fn cameron() -> impl IntoResponse {
    let content = hypertext::maud!(
        main {
            h1 { "Cameron VanHouzen" }
            img id="profile" src="/assets/images/party_cam.jpeg" alt="Cameron VanHouzen";
            p { "Ayy yo its me Cameron, I do computer stuff like this website." }
        }
    );
    layout("Cameron", content)
}

fn layout(title: &str, content: impl hypertext::Renderable) -> impl axum::response::IntoResponse {
    hypertext::maud!(
        !DOCTYPE
        html {
            head {
                title { (title) }
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                link rel="stylesheet" href="/assets/pico.min.css";
                link rel="stylesheet" href="/assets/style.css";
            }
            body {
                header {
                    nav {
                        ul {
                            li { a href="/" { "Home" } }
                            li { a href="/about" { "About" } }
                            li { a href="/projects" { "Projects" } }
                        }
                    }
                }
            }
            main {(content)}
            footer {
                p { "Copyright © 2025 Cameron VanHouzen" }
            }
        }
    )
}
