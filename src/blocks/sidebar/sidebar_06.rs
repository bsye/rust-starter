use dioxus::prelude::*;
use dioxus_icons::lucide::Ellipsis;

use super::parts::data::DOCS;
use super::parts::{DocsBrand, InsetHeader, Placeholder};
use rust_starter::components::button::{Button, ButtonSize};
use rust_starter::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use rust_starter::components::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use rust_starter::components::input::Input;
use rust_starter::components::sidebar::{
    Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarHeader, SidebarInset, SidebarMenu,
    SidebarMenuButton, SidebarMenuItem, SidebarProvider, SidebarRail,
};

#[css_module("/src/components/sidebar/style.css")]
struct Styles;

const NOT_SET_UP: &str = "Newsletter sign-up is not set up yet";

#[component]
pub fn Sidebar06() -> Element {
    rsx! {
        SidebarProvider {
            Sidebar {
                SidebarHeader { DocsBrand {} }
                SidebarContent {
                    SidebarGroup {
                        SidebarMenu {
                            for section in DOCS {
                                SidebarMenuItem { key: "{section.title}",
                                    DropdownMenu { class: Styles::dx_sidebar_dropdown_menu,
                                        DropdownMenuTrigger {
                                            class: Styles::dx_sidebar_dropdown_menu_trigger,
                                            r#as: move |attributes: Vec<Attribute>| rsx! {
                                                SidebarMenuButton { attributes,
                                                    "{section.title}"
                                                    Ellipsis { class: "ml-auto size-4" }
                                                }
                                            },
                                        }
                                        DropdownMenuContent { class: Styles::dx_sidebar_dropdown_menu_content,
                                            for (index , link) in section.links.iter().enumerate() {
                                                DropdownMenuItem::<usize> { key: "{link.title}", index, value: index,
                                                    "{link.title}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                SidebarFooter {
                    div { class: "p-1",
                        Card {
                            CardHeader {
                                CardTitle { "Subscribe to our newsletter" }
                                CardDescription { "Opt in to receive updates and news about the sidebar." }
                            }
                            CardContent {
                                form { class: "grid gap-2.5", onsubmit: move |event| event.prevent_default(),
                                    Input { r#type: "email", placeholder: "Email", aria_label: "Email" }
                                    Button { r#type: "submit", size: ButtonSize::Sm, disabled: true, title: NOT_SET_UP,
                                        "Subscribe"
                                    }
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
