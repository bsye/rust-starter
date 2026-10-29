use dioxus::prelude::*;

const COMPONENT_STYLES: [Asset; 43] = [
    asset!(
        "/src/components/accordion/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/alert_dialog/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/aspect_ratio/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/avatar/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/badge/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/button/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/calendar/style.css",
        AssetOptions::css_module()
    ),
    asset!("/src/components/card/style.css", AssetOptions::css_module()),
    asset!(
        "/src/components/checkbox/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/collapsible/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/color_picker/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/combobox/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/context_menu/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/date_picker/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/dialog/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/drag_and_drop_list/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/dropdown_menu/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/hover_card/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/input/style.css",
        AssetOptions::css_module()
    ),
    asset!("/src/components/item/style.css", AssetOptions::css_module()),
    asset!(
        "/src/components/label/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/menubar/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/navbar/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/pagination/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/popover/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/progress/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/radio_group/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/scroll_area/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/select/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/separator/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/sheet/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/sidebar/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/skeleton/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/slider/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/switch/style.css",
        AssetOptions::css_module()
    ),
    asset!("/src/components/tabs/style.css", AssetOptions::css_module()),
    asset!(
        "/src/components/textarea/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/toast/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/toggle/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/toggle_group/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/toolbar/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/tooltip/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/virtual_list/style.css",
        AssetOptions::css_module()
    ),
];

#[component]
pub fn ComponentStyles() -> Element {
    rsx! {
        for href in COMPONENT_STYLES {
            document::Stylesheet { href }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn every_generated_component_stylesheet_is_linked() {
        let source = include_str!("component_styles.rs");
        let components = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/components");
        for entry in std::fs::read_dir(components).unwrap() {
            let path = entry.unwrap().path();
            if path.join("style.css").exists() {
                let name = path.file_name().unwrap().to_string_lossy();
                let expected = format!("\"/src/components/{name}/style.css\"");
                assert!(
                    source.contains(&expected),
                    "add {expected} to COMPONENT_STYLES"
                );
            }
        }
    }
}
