use rust_starter::components::input::Input;

use dioxus::prelude::*;
use rust_starter::components::label::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { display: "flex", flex_direction: "column", gap: ".5rem",
            Label { html_for: "name", "Name" }

            Input { id: "name", placeholder: "Enter your name" }
        }

    }
}
