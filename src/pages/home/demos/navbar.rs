use dioxus::prelude::*;
use rust_starter::components::navbar::*;

use crate::app::Route;

#[component]
pub fn Demo() -> Element {
    rsx! {
        Navbar { aria_label: "Starter",
            NavbarNav { index: 0usize,
                NavbarTrigger { "Notes" }
                NavbarContent {
                    NavbarItem {
                        index: 0usize,
                        value: "notes".to_string(),
                        to: Route::Notes { page: 1 },
                        "All notes"
                    }
                    NavbarItem {
                        index: 1usize,
                        value: "new-note".to_string(),
                        to: Route::NewNote {},
                        "New note"
                    }
                }
            }
            NavbarNav { index: 1usize,
                NavbarTrigger { "Account" }
                NavbarContent {
                    NavbarItem {
                        index: 0usize,
                        value: "sign-in".to_string(),
                        to: Route::SignIn {},
                        "Sign in"
                    }
                    NavbarItem {
                        index: 1usize,
                        value: "sign-up".to_string(),
                        to: Route::SignUp {},
                        "Create account"
                    }
                }
            }
            NavbarItem {
                index: 2usize,
                value: "home".to_string(),
                to: Route::Home {},
                "Home"
            }
        }
    }
}
