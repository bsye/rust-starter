use dioxus::prelude::*;
use dioxus_icons::lucide::{Plus, Share2};
use rust_starter::components::{
    badge::{Badge, BadgeVariant},
    button::{Button, ButtonVariant},
    checkbox::Checkbox,
    input::Input,
    label::Label,
    progress::Progress,
    slider::Slider,
    switch::Switch,
};

#[component]
pub fn Sampler() -> Element {
    rsx! {
        div { class: "flex h-full flex-col justify-center gap-5 p-6",
            div { class: "flex flex-wrap items-center gap-2",
                Button {
                    Plus {}
                    "New"
                }
                Button { variant: ButtonVariant::Outline,
                    Share2 {}
                    "Share"
                }
                Badge { variant: BadgeVariant::Secondary, "Beta" }
            }
            Input { placeholder: "m@example.com", aria_label: "Email" }
            div { class: "flex items-center justify-between gap-4",
                div { class: "flex items-center gap-2",
                    Switch { id: "sampler-notifications", default_checked: true }
                    Label { html_for: "sampler-notifications", "Notifications" }
                }
                div { class: "flex items-center gap-2",
                    Checkbox { id: "sampler-remember" }
                    Label { html_for: "sampler-remember", "Remember me" }
                }
            }
            Slider { label: "Volume", horizontal: true, default_value: 60.0 }
            Progress { aria_label: "Upload", value: 66.0 }
        }
    }
}
