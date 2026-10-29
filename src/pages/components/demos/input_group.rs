use dioxus::prelude::*;
use dioxus_icons::lucide::{ArrowUp, Paperclip, Search};

use rust_starter::components::button::{ButtonSize, ButtonVariant};
use rust_starter::components::input_group::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; width: 100%; max-width: 24rem; flex-direction: column; gap: 1rem;",
            InputGroup {
                InputGroupInput { placeholder: "Search...", aria_label: "Search" }
                InputGroupAddon { Search {} }
                InputGroupAddon { align: InputGroupAlign::InlineEnd, InputGroupText { "12 results" } }
            }
            InputGroup {
                InputGroupTextarea { placeholder: "Ask, search or chat...", aria_label: "Message" }
                InputGroupAddon { align: InputGroupAlign::BlockEnd,
                    InputGroupButton { size: ButtonSize::IconXs, aria_label: "Attach a file", Paperclip {} }
                    InputGroupText { "Enter to send" }
                    InputGroupButton {
                        variant: ButtonVariant::Primary,
                        size: ButtonSize::IconXs,
                        style: "margin-left: auto;",
                        aria_label: "Send",
                        ArrowUp {}
                    }
                }
            }
        }
    }
}
