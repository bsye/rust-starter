use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

use super::component::{InputGroupControl, Styles};

#[component]
pub fn InputGroupInput(
    oninput: Option<EventHandler<FormEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = input)]
    attributes: Vec<Attribute>,
) -> Element {
    let mut control = use_context::<InputGroupControl>();
    let base = attributes!(input { class: Styles::dx_input_group_control, "data-slot": "input-group-control" });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        input {
            onmounted: move |event| control.0.set(Some(event.data())),
            oninput: move |event| {
                if let Some(handler) = &oninput {
                    handler.call(event);
                }
            },
            onkeydown: move |event| {
                if let Some(handler) = &onkeydown {
                    handler.call(event);
                }
            },
            ..merged,
        }
    }
}

#[component]
pub fn InputGroupTextarea(
    oninput: Option<EventHandler<FormEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = textarea)]
    attributes: Vec<Attribute>,
) -> Element {
    let mut control = use_context::<InputGroupControl>();
    let base = attributes!(textarea { class: Styles::dx_input_group_control, "data-slot": "input-group-control" });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        textarea {
            onmounted: move |event| control.0.set(Some(event.data())),
            oninput: move |event| {
                if let Some(handler) = &oninput {
                    handler.call(event);
                }
            },
            onkeydown: move |event| {
                if let Some(handler) = &onkeydown {
                    handler.call(event);
                }
            },
            ..merged,
        }
    }
}
