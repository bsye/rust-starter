use dioxus::prelude::*;

use rust_starter::components::bubble::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; width: 100%; max-width: 24rem; flex-direction: column; gap: 2rem; padding-bottom: 1rem;",
            Bubble { align: BubbleAlign::End, BubbleContent { "Hey there! What's up?" } }
            BubbleGroup {
                Bubble { variant: BubbleVariant::Muted, BubbleContent { "Hey! Want to see chat bubbles?" } }
                Bubble { variant: BubbleVariant::Muted,
                    BubbleContent { "I can group messages, switch sides, and keep the whole thread easy to scan." }
                    BubbleReactions { role: "img", aria_label: "Reaction: thumbs up", span { "👍" } }
                }
            }
            Bubble { align: BubbleAlign::End, BubbleContent { "Sure. Hit me with your best demo." } }
            Bubble { variant: BubbleVariant::Muted,
                BubbleContent { "You are reading a demo that is demoing itself. Very meta." }
                BubbleReactions { role: "img", aria_label: "Reactions: thumbs up, fire, eyes and 2 more",
                    span { "👍" }
                    span { "🔥" }
                    span { "👀" }
                    span { "+2" }
                }
            }
        }
    }
}
