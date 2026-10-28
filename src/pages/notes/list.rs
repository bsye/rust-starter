use dioxus::prelude::*;
use dioxus_icons::lucide::{ArrowLeft, Plus, StickyNote};

use super::{card::NoteCard, skeleton::NoteListSkeleton};
use crate::api::{NotePage, list_notes};
use crate::app::Route;
use crate::components::button::ButtonVariant;
use crate::ui::{ButtonLink, EmptyState, LoadError, PageHeader, Pager, format, neighbours};

#[component]
pub fn Notes(page: ReadSignal<u64>) -> Element {
    rsx! {
        PageHeader { title: "Notes",
            ButtonLink { to: Route::NewNote {},
                Plus {}
                "New note"
            }
        }
        SuspenseBoundary {
            fallback: |_| rsx! {
                NoteListSkeleton {}
            },
            NoteList { page }
        }
    }
}

#[component]
fn NoteList(page: ReadSignal<u64>) -> Element {
    let mut notes = use_server_future(move || list_notes(page()))?;
    match notes() {
        Some(Ok(result)) => rsx! {
            NotePageView { result }
        },
        Some(Err(error)) => rsx! {
            LoadError { message: error.to_string(), onretry: move |_| notes.restart() }
        },
        None => rsx! {
            NoteListSkeleton {}
        },
    }
}

#[component]
fn NotePageView(result: NotePage) -> Element {
    if result.total == 0 {
        return rsx! {
            EmptyState { title: "No notes yet",
                icon: rsx! {
                    StickyNote {}
                },
                ButtonLink { to: Route::NewNote {}, variant: ButtonVariant::Outline,
                    Plus {}
                    "New note"
                }
            }
        };
    }
    let (previous, next) = neighbours(result.page, result.page_count);
    rsx! {
        if result.items.is_empty() {
            EmptyState { title: "No notes on this page",
                icon: rsx! {
                    StickyNote {}
                },
                ButtonLink { to: Route::Notes { page: 1 }, variant: ButtonVariant::Outline,
                    ArrowLeft {}
                    "First page"
                }
            }
        }
        div { class: "grid gap-3",
            for note in result.items {
                NoteCard { key: "{note.id}", note }
            }
        }
        Pager {
            summary: format!(
                "Page {} of {} · {} notes",
                result.page,
                result.page_count,
                format::count(result.total),
            ),
            previous: previous.map(|page| Route::Notes { page }),
            next: next.map(|page| Route::Notes { page }),
        }
    }
}
