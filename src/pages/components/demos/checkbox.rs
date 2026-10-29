use dioxus::prelude::*;
use rust_starter::components::checkbox::*;
use rust_starter::components::label::Label;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "flex items-center gap-2",
            Checkbox { id: "terms", name: "terms" }
            Label { html_for: "terms", "Accept terms and conditions" }
        }
    }
}
