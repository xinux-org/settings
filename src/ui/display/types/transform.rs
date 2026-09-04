use serde::{Deserialize, Serialize};
use zbus::zvariant::Type;

#[derive(Serialize, Deserialize, Type, Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Transform {
    #[default]
    Normal = 0,
    Rotate90 = 1,
    Rotate180 = 2,
    Rotate270 = 3,
    Flipped = 4,
    Flipped90 = 5,
    Flipped180 = 6,
    Flipped270 = 7,
}
