use dioxus::prelude::*;

use crate::blocks::auth::{Provider, SocialButton};
use crate::hooks::CredentialsMode;
use crate::ui::{
    AuthForm, EmailField, Field, FieldGroup, PasswordField, SubmitButton, SwitchPrompt,
};
use rust_starter::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};

#[component]
pub fn Login01() -> Element {
    rsx! {
        div { class: "flex min-h-svh w-full items-center justify-center p-6 md:p-10",
            div { class: "w-full max-w-sm",
                Card {
                    CardHeader {
                        CardTitle { "Sign in to your account" }
                        CardDescription { "Enter your email below to sign in to your account" }
                    }
                    CardContent {
                        AuthForm { mode: CredentialsMode::SignIn,
                            FieldGroup {
                                EmailField {}
                                PasswordField {}
                                Field {
                                    SubmitButton {}
                                    SocialButton { provider: Provider::Google }
                                    SwitchPrompt { mode: CredentialsMode::SignIn, class: "text-center" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
