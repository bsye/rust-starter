use dioxus::prelude::*;

use crate::blocks::auth::{Provider, SocialButton};
use crate::hooks::CredentialsMode;
use crate::ui::{
    AuthForm, EmailField, Field, FieldGroup, PasswordField, SubmitButton, SwitchPrompt,
};
use rust_starter::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};

#[component]
pub fn Signup01() -> Element {
    rsx! {
        div { class: "flex min-h-svh w-full items-center justify-center p-6 md:p-10",
            div { class: "w-full max-w-sm",
                Card {
                    CardHeader {
                        CardTitle { "Create an account" }
                        CardDescription { "Enter your information below to create your account" }
                    }
                    CardContent {
                        AuthForm { mode: CredentialsMode::SignUp,
                            FieldGroup {
                                EmailField {}
                                PasswordField {}
                                Field {
                                    SubmitButton {}
                                    SocialButton { provider: Provider::Google }
                                    SwitchPrompt { mode: CredentialsMode::SignUp, class: "px-6 text-center" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
