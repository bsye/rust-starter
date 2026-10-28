use dioxus::prelude::*;

use crate::pages::{Home, NewNote, NotFound, NoteDetail, Notes, Shell};

const THEME_CSS: Asset = asset!("/assets/dx-components-theme.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[layout(Shell)]
    #[route("/")]
    Home {},
    #[route("/notes?:page")]
    Notes { page: u64 },
    #[route("/notes/new")]
    NewNote {},
    #[route("/notes/:id")]
    NoteDetail { id: i64 },
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

impl Route {
    pub fn in_notes(&self) -> bool {
        matches!(
            self,
            Route::Notes { .. } | Route::NewNote {} | Route::NoteDetail { .. }
        )
    }
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Stylesheet { href: THEME_CSS }
        document::Stylesheet { href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[cfg(test)]
mod tests {
    use super::Route;

    fn parse(path: &str) -> Route {
        path.parse().unwrap()
    }

    #[test]
    fn known_paths_resolve_to_their_pages() {
        assert_eq!(parse("/"), Route::Home {});
        assert_eq!(parse("/notes?page=3"), Route::Notes { page: 3 });
        assert_eq!(parse("/notes/new"), Route::NewNote {});
        assert_eq!(parse("/notes/42"), Route::NoteDetail { id: 42 });
    }

    #[test]
    fn notes_without_a_page_start_at_zero() {
        assert_eq!(parse("/notes"), Route::Notes { page: 0 });
    }

    #[test]
    fn unknown_paths_resolve_to_not_found() {
        let segments = vec!["missing".to_string(), "page".to_string()];
        assert_eq!(parse("/missing/page"), Route::NotFound { segments });
    }

    #[test]
    fn notes_pages_share_the_notes_section() {
        assert!(parse("/notes/new").in_notes());
        assert!(parse("/notes/7").in_notes());
        assert!(!parse("/").in_notes());
    }
}
