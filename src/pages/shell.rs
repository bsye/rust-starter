use dioxus::prelude::*;
use dioxus_icons::lucide::{LogIn, LogOut};

use crate::app::Route;
use crate::components::button::{Button, ButtonVariant};
use crate::hooks::{use_current_user_provider, use_sign_out};
use crate::ui::ButtonLink;

const NAV_LINK: &str =
    "text-muted transition-colors hover:text-foreground data-[active=true]:text-foreground";

#[component]
pub fn Shell() -> Element {
    let current = use_current_user_provider();
    let route = use_route::<Route>();
    let user = current.get();
    rsx! {
        header { class: "border-b border-border",
            nav { class: "mx-auto flex h-14 max-w-5xl items-center gap-6 px-4",
                Link { class: "font-semibold", to: Route::Home {}, "Rust starter" }
                if user.is_some() {
                    Link {
                        class: NAV_LINK,
                        "data-active": route.in_notes(),
                        to: Route::Notes { page: 1 },
                        "Notes"
                    }
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
