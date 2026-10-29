use dioxus::prelude::*;
use rust_starter::components::accordion::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger,
};

const QUESTIONS: [(&str, &str); 3] = [
    (
        "Is it accessible?",
        "Yes. It follows the WAI-ARIA disclosure pattern.",
    ),
    (
        "Is it styled?",
        "Yes. It uses the theme's tokens, like every other component.",
    ),
    (
        "Is it animated?",
        "Yes. Sections open and close with a short transition.",
    ),
];

#[component]
pub fn Demo() -> Element {
    rsx! {
        Accordion { allow_multiple_open: false, horizontal: false,
            for (index , (question , answer)) in QUESTIONS.into_iter().enumerate() {
                AccordionItem { index,
                    AccordionTrigger { "{question}" }
                    AccordionContent {
                        p { class: "pb-4 text-sm", "{answer}" }
                    }
                }
            }
        }
    }
}
