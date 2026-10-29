use dioxus::prelude::*;
use dioxus_icons::lucide::{ArrowLeft, Pencil};

use super::{delete::DeleteNoteButton, skeleton::NoteDetailSkeleton};
use crate::api::{ApiError, Note, get_note};
use crate::app::Route;
use crate::ui::{ButtonLink, LoadError, Missing, PageHeader, format};
use rust_starter::components::card::Card;

#[component]
pub fn NoteDetail(id: ReadSignal<i64>) -> Element {
    rsx! {
        SuspenseBoundary {
            fallback: |_| rsx! {
                NoteDetailSkeleton {}
            },
            NoteLoader { id }
        }
    }
}

#[component]
fn NoteLoader(id: ReadSignal<i64>) -> Element {
    let mut note = use_server_future(move || get_note(id()))?;
    match note() {
        Some(Ok(note)) => rsx! {
            NoteView { note }
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

#[component]
fn NoteView(note: Note) -> Element {
    rsx! {
        Link {
            class: "mb-3 inline-flex items-center gap-1 text-sm text-muted hover:text-foreground",
            to: Route::Notes { page: 1 },
            ArrowLeft { size: "1rem" }
            "All notes"
        }
        PageHeader { title: note.title,
            ButtonLink { to: Route::EditNote { id: note.id },
                Pencil {}
                "Edit"
            }
            DeleteNoteButton { id: note.id }
        }
        p { class: "mb-4 text-sm text-muted", {format::date(note.created_at)} }
        if !note.body.is_empty() {
            Card {
                div { class: "whitespace-pre-wrap break-words px-6", "{note.body}" }
            }
        }
    }
}
