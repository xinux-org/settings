#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RgbRange {
    Auto,
    Full,
    Limited,
}

impl From<i32> for RgbRange {
    fn from(value: i32) -> Self {
        match value {
            2 => Self::Full,
            3 => Self::Limited,
            _ => Self::Auto,
        }
    }
}
