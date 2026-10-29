use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

use super::component::Styles;
use crate::components::button::{Button, ButtonSize, ButtonVariant};

fn part(
    tag_class: &'static str,
    slot: &'static str,
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div { class: tag_class, "data-slot": slot });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        div { ..merged, {children} }
    }
}

fn text(
    tag_class: &'static str,
    slot: &'static str,
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(span { class: tag_class, "data-slot": slot });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        span { ..merged, {children} }
    }
}

#[component]
pub fn AttachmentContent(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    part(
        &Styles::dx_attachment_content,
        "attachment-content",
        attributes,
        children,
    )
}

#[component]
pub fn AttachmentTitle(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    text(
        &Styles::dx_attachment_title,
        "attachment-title",
        attributes,
        children,
    )
}

#[component]
pub fn AttachmentDescription(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    text(
        &Styles::dx_attachment_description,
        "attachment-description",
        attributes,
        children,
    )
}

#[component]
pub fn AttachmentActions(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    part(
        &Styles::dx_attachment_actions,
        "attachment-actions",
        attributes,
        children,
    )
}

#[component]
pub fn AttachmentGroup(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    part(
        &Styles::dx_attachment_group,
        "attachment-group",
        attributes,
        children,
    )
}

#[component]
pub fn AttachmentAction(
    #[props(default = ButtonVariant::Ghost)] variant: ButtonVariant,
    #[props(default = ButtonSize::IconXs)] size: ButtonSize,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = button)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(button { "data-slot": "attachment-action", r#type: "button" });
    rsx! {
        Button {
            variant,
            size,
            onclick: move |event| {
                if let Some(handler) = &onclick {
                    handler.call(event);
                }
            },
            attributes: merge_attributes(vec![base, attributes]),
            {children}
        }
    }
}

#[component]
pub fn AttachmentTrigger(
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = button)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(button {
        class: Styles::dx_attachment_trigger,
        "data-slot": "attachment-trigger",
        r#type: "button",
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        button {
            onclick: move |event| {
                if let Some(handler) = &onclick {
                    handler.call(event);
                }
            },
            ..merged,
            {children}
        }
    }
}
