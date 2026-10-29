use dioxus::prelude::*;
use dioxus_icons::lucide::ChevronRight;
use dioxus_primitives::{dioxus_attributes::attributes, merge_attributes};

#[component]
pub fn Breadcrumb(children: Element) -> Element {
    rsx! {
        nav { aria_label: "breadcrumb", "data-slot": "breadcrumb", {children} }
    }
}

#[component]
pub fn BreadcrumbList(children: Element) -> Element {
    rsx! {
        ol {
            "data-slot": "breadcrumb-list",
            class: "flex flex-wrap items-center gap-1.5 text-sm break-words text-muted sm:gap-2.5",
            {children}
        }
    }
}

#[component]
pub fn BreadcrumbItem(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(li {
        class: "inline-flex items-center gap-1.5",
        "data-slot": "breadcrumb-item",
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        li { ..merged, {children} }
    }
}

#[component]
pub fn BreadcrumbLink(href: String, children: Element) -> Element {
    rsx! {
        a {
            "data-slot": "breadcrumb-link",
            class: "transition-colors hover:text-foreground",
            href,
            {children}
        }
    }
}

#[component]
pub fn BreadcrumbPage(children: Element) -> Element {
    rsx! {
        span {
            "data-slot": "breadcrumb-page",
            role: "link",
            aria_disabled: "true",
            aria_current: "page",
            class: "font-normal text-foreground",
            {children}
        }
    }
}

#[component]
pub fn BreadcrumbSeparator(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let base = attributes!(li {
        class: "[&>svg]:size-3.5",
        role: "presentation",
        aria_hidden: "true",
        "data-slot": "breadcrumb-separator",
    });
    let merged = merge_attributes(vec![base, attributes]);
    rsx! {
        li { ..merged, ChevronRight {} }
    }
}
