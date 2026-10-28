use dioxus::prelude::*;

use crate::app::Route;

const NAV_LINK: &str =
    "text-muted transition-colors hover:text-foreground aria-[current=page]:text-foreground";

#[component]
pub fn Shell() -> Element {
    rsx! {
        header { class: "border-b border-border",
            nav { class: "mx-auto flex h-14 max-w-5xl items-center gap-6 px-4",
                Link { class: "font-semibold", to: Route::Home {}, "Rust starter" }
                Link { class: NAV_LINK, to: Route::Notes {}, "Notes" }
            }
        }
        main { class: "mx-auto max-w-5xl px-4 py-8", Outlet::<Route> {} }
    }
}
