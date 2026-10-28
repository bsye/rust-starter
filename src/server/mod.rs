use dioxus::server::axum::{Router, routing::get};

pub async fn router() -> anyhow::Result<Router> {
    Ok(dioxus::server::router(crate::app::App).route("/health", get(health)))
}

async fn health() -> &'static str {
    "ok"
}
