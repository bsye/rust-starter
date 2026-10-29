use dioxus::prelude::*;

use crate::blocks::auth::Brand;
use crate::hooks::CredentialsMode;
use crate::ui::{
    AuthForm, EmailField, Field, FieldGroup, PasswordField, SubmitButton, SwitchPrompt,
};
use rust_starter::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};

#[component]
pub fn Signup03() -> Element {
    rsx! {
        div { class: "flex min-h-svh flex-col items-center justify-center gap-6 bg-surface p-6 md:p-10",
            div { class: "flex w-full max-w-sm flex-col gap-6",
                Brand { class: "self-center" }
                Card {
                    CardHeader {
                        div { class: "flex flex-col gap-2 text-center",
                            CardTitle { "Create your account" }
                            CardDescription { "Enter your email below to create your account" }
                        }
                    }
                    CardContent {
                        AuthForm { mode: CredentialsMode::SignUp,
                            FieldGroup {
                                EmailField {}
                                PasswordField {}
                                Field {
                                    SubmitButton {}
                                    SwitchPrompt { mode: CredentialsMode::SignUp, class: "text-center" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
