use dioxus::prelude::*;
use rust_starter::components::separator::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "flex flex-col gap-1",
            span { class: "text-sm font-medium", "Dioxus components" }
            span { class: "text-sm text-muted", "Accessible building blocks for Dioxus apps." }
        }
        div { class: "my-4",
            Separator { horizontal: true, decorative: true }
        }
        div { class: "flex h-5 items-center gap-4 text-sm",
            span { "Blog" }
            Separator { height: "1rem", horizontal: false, decorative: true }
            span { "Docs" }
            Separator { height: "1rem", horizontal: false, decorative: true }
            span { "Source" }
        }
    }
}
