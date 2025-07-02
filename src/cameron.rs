use axum::{response::IntoResponse, routing::get};
use maud::{DOCTYPE, Markup, html};

pub fn router() -> axum::Router {
    axum::Router::new().route("/", get(cameron))
}

#[fastrace::trace]
async fn cameron() -> impl IntoResponse {
    let content = html!(
        main {
            h1 { "Cameron VanHouzen" }
            img id="profile" src="/assets/images/party_cam.jpeg" alt="Cameron VanHouzen" {}
            p { "Ayy yo its me Cameron, I do computer stuff like this website." }
        }
    );
    layout("Cameron", content)
}

#[fastrace::trace]
fn layout(title: &str, content: maud::Markup) -> Markup {
    html!(
        (DOCTYPE)
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
