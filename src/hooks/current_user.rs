use dioxus::prelude::*;

use crate::api::{UserInfo, request_user};

#[derive(Clone, Copy)]
pub struct CurrentUser(Signal<Option<UserInfo>>);

impl CurrentUser {
    pub fn get(&self) -> Option<UserInfo> {
        self.0.cloned()
    }

    pub fn set(&mut self, user: Option<UserInfo>) {
        self.0.set(user);
    }
}

pub fn use_current_user_provider() -> CurrentUser {
    let initial = use_server_cached(request_user);
    use_context_provider(|| CurrentUser(Signal::new(initial)))
}

pub fn use_current_user() -> CurrentUser {
    use_context()
}
