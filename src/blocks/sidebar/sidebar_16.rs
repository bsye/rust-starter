use dioxus::prelude::*;
use dioxus_icons::lucide::PanelLeft;

use super::parts::{
    NavMain, NavProjects, NavSecondary, NavUser, Placeholder, SearchForm, TeamBrand, Trail,
};
use rust_starter::components::button::{Button, ButtonSize, ButtonVariant};
use rust_starter::components::separator::Separator;
use rust_starter::components::sidebar::{
    Sidebar, SidebarContent, SidebarFooter, SidebarHeader, SidebarInset, SidebarProvider,
    use_sidebar,
};

#[component]
pub fn Sidebar16() -> Element {
    rsx! {
        SidebarProvider { class: "flex flex-col",
            SiteHeader {}
            div { class: "flex flex-1",
                Sidebar {
                    div { class: "h-14 shrink-0" }
                    SidebarHeader { TeamBrand {} }
                    SidebarContent {
                        NavMain {}
                        NavProjects {}
                        NavSecondary { class: "mt-auto" }
                    }
                    SidebarFooter { NavUser {} }
                }
                SidebarInset { Placeholder {} }
            }
        }
    }
}

#[component]
fn SiteHeader() -> Element {
    let sidebar = use_sidebar();
    rsx! {
        header { class: "sticky top-0 z-50 flex w-full items-center border-b border-border bg-background",
            div { class: "flex h-14 w-full items-center gap-2 px-4",
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::IconSm,
                    aria_label: "Toggle sidebar",
                    onclick: move |_| sidebar.toggle(),
                    PanelLeft {}
                }
                Separator { class: "mr-2", height: "1rem", horizontal: false, decorative: true }
                div { class: "hidden sm:block", Trail {} }
                div { class: "w-full sm:ml-auto sm:w-auto", SearchForm {} }
            }
        }
    }
}
