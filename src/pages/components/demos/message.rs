use dioxus::prelude::*;

use rust_starter::components::bubble::{
    Bubble, BubbleContent, BubbleGroup, BubbleReactions, BubbleVariant,
};
use rust_starter::components::marker::{Marker, MarkerContent};
use rust_starter::components::message::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; width: 100%; max-width: 24rem; flex-direction: column; gap: 1.5rem;",
            Message { align: MessageAlign::End,
                MessageAvatar { "ME" }
                MessageContent {
                    Bubble { BubbleContent { "Deploying to prod real quick." } }
                }
            }
            Message {
                MessageAvatar { "R" }
                MessageContent {
                    Bubble { variant: BubbleVariant::Muted, BubbleContent { "It's 4:55 PM. On a Friday." } }
                }
            }
            Message { align: MessageAlign::End,
                MessageAvatar { "ME" }
                MessageContent {
                    Bubble { BubbleContent { "It's a one-line change." } }
                    MessageFooter { "Delivered" }
                }
            }
            Message {
                MessageAvatar { "R" }
                MessageContent {
                    BubbleGroup {
                        Bubble { variant: BubbleVariant::Muted, BubbleContent { "It's always a one-line change." } }
                        Bubble { variant: BubbleVariant::Muted,
                            BubbleContent { "Alright, let me take a look." }
                            BubbleReactions { aria_label: "Reactions: thumbs up", span { "👍" } }
                        }
                    }
                }
            }
            Marker { role: "status",
                MarkerContent { shimmer: true, "Typing..." }
            }
        }
    }
}
