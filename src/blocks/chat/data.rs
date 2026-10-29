#[derive(Clone, Debug, PartialEq)]
pub struct FileChip {
    pub name: String,
    pub detail: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Post {
    pub id: usize,
    pub author: &'static str,
    pub initials: &'static str,
    pub mine: bool,
    pub text: String,
    pub files: Vec<FileChip>,
}

pub const THREAD_TITLE: &str = "Launch planning";
pub const THREAD_DETAIL: &str = "Acme Inc · 3 teams";
pub const ME: (&str, &str) = ("You", "YO");

struct Opening {
    author: &'static str,
    initials: &'static str,
    mine: bool,
    text: &'static str,
    file: Option<(&'static str, &'static str)>,
}

const OPENING: [Opening; 6] = [
    Opening {
        author: "Design team",
        initials: "DT",
        mine: false,
        text: "The launch page mockups are ready for review. The hero and pricing sections changed the most.",
        file: Some(("launch-page.pdf", "PDF · 2.4 MB")),
    },
    Opening {
        author: "You",
        initials: "YO",
        mine: true,
        text: "Thanks. I will go through them this afternoon and leave notes on each section.",
        file: None,
    },
    Opening {
        author: "Engineering team",
        initials: "ET",
        mine: false,
        text: "The pricing table needs a new endpoint for regional prices. We can ship it behind a flag on Monday.",
        file: None,
    },
    Opening {
        author: "You",
        initials: "YO",
        mine: true,
        text: "Monday works. Can we keep the old table until the flag is on everywhere?",
        file: None,
    },
    Opening {
        author: "Engineering team",
        initials: "ET",
        mine: false,
        text: "Yes, both stay in place until then. Here is the rollout checklist.",
        file: Some(("rollout-checklist.md", "MD · 4 KB")),
    },
    Opening {
        author: "Design team",
        initials: "DT",
        mine: false,
        text: "Updated the hero copy to match. Shout if anything reads off.",
        file: None,
    },
];

pub fn opening() -> Vec<Post> {
    OPENING
        .iter()
        .enumerate()
        .map(|(id, line)| Post {
            id,
            author: line.author,
            initials: line.initials,
            mine: line.mine,
            text: line.text.to_string(),
            files: line
                .file
                .map(|(name, detail)| FileChip {
                    name: name.to_string(),
                    detail: detail.to_string(),
                })
                .into_iter()
                .collect(),
        })
        .collect()
}
