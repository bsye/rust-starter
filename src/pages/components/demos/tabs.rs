use dioxus::prelude::*;
use rust_starter::components::tabs::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Tabs {
            default_value: "account".to_string(),
            horizontal: true,
            max_width: "24rem",
            TabList {
                TabTrigger { value: "account".to_string(), index: 0usize, "Account" }
                TabTrigger { value: "password".to_string(), index: 1usize, "Password" }
            }
            TabContent { index: 0usize, value: "account".to_string(),
                p { class: "py-4 text-sm", "Make changes to your account here." }
            }
            TabContent { index: 1usize, value: "password".to_string(),
                p { class: "py-4 text-sm", "Change your password here." }
            }
        }
    }
}
