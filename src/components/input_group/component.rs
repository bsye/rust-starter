use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

use crate::components::button::{Button, ButtonSize, ButtonVariant};

#[css_module("/src/components/input_group/style.css")]
pub(super) struct Styles;

#[derive(Clone, Copy)]
pub(super) struct InputGroupControl(pub Signal<Option<Rc<MountedData>>>);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputGroupAlign {
    #[default]
    InlineStart,
    InlineEnd,
    BlockStart,
    BlockEnd,
}

impl InputGroupAlign {
    pub fn as_str(self) -> &'static str {
        match self {
            InputGroupAlign::InlineStart => "inline-start",
            InputGroupAlign::InlineEnd => "inline-end",
            InputGroupAlign::BlockStart => "block-start",
            InputGroupAlign::BlockEnd => "block-end",
        }
    }
}

#[component]
pub fn InputGroup(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    use_context_provider(|| InputGroupControl(Signal::new(None)));
    let base = attributes!(div { class: Styles::dx_input_group, "data-slot": "input-group", role: "group" });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        div { ..merged, {children} }
    }
}

#[component]
pub fn InputGroupAddon(
    #[props(default)] align: InputGroupAlign,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let control = use_context::<InputGroupControl>();
    let base = attributes!(div {
        class: Styles::dx_input_group_addon,
        "data-slot": "input-group-addon",
        "data-align": align.as_str(),
        role: "group",
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        div {
            onclick: move |_| {
                if let Some(mounted) = control.0.peek().clone() {
                    spawn(async move {
                        let _ = mounted.set_focus(true).await;
                    });
                }
            },
            ..merged,
            {children}
        }
    }
}

#[component]
pub fn InputGroupButton(
    #[props(default = ButtonVariant::Ghost)] variant: ButtonVariant,
    #[props(default = ButtonSize::Xs)] size: ButtonSize,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = button)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(button {
        class: Styles::dx_input_group_button,
        r#type: "button"
    });
    rsx! {
        Button {
            variant,
            size,
            onclick: move |event: MouseEvent| {
                event.stop_propagation();
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
pub fn InputGroupText(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(span {
        class: Styles::dx_input_group_text
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        span { ..merged, {children} }
    }
}
