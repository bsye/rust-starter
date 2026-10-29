use dioxus::prelude::*;
use rust_starter::components::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};

const ACTIONS: [&str; 4] = ["Edit", "Undo", "Duplicate", "Delete"];

#[css_module("/src/components/dropdown_menu/style.css")]
struct Styles;

#[component]
pub fn Demo() -> Element {
    let mut selected = use_signal(|| None::<&'static str>);
    rsx! {
        DropdownMenu { class: Styles::dx_dropdown_menu, default_open: false,
            DropdownMenuTrigger { class: Styles::dx_dropdown_menu_trigger, "Open menu" }
            DropdownMenuContent { class: Styles::dx_dropdown_menu_content,
                for (index , action) in ACTIONS.into_iter().enumerate() {
                    DropdownMenuItem::<&'static str> {
                        class: Styles::dx_dropdown_menu_item,
                        value: action,
                        index,
                        disabled: action == "Undo",
                        on_select: move |value| selected.set(Some(value)),
                        "{action}"
                    }
                }
            }
        }
        if let Some(action) = selected() {
            p { class: "mt-2 text-sm text-muted", "Selected: {action}" }
        }
    }
}
