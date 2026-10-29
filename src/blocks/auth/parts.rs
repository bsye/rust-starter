use dioxus::prelude::*;
use dioxus_icons::lucide::GalleryVerticalEnd;

use crate::app::{APP_NAME, Route};

#[component]
pub fn Brand(#[props(default)] class: String) -> Element {
    rsx! {
        Link { class: "flex items-center gap-2 font-medium {class}", to: Route::Home {},
            div { class: "flex size-6 items-center justify-center rounded-md bg-primary text-primary-foreground [&_svg]:size-4",
                GalleryVerticalEnd {}
            }
            "{APP_NAME}"
        }
    }
}

#[component]
pub fn BrandIcon() -> Element {
    rsx! {
        Link {
            class: "flex size-8 items-center justify-center rounded-md [&_svg]:size-6",
            to: Route::Home {},
            aria_label: APP_NAME,
            GalleryVerticalEnd {}
        }
    }
}

#[component]
pub fn Cover(#[props(default)] class: String) -> Element {
    rsx! {
        div { class: "relative hidden items-center justify-center bg-foreground/5 text-muted [&_svg]:size-16 {class}",
            GalleryVerticalEnd {}
        }
    }
}
