#[derive(Debug, Copy, Clone)]
pub enum ColorMode {
    BT2100,
    Default,
    SDRNative,
}

impl From<i32> for ColorMode {
    fn from(value: i32) -> Self {
        match value {
            2 => Self::SDRNative,
            1 => Self::BT2100,
            _ => Self::Default,
        }
    }
}

impl From<&i32> for ColorMode {
    fn from(value: &i32) -> Self {
        Self::from(*value)
    }
}
