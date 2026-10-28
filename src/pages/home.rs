use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdNotebookText};

use crate::{app::Route, ui::ButtonLink};

#[component]
pub fn Home() -> Element {
    rsx! {
        section { class: "flex flex-col items-start gap-4",
            h1 { class: "text-3xl font-semibold tracking-tight", "Rust starter" }
            ButtonLink { to: Route::Notes {},
                Icon { icon: LdNotebookText }
                "Open notes"
            }
        }
    }
}
