use dioxus::fullstack::{FullstackContext, http::StatusCode};
use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdArrowLeft, LdFileQuestion},
};

use crate::{
    app::Route,
    ui::{ButtonLink, EmptyState},
};

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    FullstackContext::commit_http_status(StatusCode::NOT_FOUND, None);
    rsx! {
        EmptyState { title: "Page not found",
            icon: rsx! {
                Icon { icon: LdFileQuestion }
            },
            ButtonLink { to: Route::Home {},
                Icon { icon: LdArrowLeft }
                "Go home"
            }
        }
    }
}
