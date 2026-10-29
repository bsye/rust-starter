use dioxus::html::FileData;
use dioxus::prelude::*;

use super::data::{FileChip, ME, Post};
use crate::ui::format;

#[derive(Clone, Copy, PartialEq)]
pub struct Draft {
    pub text: Signal<String>,
    pub files: Signal<Vec<FileChip>>,
    thread: Signal<Vec<Post>>,
}

pub fn use_draft(thread: Signal<Vec<Post>>) -> Draft {
    Draft {
        text: use_signal(String::new),
        files: use_signal(Vec::new),
        thread,
    }
}

impl Draft {
    pub fn ready(&self) -> bool {
        !self.text.read().trim().is_empty() || !self.files.read().is_empty()
    }

    pub fn attach(mut self, picked: Vec<FileData>) {
        self.files
            .write()
            .extend(picked.into_iter().map(|file| FileChip {
                detail: format::file_detail(&file.name(), file.size()),
                name: file.name(),
            }));
    }

    pub fn remove(mut self, index: usize) {
        self.files.write().remove(index);
    }

    pub fn send(mut self) {
        if !self.ready() {
            return;
        }
        let id = self.thread.read().len();
        let post = Post {
            id,
            author: ME.0,
            initials: ME.1,
            mine: true,
            text: self.text.read().trim().to_string(),
            files: self.files.take(),
        };
        self.thread.write().push(post);
        self.text.set(String::new());
    }
}
