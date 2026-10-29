use dioxus::prelude::*;

use rust_starter::components::bubble::{Bubble, BubbleContent, BubbleVariant};
use rust_starter::components::direction::*;
use rust_starter::components::message::{Message, MessageAlign, MessageAvatar, MessageContent};

#[component]
pub fn Demo() -> Element {
    rsx! {
        DirectionProvider { direction: Direction::Rtl,
            Thread {}
        }
    }
}

#[component]
fn Thread() -> Element {
    let direction = use_direction();
    rsx! {
        div {
            dir: direction.as_str(),
            style: "display: flex; width: 100%; max-width: 24rem; flex-direction: column; gap: 1rem;",
            Message { align: MessageAlign::End,
                MessageAvatar { "أ" }
                MessageContent { Bubble { BubbleContent { "مرحبا! كيف حالك؟" } } }
            }
            Message {
                MessageAvatar { "ب" }
                MessageContent {
                    Bubble { variant: BubbleVariant::Muted, BubbleContent { "بخير، شكرا. الفقاعات تتبع اتجاه النص." } }
                }
            }
        }
    }
}
