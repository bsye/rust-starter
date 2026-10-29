use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[css_module("/src/components/attachment/style.css")]
pub(super) struct Styles;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AttachmentState {
    Idle,
    Uploading,
    Processing,
    Error,
    #[default]
    Done,
}

impl AttachmentState {
    pub fn as_str(self) -> &'static str {
        match self {
            AttachmentState::Idle => "idle",
            AttachmentState::Uploading => "uploading",
            AttachmentState::Processing => "processing",
            AttachmentState::Error => "error",
            AttachmentState::Done => "done",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AttachmentSize {
    #[default]
    Default,
    Sm,
    Xs,
}

impl AttachmentSize {
    pub fn as_str(self) -> &'static str {
        match self {
            AttachmentSize::Default => "default",
            AttachmentSize::Sm => "sm",
            AttachmentSize::Xs => "xs",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AttachmentOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl AttachmentOrientation {
    pub fn as_str(self) -> &'static str {
        match self {
            AttachmentOrientation::Horizontal => "horizontal",
            AttachmentOrientation::Vertical => "vertical",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AttachmentMediaVariant {
    #[default]
    Icon,
    Image,
}

impl AttachmentMediaVariant {
    pub fn as_str(self) -> &'static str {
        match self {
            AttachmentMediaVariant::Icon => "icon",
            AttachmentMediaVariant::Image => "image",
        }
    }
}

#[component]
pub fn Attachment(
    #[props(default)] state: AttachmentState,
    #[props(default)] size: AttachmentSize,
    #[props(default)] orientation: AttachmentOrientation,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_attachment,
        "data-slot": "attachment",
        "data-state": state.as_str(),
        "data-size": size.as_str(),
        "data-orientation": orientation.as_str(),
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        div { ..merged, {children} }
    }
}

#[component]
pub fn AttachmentMedia(
    #[props(default)] variant: AttachmentMediaVariant,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_attachment_media,
        "data-slot": "attachment-media",
        "data-variant": variant.as_str(),
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        div { ..merged, {children} }
    }
}
