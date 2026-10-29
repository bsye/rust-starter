use dioxus::prelude::*;

use super::components::catalog::CATALOG;
use crate::app::Route;
use crate::blocks::{BLOCKS, SHOWCASE, find};

mod destination;
mod sampler;

use destination::Destination;
use sampler::Sampler;

#[component]
pub fn Home() -> Element {
    let showcase =
        find(SHOWCASE).unwrap_or_else(|| panic!("{SHOWCASE} is not in the block catalog"));
    rsx! {
        section { class: "flex flex-col items-center gap-4 py-12 text-center md:py-20",
            h1 { class: "text-4xl font-semibold tracking-tight md:text-5xl", "Rust starter" }
            p { class: "max-w-xl text-lg text-balance text-muted",
                "A Dioxus fullstack scaffold: Rust on the server and in the browser, with shadcn-style components and blocks."
            }
        }
        div { class: "grid gap-6 md:grid-cols-2",
            Destination {
                to: Route::Blocks { category: String::new() },
                title: "Blocks",
                summary: format!("{} pages", BLOCKS.len()),
                div { class: "h-svh w-[250%] origin-top-left scale-40", {(showcase.render)()} }
            }
            Destination {
                to: Route::Components {},
                title: "Components",
                summary: format!("{} components", CATALOG.len()),
                Sampler {}
            }
        }
    }
}
