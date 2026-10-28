use dioxus::prelude::*;
use dioxus_icons::lucide::Save;

use crate::api::NoteInput;
use crate::components::{button::Button, input::Input, textarea::Textarea};
use crate::hooks::use_note_form;
use crate::ui::{Field, PageHeader};

#[component]
pub fn NewNote() -> Element {
    let mut note_form = use_note_form();
    let title_error = note_form.error(NoteInput::TITLE);
    let body_error = note_form.error(NoteInput::BODY);
    rsx! {
        PageHeader { title: "New note" }
        form {
            class: "flex max-w-xl flex-col gap-5",
            onsubmit: move |event| async move {
                event.prevent_default();
                note_form.submit().await;
            },
            Field { label: "Title", id: "title", error: title_error.clone(),
                Input {
                    id: "title",
                    name: "title",
                    value: note_form.title.cloned(),
                    aria_invalid: title_error.is_some(),
                    aria_describedby: "title-error",
                    oninput: move |event: FormEvent| note_form.title.set(event.value()),
                }
            }
            Field { label: "Body", id: "body", error: body_error.clone(),
                Textarea {
                    id: "body",
                    name: "body",
                    rows: 8,
                    value: note_form.body.cloned(),
                    aria_invalid: body_error.is_some(),
                    aria_describedby: "body-error",
                    oninput: move |event: FormEvent| note_form.body.set(event.value()),
                }
            }
            if let Some(message) = note_form.failure.cloned() {
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
