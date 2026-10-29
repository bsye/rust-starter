use dioxus::prelude::*;

use crate::app::Route;
use crate::blocks::{Category, in_category};
use crate::ui::{Missing, NavLink, PageHeader};

mod preview;
mod view;

use preview::BlockPreview;
pub use view::BlockView;

#[component]
pub fn Blocks(category: String) -> Element {
    let Some(selected) = Category::from_slug(&category) else {
        return rsx! {
            Missing {
                title: "This block category does not exist",
                back_to: Route::Blocks { category: String::new() },
                back_label: "All blocks",
            }
        };
    };
    rsx! {
        PageHeader { title: "Blocks" }
        nav {
            aria_label: "Block categories",
            class: "mb-8 flex flex-wrap gap-x-6 gap-y-2 text-sm",
            for category in Category::ALL {
                NavLink {
                    key: "{category.label()}",
                    to: Route::Blocks { category: category.slug().to_string() },
                    active: category == selected,
                    "{category.label()}"
                }
            }
        }
        div { class: "flex flex-col gap-12",
            for block in in_category(selected) {
                BlockPreview { key: "{block.id}", id: block.id }
            }
        }
    }
}
