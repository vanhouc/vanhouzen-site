use axum::{response::IntoResponse, routing::get};
use maud::html;

use crate::layout;

pub fn router() -> axum::Router {
    axum::Router::new().route("/", get(cameron))
}

async fn cameron() -> impl IntoResponse {
    let content = html!(
        main {
            h1 { "Cameron VanHouzen" }
            img src="/assets/images/party_cam.jpeg" alt="Cameron VanHouzen" {}
        }
    );
    layout("Cameron", content)
}
