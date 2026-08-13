#[derive(Debug, Copy, Clone)]
pub enum ColorMode {
    BT2100,
    Default,
    SDRNative,
}

impl From<u32> for ColorMode {
    fn from(value: u32) -> Self {
        match value {
            2 => Self::SDRNative,
            1 => Self::BT2100,
            _ => Self::Default,
        }
    }
}

impl From<&u32> for ColorMode {
    fn from(value: &u32) -> Self {
        Self::from(*value)
    }
}
