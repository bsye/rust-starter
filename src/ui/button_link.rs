use dioxus::prelude::*;

use crate::components::button::{ButtonSize, ButtonVariant};

#[css_module("/src/components/button/style.css")]
struct Styles;

#[component]
pub fn ButtonLink(
    #[props(into)] to: NavigationTarget,
    #[props(default)] variant: ButtonVariant,
    children: Element,
) -> Element {
    rsx! {
        Link {
            class: Styles::dx_button.to_string(),
            "data-style": variant.class(),
            "data-size": ButtonSize::Default.class(),
            to,
            {children}
        }
    }
}
