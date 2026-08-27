use serde::{Deserialize, Serialize};
use zbus::zvariant::Type;

#[derive(Serialize, Deserialize, Type, Debug, Copy, Clone, PartialEq, Eq)]
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

impl Transform {
    pub fn is_rotated(&self) -> bool {
        matches!(
            self,
            Transform::Rotate90
                | Transform::Rotate270
                | Transform::Flipped90
                | Transform::Flipped270
        )
    }

    pub fn transform_dimensions(&self, width: i32, height: i32) -> (i32, i32) {
        if self.is_rotated() {
            (height, width)
        } else {
            (width, height)
        }
    }
}
