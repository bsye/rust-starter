use dioxus::prelude::*;
use dioxus_icons::lucide::{House, Inbox, NotebookText, Search, Settings};
use rust_starter::components::separator::Separator;
use rust_starter::components::sidebar::*;
use rust_starter::components::skeleton::Skeleton;

use crate::app::Route;

#[component]
pub fn Demo() -> Element {
    rsx! {
        iframe {
            class: "h-[28rem] w-full rounded-md border border-border",
            title: "Sidebar layout",
            src: Route::SidebarDemo {}.to_string(),
        }
    }
}

#[component]
pub fn SidebarDemo() -> Element {
    rsx! {
        SidebarProvider {
            Sidebar { collapsible: SidebarCollapsible::Icon,
                SidebarHeader {
                    span { class: "truncate px-2 text-sm font-semibold", "Rust starter" }
                }
                SidebarContent {
                    SidebarGroup {
                        SidebarGroupLabel { "Workspace" }
                        SidebarGroupContent {
                            SidebarMenu {
                                SidebarMenuItem {
                                    SidebarMenuButton { is_active: true,
                                        House {}
                                        span { "Home" }
                                    }
                                }
                                SidebarMenuItem {
                                    SidebarMenuButton {
                                        Inbox {}
                                        span { "Inbox" }
                                    }
                                }
                                SidebarMenuItem {
                                    SidebarMenuButton {
                                        NotebookText {}
                                        span { "Notes" }
                                    }
                                }
                                SidebarMenuItem {
                                    SidebarMenuButton {
                                        Search {}
                                        span { "Search" }
                                    }
                                }
                            }
                        }
                    }
                }
                SidebarFooter {
                    SidebarMenu {
                        SidebarMenuItem {
                            SidebarMenuButton {
                                Settings {}
                                span { "Settings" }
                            }
                        }
                    }
                }
                SidebarRail {}
            }
            SidebarInset {
                header { class: "flex h-14 shrink-0 items-center gap-3 border-b border-border px-4",
                    SidebarTrigger {}
                    Separator { height: "1rem", horizontal: false }
                    span { class: "text-sm font-medium", "Home" }
                }
                div { class: "flex flex-1 flex-col gap-4 p-4",
                    div { class: "grid gap-4 md:grid-cols-3",
                        Skeleton { class: "h-24 w-full" }
                        Skeleton { class: "h-24 w-full" }
                        Skeleton { class: "h-24 w-full" }
                    }
                    Skeleton { class: "h-48 w-full" }
                }
            }
        }
    }
}
