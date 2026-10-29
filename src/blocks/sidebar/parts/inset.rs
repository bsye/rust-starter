use dioxus::prelude::*;

use super::data::{CURRENT_PAGE, CURRENT_SECTION};
use crate::ui::{
    Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator,
};
use rust_starter::components::separator::Separator;
use rust_starter::components::sidebar::SidebarTrigger;

#[component]
pub fn InsetHeader(
    #[props(default = "flex h-16 shrink-0 items-center gap-2 border-b border-border px-4".to_string())]
    class: String,
) -> Element {
    rsx! {
        header { class,
            SidebarTrigger {}
            Separator { class: "mr-2", height: "1rem", horizontal: false, decorative: true }
            Trail {}
        }
    }
}

#[component]
pub fn Trail() -> Element {
    rsx! {
        Breadcrumb {
            BreadcrumbList {
                BreadcrumbItem { class: "hidden md:block",
                    BreadcrumbLink { href: "#", "{CURRENT_SECTION}" }
                }
                BreadcrumbSeparator { class: "hidden md:block" }
                BreadcrumbItem {
                    BreadcrumbPage { "{CURRENT_PAGE}" }
                }
            }
        }
    }
}

#[component]
pub fn Placeholder(
    #[props(default = "flex flex-1 flex-col gap-4 p-4".to_string())] class: String,
) -> Element {
    rsx! {
        div { class,
            div { class: "grid auto-rows-min gap-4 md:grid-cols-3",
                for index in 0..3 {
                    div { key: "{index}", class: "aspect-video rounded-xl bg-surface/50" }
                }
            }
            div { class: "min-h-[100vh] flex-1 rounded-xl bg-surface/50 md:min-h-min" }
        }
    }
}
