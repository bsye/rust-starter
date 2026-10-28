use dioxus::prelude::*;

use crate::{app::Route, hooks::use_current_user, ui::Redirect};

#[component]
pub fn RequireUser() -> Element {
    if use_current_user().get().is_none() {
        return rsx! {
            Redirect { to: Route::SignIn {} }
        };
    }
    rsx! {
        Outlet::<Route> {}
    }
}
