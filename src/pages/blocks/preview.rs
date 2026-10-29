use dioxus::prelude::*;
use dioxus_icons::lucide::{CodeXml, Eye, SquareArrowOutUpRight};

use crate::app::Route;
use crate::blocks::find;
use rust_starter::components::tabs::{TabList, TabTrigger, Tabs};

const FRAME: &str = "h-[44rem] w-full rounded-xl border border-border";
const PREVIEW: &str = "preview";
const CODE: &str = "code";

#[component]
pub fn BlockPreview(id: &'static str) -> Element {
    let block = find(id).unwrap_or_else(|| panic!("{id} is not in the block catalog"));
    let href = Route::BlockView { id: id.to_string() }.to_string();
    let mut view = use_signal(|| PREVIEW.to_string());
    let showing_code = view() == CODE;
    rsx! {
        section { id, class: "flex scroll-mt-6 flex-col gap-3",
            div { class: "flex flex-wrap items-center gap-x-4 gap-y-2",
                a { class: "text-sm font-medium hover:underline", href: "#{id}", "{block.description}" }
                div { class: "ml-auto flex items-center gap-2",
                    Tabs {
                        value: Some(view()),
                        on_value_change: move |value| view.set(value),
                        horizontal: true,
                        TabList {
                            TabTrigger { value: PREVIEW.to_string(), index: 0usize,
                                span { class: "flex items-center gap-1.5 [&_svg]:size-4",
                                    Eye {}
                                    "Preview"
                                }
                            }
                            TabTrigger { value: CODE.to_string(), index: 1usize,
                                span { class: "flex items-center gap-1.5 [&_svg]:size-4",
                                    CodeXml {}
                                    "Code"
                                }
                            }
                        }
                    }
                    a {
                        class: "flex size-9 items-center justify-center rounded-md text-muted hover:bg-surface hover:text-foreground [&_svg]:size-4",
                        href: "{href}",
                        target: "_blank",
                        title: "Open in a new tab",
                        aria_label: "Open {block.description} in a new tab",
                        SquareArrowOutUpRight {}
                    }
                }
            }
            iframe {
                class: FRAME,
                hidden: showing_code,
                title: "{block.description}",
                "loading": "lazy",
                src: "{href}",
            }
            pre {
                class: "{FRAME} overflow-auto bg-surface p-4 text-xs leading-relaxed",
                hidden: !showing_code,
                code { "{block.source}" }
            }
        }
    }
}
