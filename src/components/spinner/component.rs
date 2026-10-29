use dioxus::prelude::*;
use dioxus_icons::lucide::LoaderCircle;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[css_module("/src/components/spinner/style.css")]
struct Styles;

#[component]
pub fn Spinner(#[props(extends = SvgAttributes)] attributes: Vec<Attribute>) -> Element {
    let base = attributes!(svg {
        class: Styles::dx_spinner,
        role: "status",
        aria_label: "Loading",
        "data-slot": "spinner",
    });
    rsx! {
        LoaderCircle { attributes: merge_attributes(vec![base, attributes]) }
    }
}
