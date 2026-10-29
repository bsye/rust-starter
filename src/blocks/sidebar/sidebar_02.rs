use dioxus::prelude::*;

use super::parts::data::DOCS;
use super::parts::{InsetHeader, SearchForm, SectionTrigger, VersionSwitcher};
use rust_starter::components::collapsible::{Collapsible, CollapsibleContent};
use rust_starter::components::sidebar::{
    Sidebar, SidebarContent, SidebarGroup, SidebarGroupContent, SidebarHeader, SidebarInset,
    SidebarMenu, SidebarMenuButton, SidebarMenuItem, SidebarProvider, SidebarRail,
};

#[component]
pub fn Sidebar02() -> Element {
    rsx! {
        SidebarProvider {
            Sidebar {
                SidebarHeader {
                    VersionSwitcher {}
                    SearchForm {}
                }
                SidebarContent {
                    for section in DOCS {
                        Collapsible { key: "{section.title}", default_open: true,
                            SidebarGroup {
                                SectionTrigger { title: section.title }
                                CollapsibleContent {
                                    SidebarGroupContent {
                                        SidebarMenu {
                                            for link in section.links {
                                                SidebarMenuItem { key: "{link.title}",
                                                    SidebarMenuButton {
                                                        is_active: link.active,
                                                        r#as: move |attributes: Vec<Attribute>| rsx! {
                                                            a { href: "#", ..attributes, "{link.title}" }
                                                        },
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                SidebarRail {}
            }
            SidebarInset {
                InsetHeader { class: "sticky top-0 flex h-16 shrink-0 items-center gap-2 border-b border-border bg-background px-4" }
                div { class: "flex flex-1 flex-col gap-4 p-4",
                    for index in 0..24 {
                        div { key: "{index}", class: "h-12 w-full rounded-lg bg-surface/50" }
                    }
                }
            }
        }
    }
}
