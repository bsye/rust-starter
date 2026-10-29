use dioxus::prelude::*;

use crate::app::Route;
use crate::hooks::{CredentialsMode, use_current_user};
use crate::ui::{
    AuthForm, EmailField, PageHeader, PasswordField, Redirect, SubmitButton, SwitchPrompt,
};

#[component]
pub fn SignIn() -> Element {
    rsx! {
        CredentialsPage { mode: CredentialsMode::SignIn }
    }
}

#[component]
pub fn SignUp() -> Element {
    rsx! {
        CredentialsPage { mode: CredentialsMode::SignUp }
    }
}

#[component]
fn CredentialsPage(mode: CredentialsMode) -> Element {
    if use_current_user().get().is_some() {
        return rsx! {
            Redirect { to: Route::Notes { page: 1 } }
        };
    }
    rsx! {
        div { class: "mx-auto max-w-sm",
            PageHeader {
                title: match mode {
                    CredentialsMode::SignIn => "Sign in",
                    CredentialsMode::SignUp => "Create account",
                },
            }
            AuthForm { mode, class: "flex flex-col gap-5",
                EmailField {}
                PasswordField {}
                SubmitButton {}
            }
            SwitchPrompt { mode, class: "mt-6" }
        }
    }
}
