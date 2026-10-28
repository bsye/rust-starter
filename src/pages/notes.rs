use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdStickyNote};

use crate::ui::{EmptyState, PageHeader};

#[component]
pub fn Notes() -> Element {
    rsx! {
        PageHeader { title: "Notes" }
        EmptyState { title: "No notes yet",
            icon: rsx! {
                Icon { icon: LdStickyNote }
            },
        }
    }
}
