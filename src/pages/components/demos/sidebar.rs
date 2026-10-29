use dioxus::prelude::*;

use crate::app::Route;

const BLOCK: &str = "sidebar-07";

#[component]
pub fn Demo() -> Element {
    rsx! {
        iframe {
            class: "h-[28rem] w-full rounded-md border border-border",
            title: "Sidebar layout",
            "loading": "lazy",
            src: Route::BlockView { id: BLOCK.to_string() }.to_string(),
        }
    }
}
