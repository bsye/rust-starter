use dioxus::prelude::*;

use rust_starter::components::card::{Card, CardContent, CardHeader};
use rust_starter::components::skeleton::Skeleton;

const SKELETON_CARDS: usize = 4;

#[component]
pub fn NoteListSkeleton() -> Element {
    rsx! {
        div { class: "grid gap-3",
            for index in 0..SKELETON_CARDS {
                Card { key: "{index}",
                    CardHeader {
                        Skeleton { class: "h-5 w-48" }
                        Skeleton { class: "h-4 w-24" }
                    }
                    CardContent {
                        Skeleton { class: "h-4 w-full" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn NoteDetailSkeleton() -> Element {
    rsx! {
        div { class: "mb-6 flex items-center justify-between gap-4",
            Skeleton { class: "h-8 w-64" }
            Skeleton { class: "h-9 w-28" }
        }
        Skeleton { class: "mb-4 h-4 w-24" }
        Card {
            CardContent {
                Skeleton { class: "h-4 w-full" }
            }
        }
    }
}
