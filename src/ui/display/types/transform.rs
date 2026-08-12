#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Transform {
    Normal,
    Rotate90,
    Rotate180,
    Rotate270,
    Flipped,
    Flipped90,
    Flipped180,
    Flipped270,
}

impl From<u32> for Transform {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Normal,
            1 => Self::Rotate90,
            2 => Self::Rotate180,
            3 => Self::Rotate270,
            4 => Self::Flipped,
            5 => Self::Flipped90,
            6 => Self::Flipped180,
            7 => Self::Flipped270,
            _ => Self::Normal,
        }
    }
}
