#[derive(Debug, Copy, Clone)]
pub enum RgbRange {
    Auto,
    Full,
    Limited,
}

impl TryFrom<i32> for RgbRange {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Auto),
            2 => Ok(Self::Full),
            3 => Ok(Self::Limited),
            _ => Err("Unknow rgb range".into()),
        }
    }
}
