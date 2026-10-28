use dioxus::prelude::*;

use super::form::NoteFormView;
use crate::ui::PageHeader;

#[component]
pub fn NewNote() -> Element {
    rsx! {
        PageHeader { title: "New note" }
        NoteFormView { existing: None }
    }
}
