use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[css_module("/src/components/marker/style.css")]
struct Styles;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MarkerVariant {
    #[default]
    Default,
    Separator,
    Border,
}

impl MarkerVariant {
    pub fn as_str(self) -> &'static str {
        match self {
            MarkerVariant::Default => "default",
            MarkerVariant::Separator => "separator",
            MarkerVariant::Border => "border",
        }
    }
}

#[component]
pub fn Marker(
    #[props(default)] variant: MarkerVariant,
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_marker,
        "data-slot": "marker",
        "data-variant": variant.as_str(),
    });
    let merged = merge_attributes(vec![base, attributes]);
    if let Some(render) = r#as {
        return render.call(merged);
    }
    rsx! {
        div { ..merged, {children} }
    }
}

#[component]
pub fn MarkerIcon(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(span {
        class: Styles::dx_marker_icon,
        "data-slot": "marker-icon",
        aria_hidden: "true",
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        span { ..merged, {children} }
    }
}

#[component]
pub fn MarkerContent(
    #[props(default)] shimmer: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(span {
        class: Styles::dx_marker_content,
        "data-slot": "marker-content",
        "data-shimmer": shimmer,
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        span { ..merged, {children} }
    }
}
