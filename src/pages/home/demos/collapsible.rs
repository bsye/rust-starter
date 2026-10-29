use dioxus::prelude::*;
use rust_starter::components::collapsible::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Collapsible {
            CollapsibleTrigger {
                b { "Recent activity" }
            }
            CollapsibleList {
                CollapsibleItem { "Added a new feature to the collapsible component" }
                CollapsibleContent {
                    CollapsibleItem { "Fixed a bug in the collapsible component" }
                    CollapsibleItem { "Updated the documentation for the collapsible component" }
                }
            }
        }
    }
}
