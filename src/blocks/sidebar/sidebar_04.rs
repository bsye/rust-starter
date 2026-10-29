use dioxus::prelude::*;

use super::parts::{DocsBrand, DocsTree, InsetHeader, Placeholder};
use rust_starter::components::sidebar::{
    Sidebar, SidebarContent, SidebarGroup, SidebarHeader, SidebarInset, SidebarProvider,
    SidebarVariant,
};

#[component]
pub fn Sidebar04() -> Element {
    rsx! {
        SidebarProvider {
            Sidebar { variant: SidebarVariant::Floating,
                SidebarHeader { DocsBrand {} }
                SidebarContent {
                    SidebarGroup { DocsTree {} }
                }
            }
            SidebarInset {
                InsetHeader { class: "flex h-16 shrink-0 items-center gap-2 px-4" }
                Placeholder { class: "flex flex-1 flex-col gap-4 p-4 pt-0" }
            }
        }
    }
}
