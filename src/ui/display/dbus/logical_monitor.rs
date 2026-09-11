use std::collections::HashMap;

use serde::{Deserialize};
use zbus::zvariant::{OwnedValue, Type};

use super::monitor_spec::MonitorSpec;
use super::transform::Transform;

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
