use dioxus::prelude::*;
use rust_starter::components::label::Label;
use rust_starter::components::switch::*;

#[component]
pub fn Demo() -> Element {
    let mut checked = use_signal(|| false);
    rsx! {
        div { class: "flex items-center gap-2",
            Switch {
                id: "airplane-mode",
                checked: checked(),
                on_checked_change: move |value| checked.set(value),
            }
            Label { html_for: "airplane-mode", "Airplane mode" }
        }
    }
}
