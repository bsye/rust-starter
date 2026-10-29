use dioxus::prelude::*;

use super::composer::Composer;
use super::data::{THREAD_DETAIL, THREAD_TITLE, opening};
use super::draft::use_draft;
use super::post::PostView;
use rust_starter::components::marker::{Marker, MarkerContent, MarkerVariant};
use rust_starter::components::message_scroller::{
    MessageScroller, MessageScrollerButton, MessageScrollerContent, MessageScrollerItem,
    MessageScrollerProvider, MessageScrollerViewport,
};

#[component]
pub fn Chat01() -> Element {
    let thread = use_signal(opening);
    let draft = use_draft(thread);
    rsx! {
        div { class: "flex h-svh flex-col bg-background",
            header { class: "flex h-14 shrink-0 items-center gap-3 border-b border-border px-4",
                div { class: "flex min-w-0 flex-col",
                    h1 { class: "truncate text-sm font-semibold", "{THREAD_TITLE}" }
                    p { class: "truncate text-xs text-muted", "{THREAD_DETAIL}" }
                }
            }
            div { class: "min-h-0 flex-1",
                MessageScrollerProvider { auto_scroll: true,
                    MessageScroller {
                        MessageScrollerViewport {
                            MessageScrollerContent { class: "mx-auto w-full max-w-3xl p-4",
                                MessageScrollerItem {
                                    Marker { variant: MarkerVariant::Separator,
                                        MarkerContent { "Today" }
                                    }
                                }
                                for post in thread() {
                                    MessageScrollerItem { key: "{post.id}", message_id: "{post.id}",
                                        PostView { post: post.clone() }
                                    }
                                }
                            }
                        }
                        MessageScrollerButton {}
                    }
                }
            }
            div { class: "mx-auto w-full max-w-3xl shrink-0 p-4 pt-0",
                Composer { draft }
            }
        }
    }
}
