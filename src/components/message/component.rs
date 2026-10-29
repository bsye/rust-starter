use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[css_module("/src/components/message/style.css")]
struct Styles;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MessageAlign {
    #[default]
    Start,
    End,
}

impl MessageAlign {
    pub fn as_str(self) -> &'static str {
        match self {
            MessageAlign::Start => "start",
            MessageAlign::End => "end",
        }
    }
}

fn part(
    class: &'static str,
    slot: &'static str,
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div { class: class, "data-slot": slot });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        div { ..merged, {children} }
    }
}

#[component]
pub fn MessageGroup(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    part(
        &Styles::dx_message_group,
        "message-group",
        attributes,
        children,
    )
}

#[component]
pub fn Message(
    #[props(default)] align: MessageAlign,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div { "data-align": align.as_str() });
    part(
        &Styles::dx_message,
        "message",
        merge_attributes(vec![base, attributes]),
        children,
    )
}

#[component]
pub fn MessageAvatar(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    part(
        &Styles::dx_message_avatar,
        "message-avatar",
        attributes,
        children,
    )
}

#[component]
pub fn MessageContent(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    part(
        &Styles::dx_message_content,
        "message-content",
        attributes,
        children,
    )
}

#[component]
pub fn MessageHeader(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    part(
        &Styles::dx_message_header,
        "message-header",
        attributes,
        children,
    )
}

#[component]
pub fn MessageFooter(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    part(
        &Styles::dx_message_footer,
        "message-footer",
        attributes,
        children,
    )
}
