use dioxus::prelude::*;
use dioxus_icons::lucide::ArrowRight;
use rust_starter::components::card::{Card, CardContent, CardFooter};

use crate::app::Route;

#[component]
pub fn Destination(to: Route, title: &'static str, summary: String, children: Element) -> Element {
    rsx! {
        div { class: "group relative grid",
            Card {
                CardContent {
                    div {
                        class: "h-[40svh] overflow-hidden rounded-xl border border-border bg-background",
                        "inert": true,
                        aria_hidden: "true",
                        {children}
                    }
                }
                CardFooter {
                    div { class: "flex w-full items-center justify-between gap-4",
                        div { class: "flex flex-col gap-1",
                            Link {
                                class: "font-semibold after:absolute after:inset-0 after:rounded-2xl group-hover:underline",
                                to,
                                "{title}"
                            }
                            span { class: "text-sm text-muted tabular-nums", "{summary}" }
                        }
                        span { class: "text-muted transition-colors group-hover:text-foreground [&_svg]:size-5",
                            ArrowRight {}
                        }
                    }
                }
            }
        }
    }
}
