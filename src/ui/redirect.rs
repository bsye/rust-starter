use dioxus::fullstack::{
    FullstackContext, StatusCode,
    http::{HeaderValue, header::LOCATION},
};
use dioxus::prelude::*;

use crate::app::Route;

#[component]
pub fn Redirect(to: Route) -> Element {
    if let Some(context) = FullstackContext::current() {
        let location =
            HeaderValue::from_str(&to.to_string()).expect("routes render as valid header values");
        context.add_response_header(LOCATION, location);
        FullstackContext::commit_http_status(StatusCode::SEE_OTHER, None);
    }
    let nav = navigator();
    use_effect(move || {
        nav.replace(to.clone());
    });
    rsx! {}
}
