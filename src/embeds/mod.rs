mod project;
mod user;

use time::OffsetDateTime;
use twilight_mention::{
    timestamp::{Timestamp, TimestampStyle},
    Mention,
};

pub use project::Project;
use twilight_validate::embed::FIELD_VALUE_LENGTH;
pub use user::User;

pub enum Color {
    Error,
    Success,
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        match self {
            Self::Error => 0xff0000,
            Self::Success => 0xcc6600,
        }
    }
}

pub trait Extend<T> {
    fn extend(&mut self, data: T) -> &mut Self;
}

pub fn timestamp(datetime: OffsetDateTime) -> String {
    Timestamp::new(
        datetime.unix_timestamp().try_into().unwrap(),
        Some(TimestampStyle::RelativeTime),
    )
    .mention()
    .to_string()
}

fn trim_len(value: &str, len: usize) -> &str {
    let mut utf16_len = 0;
    let end = value
        .char_indices()
        .find_map(|(i, ch)| {
            utf16_len += ch.len_utf16();
            if utf16_len > len {
                Some(i)
            } else {
                None
            }
        })
        .unwrap_or(value.len());
    &value[..end]
}

fn trim_field(value: &str) -> &str {
    trim_len(value, FIELD_VALUE_LENGTH)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_len_longer() {
        let original = "Zażółć gęślą jaźń";
        let expected = "Zażółć gęś";
        let actual = trim_len(original, 10);
        assert_eq!(actual, expected);
        assert_eq!(actual.encode_utf16().collect::<Vec<_>>().len(), 10);
    }

    #[test]
    fn trim_len_shorter() {
        let original = "Zażółć gęślą jaźń";
        let actual = trim_len(original, 20);
        assert_eq!(actual, original);
        assert_eq!(
            actual.encode_utf16().collect::<Vec<_>>().len(),
            original.encode_utf16().collect::<Vec<_>>().len()
        );
    }

    #[test]
    fn trim_len_equal() {
        let original = "Zażółć gęś";
        let actual = trim_len(original, 10);
        assert_eq!(actual, original);
        assert_eq!(actual.encode_utf16().collect::<Vec<_>>().len(), 10);
    }
}
