use dioxus::prelude::*;

use super::parts::{InsetHeader, NavMain, NavProjects, NavUser, Placeholder, TeamSwitcher};
use rust_starter::components::sidebar::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarFooter, SidebarHeader, SidebarInset,
    SidebarProvider, SidebarRail,
};

#[component]
pub fn Sidebar07() -> Element {
    rsx! {
        SidebarProvider {
            Sidebar { collapsible: SidebarCollapsible::Icon,
                SidebarHeader { TeamSwitcher {} }
                SidebarContent {
                    NavMain {}
                    NavProjects {}
                }
                SidebarFooter { NavUser {} }
                SidebarRail {}
            }
            SidebarInset {
                InsetHeader { class: "flex h-16 shrink-0 items-center gap-2 px-4" }
                Placeholder { class: "flex flex-1 flex-col gap-4 p-4 pt-0" }
            }
        }
    }
}
