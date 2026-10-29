use dioxus::prelude::*;

use super::parts::{
    InsetHeader, NavMain, NavProjects, NavSecondary, NavUser, Placeholder, TeamBrand,
};
use rust_starter::components::sidebar::{
    Sidebar, SidebarContent, SidebarFooter, SidebarHeader, SidebarInset, SidebarProvider,
    SidebarVariant,
};

#[component]
pub fn Sidebar08() -> Element {
    rsx! {
        SidebarProvider {
            Sidebar { variant: SidebarVariant::Inset,
                SidebarHeader { TeamBrand {} }
                SidebarContent {
                    NavMain {}
                    NavProjects {}
                    NavSecondary { class: "mt-auto" }
                }
                SidebarFooter { NavUser {} }
            }
            SidebarInset {
                InsetHeader { class: "flex h-16 shrink-0 items-center gap-2 px-4" }
                Placeholder { class: "flex flex-1 flex-col gap-4 p-4 pt-0" }
            }
        }
    }
}
