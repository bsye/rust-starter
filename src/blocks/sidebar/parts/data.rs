use dioxus::prelude::*;
use dioxus_icons::lucide::{
    AudioWaveform, BookOpen, Bot, ChartPie, Command, Frame, GalleryVerticalEnd, LifeBuoy, Map,
    Send, Settings2, SquareTerminal,
};

pub struct DocSection {
    pub title: &'static str,
    pub links: &'static [DocLink],
}

pub struct DocLink {
    pub title: &'static str,
    pub active: bool,
}

pub struct AppSection {
    pub title: &'static str,
    pub icon: fn() -> Element,
    pub open: bool,
    pub links: &'static [&'static str],
}

pub struct Team {
    pub name: &'static str,
    pub plan: &'static str,
    pub icon: fn() -> Element,
}

pub struct Project {
    pub name: &'static str,
    pub icon: fn() -> Element,
}

pub struct Shortcut {
    pub title: &'static str,
    pub icon: fn() -> Element,
}

pub struct User {
    pub name: &'static str,
    pub email: &'static str,
    pub initials: &'static str,
}

const fn link(title: &'static str) -> DocLink {
    DocLink {
        title,
        active: false,
    }
}

pub const VERSIONS: [&str; 3] = ["1.0.1", "1.1.0-alpha", "2.0.0-beta1"];

pub const CURRENT_SECTION: &str = "Build your application";
pub const CURRENT_PAGE: &str = "Data fetching";

pub const DOCS: &[DocSection] = &[
    DocSection {
        title: "Getting started",
        links: &[link("Installation"), link("Project structure")],
    },
    DocSection {
        title: CURRENT_SECTION,
        links: &[
            link("Routing"),
            DocLink {
                title: CURRENT_PAGE,
                active: true,
            },
            link("Rendering"),
            link("Caching"),
            link("Styling"),
            link("Optimizing"),
            link("Configuring"),
            link("Testing"),
            link("Authentication"),
            link("Deploying"),
            link("Upgrading"),
            link("Examples"),
        ],
    },
    DocSection {
        title: "API reference",
        links: &[
            link("Components"),
            link("File conventions"),
            link("Functions"),
            link("Configuration options"),
            link("CLI"),
            link("Edge runtime"),
        ],
    },
    DocSection {
        title: "Architecture",
        links: &[
            link("Accessibility"),
            link("Fast refresh"),
            link("Compiler"),
            link("Supported browsers"),
            link("Bundler"),
        ],
    },
    DocSection {
        title: "Community",
        links: &[link("Contribution guide")],
    },
];

pub const APP_SECTIONS: &[AppSection] = &[
    AppSection {
        title: "Playground",
        icon: || rsx! { SquareTerminal {} },
        open: true,
        links: &["History", "Starred", "Settings"],
    },
    AppSection {
        title: "Models",
        icon: || rsx! { Bot {} },
        open: false,
        links: &["Genesis", "Explorer", "Quantum"],
    },
    AppSection {
        title: "Documentation",
        icon: || rsx! { BookOpen {} },
        open: false,
        links: &["Introduction", "Get started", "Tutorials", "Changelog"],
    },
    AppSection {
        title: "Settings",
        icon: || rsx! { Settings2 {} },
        open: false,
        links: &["General", "Team", "Billing", "Limits"],
    },
];

pub const TEAMS: &[Team] = &[
    Team {
        name: "Acme Inc",
        plan: "Enterprise",
        icon: || rsx! { GalleryVerticalEnd {} },
    },
    Team {
        name: "Acme Corp",
        plan: "Startup",
        icon: || rsx! { AudioWaveform {} },
    },
    Team {
        name: "Acme Labs",
        plan: "Free",
        icon: || rsx! { Command {} },
    },
];

pub const PROJECTS: &[Project] = &[
    Project {
        name: "Design engineering",
        icon: || rsx! { Frame {} },
    },
    Project {
        name: "Sales and marketing",
        icon: || rsx! { ChartPie {} },
    },
    Project {
        name: "Travel",
        icon: || rsx! { Map {} },
    },
];

pub const SECONDARY: &[Shortcut] = &[
    Shortcut {
        title: "Support",
        icon: || rsx! { LifeBuoy {} },
    },
    Shortcut {
        title: "Feedback",
        icon: || rsx! { Send {} },
    },
];

pub const USER: User = User {
    name: "Demo user",
    email: "demo@example.com",
    initials: "DU",
};
