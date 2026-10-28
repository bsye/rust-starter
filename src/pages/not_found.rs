use dioxus::prelude::*;

use crate::{app::Route, ui::Missing};

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        Missing {
            title: "Page not found",
            back_to: Route::Home {},
            back_label: "Go home",
        }
    }
}
