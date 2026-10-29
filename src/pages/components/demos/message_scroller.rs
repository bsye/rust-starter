use dioxus::html::input_data::keyboard_types::NamedKey;
use dioxus::prelude::*;
use dioxus_icons::lucide::{ArrowUp, History, MessageSquareReply};

use rust_starter::components::bubble::{Bubble, BubbleContent, BubbleVariant};
use rust_starter::components::button::{Button, ButtonSize, ButtonVariant};
use rust_starter::components::input_group::{
    InputGroup, InputGroupAddon, InputGroupAlign, InputGroupButton, InputGroupTextarea,
};
use rust_starter::components::message::{Message, MessageAlign, MessageContent};
use rust_starter::components::message_scroller::*;

mod data;

use data::{Entry, OLDER, OPENING, REPLY, entries};

#[component]
pub fn Demo() -> Element {
    let mut thread = use_signal(|| entries(&OPENING, 100));
    let mut older_loaded = use_signal(|| false);
    let mut draft = use_signal(String::new);
    let mut next_id = use_signal(|| 200);
    let mut add = move |mine: bool, text: String| {
        let id = next_id();
        next_id.set(id + 1);
        thread.write().push(Entry { id, mine, text });
    };
    let mut send = move || {
        let text = draft().trim().to_string();
        if !text.is_empty() {
            add(true, text);
            draft.set(String::new());
        }
    };
    rsx! {
        div { style: "display: flex; height: 30rem; width: 100%; max-width: 26rem; flex-direction: column; gap: 0.75rem;",
            div { style: "display: flex; gap: 0.5rem;",
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    disabled: older_loaded(),
                    onclick: move |_| {
                        let mut older = entries(&OLDER, 0);
                        older.extend(thread());
                        thread.set(older);
                        older_loaded.set(true);
                    },
                    History {}
                    "Load older"
                }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    onclick: move |_| add(false, REPLY.to_string()),
                    MessageSquareReply {}
                    "Add a reply"
                }
            }
            div { style: "display: flex; min-height: 0; flex: 1; border: 1px solid var(--primary-color-6); border-radius: 0.75rem; overflow: hidden;",
                MessageScrollerProvider { auto_scroll: true,
                    MessageScroller {
                        MessageScrollerViewport {
                            MessageScrollerContent { style: "padding: 1rem; gap: 1rem;",
                                for entry in thread() {
                                    MessageScrollerItem { key: "{entry.id}", message_id: "{entry.id}", scroll_anchor: entry.mine,
                                        Message { align: if entry.mine { MessageAlign::End } else { MessageAlign::Start },
                                            MessageContent {
                                                Bubble { variant: if entry.mine { BubbleVariant::Default } else { BubbleVariant::Muted },
                                                    BubbleContent { "{entry.text}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        MessageScrollerButton {}
                    }
                }
            }
            InputGroup {
                InputGroupTextarea {
                    placeholder: "Ask a question",
                    aria_label: "Message",
                    value: draft(),
                    oninput: move |event: FormEvent| draft.set(event.value()),
                    onkeydown: move |event: KeyboardEvent| {
                        if event.key() == Key::Named(NamedKey::Enter) && !event.modifiers().shift() {
                            event.prevent_default();
                            send();
                        }
                    },
                }
                InputGroupAddon { align: InputGroupAlign::BlockEnd,
                    InputGroupButton {
                        variant: ButtonVariant::Primary,
                        size: ButtonSize::IconXs,
                        style: "margin-left: auto;",
                        aria_label: "Send",
                        disabled: draft().trim().is_empty(),
                        onclick: move |_| send(),
                        ArrowUp {}
                    }
                }
            }
        }
    }
}
