use dioxus::html::input_data::keyboard_types::NamedKey;
use dioxus::prelude::*;
use dioxus_icons::lucide::{ArrowUp, Paperclip, X};

use super::draft::Draft;
use rust_starter::components::attachment::{
    Attachment, AttachmentAction, AttachmentActions, AttachmentContent, AttachmentDescription,
    AttachmentGroup, AttachmentSize, AttachmentTitle,
};
use rust_starter::components::button::{ButtonSize, ButtonVariant};
use rust_starter::components::input_group::{
    InputGroup, InputGroupAddon, InputGroupAlign, InputGroupButton, InputGroupText,
    InputGroupTextarea,
};

const FILE_INPUT: &str = "chat-01-files";

#[component]
pub fn Composer(draft: Draft) -> Element {
    let mut text = draft.text;
    rsx! {
        InputGroup {
            if !draft.files.read().is_empty() {
                InputGroupAddon { align: InputGroupAlign::BlockStart,
                    AttachmentGroup { class: "w-full",
                        for (index, file) in draft.files.read().iter().enumerate() {
                            Attachment { key: "{index}-{file.name}", size: AttachmentSize::Sm,
                                AttachmentContent {
                                    AttachmentTitle { "{file.name}" }
                                    AttachmentDescription { "{file.detail}" }
                                }
                                AttachmentActions {
                                    AttachmentAction {
                                        aria_label: "Remove {file.name}",
                                        onclick: move |_| draft.remove(index),
                                        X {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
            InputGroupTextarea {
                placeholder: "Write a message",
                aria_label: "Message",
                value: text(),
                oninput: move |event: FormEvent| text.set(event.value()),
                onkeydown: move |event: KeyboardEvent| {
                    if event.key() == Key::Named(NamedKey::Enter) && !event.modifiers().shift() {
                        event.prevent_default();
                        draft.send();
                    }
                },
            }
            InputGroupAddon { align: InputGroupAlign::BlockEnd,
                input {
                    id: FILE_INPUT,
                    class: "sr-only",
                    tabindex: "-1",
                    aria_hidden: "true",
                    r#type: "file",
                    multiple: true,
                    onchange: move |event: FormEvent| draft.attach(event.files()),
                }
                InputGroupButton {
                    size: ButtonSize::IconXs,
                    aria_label: "Attach files",
                    onclick: move |_| {
                        document::eval(&format!("document.getElementById('{FILE_INPUT}').click()"));
                    },
                    Paperclip {}
                }
                InputGroupText { "Enter to send, Shift Enter for a new line" }
                InputGroupButton {
                    variant: ButtonVariant::Primary,
                    size: ButtonSize::IconXs,
                    class: "ml-auto",
                    aria_label: "Send",
                    disabled: !draft.ready(),
                    onclick: move |_| draft.send(),
                    ArrowUp {}
                }
            }
        }
    }
}
