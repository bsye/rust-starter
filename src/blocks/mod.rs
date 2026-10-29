use dioxus::prelude::*;

mod auth;
mod chat;
mod login;
mod sidebar;
mod signup;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Category {
    Featured,
    Sidebar,
    Login,
    Signup,
    Chat,
}

impl Category {
    pub const ALL: [Category; 5] = [
        Category::Featured,
        Category::Sidebar,
        Category::Login,
        Category::Signup,
        Category::Chat,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Category::Featured => "Featured",
            Category::Sidebar => "Sidebar",
            Category::Login => "Login",
            Category::Signup => "Signup",
            Category::Chat => "Chat",
        }
    }

    pub fn slug(self) -> &'static str {
        match self {
            Category::Featured => "",
            Category::Sidebar => "sidebar",
            Category::Login => "login",
            Category::Signup => "signup",
            Category::Chat => "chat",
        }
    }

    pub fn from_slug(slug: &str) -> Option<Category> {
        Category::ALL
            .into_iter()
            .find(|category| category.slug() == slug)
    }
}

pub struct Block {
    pub id: &'static str,
    pub description: &'static str,
    pub category: Category,
    pub featured: bool,
    pub render: fn() -> Element,
    pub source: &'static str,
}

macro_rules! blocks {
    ($($id:literal: $module:ident::$file:ident::$component:ident, $category:ident, $featured:literal, $description:literal;)*) => {
        pub const BLOCKS: &[Block] = &[$(Block {
            id: $id,
            description: $description,
            category: Category::$category,
            featured: $featured,
            render: || rsx! { $module::$component {} },
            source: include_str!(concat!(stringify!($module), "/", stringify!($file), ".rs")),
        },)*];
    };
}

blocks! {
    "sidebar-01": sidebar::sidebar_01::Sidebar01, Sidebar, false, "A simple sidebar with navigation grouped by section.";
    "sidebar-02": sidebar::sidebar_02::Sidebar02, Sidebar, false, "A sidebar with collapsible sections.";
    "sidebar-03": sidebar::sidebar_03::Sidebar03, Sidebar, true, "A sidebar with submenus.";
    "sidebar-04": sidebar::sidebar_04::Sidebar04, Sidebar, false, "A floating sidebar with submenus.";
    "sidebar-05": sidebar::sidebar_05::Sidebar05, Sidebar, false, "A sidebar with collapsible submenus.";
    "sidebar-06": sidebar::sidebar_06::Sidebar06, Sidebar, false, "A sidebar with submenus as dropdowns.";
    "sidebar-07": sidebar::sidebar_07::Sidebar07, Sidebar, true, "A sidebar that collapses to icons.";
    "sidebar-08": sidebar::sidebar_08::Sidebar08, Sidebar, false, "An inset sidebar with secondary navigation.";
    "sidebar-11": sidebar::sidebar_11::Sidebar11, Sidebar, false, "A sidebar with a collapsible file tree.";
    "sidebar-12": sidebar::sidebar_12::Sidebar12, Sidebar, false, "A sidebar with a calendar.";
    "sidebar-13": sidebar::sidebar_13::Sidebar13, Sidebar, false, "A sidebar in a dialog.";
    "sidebar-14": sidebar::sidebar_14::Sidebar14, Sidebar, false, "A sidebar on the right.";
    "sidebar-16": sidebar::sidebar_16::Sidebar16, Sidebar, false, "A sidebar with a sticky site header.";
    "login-01": login::login_01::Login01, Login, false, "A simple sign-in form.";
    "login-02": login::login_02::Login02, Login, false, "A two column sign-in page with a cover.";
    "login-03": login::login_03::Login03, Login, true, "A sign-in page with a muted background.";
    "login-04": login::login_04::Login04, Login, true, "A sign-in page with a form and a cover.";
    "login-05": login::login_05::Login05, Login, false, "A sign-in page with the brand on top.";
    "signup-01": signup::signup_01::Signup01, Signup, false, "A simple sign-up form.";
    "signup-02": signup::signup_02::Signup02, Signup, false, "A two column sign-up page with a cover.";
    "signup-03": signup::signup_03::Signup03, Signup, false, "A sign-up page with a muted background.";
    "signup-04": signup::signup_04::Signup04, Signup, false, "A sign-up page with a form and a cover.";
    "signup-05": signup::signup_05::Signup05, Signup, false, "A sign-up page with social providers.";
    "chat-01": chat::chat_01::Chat01, Chat, true, "A chat thread with attachments and a composer.";
}

pub const SHOWCASE: &str = "sidebar-07";

pub fn in_category(category: Category) -> impl Iterator<Item = &'static Block> {
    BLOCKS.iter().filter(move |block| {
        block.category == category || (category == Category::Featured && block.featured)
    })
}

pub fn find(id: &str) -> Option<&'static Block> {
    BLOCKS.iter().find(|block| block.id == id)
}

#[cfg(test)]
mod tests {
    use super::{BLOCKS, Category, SHOWCASE, find, in_category};

    #[test]
    fn block_ids_are_unique() {
        let mut ids: Vec<&str> = BLOCKS.iter().map(|block| block.id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), BLOCKS.len());
    }

    #[test]
    fn every_category_has_blocks() {
        for category in Category::ALL {
            assert!(
                in_category(category).next().is_some(),
                "{} is empty",
                category.label()
            );
        }
    }

    #[test]
    fn featured_lists_only_featured_blocks() {
        assert!(in_category(Category::Featured).all(|block| block.featured));
    }

    #[test]
    fn each_block_shows_the_file_that_renders_it() {
        for block in BLOCKS {
            let component = block.id.replace('-', "");
            assert!(
                block
                    .source
                    .to_lowercase()
                    .contains(&format!("pub fn {component}()")),
                "{} shows the wrong source",
                block.id
            );
        }
    }

    #[test]
    fn slugs_round_trip_and_unknown_slugs_are_rejected() {
        for category in Category::ALL {
            assert_eq!(Category::from_slug(category.slug()), Some(category));
        }
        assert_eq!(Category::from_slug("missing"), None);
    }

    #[test]
    fn unknown_blocks_are_not_found() {
        assert!(find(SHOWCASE).is_some());
        assert!(find("missing").is_none());
    }
}
