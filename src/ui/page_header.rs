use dioxus::prelude::*;

#[component]
pub fn PageHeader(title: String, children: Element) -> Element {
    rsx! {
        div { class: "mb-6 flex items-center justify-between gap-4",
            h1 { class: "text-2xl font-semibold tracking-tight", "{title}" }
            div { class: "flex items-center gap-2", {children} }
        }
    }
}
