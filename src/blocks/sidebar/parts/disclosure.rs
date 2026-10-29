use dioxus::prelude::*;
use dioxus_icons::lucide::ChevronRight;

use rust_starter::components::collapsible::CollapsibleTrigger;

#[css_module("/src/components/sidebar/style.css")]
struct Styles;

#[component]
pub fn SectionTrigger(title: &'static str) -> Element {
    rsx! {
        CollapsibleTrigger {
            class: Styles::dx_sidebar_collapsible_trigger,
            r#as: move |attributes: Vec<Attribute>| rsx! {
                button {
                    class: "group/disclosure flex h-8 w-full items-center rounded-md px-2 text-sm font-medium hover:bg-surface",
                    ..attributes,
                    "{title}"
                    ChevronRight { class: "ml-auto size-4 transition-transform group-data-[open=true]/disclosure:rotate-90" }
                }
            },
        }
    }
}
