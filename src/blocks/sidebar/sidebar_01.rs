use dioxus::prelude::*;

use super::parts::{DocsGroups, InsetHeader, Placeholder, SearchForm, VersionSwitcher};
use rust_starter::components::sidebar::{
    Sidebar, SidebarContent, SidebarHeader, SidebarInset, SidebarProvider, SidebarRail,
};

#[component]
pub fn Sidebar01() -> Element {
    rsx! {
        SidebarProvider {
            Sidebar {
                SidebarHeader {
                    VersionSwitcher {}
                    SearchForm {}
                }
                SidebarContent { DocsGroups {} }
                SidebarRail {}
            }
            SidebarInset {
                InsetHeader {}
                Placeholder {}
            }
        }
    }
}
