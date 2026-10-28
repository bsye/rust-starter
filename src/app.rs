use dioxus::prelude::*;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Stylesheet { href: TAILWIND_CSS }
        main { class: "mx-auto max-w-3xl p-6",
            h1 { class: "text-3xl font-semibold", "Rust starter" }
        }
    }
}
