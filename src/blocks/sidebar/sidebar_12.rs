use dioxus::prelude::*;
use dioxus_icons::lucide::{Check, Plus};
use time::{Date, macros::date};

use super::parts::{NavUser, SectionTrigger};
use crate::ui::{Breadcrumb, BreadcrumbItem, BreadcrumbList, BreadcrumbPage};
use rust_starter::components::calendar::Calendar;
use rust_starter::components::collapsible::{Collapsible, CollapsibleContent};
use rust_starter::components::separator::Separator;
use rust_starter::components::sidebar::{
    Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent, SidebarHeader,
    SidebarInset, SidebarMenu, SidebarMenuButton, SidebarMenuItem, SidebarProvider, SidebarRail,
    SidebarSeparator, SidebarTrigger,
};

const MONTH: Date = date!(2026 - 10 - 01);

const CALENDARS: [(&str, &[&str]); 3] = [
    ("My calendars", &["Personal", "Work", "Family"]),
    ("Favorites", &["Holidays", "Birthdays"]),
    ("Other", &["Travel", "Reminders", "Deadlines"]),
];

#[component]
pub fn Sidebar12() -> Element {
    let mut selected = use_signal(|| None::<Date>);
    let mut month = use_signal(|| MONTH);
    rsx! {
        SidebarProvider {
            Sidebar {
                SidebarHeader { class: "h-16 border-b border-border", NavUser {} }
                SidebarContent {
                    SidebarGroup {
                        SidebarGroupContent {
                            Calendar {
                                selected_date: selected(),
                                on_date_change: move |date| selected.set(date),
                                view_date: month(),
                                on_view_change: move |date| month.set(date),
                            }
                        }
                    }
                    SidebarSeparator {}
                    for (index , (name , items)) in CALENDARS.into_iter().enumerate() {
                        SidebarGroup { key: "{name}",
                            Collapsible { default_open: index == 0,
                                SectionTrigger { title: name }
                                CollapsibleContent {
                                    SidebarGroupContent {
                                        SidebarMenu {
                                            for (position , item) in items.iter().enumerate() {
                                                SidebarMenuItem { key: "{item}",
                                                    SidebarMenuButton {
                                                        div {
                                                            "data-active": position < 2,
                                                            class: "group/calendar flex aspect-square size-4 shrink-0 items-center justify-center rounded-sm border border-border text-primary-foreground data-[active=true]:border-primary data-[active=true]:bg-primary",
                                                            Check { class: "hidden size-3 group-data-[active=true]/calendar:block" }
                                                        }
                                                        "{item}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        SidebarSeparator {}
                    }
                }
                SidebarFooter {
                    SidebarMenu {
                        SidebarMenuItem {
                            SidebarMenuButton {
                                Plus {}
                                span { "New calendar" }
                            }
                        }
                    }
                }
                SidebarRail {}
            }
            SidebarInset {
                header { class: "sticky top-0 flex h-16 shrink-0 items-center gap-2 border-b border-border bg-background px-4",
                    SidebarTrigger {}
                    Separator { class: "mr-2", height: "1rem", horizontal: false, decorative: true }
                    Breadcrumb {
                        BreadcrumbList {
                            BreadcrumbItem { BreadcrumbPage { "October 2026" } }
                        }
                    }
                }
                div { class: "grid auto-rows-min gap-4 p-4 md:grid-cols-5",
                    for index in 0..20 {
                        div { key: "{index}", class: "aspect-square rounded-xl bg-surface/50" }
                    }
                }
            }
        }
    }
}
