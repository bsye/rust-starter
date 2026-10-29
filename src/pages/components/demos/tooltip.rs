use dioxus::prelude::*;
use dioxus_primitives::ContentSide;
use rust_starter::components::tooltip::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Tooltip {
            TooltipTrigger { "Hover me" }
            TooltipContent { side: ContentSide::Top, "Add to library" }
        }
    }
}
