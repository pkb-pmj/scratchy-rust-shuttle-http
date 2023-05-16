mod user;

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
