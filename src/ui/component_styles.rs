use dioxus::prelude::*;

const COMPONENT_STYLES: [Asset; 8] = [
    asset!(
        "/src/components/alert_dialog/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/button/style.css",
        AssetOptions::css_module()
    ),
    asset!("/src/components/card/style.css", AssetOptions::css_module()),
    asset!(
        "/src/components/input/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/label/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/pagination/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/skeleton/style.css",
        AssetOptions::css_module()
    ),
    asset!(
        "/src/components/textarea/style.css",
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
