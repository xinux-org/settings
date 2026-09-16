use std::collections::HashMap;

use serde::Deserialize;
use zbus::zvariant::{OwnedValue, Type};

use super::MonitorSpec;

#[derive(Deserialize, Type, Debug, Clone, PartialEq)]
pub struct LogicalMonitor {
    // x position
    pub x: i32,
    // y position
    pub y: i32,
    // scale
    pub scale: f64,
    // transform
    pub transform: Transform,
    // true if this is the primary logical monitor
    pub is_primary: bool,
    // monitors displaying this logical monitor
    pub monitors: Vec<MonitorSpec>,
    pub properties: HashMap<String, OwnedValue>,
}

#[derive(Deserialize, Type, Default, Debug, Copy, Clone, PartialEq, Eq)]
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
