use dioxus::prelude::*;
use dioxus_icons::lucide::{BadgeCheck, Bell, ChevronsUpDown, CreditCard, LogOut, Plus, Sparkles};

use super::data::{Shortcut, TEAMS, USER};
use rust_starter::components::avatar::{Avatar, AvatarFallback, AvatarImageSize, AvatarShape};
use rust_starter::components::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use rust_starter::components::separator::Separator;
use rust_starter::components::sidebar::{
    SidebarMenu, SidebarMenuButton, SidebarMenuButtonSize, SidebarMenuItem,
};

#[css_module("/src/components/sidebar/style.css")]
struct Styles;

const USER_ACTIONS: [Shortcut; 3] = [
    Shortcut {
        title: "Account",
        icon: || rsx! { BadgeCheck {} },
    },
    Shortcut {
        title: "Billing",
        icon: || rsx! { CreditCard {} },
    },
    Shortcut {
        title: "Notifications",
        icon: || rsx! { Bell {} },
    },
];

#[component]
pub fn TeamSwitcher() -> Element {
    let mut active = use_signal(|| 0usize);
    let team = &TEAMS[active()];
    rsx! {
        SidebarMenu {
            SidebarMenuItem {
                DropdownMenu { class: Styles::dx_sidebar_dropdown_menu,
                    DropdownMenuTrigger {
                        class: Styles::dx_sidebar_dropdown_menu_trigger,
                        r#as: move |attributes: Vec<Attribute>| rsx! {
                            SidebarMenuButton { size: SidebarMenuButtonSize::Lg, attributes,
                                div { class: "flex aspect-square size-8 items-center justify-center rounded-lg bg-primary text-primary-foreground [&_svg]:size-4",
                                    {(team.icon)()}
                                }
                                div { class: "grid flex-1 text-left text-sm leading-tight",
                                    span { class: "truncate font-medium", "{team.name}" }
                                    span { class: "truncate text-xs", "{team.plan}" }
                                }
                                ChevronsUpDown { class: "ml-auto size-4" }
                            }
                        },
                    }
                    DropdownMenuContent { class: Styles::dx_sidebar_dropdown_menu_content,
                        div { class: "px-2 py-1.5 text-xs text-muted", "Teams" }
                        for (index , team) in TEAMS.iter().enumerate() {
                            DropdownMenuItem::<usize> {
                                key: "{team.name}",
                                index,
                                value: index,
                                on_select: move |index| active.set(index),
                                div { class: "flex size-6 items-center justify-center rounded-md border border-border [&_svg]:size-3.5",
                                    {(team.icon)()}
                                }
                                "{team.name}"
                                span { class: "ml-auto text-xs tracking-widest text-muted", "⌘{index + 1}" }
                            }
                        }
                        Separator { class: Styles::dx_sidebar_dropdown_separator, decorative: true }
                        DropdownMenuItem::<usize> { index: TEAMS.len(), value: TEAMS.len(),
                            div { class: "flex size-6 items-center justify-center rounded-md border border-border [&_svg]:size-4",
                                Plus {}
                            }
                            span { class: "font-medium text-muted", "Add team" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn UserCard() -> Element {
    let user = &USER;
    rsx! {
        Avatar { size: AvatarImageSize::Small, shape: AvatarShape::Rounded, aria_label: user.name,
            AvatarFallback { "{user.initials}" }
        }
        div { class: "grid flex-1 text-left text-sm leading-tight",
            span { class: "truncate font-medium", "{user.name}" }
            span { class: "truncate text-xs", "{user.email}" }
        }
    }
}

#[component]
pub fn NavUser() -> Element {
    rsx! {
        SidebarMenu {
            SidebarMenuItem {
                DropdownMenu { class: Styles::dx_sidebar_dropdown_menu,
                    DropdownMenuTrigger {
                        class: Styles::dx_sidebar_dropdown_menu_trigger,
                        r#as: move |attributes: Vec<Attribute>| rsx! {
                            SidebarMenuButton { size: SidebarMenuButtonSize::Lg, attributes,
                                UserCard {}
                                ChevronsUpDown { class: "ml-auto size-4" }
                            }
                        },
                    }
                    DropdownMenuContent { class: Styles::dx_sidebar_dropdown_menu_content,
                        div { class: "flex items-center gap-2 px-1 py-1.5", UserCard {} }
                        Separator { class: Styles::dx_sidebar_dropdown_separator, decorative: true }
                        DropdownMenuItem::<usize> { index: 0usize, value: 0usize,
                            Sparkles {}
                            "Upgrade to Pro"
                        }
                        Separator { class: Styles::dx_sidebar_dropdown_separator, decorative: true }
                        for (offset , action) in USER_ACTIONS.iter().enumerate() {
                            DropdownMenuItem::<usize> { key: "{action.title}", index: offset + 1, value: offset + 1,
                                {(action.icon)()}
                                "{action.title}"
                            }
                        }
                        Separator { class: Styles::dx_sidebar_dropdown_separator, decorative: true }
                        DropdownMenuItem::<usize> { index: USER_ACTIONS.len() + 1, value: USER_ACTIONS.len() + 1,
                            LogOut {}
                            "Log out"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn TeamBrand() -> Element {
    let team = &TEAMS[0];
    rsx! {
        SidebarMenu {
            SidebarMenuItem {
                SidebarMenuButton {
                    size: SidebarMenuButtonSize::Lg,
                    r#as: move |attributes: Vec<Attribute>| rsx! {
                        a { href: "#", ..attributes,
                            div { class: "flex aspect-square size-8 items-center justify-center rounded-lg bg-primary text-primary-foreground [&_svg]:size-4",
                                {(team.icon)()}
                            }
                            div { class: "grid flex-1 text-left text-sm leading-tight",
                                span { class: "truncate font-medium", "{team.name}" }
                                span { class: "truncate text-xs", "{team.plan}" }
                            }
                        }
                    },
                }
            }
        }
    }
}
