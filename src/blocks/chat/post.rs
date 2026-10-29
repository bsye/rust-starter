use dioxus::prelude::*;
use dioxus_icons::lucide::FileText;

use super::data::{FileChip, Post};
use rust_starter::components::attachment::{
    Attachment, AttachmentContent, AttachmentDescription, AttachmentMedia, AttachmentSize,
    AttachmentTitle,
};
use rust_starter::components::bubble::{Bubble, BubbleContent, BubbleVariant};
use rust_starter::components::message::{
    Message, MessageAlign, MessageAvatar, MessageContent, MessageHeader,
};

#[component]
pub fn PostView(post: Post) -> Element {
    rsx! {
        Message { align: if post.mine { MessageAlign::End } else { MessageAlign::Start },
            MessageAvatar { "{post.initials}" }
            MessageContent {
                MessageHeader { "{post.author}" }
                if !post.text.is_empty() {
                    Bubble { variant: if post.mine { BubbleVariant::Default } else { BubbleVariant::Muted },
                        BubbleContent { "{post.text}" }
                    }
                }
                for (index, file) in post.files.iter().enumerate() {
                    FileView { key: "{index}", file: file.clone() }
                }
            }
        }
    }
}

#[component]
pub fn FileView(file: FileChip, #[props(default)] size: AttachmentSize) -> Element {
    rsx! {
        Attachment { size,
            AttachmentMedia { FileText {} }
            AttachmentContent {
                AttachmentTitle { "{file.name}" }
                AttachmentDescription { "{file.detail}" }
            }
        }
    }
}
