use dioxus::prelude::*;

use rust_starter::components::card::Card;

#[component]
pub fn EmptyState(title: String, icon: Element, children: Element) -> Element {
    rsx! {
        Card {
            div { class: "flex flex-col items-center gap-4 px-6 py-12 text-center",
                div { class: "text-muted [&_svg]:size-8", {icon} }
                p { class: "text-lg font-medium", "{title}" }
                {children}
            }
        }
    }
}
