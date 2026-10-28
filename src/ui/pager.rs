use dioxus::prelude::*;

use crate::app::Route;
use crate::components::pagination::{
    Pagination, PaginationContent, PaginationItem, PaginationNext, PaginationPrevious,
};

pub fn neighbours(page: u64, page_count: u64) -> (Option<u64>, Option<u64>) {
    let previous = (page > 1 && page_count > 0).then(|| (page - 1).min(page_count));
    let next = (page < page_count).then(|| page + 1);
    (previous, next)
}

#[component]
pub fn Pager(summary: String, previous: Option<Route>, next: Option<Route>) -> Element {
    let nav = navigator();
    rsx! {
        div { class: "mt-6 flex flex-col items-center gap-3 sm:flex-row sm:justify-between",
            p { class: "whitespace-nowrap text-sm text-muted tabular-nums", "{summary}" }
            Pagination {
                PaginationContent {
                    if let Some(route) = previous {
                        PaginationItem {
                            PaginationPrevious {
                                href: route.to_string(),
                                onclick: move |event: MouseEvent| {
                                    event.prevent_default();
                                    nav.push(route.clone());
                                },
                            }
                        }
                    }
                    if let Some(route) = next {
                        PaginationItem {
                            PaginationNext {
                                href: route.to_string(),
                                onclick: move |event: MouseEvent| {
                                    event.prevent_default();
                                    nav.push(route.clone());
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::neighbours;

    #[test]
    fn middle_page_has_both_neighbours() {
        assert_eq!(neighbours(3, 5), (Some(2), Some(4)));
    }

    #[test]
    fn first_and_last_pages_have_one_neighbour() {
        assert_eq!(neighbours(1, 5), (None, Some(2)));
        assert_eq!(neighbours(5, 5), (Some(4), None));
    }

    #[test]
    fn page_past_the_end_points_back_to_the_last_page() {
        assert_eq!(neighbours(9, 5), (Some(5), None));
    }

    #[test]
    fn single_page_has_no_neighbours() {
        assert_eq!(neighbours(1, 1), (None, None));
    }
}
