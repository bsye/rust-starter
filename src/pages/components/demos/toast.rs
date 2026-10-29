use rust_starter::components::button::Button;

use dioxus::prelude::*;
use dioxus_primitives::toast::{ToastOptions, use_toast};
use rust_starter::components::toast::*;
use std::time::Duration;

#[component]
pub fn Demo() -> Element {
    rsx! {
        ToastProvider { ToastButton {} }
    }
}

#[component]
fn ToastButton() -> Element {
    let toast_api = use_toast();

    rsx! {
        Button {
            r#type: "button",
            "data-style": "outline",
            onclick: move |_| {
                toast_api
                    .info(
                        "Note saved".to_string(),
                        ToastOptions::new()
                            .description("Your changes are stored.")
                            .duration(Duration::from_secs(5))
                            .permanent(false),
                    );
            },
            "Show toast"
        }
    }
}
