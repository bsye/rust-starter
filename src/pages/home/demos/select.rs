use dioxus::prelude::*;
use rust_starter::components::select::*;

const FRUITS: [&str; 5] = ["Apple", "Banana", "Blueberry", "Grapes", "Pineapple"];

#[component]
pub fn Demo() -> Element {
    rsx! {
        Select::<Option<&'static str>> { width: "12rem",
            SelectGroup {
                SelectGroupLabel { "Fruits" }
                for (index , fruit) in FRUITS.into_iter().enumerate() {
                    SelectOption::<Option<&'static str>> {
                        index,
                        value: fruit,
                        text_value: fruit,
                        disabled: fruit == "Grapes",
                        "{fruit}"
                    }
                }
            }
        }
    }
}
