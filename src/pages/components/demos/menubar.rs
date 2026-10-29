use dioxus::prelude::*;
use rust_starter::components::menubar::*;

#[component]
pub fn Demo() -> Element {
    let mut selected = use_signal(|| None::<String>);
    rsx! {
        div {
            Menubar {
                MenubarMenu { index: 0usize,
                    MenubarTrigger { "File" }
                    MenubarContent {
                        MenubarItem {
                            index: 0usize,
                            value: "new".to_string(),
                            on_select: move |value| {
                                selected.set(Some(value));
                            },
                            "New"
                        }
                        MenubarItem {
                            index: 1usize,
                            value: "open".to_string(),
                            disabled: true,
                            on_select: move |value| {
                                selected.set(Some(value));
                            },
                            "Open"
                        }
                        MenubarItem {
                            index: 2usize,
                            value: "save".to_string(),
                            on_select: move |value| {
                                selected.set(Some(value));
                            },
                            "Save"
                        }
                    }
                }
                MenubarMenu { index: 1usize,
                    MenubarTrigger { "Edit" }
                    MenubarContent {
                        MenubarItem {
                            index: 0usize,
                            value: "cut".to_string(),
                            on_select: move |value| {
                                selected.set(Some(value));
                            },
                            "Cut"
                        }
                        MenubarItem {
                            index: 1usize,
                            value: "copy".to_string(),
                            on_select: move |value| {
                                selected.set(Some(value));
                            },
                            "Copy"
                        }
                        MenubarItem {
                            index: 2usize,
                            value: "paste".to_string(),
                            on_select: move |value| {
                                selected.set(Some(value));
                            },
                            "Paste"
                        }
                    }
                }
            }
        }
        if let Some(value) = selected() {
            p { class: "mt-2 text-sm text-muted", "Selected: {value}" }
        }
    }
}
