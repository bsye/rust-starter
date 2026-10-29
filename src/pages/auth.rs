use dioxus::prelude::*;
use dioxus_icons::lucide::{LogIn, UserPlus};

use crate::api::{Credentials, FieldErrors};
use crate::app::Route;
use crate::hooks::{CredentialsMode, use_credentials_form, use_current_user};
use crate::ui::{Field, PageHeader, Redirect};
use rust_starter::components::{button::Button, input::Input};

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
    let signing_up = mode == CredentialsMode::SignUp;
    rsx! {
        div { class: "mx-auto max-w-sm",
            PageHeader { title: if signing_up { "Create account" } else { "Sign in" } }
            CredentialsFormView { mode }
            p { class: "mt-6 text-sm text-muted",
                if signing_up {
                    "Already have an account? "
                    Link { class: "text-foreground underline", to: Route::SignIn {}, "Sign in" }
                } else {
                    "No account yet? "
                    Link { class: "text-foreground underline", to: Route::SignUp {}, "Create one" }
                }
            }
        }
    }
}

#[component]
fn CredentialsFormView(mode: CredentialsMode) -> Element {
    let mut form = use_credentials_form(mode);
    let email_error = form.error(Credentials::EMAIL);
    let password_error = form.error(Credentials::PASSWORD);
    let signing_up = mode == CredentialsMode::SignUp;
    rsx! {
        form {
            class: "flex flex-col gap-5",
            onsubmit: move |event| async move {
                event.prevent_default();
                form.submit().await;
            },
            Field { label: "Email", id: "email", error: email_error.clone(),
                Input {
                    id: "email",
                    name: "email",
                    r#type: "email",
                    autocomplete: "email",
                    value: form.email.cloned(),
                    aria_invalid: email_error.is_some(),
                    aria_describedby: "email-error",
                    oninput: move |event: FormEvent| form.email.set(event.value()),
                }
            }
            Field { label: "Password", id: "password", error: password_error.clone(),
                Input {
                    id: "password",
                    name: "password",
                    r#type: "password",
                    autocomplete: if signing_up { "new-password" } else { "current-password" },
                    value: form.password.cloned(),
                    aria_invalid: password_error.is_some(),
                    aria_describedby: "password-error",
                    oninput: move |event: FormEvent| form.password.set(event.value()),
                }
            }
            if let Some(message) = form.error(FieldErrors::FORM) {
                p { class: "text-sm text-destructive", role: "alert", "{message}" }
            }
            Button { r#type: "submit", disabled: form.saving.cloned(),
                if signing_up {
                    UserPlus {}
                    "Create account"
                } else {
                    LogIn {}
                    "Sign in"
                }
            }
        }
    }
}
