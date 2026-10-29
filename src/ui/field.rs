use dioxus::prelude::*;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

use rust_starter::components::{label::Label, separator::Separator};

#[component]
pub fn FieldGroup(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "flex w-full flex-col gap-7 [&>[data-slot=field-group]]:gap-4",
        "data-slot": "field-group",
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        div { ..merged, {children} }
    }
}

#[component]
pub fn Field(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: "flex w-full flex-col gap-3",
        role: "group",
        "data-slot": "field",
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        div { ..merged, {children} }
    }
}

#[component]
pub fn FieldLabel(html_for: String, children: Element) -> Element {
    rsx! {
        Label { html_for, {children} }
    }
}

#[component]
pub fn FieldDescription(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(p {
        class: "text-sm leading-normal text-muted [&_a]:underline [&_a]:underline-offset-4 [&_a:hover]:text-foreground",
        "data-slot": "field-description",
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        p { ..merged, {children} }
    }
}

#[component]
pub fn FieldError(id: String, message: Option<String>) -> Element {
    rsx! {
        if let Some(message) = message {
            p { id, role: "alert", class: "text-sm text-destructive", "{message}" }
        }
    }
}

#[component]
pub fn FieldSeparator(
    #[props(default = "bg-background".to_string())] behind: String,
    children: Element,
) -> Element {
    rsx! {
        div { "data-slot": "field-separator", class: "relative -my-2 h-5 text-sm",
            div { class: "absolute inset-x-0 top-1/2", Separator { decorative: true } }
            span { class: "relative mx-auto block w-fit px-2 text-muted {behind}", {children} }
        }
    }
}
