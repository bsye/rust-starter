use dioxus::prelude::*;

use super::demos;

#[derive(Clone, Copy, PartialEq)]
pub enum Category {
    Forms,
    Navigation,
    Overlays,
    Feedback,
    Disclosure,
    DataDisplay,
}

impl Category {
    pub const ALL: [Category; 6] = [
        Category::Forms,
        Category::Navigation,
        Category::Overlays,
        Category::Feedback,
        Category::Disclosure,
        Category::DataDisplay,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Category::Forms => "Forms",
            Category::Navigation => "Navigation",
            Category::Overlays => "Overlays",
            Category::Feedback => "Feedback",
            Category::Disclosure => "Disclosure",
            Category::DataDisplay => "Data display",
        }
    }

    pub fn anchor(self) -> &'static str {
        match self {
            Category::Forms => "forms",
            Category::Navigation => "navigation",
            Category::Overlays => "overlays",
            Category::Feedback => "feedback",
            Category::Disclosure => "disclosure",
            Category::DataDisplay => "data-display",
        }
    }
}

pub struct Entry {
    pub id: &'static str,
    pub name: &'static str,
    pub category: Category,
    pub wide: bool,
    pub render: fn() -> Element,
}

macro_rules! catalog {
    ($($id:ident: $name:literal, $category:ident, $wide:literal;)*) => {
        pub const CATALOG: &[Entry] = &[$(Entry {
            id: stringify!($id),
            name: $name,
            category: Category::$category,
            wide: $wide,
            render: || rsx! { demos::$id::Demo {} },
        },)*];
    };
}

catalog! {
    button: "Button", Forms, false;
    input: "Input", Forms, false;
    textarea: "Textarea", Forms, false;
    label: "Label", Forms, false;
    checkbox: "Checkbox", Forms, false;
    switch: "Switch", Forms, false;
    radio_group: "Radio group", Forms, false;
    toggle: "Toggle", Forms, false;
    toggle_group: "Toggle group", Forms, false;
    select: "Select", Forms, false;
    combobox: "Combobox", Forms, false;
    slider: "Slider", Forms, false;
    date_picker: "Date picker", Forms, false;
    calendar: "Calendar", Forms, true;
    color_picker: "Color picker", Forms, true;
    navbar: "Navbar", Navigation, true;
    sidebar: "Sidebar", Navigation, true;
    menubar: "Menubar", Navigation, true;
    toolbar: "Toolbar", Navigation, true;
    tabs: "Tabs", Navigation, true;
    pagination: "Pagination", Navigation, false;
    dropdown_menu: "Dropdown menu", Navigation, false;
    context_menu: "Context menu", Navigation, false;
    dialog: "Dialog", Overlays, false;
    alert_dialog: "Alert dialog", Overlays, false;
    sheet: "Sheet", Overlays, false;
    popover: "Popover", Overlays, false;
    tooltip: "Tooltip", Overlays, false;
    hover_card: "Hover card", Overlays, false;
    toast: "Toast", Feedback, false;
    progress: "Progress", Feedback, false;
    skeleton: "Skeleton", Feedback, false;
    badge: "Badge", Feedback, false;
    accordion: "Accordion", Disclosure, false;
    collapsible: "Collapsible", Disclosure, false;
    card: "Card", DataDisplay, false;
    avatar: "Avatar", DataDisplay, false;
    separator: "Separator", DataDisplay, false;
    aspect_ratio: "Aspect ratio", DataDisplay, false;
    item: "Item", DataDisplay, false;
    scroll_area: "Scroll area", DataDisplay, false;
    drag_and_drop_list: "Drag and drop list", DataDisplay, true;
    virtual_list: "Virtual list", DataDisplay, true;
}

pub fn in_category(category: Category) -> impl Iterator<Item = &'static Entry> {
    CATALOG
        .iter()
        .filter(move |entry| entry.category == category)
}

pub fn find(id: &str) -> &'static Entry {
    CATALOG
        .iter()
        .find(|entry| entry.id == id)
        .unwrap_or_else(|| panic!("{id} is not in the component catalog"))
}

#[cfg(test)]
mod tests {
    use super::{CATALOG, Category, find, in_category};

    #[test]
    fn every_installed_component_is_shown_once() {
        let components = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/components");
        let mut installed: Vec<String> = std::fs::read_dir(components)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.is_dir())
            .map(|path| path.file_name().unwrap().to_string_lossy().into_owned())
            .collect();
        installed.sort();
        let mut shown: Vec<String> = CATALOG.iter().map(|entry| entry.id.to_string()).collect();
        shown.sort();
        assert_eq!(shown, installed);
    }

    #[test]
    fn every_category_has_components() {
        for category in Category::ALL {
            assert!(
                in_category(category).next().is_some(),
                "{} is empty",
                category.label()
            );
        }
    }

    #[test]
    #[should_panic(expected = "missing is not in the component catalog")]
    fn an_unknown_component_fails_loudly() {
        find("missing");
    }
}
