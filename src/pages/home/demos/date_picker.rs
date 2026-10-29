use dioxus::prelude::*;
use rust_starter::components::date_picker::*;

use time::Date;

#[component]
pub fn Demo() -> Element {
    let mut selected_date = use_signal(|| None::<Date>);

    rsx! {
        div {
            DatePicker {
                selected_date: selected_date(),
                on_value_change: move |v| {
                    selected_date.set(v);
                },
            }
        }
    }
}
