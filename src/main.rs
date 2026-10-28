mod app;
#[cfg(feature = "server")]
mod server;

fn main() {
    #[cfg(feature = "server")]
    dioxus::serve(server::router);

    #[cfg(not(feature = "server"))]
    dioxus::launch(app::App);
}
