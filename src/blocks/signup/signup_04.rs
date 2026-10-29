use dioxus::prelude::*;

use crate::blocks::auth::{Cover, Provider, SocialButton};
use crate::hooks::CredentialsMode;
use crate::ui::{
    AuthForm, EmailField, Field, FieldGroup, FieldSeparator, PasswordField, SubmitButton,
    SwitchPrompt,
};
use rust_starter::components::card::Card;

#[component]
pub fn Signup04() -> Element {
    rsx! {
        div { class: "flex min-h-svh flex-col items-center justify-center bg-surface p-6 md:p-10",
            div { class: "w-full max-w-sm md:max-w-4xl",
                Card {
                    div { class: "-my-6 grid md:grid-cols-2",
                        AuthForm { mode: CredentialsMode::SignUp, class: "p-6 md:p-8",
                            FieldGroup {
                                div { class: "flex flex-col items-center gap-2 text-center",
                                    h1 { class: "text-2xl font-bold", "Create your account" }
                                    p { class: "text-sm text-balance text-muted",
                                        "Enter your email below to create your account"
                                    }
                                }
                                EmailField {}
                                PasswordField {}
                                Field { SubmitButton {} }
                                FieldSeparator { behind: "bg-card", "Or continue with" }
                                Field { class: "grid grid-cols-3 gap-4",
                                    SocialButton { provider: Provider::Apple, icon_only: true }
                                    SocialButton { provider: Provider::Google, icon_only: true }
                                    SocialButton { provider: Provider::Meta, icon_only: true }
                                }
                                SwitchPrompt { mode: CredentialsMode::SignUp, class: "text-center" }
                            }
                        }
                        Cover { class: "md:flex md:rounded-r-2xl" }
                    }
                }
            }
        }
    }
}
