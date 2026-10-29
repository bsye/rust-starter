use dioxus::prelude::*;
use dioxus_icons::lucide::{LogIn, LogOut};

use crate::app::{APP_NAME, Route};
use crate::hooks::{use_current_user, use_sign_out};
use crate::ui::{ButtonLink, NavLink};
use rust_starter::components::button::{Button, ButtonVariant};

#[component]
pub fn Shell() -> Element {
    let current = use_current_user();
    let route = use_route::<Route>();
    let user = current.get();
    rsx! {
        header { class: "border-b border-border",
            nav { class: "mx-auto flex h-14 max-w-5xl items-center gap-6 px-4",
                Link { class: "font-semibold", to: Route::Home {}, "{APP_NAME}" }
                div { class: "hidden items-center gap-6 sm:flex",
                    NavLink {
                        to: Route::Components {},
                        active: route == Route::Components {},
                        "Components"
                    }
                    NavLink {
                        to: Route::Blocks { category: String::new() },
                        active: route.in_blocks(),
                        "Blocks"
                    }
                }
                if user.is_some() {
                    NavLink { to: Route::Notes { page: 1 }, active: route.in_notes(), "Notes" }
                }
                div { class: "ml-auto flex items-center gap-3",
                    if let Some(user) = user {
                        span {
                            class: "hidden max-w-48 truncate text-sm text-muted sm:inline",
                            title: "{user.email}",
                            "{user.email}"
                        }
                        SignOutButton {}
                    } else if !route.is_auth() {
                        ButtonLink { to: Route::SignIn {}, variant: ButtonVariant::Ghost,
                            LogIn {}
                            "Sign in"
                        }
                    }
                }
            }
        }
        main { class: "mx-auto max-w-5xl px-4 py-8", Outlet::<Route> {} }
    }
}

#[component]
fn SignOutButton() -> Element {
    let sign_out = use_sign_out();
    rsx! {
        Button {
            variant: ButtonVariant::Ghost,
            title: sign_out.failure.cloned().unwrap_or_default(),
            onclick: move |_| sign_out.run(),
            LogOut {}
            "Sign out"
        }
    }
}
