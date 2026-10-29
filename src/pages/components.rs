use dioxus::prelude::*;
use rust_starter::components::card::{Card, CardContent, CardHeader, CardTitle};

use crate::ui::PageHeader;

pub(super) mod catalog;
mod demos;

use catalog::Category;

#[component]
pub fn Components() -> Element {
    rsx! {
        PageHeader { title: "Components" }
        div { class: "lg:grid lg:grid-cols-[10rem_minmax(0,1fr)] lg:gap-10",
            Contents {}
            div { class: "flex min-w-0 flex-col gap-12",
                for category in Category::ALL {
                    section {
                        key: "{category.anchor()}",
                        id: category.anchor(),
                        class: "flex flex-col gap-4",
                        h2 { class: "text-xl font-semibold tracking-tight", "{category.label()}" }
                        div { class: "grid gap-4 md:grid-cols-2",
                            for entry in catalog::in_category(category) {
                                DemoCard { key: "{entry.id}", id: entry.id }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Contents() -> Element {
    rsx! {
        nav {
            aria_label: "Components",
            class: "mb-8 text-sm lg:sticky lg:top-6 lg:mb-0 lg:max-h-[calc(100vh-3rem)] lg:self-start lg:overflow-y-auto",
            ul { class: "flex flex-wrap gap-x-4 gap-y-2 lg:flex-col lg:gap-4",
                for category in Category::ALL {
                    li { key: "{category.anchor()}",
                        a {
                            class: "font-medium hover:text-foreground",
                            href: "#{category.anchor()}",
                            "{category.label()}"
                        }
                        ul { class: "mt-2 hidden flex-col gap-1.5 lg:flex",
                            for entry in catalog::in_category(category) {
                                li { key: "{entry.id}",
                                    a {
                                        class: "text-muted hover:text-foreground",
                                        href: "#{entry.id}",
                                        "{entry.name}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DemoCard(id: &'static str) -> Element {
    let entry = catalog::find(id);
    rsx! {
        div {
            id: entry.id,
            class: if entry.wide { "grid min-w-0 grid-cols-1 md:col-span-2" } else { "grid min-w-0 grid-cols-1" },
            Card {
                CardHeader {
                    CardTitle { "{entry.name}" }
                }
                CardContent {
                    div { class: "overflow-x-auto", {(entry.render)()} }
                }
            }
        }
    }
}
