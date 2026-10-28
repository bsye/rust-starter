use dioxus::prelude::*;
use dioxus::router::Navigator;

use crate::api::delete_note;
use crate::app::Route;

#[derive(Clone, Copy)]
pub struct DeleteNote {
    pub open: Signal<bool>,
    pub failure: Signal<Option<String>>,
    pub deleting: Signal<bool>,
    id: i64,
    nav: Navigator,
}

pub fn use_delete_note(id: i64) -> DeleteNote {
    DeleteNote {
        open: use_signal(|| false),
        failure: use_signal(|| None),
        deleting: use_signal(|| false),
        id,
        nav: navigator(),
    }
}

impl DeleteNote {
    pub async fn confirm(mut self) {
        if self.deleting.cloned() {
            return;
        }
        self.deleting.set(true);
        match delete_note(self.id).await {
            Ok(()) => {
                self.nav.push(Route::Notes { page: 1 });
            }
            Err(error) => {
                self.failure.set(Some(error.to_string()));
                self.deleting.set(false);
                self.open.set(false);
            }
        }
    }
}
