use dioxus::prelude::*;

use crate::pages::{Home, NotFound, Notes, Shell};

const THEME_CSS: Asset = asset!("/assets/dx-components-theme.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[layout(Shell)]
    #[route("/")]
    Home {},
    #[route("/notes")]
    Notes {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
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

    #[test]
    fn known_paths_resolve_to_their_pages() {
        assert_eq!("/".parse::<Route>().unwrap(), Route::Home {});
        assert_eq!("/notes".parse::<Route>().unwrap(), Route::Notes {});
    }

    #[test]
    fn unknown_paths_resolve_to_not_found() {
        let segments = vec!["missing".to_string(), "page".to_string()];
        assert_eq!(
            "/missing/page".parse::<Route>().unwrap(),
            Route::NotFound { segments }
        );
    }
}
