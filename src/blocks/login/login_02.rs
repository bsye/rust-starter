use dioxus::prelude::*;

use crate::blocks::auth::{Brand, Cover, Provider, SocialButton};
use crate::hooks::CredentialsMode;
use crate::ui::{
    AuthForm, EmailField, Field, FieldGroup, FieldSeparator, PasswordField, SubmitButton,
    SwitchPrompt,
};

#[component]
pub fn Login02() -> Element {
    rsx! {
        div { class: "grid min-h-svh lg:grid-cols-2",
            div { class: "flex flex-col gap-4 p-6 md:p-10",
                div { class: "flex justify-center gap-2 md:justify-start", Brand {} }
                div { class: "flex flex-1 items-center justify-center",
                    div { class: "w-full max-w-xs",
                        AuthForm { mode: CredentialsMode::SignIn, class: "flex flex-col gap-6",
                            FieldGroup {
                                div { class: "flex flex-col items-center gap-1 text-center",
                                    h1 { class: "text-2xl font-bold", "Sign in to your account" }
                                    p { class: "text-sm text-balance text-muted",
                                        "Enter your email below to sign in to your account"
                                    }
                                }
                                EmailField {}
                                PasswordField {}
                                Field { SubmitButton {} }
                                FieldSeparator { "Or continue with" }
                                Field {
                                    SocialButton { provider: Provider::GitHub }
                                    SwitchPrompt { mode: CredentialsMode::SignIn, class: "text-center" }
                                }
                            }
                        }
                    }
                }
            }
            Cover { class: "lg:flex" }
        }
    }
}
