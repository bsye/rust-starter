use dioxus::prelude::*;
use dioxus_icons::lucide::{Check, ChevronsUpDown, GalleryVerticalEnd};

use super::data::{DOCS, VERSIONS};
use rust_starter::components::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use rust_starter::components::input::Input;
use rust_starter::components::sidebar::{
    SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarMenu, SidebarMenuButton,
    SidebarMenuButtonSize, SidebarMenuItem, SidebarMenuSub, SidebarMenuSubButton,
    SidebarMenuSubItem,
};

#[css_module("/src/components/sidebar/style.css")]
struct Styles;

#[component]
fn DocsLogo(version: String) -> Element {
    rsx! {
        div { class: "flex aspect-square size-8 items-center justify-center rounded-lg bg-primary text-primary-foreground [&_svg]:size-4",
            GalleryVerticalEnd {}
        }
        div { class: "flex flex-col gap-0.5 leading-none",
            span { class: "font-medium", "Documentation" }
            span { "v{version}" }
        }
    }
}

#[component]
pub fn DocsBrand() -> Element {
    rsx! {
        SidebarMenu {
            SidebarMenuItem {
                SidebarMenuButton {
                    size: SidebarMenuButtonSize::Lg,
                    r#as: move |attributes: Vec<Attribute>| rsx! {
                        a { href: "#", ..attributes,
                            DocsLogo { version: "1.0.0" }
                        }
                    },
                }
            }
        }
    }
}

#[component]
pub fn VersionSwitcher() -> Element {
    let mut selected = use_signal(|| VERSIONS[0]);
    rsx! {
        SidebarMenu {
            SidebarMenuItem {
                DropdownMenu { class: Styles::dx_sidebar_dropdown_menu,
                    DropdownMenuTrigger {
                        class: Styles::dx_sidebar_dropdown_menu_trigger,
                        r#as: move |attributes: Vec<Attribute>| rsx! {
                            SidebarMenuButton { size: SidebarMenuButtonSize::Lg, attributes,
                                DocsLogo { version: selected() }
                                ChevronsUpDown { class: "ml-auto size-4" }
                            }
                        },
                    }
                    DropdownMenuContent { class: Styles::dx_sidebar_dropdown_menu_content,
                        for (index , version) in VERSIONS.into_iter().enumerate() {
                            DropdownMenuItem::<&'static str> {
                                key: "{version}",
                                index,
                                value: version,
                                on_select: move |version| selected.set(version),
                                "v{version}"
                                if version == selected() {
                                    Check { class: "ml-auto size-4" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SearchForm(
    #[props(default = "Search the docs...".to_string())] placeholder: String,
) -> Element {
    rsx! {
        form { onsubmit: move |event| event.prevent_default(),
            SidebarGroup { class: "py-0",
                SidebarGroupContent {
                    label { r#for: "search", class: "sr-only", "Search" }
                    Input { id: "search", r#type: "search", class: "w-full", placeholder }
                }
            }
        }
    }
}

#[component]
pub fn DocsGroups() -> Element {
    rsx! {
        for section in DOCS {
            SidebarGroup { key: "{section.title}",
                SidebarGroupLabel { "{section.title}" }
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

#[component]
pub fn DocsTree() -> Element {
    rsx! {
        SidebarMenu {
            for section in DOCS {
                SidebarMenuItem { key: "{section.title}",
                    SidebarMenuButton {
                        r#as: move |attributes: Vec<Attribute>| rsx! {
                            a { href: "#", ..attributes,
                                span { class: "font-medium", "{section.title}" }
                            }
                        },
                    }
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
        }
    }
}
