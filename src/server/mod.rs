mod audit;
pub mod auth;
mod config;
mod db;
mod entities;
mod migrations;
pub mod notes;
mod origin;
mod seed;

use dioxus::server::axum::{Extension, Router, middleware, routing::get};

pub use seed::seed;

pub async fn router() -> anyhow::Result<Router> {
    let config = config::Config::from_env()?;
    let db = db::connect(&config.database_url).await?;
    let router = dioxus::server::router(crate::app::App)
        .route("/health", get(health))
        .layer(Extension(db.clone()));
    let router = auth::layers(router, &db, &config).await?;
    Ok(router.layer(middleware::from_fn_with_state(
        config.public_origin.clone(),
        origin::require_same_origin,
    )))
}

async fn health() -> &'static str {
    "ok"
}
