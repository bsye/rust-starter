use dioxus::prelude::*;
use dioxus_icons::lucide::{ChevronRight, Ellipsis, Folder, Forward, Trash2};

use super::data::{APP_SECTIONS, PROJECTS, SECONDARY};
use rust_starter::components::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use rust_starter::components::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use rust_starter::components::separator::Separator;
use rust_starter::components::sidebar::{
    SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarMenu, SidebarMenuAction,
    SidebarMenuButton, SidebarMenuButtonSize, SidebarMenuItem, SidebarMenuSub,
    SidebarMenuSubButton, SidebarMenuSubItem,
};

#[css_module("/src/components/sidebar/style.css")]
struct Styles;

const DISCLOSURE: &str = "group/disclosure";
const CHEVRON: &str =
    "ml-auto size-4 transition-transform duration-200 group-data-[open=true]/disclosure:rotate-90";

#[component]
pub fn NavMain() -> Element {
    rsx! {
        SidebarGroup {
            SidebarGroupLabel { "Platform" }
            SidebarMenu {
                for section in APP_SECTIONS {
                    Collapsible {
                        key: "{section.title}",
                        default_open: section.open,
                        r#as: move |attributes: Vec<Attribute>| rsx! {
                            SidebarMenuItem { attributes,
                                CollapsibleTrigger {
                                    class: Styles::dx_sidebar_collapsible_trigger,
                                    r#as: move |attributes: Vec<Attribute>| rsx! {
                                        SidebarMenuButton {
                                            class: DISCLOSURE,
                                            tooltip: rsx! { "{section.title}" },
                                            attributes,
                                            {(section.icon)()}
                                            span { "{section.title}" }
                                            ChevronRight { class: CHEVRON }
                                        }
                                    },
                                }
                                CollapsibleContent {
                                    SidebarMenuSub {
                                        for link in section.links {
                                            SidebarMenuSubItem { key: "{link}",
                                                SidebarMenuSubButton {
                                                    r#as: move |attributes: Vec<Attribute>| rsx! {
                                                        a { href: "#", ..attributes, span { "{link}" } }
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
}

#[component]
pub fn NavProjects() -> Element {
    rsx! {
        SidebarGroup { class: "in-data-[collapsible=icon]:hidden",
            SidebarGroupLabel { "Projects" }
            SidebarMenu {
                for project in PROJECTS {
                    SidebarMenuItem { key: "{project.name}",
                        SidebarMenuButton {
                            r#as: move |attributes: Vec<Attribute>| rsx! {
                                a { href: "#", ..attributes,
                                    {(project.icon)()}
                                    span { "{project.name}" }
                                }
                            },
                        }
                        ProjectActions {}
                    }
                }
                SidebarMenuItem {
                    SidebarMenuButton { class: "text-muted",
                        Ellipsis {}
                        span { "More" }
                    }
                }
            }
        }
    }
}

#[component]
fn ProjectActions() -> Element {
    rsx! {
        DropdownMenu { class: Styles::dx_sidebar_dropdown_menu,
            DropdownMenuTrigger {
                class: Styles::dx_sidebar_dropdown_menu_trigger,
                r#as: move |attributes: Vec<Attribute>| rsx! {
                    SidebarMenuAction { show_on_hover: true, attributes,
                        Ellipsis {}
                        span { class: "sr-only", "More" }
                    }
                },
            }
            DropdownMenuContent { class: Styles::dx_sidebar_dropdown_menu_content,
                DropdownMenuItem::<usize> { index: 0usize, value: 0usize,
                    Folder { class: "text-muted" }
                    span { "View project" }
                }
                DropdownMenuItem::<usize> { index: 1usize, value: 1usize,
                    Forward { class: "text-muted" }
                    span { "Share project" }
                }
                Separator { class: Styles::dx_sidebar_dropdown_separator, decorative: true }
                DropdownMenuItem::<usize> { index: 2usize, value: 2usize,
                    Trash2 { class: "text-muted" }
                    span { "Delete project" }
                }
            }
        }
    }
}

#[component]
pub fn NavSecondary(#[props(default)] class: String) -> Element {
    rsx! {
        SidebarGroup { class,
            SidebarGroupContent {
                SidebarMenu {
                    for shortcut in SECONDARY {
                        SidebarMenuItem { key: "{shortcut.title}",
                            SidebarMenuButton {
                                size: SidebarMenuButtonSize::Sm,
                                r#as: move |attributes: Vec<Attribute>| rsx! {
                                    a { href: "#", ..attributes,
                                        {(shortcut.icon)()}
                                        span { "{shortcut.title}" }
                                    }
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}
