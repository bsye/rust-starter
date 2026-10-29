#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    pub id: usize,
    pub mine: bool,
    pub text: String,
}

pub const OPENING: [(bool, &str); 6] = [
    (true, "Why does my chat jump when a reply streams in?"),
    (
        false,
        "A plain overflow container keeps its scroll position while content grows below it, so new text lands out of view.",
    ),
    (true, "So how do I keep the latest message visible?"),
    (
        false,
        "Turn on auto scroll. The viewport follows the end while you are at the bottom and stops as soon as you scroll up.",
    ),
    (true, "And when I send a new question?"),
    (
        false,
        "Mark your turn as a scroll anchor. It settles near the top with the previous message peeking above, and the reply fills the space below.",
    ),
];

pub const OLDER: [(bool, &str); 4] = [
    (true, "Can I load older messages without losing my place?"),
    (
        false,
        "Yes. Prepended rows keep the first visible message where it was.",
    ),
    (true, "Even in browsers without native scroll anchoring?"),
    (
        false,
        "The scroller measures and corrects the position itself when the browser does not.",
    ),
];

pub const REPLY: &str = "Here is a longer reply so you can watch it fill the space under your question. Once it is taller than that space, the scroller hands back to following the end, exactly like a streamed answer would. Scroll up at any point and it leaves you alone; the button at the bottom brings you back.";

pub fn entries(lines: &[(bool, &str)], first_id: usize) -> Vec<Entry> {
    lines
        .iter()
        .enumerate()
        .map(|(index, (mine, text))| Entry {
            id: first_id + index,
            mine: *mine,
            text: text.to_string(),
        })
        .collect()
}
