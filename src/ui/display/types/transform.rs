use serde::Deserialize;
use zbus::zvariant::Type;

#[derive(Deserialize, Type, Debug, Copy, Clone, PartialEq)]
pub enum Transform {
    Normal = 0,
    Rotate90 = 1,
    Rotate180 = 2,
    Rotate270 = 3,
    Flipped = 4,
    Flipped90 = 5,
    Flipped180 = 6,
    Flipped270 = 7,
}
