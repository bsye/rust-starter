use dioxus::prelude::*;

use crate::app::Route;

#[component]
pub fn NavLink(to: Route, active: bool, children: Element) -> Element {
    rsx! {
        Link {
            class: "text-muted transition-colors hover:text-foreground data-[active=true]:text-foreground",
            "data-active": active,
            to,
            {children}
        }
    }
}
