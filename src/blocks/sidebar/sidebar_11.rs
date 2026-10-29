use dioxus::prelude::*;
use dioxus_icons::lucide::{ChevronRight, File, Folder};

use super::parts::{InsetHeader, Placeholder};
use rust_starter::components::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use rust_starter::components::sidebar::{
    Sidebar, SidebarContent, SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarInset,
    SidebarMenu, SidebarMenuBadge, SidebarMenuButton, SidebarMenuItem, SidebarMenuSub,
    SidebarProvider, SidebarRail,
};

#[css_module("/src/components/sidebar/style.css")]
struct Styles;

#[derive(PartialEq)]
enum Node {
    File(&'static str),
    Folder(&'static str, bool, &'static [Node]),
}

const ACTIVE_FILE: &str = "component.rs";

const CHANGES: [(&str, &str); 3] = [
    ("README.md", "M"),
    ("src/blocks/mod.rs", "U"),
    ("src/app.rs", "M"),
];

const TREE: &[Node] = &[
    Node::Folder(
        "src",
        true,
        &[
            Node::Folder(
                "pages",
                false,
                &[
                    Node::File("home.rs"),
                    Node::File("auth.rs"),
                    Node::Folder("notes", false, &[Node::File("mod.rs")]),
                ],
            ),
            Node::Folder(
                "components",
                true,
                &[
                    Node::Folder(
                        "button",
                        true,
                        &[Node::File(ACTIVE_FILE), Node::File("style.css")],
                    ),
                    Node::File("mod.rs"),
                ],
            ),
            Node::Folder("hooks", false, &[Node::File("current_user.rs")]),
            Node::File("app.rs"),
            Node::File("main.rs"),
        ],
    ),
    Node::Folder(
        "assets",
        false,
        &[Node::File("favicon.ico"), Node::File("tailwind.css")],
    ),
    Node::File("Cargo.toml"),
    Node::File("Cargo.lock"),
    Node::File("Dioxus.toml"),
    Node::File("Dockerfile"),
    Node::File("compose.yaml"),
    Node::File("README.md"),
];

#[component]
pub fn Sidebar11() -> Element {
    rsx! {
        SidebarProvider {
            Sidebar {
                SidebarContent {
                    SidebarGroup {
                        SidebarGroupLabel { "Changes" }
                        SidebarGroupContent {
                            SidebarMenu {
                                for (file , state) in CHANGES {
                                    SidebarMenuItem { key: "{file}",
                                        SidebarMenuButton {
                                            File {}
                                            span { class: "truncate", "{file}" }
                                        }
                                        SidebarMenuBadge { "{state}" }
                                    }
                                }
                            }
                        }
                    }
                    SidebarGroup {
                        SidebarGroupLabel { "Files" }
                        SidebarGroupContent {
                            SidebarMenu {
                                for node in TREE {
                                    Tree { node }
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

#[component]
fn Tree(node: &'static Node) -> Element {
    match node {
        Node::File(name) => rsx! {
            SidebarMenuItem {
                SidebarMenuButton { is_active: *name == ACTIVE_FILE,
                    File {}
                    span { class: "truncate", title: "{name}", "{name}" }
                }
            }
        },
        Node::Folder(name, open, children) => rsx! {
            Collapsible {
                default_open: *open,
                r#as: move |attributes: Vec<Attribute>| rsx! {
                    SidebarMenuItem { attributes,
                        CollapsibleTrigger {
                            class: Styles::dx_sidebar_collapsible_trigger,
                            r#as: move |attributes: Vec<Attribute>| rsx! {
                                SidebarMenuButton { class: "group/disclosure", attributes,
                                    span { class: "flex min-w-0 flex-1 items-center gap-2 [&_svg]:size-4 [&_svg]:shrink-0",
                                        ChevronRight { class: "transition-transform group-data-[open=true]/disclosure:rotate-90" }
                                        Folder {}
                                        span { class: "truncate", title: "{name}", "{name}" }
                                    }
                                }
                            },
                        }
                        CollapsibleContent {
                            SidebarMenuSub {
                                for child in children.iter() {
                                    Tree { node: child }
                                }
                            }
                        }
                    }
                },
            }
        },
    }
}
