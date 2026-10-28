use dioxus::prelude::*;
use dioxus_icons::lucide::{CircleAlert, RotateCw};

use super::EmptyState;
use crate::components::button::{Button, ButtonVariant};

#[component]
pub fn LoadError(message: String, onretry: EventHandler<MouseEvent>) -> Element {
    rsx! {
        EmptyState { title: message,
            icon: rsx! {
                CircleAlert {}
            },
            Button {
                variant: ButtonVariant::Outline,
                onclick: move |event| onretry.call(event),
                RotateCw {}
                "Try again"
            }
        }
    }
}
