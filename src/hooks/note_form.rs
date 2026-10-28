use dioxus::prelude::*;
use dioxus::router::Navigator;

use crate::api::{FieldErrors, Note, NoteInput, create_note, update_note};
use crate::app::Route;

#[derive(Clone, Copy)]
pub struct NoteForm {
    pub title: Signal<String>,
    pub body: Signal<String>,
    pub errors: Signal<FieldErrors>,
    pub saving: Signal<bool>,
    id: Option<i64>,
    nav: Navigator,
}

pub fn use_note_form(existing: Option<Note>) -> NoteForm {
    let id = existing.as_ref().map(|note| note.id);
    let (title, body) = existing
        .map(|note| (note.title, note.body))
        .unwrap_or_default();
    NoteForm {
        title: use_signal(|| title),
        body: use_signal(|| body),
        errors: use_signal(FieldErrors::default),
        saving: use_signal(|| false),
        id,
        nav: navigator(),
    }
}

impl NoteForm {
    pub fn error(&self, field: &str) -> Option<String> {
        self.errors.read().get(field).map(str::to_string)
    }

    pub async fn submit(mut self) {
        self.saving.set(true);
        let input = NoteInput {
            title: self.title.cloned(),
            body: self.body.cloned(),
        };
        let result = match (input.validated(), self.id) {
            (Ok(input), Some(id)) => update_note(id, input).await,
            (Ok(input), None) => create_note(input).await,
            (Err(error), _) => Err(error),
        };
        match result {
            Ok(note) => {
                self.nav.push(Route::NoteDetail { id: note.id });
            }
            Err(error) => {
                self.errors.set(error.into_field_errors());
                self.saving.set(false);
            }
        }
    }
}
