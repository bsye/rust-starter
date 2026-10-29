use dioxus::prelude::*;
use rust_starter::components::toggle::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Toggle { width: "2rem", height: "2rem",
            em { "B" }
        }
    }
}
