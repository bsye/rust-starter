use dioxus::prelude::*;
use dioxus_icons::lucide::{Minus, Plus};

use super::parts::data::DOCS;
use super::parts::{DocsBrand, InsetHeader, Placeholder, SearchForm};
use rust_starter::components::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use rust_starter::components::sidebar::{
    Sidebar, SidebarContent, SidebarGroup, SidebarHeader, SidebarInset, SidebarMenu,
    SidebarMenuButton, SidebarMenuItem, SidebarMenuSub, SidebarMenuSubButton, SidebarMenuSubItem,
    SidebarProvider, SidebarRail,
};

#[css_module("/src/components/sidebar/style.css")]
struct Styles;

#[component]
pub fn Sidebar05() -> Element {
    rsx! {
        SidebarProvider {
            Sidebar {
                SidebarHeader {
                    DocsBrand {}
                    SearchForm {}
                }
                SidebarContent {
                    SidebarGroup {
                        SidebarMenu {
                            for (index , section) in DOCS.iter().enumerate() {
                                Collapsible {
                                    key: "{section.title}",
                                    default_open: index == 1,
                                    r#as: move |attributes: Vec<Attribute>| rsx! {
                                        SidebarMenuItem { attributes,
                                            CollapsibleTrigger {
                                                class: Styles::dx_sidebar_collapsible_trigger,
                                                r#as: move |attributes: Vec<Attribute>| rsx! {
                                                    SidebarMenuButton { class: "group/disclosure", attributes,
                                                        "{section.title}"
                                                        Plus { class: "ml-auto size-4 group-data-[open=true]/disclosure:hidden" }
                                                        Minus { class: "ml-auto hidden size-4 group-data-[open=true]/disclosure:block" }
                                                    }
                                                },
                                            }
                                            CollapsibleContent {
                                                SidebarMenuSub {
                                                    for link in section.links {
                                                        SidebarMenuSubItem { key: "{link.title}",
                                                            SidebarMenuSubButton {
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
                                    },
                                }
                            }
                        }
                    }
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
