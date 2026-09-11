use serde::{Deserialize, Serialize};
use zbus::zvariant::Type;

// current layout mode represents the way logical monitors are laid out on the screen
#[derive(Serialize, Deserialize, Type, Debug, Copy, Clone, PartialEq, Eq)]
pub enum LayoutMode {
    // With logical mode, the dimension of a logical monitor is the dimension
    // of the monitor mode, divided by the logical monitor scale.
    Logical = 1,
    // With physical layout mode, each logical monitor has the same dimensions
    // as the monitor modes of the associated monitors assigned to it, no
    // matter what scale is in use.
    Physical = 2,
}
