use dioxus::prelude::*;
use dioxus_primitives::ContentSide;
use rust_starter::components::hover_card::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        HoverCard {
            HoverCardTrigger {
                span { class: "font-medium underline underline-offset-4", "@dioxuslabs" }
            }
            HoverCardContent { side: ContentSide::Bottom,
                div { class: "flex w-64 flex-col gap-1 p-4 text-sm",
                    span { class: "font-semibold", "Dioxus" }
                    span { class: "text-muted", "Fullstack apps in Rust for the web, desktop and mobile." }
                }
            }
        }
    }
}
