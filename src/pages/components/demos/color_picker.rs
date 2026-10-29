use dioxus::prelude::*;

use dioxus_primitives::color_picker::Color;
use palette::{Hsv, IntoColor, encoding};
use rust_starter::components::color_picker::*;

#[component]
pub fn Demo() -> Element {
    let mut color = use_signal(|| -> Hsv<encoding::Srgb, f64> {
        Color::new(155, 128, 255).into_format::<f64>().into_color()
    });

    rsx! {
        ColorPicker {
            label: "Pick a color",
            color: color(),
            on_color_change: move |c| {
                color.set(c);
            },
        }
    }
}
