use dioxus::prelude::*;
use rust_starter::components::input::*;

#[component]
pub fn Demo() -> Element {
    let mut name = use_signal(String::new);
    rsx! {
        Input {
            oninput: move |e: FormEvent| name.set(e.value()),
            placeholder: "Enter your name",
            value: name,
        }
        if !name.read().is_empty() {
            p { id: "input-greeting", "Hello, {name}!" }
        }
    }
}
