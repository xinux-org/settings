use std::collections::HashMap;

use zbus::zvariant;

use super::monitor_spec::{MonitorSpec, RawMonitorSpec};
use super::transform::Transform;

pub type RawLogicalMonitor = (
    i32,
    i32,
    f64,
    u32,
    bool,
    Vec<RawMonitorSpec>,
    HashMap<String, zvariant::OwnedValue>,
);

#[derive(Debug)]
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
}

impl From<RawLogicalMonitor> for LogicalMonitor {
    fn from(value: RawLogicalMonitor) -> Self {
        Self {
            x: value.0,
            y: value.1,
            scale: value.2,
            transform: Transform::from(value.3),
            is_primary: value.4,
            monitors: value.5.iter().map(MonitorSpec::from).collect(),
        }
    }
}

impl From<&RawLogicalMonitor> for LogicalMonitor {
    fn from(value: &RawLogicalMonitor) -> Self {
        Self::from(value.to_owned())
    }
}
