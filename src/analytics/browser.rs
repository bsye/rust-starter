use dioxus::prelude::*;

use super::{distinct_id, request_settings};
use crate::hooks::CurrentUser;

const POSTHOG_INIT: &str = include_str!("posthog_init.js");
const POSTHOG_DEFAULTS: &str = "2026-08-30";

#[derive(Debug, PartialEq)]
enum IdentityChange {
    Identify(i64),
    Reset,
}

fn identity_change(previous: Option<i64>, current: Option<i64>) -> Option<IdentityChange> {
    match (previous, current) {
        (previous, Some(id)) if previous != Some(id) => Some(IdentityChange::Identify(id)),
        (Some(_), None) => Some(IdentityChange::Reset),
        _ => None,
    }
}

fn run_script(script: &str, data: impl serde::Serialize) {
    if let Err(error) = document::eval(script).send(data) {
        dioxus::logger::tracing::error!("analytics script failed: {error}");
    }
}

pub fn use_analytics(current: CurrentUser) {
    let settings = use_server_cached(request_settings);
    let enabled = settings.is_some();
    let mut identified = use_hook(|| CopyValue::new(None::<i64>));
    use_effect(move || {
        if let Some(settings) = &settings {
            run_script(
                POSTHOG_INIT,
                (&settings.token, &settings.host, POSTHOG_DEFAULTS),
            );
        }
    });
    use_effect(move || {
        let user_id = current.get().map(|user| user.id);
        if !enabled {
            return;
        }
        match identity_change(identified.cloned(), user_id) {
            Some(IdentityChange::Identify(id)) => {
                run_script("posthog.identify(await dioxus.recv())", distinct_id(id))
            }
            Some(IdentityChange::Reset) => run_script("posthog.reset()", ()),
            None => {}
        }
        identified.set(user_id);
    });
}

#[cfg(test)]
mod tests {
    use super::{IdentityChange, identity_change};

    #[test]
    fn signing_in_identifies_the_user() {
        assert_eq!(
            identity_change(None, Some(7)),
            Some(IdentityChange::Identify(7))
        );
    }

    #[test]
    fn switching_users_identifies_the_new_one() {
        assert_eq!(
            identity_change(Some(7), Some(8)),
            Some(IdentityChange::Identify(8))
        );
    }

    #[test]
    fn signing_out_resets() {
        assert_eq!(identity_change(Some(7), None), Some(IdentityChange::Reset));
    }

    #[test]
    fn no_change_does_nothing() {
        assert_eq!(identity_change(Some(7), Some(7)), None);
        assert_eq!(identity_change(None, None), None);
    }
}
