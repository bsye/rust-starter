use dioxus::prelude::*;
use dioxus::router::Navigator;

use super::current_user::{CurrentUser, use_current_user};
use crate::api::sign_out;
use crate::app::Route;

#[derive(Clone, Copy)]
pub struct SignOut {
    pub failure: Signal<Option<String>>,
    current: CurrentUser,
    nav: Navigator,
}

pub fn use_sign_out() -> SignOut {
    SignOut {
        failure: use_signal(|| None),
        current: use_current_user(),
        nav: navigator(),
    }
}

impl SignOut {
    pub async fn run(mut self) {
        match sign_out().await {
            Ok(()) => {
                self.current.set(None);
                self.nav.push(Route::SignIn {});
            }
            Err(error) => self.failure.set(Some(error.to_string())),
        }
    }
}
