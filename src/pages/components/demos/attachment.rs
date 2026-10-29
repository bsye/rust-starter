use dioxus::prelude::*;
use dioxus_icons::lucide::{FileCode, FileImage, X};

use rust_starter::components::attachment::*;
use rust_starter::components::spinner::Spinner;

const IMAGES: [(&str, &str); 3] = [
    ("workspace.png", "PNG · 820 KB"),
    ("desk-reference.jpg", "JPG · 1.1 MB"),
    ("office-reference.jpg", "JPG · 940 KB"),
];

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; width: 100%; max-width: 24rem; flex-direction: column; gap: 0.75rem;",
            AttachmentGroup {
                for (name, meta) in IMAGES {
                    Attachment { key: "{name}", orientation: AttachmentOrientation::Vertical,
                        AttachmentMedia { variant: AttachmentMediaVariant::Image, FileImage {} }
                        AttachmentContent {
                            AttachmentTitle { "{name}" }
                            AttachmentDescription { "{meta}" }
                        }
                    }
                }
            }
            Attachment { state: AttachmentState::Uploading, style: "width: 100%;",
                AttachmentMedia { Spinner {} }
                AttachmentContent {
                    AttachmentTitle { "sales-dashboard.pdf" }
                    AttachmentDescription { "Uploading · 64%" }
                }
                AttachmentActions {
                    AttachmentAction { aria_label: "Cancel upload", X {} }
                }
            }
            Attachment { style: "width: 100%;",
                AttachmentMedia { FileCode {} }
                AttachmentContent {
                    AttachmentTitle { "message-renderer.rs" }
                    AttachmentDescription { "Rust · 12 KB" }
                }
                AttachmentActions {
                    AttachmentAction { aria_label: "Remove message-renderer.rs", X {} }
                }
            }
        }
    }
}
