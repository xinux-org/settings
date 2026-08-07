// current layout mode represents the way logical monitors are laid out on the screen
#[derive(Debug)]
pub enum LayoutMode {
    // With logical mode, the dimension of a logical monitor is the dimension
    // of the monitor mode, divided by the logical monitor scale.
    Logical,
    // With physical layout mode, each logical monitor has the same dimensions
    // as the monitor modes of the associated monitors assigned to it, no
    // matter what scale is in use.
    Physical,
}

impl TryFrom<u32> for LayoutMode {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Logical),
            2 => Ok(Self::Physical),
            _ => Err("Unknown layout mode".to_string()),
        }
    }
}
