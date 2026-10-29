use dioxus::prelude::*;

use super::parts::{DocsBrand, DocsTree, InsetHeader, Placeholder};
use rust_starter::components::sidebar::{
    Sidebar, SidebarContent, SidebarGroup, SidebarHeader, SidebarInset, SidebarProvider,
    SidebarRail,
};

#[component]
pub fn Sidebar03() -> Element {
    rsx! {
        SidebarProvider {
            Sidebar {
                SidebarHeader { DocsBrand {} }
                SidebarContent {
                    SidebarGroup { DocsTree {} }
                }
                SidebarRail {}
            }
            SidebarInset {
                InsetHeader {}
                Placeholder {}
            }
        }
    }
}
