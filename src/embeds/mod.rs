mod project;
mod user;

use time::OffsetDateTime;
use twilight_mention::{
    timestamp::{Timestamp, TimestampStyle},
    Mention,
};

pub use project::Project;
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
