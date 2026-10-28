use dioxus::prelude::*;
use dioxus::router::Navigator;

use crate::api::{ApiError, FieldErrors, NoteInput, create_note};
use crate::app::Route;

#[derive(Clone, Copy)]
pub struct NoteForm {
    pub title: Signal<String>,
    pub body: Signal<String>,
    pub errors: Signal<FieldErrors>,
    pub failure: Signal<Option<String>>,
    pub saving: Signal<bool>,
    nav: Navigator,
}

pub fn use_note_form() -> NoteForm {
    NoteForm {
        title: use_signal(String::new),
        body: use_signal(String::new),
        errors: use_signal(FieldErrors::default),
        failure: use_signal(|| None),
        saving: use_signal(|| false),
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
        let result = match input.validated() {
            Ok(input) => create_note(input).await,
            Err(error) => Err(error),
        };
        match result {
            Ok(note) => {
                self.nav.push(Route::NoteDetail { id: note.id });
            }
            Err(ApiError::Invalid(errors)) => {
                self.errors.set(errors);
                self.failure.set(None);
                self.saving.set(false);
            }
            Err(error) => {
                self.failure.set(Some(error.to_string()));
                self.saving.set(false);
            }
        }
    }
}
