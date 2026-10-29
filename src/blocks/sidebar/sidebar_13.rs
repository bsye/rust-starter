use dioxus::prelude::*;
use dioxus_icons::lucide::{
    Bell, Check, Globe, House, Keyboard, Link, Lock, Menu, MessageCircle, Paintbrush, Settings,
    Video,
};

use super::parts::data::Shortcut;
use crate::ui::{
    Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator,
};
use rust_starter::components::button::{Button, ButtonSize};
use rust_starter::components::dialog::{Dialog, DialogDescription, DialogTitle};
use rust_starter::components::sidebar::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarGroup, SidebarGroupContent, SidebarMenu,
    SidebarMenuButton, SidebarMenuItem, SidebarProvider,
};

const ACTIVE: &str = "Messages and media";

const SETTINGS: [Shortcut; 12] = [
    Shortcut {
        title: "Notifications",
        icon: || rsx! { Bell {} },
    },
    Shortcut {
        title: "Navigation",
        icon: || rsx! { Menu {} },
    },
    Shortcut {
        title: "Home",
        icon: || rsx! { House {} },
    },
    Shortcut {
        title: "Appearance",
        icon: || rsx! { Paintbrush {} },
    },
    Shortcut {
        title: ACTIVE,
        icon: || rsx! { MessageCircle {} },
    },
    Shortcut {
        title: "Language and region",
        icon: || rsx! { Globe {} },
    },
    Shortcut {
        title: "Accessibility",
        icon: || rsx! { Keyboard {} },
    },
    Shortcut {
        title: "Mark as read",
        icon: || rsx! { Check {} },
    },
    Shortcut {
        title: "Audio and video",
        icon: || rsx! { Video {} },
    },
    Shortcut {
        title: "Connected accounts",
        icon: || rsx! { Link {} },
    },
    Shortcut {
        title: "Privacy and visibility",
        icon: || rsx! { Lock {} },
    },
    Shortcut {
        title: "Advanced",
        icon: || rsx! { Settings {} },
    },
];

#[component]
pub fn Sidebar13() -> Element {
    let mut open = use_signal(|| true);
    rsx! {
        div { class: "flex h-svh items-center justify-center",
            Button { size: ButtonSize::Sm, onclick: move |_| open.set(true),
                Settings {}
                "Open settings"
            }
            Dialog { open: open(), on_open_change: move |value| open.set(value),
                DialogTitle { class: "sr-only", "Settings" }
                DialogDescription { class: "sr-only", "Customize your settings here." }
                div { class: "h-[480px] overflow-hidden text-left",
                SidebarProvider {
                    Sidebar { collapsible: SidebarCollapsible::None,
                        SidebarContent {
                            SidebarGroup {
                                SidebarGroupContent {
                                    SidebarMenu {
                                        for setting in SETTINGS.iter() {
                                            SidebarMenuItem { key: "{setting.title}",
                                                SidebarMenuButton {
                                                    is_active: setting.title == ACTIVE,
                                                    r#as: move |attributes: Vec<Attribute>| rsx! {
                                                        a { href: "#", ..attributes,
                                                            {(setting.icon)()}
                                                            span { "{setting.title}" }
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
                    main { class: "flex h-[480px] flex-1 flex-col overflow-hidden",
                        header { class: "flex h-16 shrink-0 items-center gap-2 px-4",
                            Breadcrumb {
                                BreadcrumbList {
                                    BreadcrumbItem { class: "hidden md:block",
                                        BreadcrumbLink { href: "#", "Settings" }
                                    }
                                    BreadcrumbSeparator { class: "hidden md:block" }
                                    BreadcrumbItem { BreadcrumbPage { "{ACTIVE}" } }
                                }
                            }
                        }
                        div { class: "flex flex-1 flex-col gap-4 overflow-y-auto p-4 pt-0",
                            for index in 0..10 {
                                div { key: "{index}", class: "aspect-video max-w-3xl rounded-xl bg-surface/50" }
                            }
                        }
                    }
                }
                }
            }
        }
    }
}
