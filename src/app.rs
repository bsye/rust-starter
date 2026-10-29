use dioxus::prelude::*;

use crate::hooks::use_current_user_provider;
use crate::pages::{
    EditNote, Home, NewNote, NotFound, NoteDetail, Notes, RequireUser, Shell, SignIn, SignUp,
};
use crate::ui::ComponentStyles;

const THEME_CSS: Asset = asset!("/assets/dx-components-theme.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[layout(Shell)]
    #[route("/")]
    Home {},
    #[route("/sign-in")]
    SignIn {},
    #[route("/sign-up")]
    SignUp {},
    #[layout(RequireUser)]
    #[route("/notes?:page")]
    Notes { page: u64 },
    #[route("/notes/new")]
    NewNote {},
    #[route("/notes/:id")]
    NoteDetail { id: i64 },
    #[route("/notes/:id/edit")]
    EditNote { id: i64 },
    #[end_layout]
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

impl Route {
    pub fn is_auth(&self) -> bool {
        matches!(self, Route::SignIn {} | Route::SignUp {})
    }

    pub fn in_notes(&self) -> bool {
        matches!(
            self,
            Route::Notes { .. }
                | Route::NewNote {}
                | Route::NoteDetail { .. }
                | Route::EditNote { .. }
        )
    }
}

#[component]
pub fn App() -> Element {
    use_current_user_provider();
    rsx! {
        document::Stylesheet { href: THEME_CSS }
        document::Stylesheet { href: TAILWIND_CSS }
        ComponentStyles {}
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
        assert_eq!(parse("/notes/42/edit"), Route::EditNote { id: 42 });
        assert_eq!(parse("/sign-in"), Route::SignIn {});
        assert_eq!(parse("/sign-up"), Route::SignUp {});
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
    fn sign_in_and_sign_up_are_auth_pages() {
        assert!(parse("/sign-in").is_auth());
        assert!(parse("/sign-up").is_auth());
        assert!(!parse("/notes").is_auth());
    }

    #[test]
    fn notes_pages_share_the_notes_section() {
        assert!(parse("/notes/new").in_notes());
        assert!(parse("/notes/7").in_notes());
        assert!(parse("/notes/7/edit").in_notes());
        assert!(!parse("/sign-in").in_notes());
        assert!(!parse("/").in_notes());
    }
}
