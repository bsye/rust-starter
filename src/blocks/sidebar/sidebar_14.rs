use dioxus::prelude::*;

use super::parts::{DocsTree, Placeholder, Trail};
use rust_starter::components::sidebar::{
    Sidebar, SidebarContent, SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarInset,
    SidebarProvider, SidebarRail, SidebarSide, SidebarTrigger,
};

#[component]
pub fn Sidebar14() -> Element {
    rsx! {
        SidebarProvider {
            Sidebar { side: SidebarSide::Right,
                SidebarContent {
                    SidebarGroup {
                        SidebarGroupLabel { "Table of contents" }
                        SidebarGroupContent { DocsTree {} }
                    }
                }
                SidebarRail {}
            }
            SidebarInset {
                header { class: "flex h-16 shrink-0 items-center gap-2 border-b border-border px-4",
                    Trail {}
                    div { class: "ml-auto rotate-180", SidebarTrigger {} }
                }
                Placeholder {}
            }
        }
    }
}
