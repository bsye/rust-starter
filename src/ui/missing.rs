use dioxus::fullstack::{FullstackContext, StatusCode};
use dioxus::prelude::*;
use dioxus_icons::lucide::{ArrowLeft, FileQuestionMark};

use super::{ButtonLink, EmptyState};
use crate::{app::Route, components::button::ButtonVariant};

#[component]
pub fn Missing(title: String, back_to: Route, back_label: String) -> Element {
    FullstackContext::commit_http_status(StatusCode::NOT_FOUND, None);
    rsx! {
        EmptyState { title,
            icon: rsx! {
                FileQuestionMark {}
            },
            ButtonLink { to: back_to, variant: ButtonVariant::Outline,
                ArrowLeft {}
                "{back_label}"
            }
        }
    }
}
