use dioxus::prelude::*;
use dioxus_icons::lucide::{NotebookText, UserPlus};

use crate::{app::Route, hooks::use_current_user, ui::ButtonLink};

#[component]
pub fn Home() -> Element {
    let signed_in = use_current_user().get().is_some();
    rsx! {
        section { class: "flex flex-col items-start gap-4",
            h1 { class: "text-3xl font-semibold tracking-tight", "Rust starter" }
            if signed_in {
                ButtonLink { to: Route::Notes { page: 1 },
                    NotebookText {}
                    "Open notes"
                }
            } else {
                ButtonLink { to: Route::SignUp {},
                    UserPlus {}
                    "Create account"
                }
            }
        }
    }
}
