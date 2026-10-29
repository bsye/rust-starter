use dioxus::prelude::*;
use rust_starter::components::avatar::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "flex flex-wrap items-center gap-4",
            Avatar { size: AvatarImageSize::Small, aria_label: "Ada Lovelace",
                AvatarFallback { "AL" }
            }
            Avatar { size: AvatarImageSize::Medium, aria_label: "Grace Hopper",
                AvatarFallback { "GH" }
            }
            Avatar { size: AvatarImageSize::Large, aria_label: "Alan Turing",
                AvatarFallback { "AT" }
            }
            Avatar {
                size: AvatarImageSize::Medium,
                shape: AvatarShape::Rounded,
                aria_label: "Katherine Johnson",
                AvatarFallback { "KJ" }
            }
        }
    }
}
