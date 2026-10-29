mod analytics;
mod api;
mod app;
mod blocks;
mod hooks;
mod pages;
#[cfg(feature = "server")]
mod server;
mod ui;

#[cfg(feature = "server")]
fn main() -> anyhow::Result<()> {
    server::run(std::env::args().skip(1))
}

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(app::App);
}
