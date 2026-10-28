mod config;
mod db;
mod entities;
mod migrations;
pub mod notes;
mod seed;

use dioxus::server::axum::{Extension, Router, routing::get};

pub use seed::seed;

pub async fn router() -> anyhow::Result<Router> {
    let config = config::Config::from_env()?;
    let db = db::connect(&config.database_url).await?;
    Ok(dioxus::server::router(crate::app::App)
        .route("/health", get(health))
        .layer(Extension(db)))
}

async fn health() -> &'static str {
    "ok"
}
