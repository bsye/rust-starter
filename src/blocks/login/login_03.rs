use dioxus::prelude::*;

use crate::blocks::auth::{Brand, Provider, SocialButton};
use crate::hooks::CredentialsMode;
use crate::ui::{
    AuthForm, EmailField, Field, FieldGroup, FieldSeparator, PasswordField, SubmitButton,
    SwitchPrompt,
};
use rust_starter::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};

#[component]
pub fn Login03() -> Element {
    rsx! {
        div { class: "flex min-h-svh flex-col items-center justify-center gap-6 bg-surface p-6 md:p-10",
            div { class: "flex w-full max-w-sm flex-col gap-6",
                Brand { class: "self-center" }
                Card {
                    CardHeader {
                        div { class: "flex flex-col gap-2 text-center",
                            CardTitle { "Welcome back" }
                            CardDescription { "Sign in with your Apple or Google account" }
                        }
                    }
                    CardContent {
                        AuthForm { mode: CredentialsMode::SignIn,
                            FieldGroup {
                                Field {
                                    SocialButton { provider: Provider::Apple }
                                    SocialButton { provider: Provider::Google }
                                }
                                FieldSeparator { behind: "bg-card", "Or continue with" }
                                EmailField {}
                                PasswordField {}
                                Field {
                                    SubmitButton {}
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
