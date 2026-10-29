use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[css_module("/src/components/bubble/style.css")]
struct Styles;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BubbleVariant {
    #[default]
    Default,
    Secondary,
    Muted,
    Tinted,
    Outline,
    Ghost,
    Destructive,
}

impl BubbleVariant {
    pub fn as_str(self) -> &'static str {
        match self {
            BubbleVariant::Default => "default",
            BubbleVariant::Secondary => "secondary",
            BubbleVariant::Muted => "muted",
            BubbleVariant::Tinted => "tinted",
            BubbleVariant::Outline => "outline",
            BubbleVariant::Ghost => "ghost",
            BubbleVariant::Destructive => "destructive",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BubbleAlign {
    #[default]
    Start,
    End,
}

impl BubbleAlign {
    pub fn as_str(self) -> &'static str {
        match self {
            BubbleAlign::Start => "start",
            BubbleAlign::End => "end",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BubbleSide {
    Top,
    #[default]
    Bottom,
}

impl BubbleSide {
    pub fn as_str(self) -> &'static str {
        match self {
            BubbleSide::Top => "top",
            BubbleSide::Bottom => "bottom",
        }
    }
}

#[component]
pub fn BubbleGroup(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div { class: Styles::dx_bubble_group, "data-slot": "bubble-group" });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        div { ..merged, {children} }
    }
}

#[component]
pub fn Bubble(
    #[props(default)] variant: BubbleVariant,
    #[props(default)] align: BubbleAlign,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_bubble,
        "data-slot": "bubble",
        "data-variant": variant.as_str(),
        "data-align": align.as_str(),
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        div { ..merged, {children} }
    }
}

#[component]
pub fn BubbleContent(
    r#as: Option<Callback<Vec<Attribute>, Element>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div { class: Styles::dx_bubble_content, "data-slot": "bubble-content" });
    let merged = merge_attributes(vec![base, attributes]);
    if let Some(render) = r#as {
        return render.call(merged);
    }
    rsx! {
        div { ..merged, {children} }
    }
}

#[component]
pub fn BubbleReactions(
    #[props(default)] side: BubbleSide,
    #[props(default = BubbleAlign::End)] align: BubbleAlign,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_bubble_reactions,
        "data-slot": "bubble-reactions",
        "data-side": side.as_str(),
        "data-align": align.as_str(),
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        div { ..merged, {children} }
    }
}
