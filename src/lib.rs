mod app;
pub mod components;
mod pages;
#[cfg(feature = "server")]
mod server;
mod ui;

pub fn run() {
    #[cfg(feature = "server")]
    dioxus::serve(server::router);

    #[cfg(not(feature = "server"))]
    dioxus::launch(app::App);
}
