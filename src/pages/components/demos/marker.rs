use dioxus::prelude::*;
use dioxus_icons::lucide::{GitBranch, Search};

use rust_starter::components::marker::*;
use rust_starter::components::spinner::Spinner;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; width: 100%; max-width: 24rem; flex-direction: column; gap: 2rem;",
            Marker {
                MarkerIcon { GitBranch {} }
                MarkerContent { "Switched to a new branch" }
            }
            Marker { role: "status",
                MarkerIcon { Spinner {} }
                MarkerContent { shimmer: true, "Thinking..." }
            }
            Marker { variant: MarkerVariant::Separator,
                MarkerContent { "Conversation compacted" }
            }
            Marker { variant: MarkerVariant::Border,
                MarkerIcon { Search {} }
                MarkerContent { "Explored 4 files" }
            }
        }
    }
}
