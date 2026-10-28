use dioxus::prelude::*;

use super::{form::NoteFormView, skeleton::NoteDetailSkeleton};
use crate::api::{ApiError, get_note};
use crate::app::Route;
use crate::ui::{LoadError, Missing, PageHeader};

#[component]
pub fn EditNote(id: ReadSignal<i64>) -> Element {
    rsx! {
        SuspenseBoundary {
            fallback: |_| rsx! {
                NoteDetailSkeleton {}
            },
            EditLoader { id }
        }
    }
}

#[component]
fn EditLoader(id: ReadSignal<i64>) -> Element {
    let mut note = use_server_future(move || get_note(id()))?;
    match note() {
        Some(Ok(note)) => rsx! {
            div { key: "{note.id}",
                PageHeader { title: "Edit note" }
                NoteFormView { existing: Some(note) }
            }
        },
        Some(Err(ApiError::NotFound)) => rsx! {
            Missing {
                title: "Note not found",
                back_to: Route::Notes { page: 1 },
                back_label: "All notes",
            }
        },
        Some(Err(error)) => rsx! {
            LoadError { message: error.to_string(), onretry: move |_| note.restart() }
        },
        None => rsx! {
            NoteDetailSkeleton {}
        },
    }
}
