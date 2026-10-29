use dioxus::prelude::*;
use dioxus_icons::lucide::{LogIn, UserPlus};

use super::{Field, FieldDescription, FieldError, FieldLabel};
use crate::api::{Credentials, FieldErrors, PASSWORD_MIN_CHARS};
use crate::app::Route;
use crate::hooks::{CredentialsForm, CredentialsMode, use_credentials_form};
use rust_starter::components::{button::Button, input::Input};

#[component]
pub fn AuthForm(
    mode: CredentialsMode,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let form = use_credentials_form(mode);
    use_context_provider(|| form);
    rsx! {
        form {
            class,
            onsubmit: move |event| async move {
                event.prevent_default();
                form.submit().await;
            },
            {children}
        }
    }
}

#[component]
pub fn EmailField() -> Element {
    let form = use_context::<CredentialsForm>();
    let mut email = form.email;
    let error = form.error(Credentials::EMAIL);
    rsx! {
        Field {
            FieldLabel { html_for: "email", "Email" }
            Input {
                id: "email",
                name: "email",
                r#type: "email",
                autocomplete: "email",
                placeholder: "m@example.com",
                value: email.cloned(),
                aria_invalid: error.is_some(),
                aria_describedby: "email-error",
                oninput: move |event: FormEvent| email.set(event.value()),
            }
            FieldError { id: "email-error", message: error }
        }
    }
}

#[component]
pub fn PasswordField() -> Element {
    let form = use_context::<CredentialsForm>();
    let mut password = form.password;
    let error = form.error(Credentials::PASSWORD);
    let signing_up = form.mode() == CredentialsMode::SignUp;
    rsx! {
        Field {
            FieldLabel { html_for: "password", "Password" }
            Input {
                id: "password",
                name: "password",
                r#type: "password",
                autocomplete: if signing_up { "new-password" } else { "current-password" },
                value: password.cloned(),
                aria_invalid: error.is_some(),
                aria_describedby: "password-error",
                oninput: move |event: FormEvent| password.set(event.value()),
            }
            if signing_up && error.is_none() {
                FieldDescription { "Use at least {PASSWORD_MIN_CHARS} characters." }
            }
            FieldError { id: "password-error", message: error }
        }
    }
}

#[component]
pub fn SubmitButton() -> Element {
    let form = use_context::<CredentialsForm>();
    rsx! {
        FieldError { id: "form-error", message: form.error(FieldErrors::FORM) }
        Button { r#type: "submit", disabled: form.saving.cloned(),
            match form.mode() {
                CredentialsMode::SignIn => rsx! {
                    LogIn {}
                    "Sign in"
                },
                CredentialsMode::SignUp => rsx! {
                    UserPlus {}
                    "Create account"
                },
            }
        }
    }
}

#[component]
pub fn SwitchPrompt(mode: CredentialsMode, #[props(default)] class: String) -> Element {
    rsx! {
        FieldDescription { class,
            match mode {
                CredentialsMode::SignIn => rsx! {
                    "No account yet? "
                    Link { to: Route::SignUp {}, "Create one" }
                },
                CredentialsMode::SignUp => rsx! {
                    "Already have an account? "
                    Link { to: Route::SignIn {}, "Sign in" }
                },
            }
        }
    }
}
