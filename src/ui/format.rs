use chrono::{DateTime, Utc};
use num_format::{Locale, ToFormattedString};

pub fn date(value: DateTime<Utc>) -> String {
    value.format("%-d %b %Y").to_string()
}

pub fn count(value: u64) -> String {
    value.to_formatted_string(&Locale::en)
}

const KILOBYTE: u64 = 1_000;
const MEGABYTE: u64 = 1_000_000;

pub fn file_size(bytes: u64) -> String {
    if bytes < KILOBYTE {
        format!("{bytes} B")
    } else if bytes < MEGABYTE {
        format!("{} KB", (bytes + KILOBYTE / 2) / KILOBYTE)
    } else {
        format!("{:.1} MB", bytes as f64 / MEGABYTE as f64)
    }
}

pub fn file_detail(name: &str, bytes: u64) -> String {
    match name.rsplit_once('.') {
        Some((stem, extension)) if !stem.is_empty() && !extension.is_empty() => {
            format!("{} · {}", extension.to_uppercase(), file_size(bytes))
        }
        _ => file_size(bytes),
    }
}

#[cfg(test)]
mod tests {
    use super::{file_detail, file_size};

    #[test]
    fn file_sizes_use_the_largest_unit_below_a_thousand() {
        assert_eq!(file_size(0), "0 B");
        assert_eq!(file_size(999), "999 B");
        assert_eq!(file_size(1_000), "1 KB");
        assert_eq!(file_size(1_499), "1 KB");
        assert_eq!(file_size(1_500), "2 KB");
        assert_eq!(file_size(2_400_000), "2.4 MB");
    }

    #[test]
    fn a_file_detail_names_the_type_from_the_extension() {
        assert_eq!(file_detail("plan.final.pdf", 2_400_000), "PDF · 2.4 MB");
        assert_eq!(file_detail("README", 500), "500 B");
        assert_eq!(file_detail(".env", 12), "12 B");
        assert_eq!(file_detail("notes.", 12), "12 B");
    }
}
