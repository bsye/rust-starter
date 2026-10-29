use dioxus::prelude::*;

use crate::app::APP_NAME;
use crate::blocks::auth::{BrandIcon, Provider, SocialButton};
use crate::hooks::CredentialsMode;
use crate::ui::{
    AuthForm, EmailField, Field, FieldGroup, FieldSeparator, PasswordField, SubmitButton,
    SwitchPrompt,
};

#[component]
pub fn Login05() -> Element {
    rsx! {
        div { class: "flex min-h-svh flex-col items-center justify-center gap-6 bg-background p-6 md:p-10",
            div { class: "w-full max-w-sm",
                AuthForm { mode: CredentialsMode::SignIn,
                    FieldGroup {
                        div { class: "flex flex-col items-center gap-2 text-center",
                            BrandIcon {}
                            h1 { class: "text-xl font-bold", "Welcome to {APP_NAME}" }
                            SwitchPrompt { mode: CredentialsMode::SignIn }
                        }
                        EmailField {}
                        PasswordField {}
                        Field { SubmitButton {} }
                        FieldSeparator { "Or" }
                        Field { class: "grid gap-4 sm:grid-cols-2",
                            SocialButton { provider: Provider::Apple }
                            SocialButton { provider: Provider::Google }
                        }
                    }
                }
            }
        }
    }
}
