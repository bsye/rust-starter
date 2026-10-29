mod audit;
pub mod auth;
mod command;
mod config;
mod db;
mod entities;
mod migrations;
pub mod notes;
mod origin;
mod seed;

use command::Command;
use dioxus::server::axum::{Extension, Router, middleware, routing::get};

pub fn run(args: impl Iterator<Item = String>) -> anyhow::Result<()> {
    match Command::parse(args)? {
        Command::Serve => dioxus::serve(router),
        Command::Seed { email, count } => {
            tokio::runtime::Runtime::new()?.block_on(seed::seed(&email, count))
        }
    }
}

pub async fn router() -> anyhow::Result<Router> {
    let config = config::Config::from_env()?;
    let db = db::connect(&config.database_url).await?;
    crate::analytics::start(config.analytics.as_ref()).await?;
    let router = dioxus::server::router(crate::app::App)
        .route("/health", get(health))
        .layer(Extension(db.clone()))
        .layer(Extension(config.analytics.clone()));
    let router = auth::layers(router, &db, &config).await?;
    Ok(router.layer(middleware::from_fn_with_state(
        config.public_origin.clone(),
        origin::require_same_origin,
    )))
}

async fn health() -> &'static str {
    "ok"
}
