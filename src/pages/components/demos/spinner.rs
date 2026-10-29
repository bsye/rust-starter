use dioxus::prelude::*;

use rust_starter::components::button::{Button, ButtonVariant};
use rust_starter::components::spinner::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; align-items: center; gap: 1.5rem;",
            Spinner {}
            Spinner { style: "width: 1.5rem; height: 1.5rem;" }
            Spinner { style: "width: 2rem; height: 2rem;" }
            Button { variant: ButtonVariant::Secondary, disabled: true,
                Spinner {}
                "Loading"
            }
        }
    }
}
