use dioxus::prelude::*;

use crate::api::Note;
use crate::app::Route;
use crate::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::ui::format;

const CARD_LINK: &str = "block rounded-2xl outline-offset-2 hover:outline-2 hover:outline-border focus-visible:outline-2 focus-visible:outline-border";

#[component]
pub fn NoteCard(note: Note) -> Element {
    rsx! {
        Link { class: CARD_LINK, to: Route::NoteDetail { id: note.id },
            Card {
                CardHeader {
                    CardTitle {
                        span { class: "break-words", "{note.title}" }
                    }
                    CardDescription { {format::date(note.created_at)} }
                }
                if !note.body.is_empty() {
                    CardContent {
                        p { class: "line-clamp-2 break-words text-muted", "{note.body}" }
                    }
                }
            }
        }
    }
}
