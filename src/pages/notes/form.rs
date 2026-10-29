use dioxus::prelude::*;
use dioxus_icons::lucide::Save;

use crate::api::{FieldErrors, Note, NoteInput};
use crate::hooks::use_note_form;
use crate::ui::{Field, FieldError, FieldLabel};
use rust_starter::components::{button::Button, input::Input, textarea::Textarea};

#[component]
pub fn NoteFormView(existing: Option<Note>) -> Element {
    let mut note_form = use_note_form(existing);
    let title_error = note_form.error(NoteInput::TITLE);
    let body_error = note_form.error(NoteInput::BODY);
    rsx! {
        form {
            class: "flex max-w-xl flex-col gap-5",
            onsubmit: move |event| async move {
                event.prevent_default();
                note_form.submit().await;
            },
            Field {
                FieldLabel { html_for: "title", "Title" }
                Input {
                    id: "title",
                    name: "title",
                    value: note_form.title.cloned(),
                    aria_invalid: title_error.is_some(),
                    aria_describedby: "title-error",
                    oninput: move |event: FormEvent| note_form.title.set(event.value()),
                }
                FieldError { id: "title-error", message: title_error }
            }
            Field {
                FieldLabel { html_for: "body", "Body" }
                Textarea {
                    id: "body",
                    name: "body",
                    rows: 8,
                    value: note_form.body.cloned(),
                    aria_invalid: body_error.is_some(),
                    aria_describedby: "body-error",
                    oninput: move |event: FormEvent| note_form.body.set(event.value()),
                }
                FieldError { id: "body-error", message: body_error }
            }
            if let Some(message) = note_form.error(FieldErrors::FORM) {
                p { class: "text-sm text-destructive", role: "alert", "{message}" }
            }
            div {
                Button { r#type: "submit", disabled: note_form.saving.cloned(),
                    Save {}
                    "Save note"
                }
            }
        }
    }
}
