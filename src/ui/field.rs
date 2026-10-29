use dioxus::prelude::*;

use rust_starter::components::label::Label;

#[component]
pub fn Field(label: String, id: String, error: Option<String>, children: Element) -> Element {
    rsx! {
        div { class: "flex flex-col gap-2",
            Label { html_for: id.clone(), "{label}" }
            {children}
            if let Some(error) = error {
                p { id: "{id}-error", class: "text-sm text-destructive", "{error}" }
            }
        }
    }
}
