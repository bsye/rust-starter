use chrono::{DateTime, Utc};
use num_format::{Locale, ToFormattedString};

pub fn date(value: DateTime<Utc>) -> String {
    value.format("%-d %b %Y").to_string()
}

pub fn count(value: u64) -> String {
    value.to_formatted_string(&Locale::en)
}
