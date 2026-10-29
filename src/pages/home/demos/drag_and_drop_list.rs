use dioxus::prelude::*;
use rust_starter::components::avatar::{Avatar, AvatarFallback, AvatarImageSize};
use rust_starter::components::drag_and_drop_list::*;

struct Task {
    title: &'static str,
    status: &'static str,
    assignee: &'static str,
}

const TASKS: [Task; 4] = [
    Task {
        title: "Ship the roadmap",
        status: "In progress",
        assignee: "AL",
    },
    Task {
        title: "Redesign onboarding",
        status: "In review",
        assignee: "GH",
    },
    Task {
        title: "Audit webhook logs",
        status: "To do",
        assignee: "AT",
    },
    Task {
        title: "Draft the changelog",
        status: "Blocked",
        assignee: "KJ",
    },
];

#[component]
pub fn Demo() -> Element {
    let items: Vec<Element> = TASKS
        .iter()
        .map(|task| {
            rsx! {
                div { key: "{task.title}", class: "flex w-full min-w-0 items-center gap-3",
                    div { class: "flex min-w-0 flex-1 flex-col",
                        span { class: "truncate text-sm font-medium", "{task.title}" }
                        span { class: "text-xs text-muted", "{task.status}" }
                    }
                    Avatar { size: AvatarImageSize::Small, aria_label: "Assignee {task.assignee}",
                        AvatarFallback { "{task.assignee}" }
                    }
                }
            }
        })
        .collect();
    rsx! {
        DragAndDropList { items }
    }
}
