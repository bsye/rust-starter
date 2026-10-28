use dioxus::prelude::*;
use dioxus_icons::lucide::Trash2;

use crate::components::alert_dialog::{
    AlertDialog, AlertDialogAction, AlertDialogActions, AlertDialogCancel, AlertDialogDescription,
    AlertDialogTitle,
};
use crate::components::button::{Button, ButtonVariant};
use crate::hooks::use_delete_note;

#[component]
pub fn DeleteNoteButton(id: i64) -> Element {
    let mut delete = use_delete_note(id);
    rsx! {
        Button {
            variant: ButtonVariant::Destructive,
            title: delete.failure.cloned().unwrap_or_default(),
            onclick: move |_| delete.open.set(true),
            Trash2 {}
            "Delete"
        }
        AlertDialog {
            open: Some(delete.open.cloned()),
            on_open_change: move |open| delete.open.set(open),
            AlertDialogTitle { "Delete this note?" }
            AlertDialogDescription { "It will disappear from your notes." }
            AlertDialogActions {
                AlertDialogCancel { "Cancel" }
                AlertDialogAction {
                    on_click: move |_| delete.confirm(),
                    "Delete"
                }
            }
        }
    }
}
