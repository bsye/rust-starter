use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
use dioxus_registry_primitives::message_scroller as primitive;

pub use primitive::{
    Align, Behavior, DefaultScrollPosition, MessageScrollerCommands, ScrollDirection,
    ScrollOptions, Scrollable, Visibility, use_message_scroller, use_message_scroller_scrollable,
    use_message_scroller_visibility,
};

#[css_module("/src/components/message_scroller/style.css")]
pub(super) struct Styles;

#[component]
pub fn MessageScrollerProvider(
    #[props(default)] auto_scroll: bool,
    #[props(default)] default_scroll_position: DefaultScrollPosition,
    #[props(default = 8.0)] scroll_edge_threshold: f64,
    #[props(default = 64.0)] scroll_previous_item_peek: f64,
    #[props(default)] scroll_margin: f64,
    children: Element,
) -> Element {
    rsx! {
        primitive::MessageScrollerProvider {
            auto_scroll,
            default_scroll_position,
            scroll_edge_threshold,
            scroll_previous_item_peek,
            scroll_margin,
            {children}
        }
    }
}

#[component]
pub fn MessageScroller(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base =
        attributes!(div { class: Styles::dx_message_scroller, "data-slot": "message-scroller" });
    rsx! {
        primitive::MessageScroller { attributes: merge_attributes(vec![base, attributes]), {children} }
    }
}

#[component]
pub fn MessageScrollerViewport(
    #[props(default = true)] preserve_scroll_on_prepend: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_message_scroller_viewport,
        "data-slot": "message-scroller-viewport",
    });
    rsx! {
        primitive::MessageScrollerViewport {
            preserve_scroll_on_prepend,
            attributes: merge_attributes(vec![base, attributes]),
            {children}
        }
    }
}

#[component]
pub fn MessageScrollerContent(
    spacer_class: Option<String>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_message_scroller_content,
        "data-slot": "message-scroller-content",
    });
    rsx! {
        primitive::MessageScrollerContent {
            spacer_class,
            attributes: merge_attributes(vec![base, attributes]),
            {children}
        }
    }
}
