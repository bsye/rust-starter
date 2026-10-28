use dioxus::prelude::*;
use dioxus::router::Navigator;

use super::current_user::{CurrentUser, use_current_user};
use crate::api::{Credentials, FieldErrors, sign_in, sign_up};
use crate::app::Route;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CredentialsMode {
    SignIn,
    SignUp,
}

#[derive(Clone, Copy)]
pub struct CredentialsForm {
    pub email: Signal<String>,
    pub password: Signal<String>,
    pub errors: Signal<FieldErrors>,
    pub saving: Signal<bool>,
    mode: CredentialsMode,
    current: CurrentUser,
    nav: Navigator,
}

pub fn use_credentials_form(mode: CredentialsMode) -> CredentialsForm {
    CredentialsForm {
        email: use_signal(String::new),
        password: use_signal(String::new),
        errors: use_signal(FieldErrors::default),
        saving: use_signal(|| false),
        mode,
        current: use_current_user(),
        nav: navigator(),
    }
}

impl CredentialsForm {
    pub fn error(&self, field: &str) -> Option<String> {
        self.errors.read().get(field).map(str::to_string)
    }

    pub async fn submit(mut self) {
        self.saving.set(true);
        let credentials = Credentials {
            email: self.email.cloned(),
            password: self.password.cloned(),
        };
        let result = match self.mode {
            CredentialsMode::SignIn => sign_in(credentials).await,
            CredentialsMode::SignUp => match credentials.validated_for_sign_up() {
                Ok(credentials) => sign_up(credentials).await,
                Err(error) => Err(error),
            },
        };
        match result {
            Ok(user) => {
                self.current.set(Some(user));
                self.nav.push(Route::Notes { page: 1 });
            }
            Err(error) => {
                self.errors.set(error.into_field_errors());
                self.saving.set(false);
            }
        }
    }
}
