use dioxus::prelude::*;
use dioxus_icons::lucide::NotebookText;

use crate::{app::Route, ui::ButtonLink};

#[component]
pub fn Home() -> Element {
    rsx! {
        section { class: "flex flex-col items-start gap-4",
            h1 { class: "text-3xl font-semibold tracking-tight", "Rust starter" }
            ButtonLink { to: Route::Notes { page: 1 },
                NotebookText {}
                "Open notes"
            }
        }
    }
}
