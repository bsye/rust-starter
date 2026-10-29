use dioxus::prelude::*;

use crate::blocks::find;
use crate::pages::NotFound;

#[component]
pub fn BlockView(id: String) -> Element {
    match find(&id) {
        Some(block) => (block.render)(),
        None => rsx! {
            NotFound { segments: vec!["blocks".to_string(), id] }
        },
    }
}
