use dioxus::prelude::*;
use dioxus_icons::lucide::ArrowDown;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
use dioxus_registry_primitives::message_scroller as primitive;

use super::component::Styles;
use crate::components::button::{Button, ButtonSize, ButtonVariant};
use primitive::{Behavior, ScrollDirection, use_message_scroller, use_message_scroller_scrollable};

#[component]
pub fn MessageScrollerItem(
    message_id: Option<String>,
    #[props(default)] scroll_anchor: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div { class: Styles::dx_message_scroller_item, "data-slot": "message-scroller-item" });
    rsx! {
        primitive::MessageScrollerItem {
            message_id,
            scroll_anchor,
            attributes: merge_attributes(vec![base, attributes]),
            {children}
        }
    }
}

#[component]
pub fn MessageScrollerButton(
    #[props(default)] direction: ScrollDirection,
    #[props(default = Behavior::Smooth)] behavior: Behavior,
    #[props(default = ButtonVariant::Outline)] variant: ButtonVariant,
    #[props(default = ButtonSize::IconSm)] size: ButtonSize,
    #[props(extends = GlobalAttributes)]
    #[props(extends = button)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let commands = use_message_scroller();
    let active = use_message_scroller_scrollable()().toward(direction);
    let base = attributes!(button {
        class: Styles::dx_message_scroller_button,
        "data-slot": "message-scroller-button",
        "data-direction": direction.as_str(),
        "data-active": active,
        tabindex: if active { "0" } else { "-1" },
        aria_hidden: !active,
    });
    let label = match direction {
        ScrollDirection::Start => "Scroll to start",
        ScrollDirection::End => "Scroll to end",
    };
    rsx! {
        Button {
            variant,
            size,
            onclick: move |_| commands.scroll_toward(direction, behavior),
            attributes: merge_attributes(vec![base, attributes]),
            if children.is_ok() && children != VNode::empty() {
                {children}
            } else {
                ArrowDown {}
                span { class: Styles::dx_message_scroller_label, "{label}" }
            }
        }
    }
}
