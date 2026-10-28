mod api;
mod app;
pub mod components;
mod hooks;
mod pages;
#[cfg(feature = "server")]
mod server;
mod ui;

#[cfg(feature = "server")]
pub use server::seed;

pub fn run() {
    #[cfg(feature = "server")]
    dioxus::serve(server::router);

    #[cfg(not(feature = "server"))]
    dioxus::launch(app::App);
}
