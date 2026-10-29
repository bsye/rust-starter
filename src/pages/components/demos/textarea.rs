use dioxus::prelude::*;
use rust_starter::components::label::Label;
use rust_starter::components::textarea::*;

#[component]
pub fn Demo() -> Element {
    let mut message = use_signal(String::new);
    rsx! {
        div { class: "flex flex-col gap-2",
            Label { html_for: "message", "Your message" }
            Textarea {
                id: "message",
                variant: TextareaVariant::Default,
                placeholder: "Type your message here.",
                value: message,
                oninput: move |event: FormEvent| message.set(event.value()),
            }
            span { class: "text-sm tabular-nums text-muted", "{message.read().chars().count()} characters" }
        }
    }
}
