use std::collections::HashMap;

use zbus::zvariant;

use super::layout_mode::LayoutMode;
use super::logical_monitor::{LogicalMonitor, RawLogicalMonitor};
use super::monitor::{Monitor, RawMonitor};

type RawDisplayState = (
    u32,
    Vec<RawMonitor>,
    Vec<RawLogicalMonitor>,
    HashMap<String, zvariant::OwnedValue>,
);

#[derive(Debug)]
pub struct DisplayState {
    // configuration serial
    serial: u32,
    // available monitors
    monitors: Vec<Monitor>,
    // current logical monitor configuration
    logical_monitors: Vec<LogicalMonitor>,
    // Represents in what way logical monitors are laid out on the screen.
    // The layout mode can be either of the ones listed below.
    // Absence of this property means the layout mode cannot be changed,
    // and that "logical" mode is assumed to be used.
    layout_mode: Option<LayoutMode>,
    // True if the layout mode can be changed.
    //  Absence of this means the layout mode cannot be changed.
    supports_changing_layout_mode: Option<bool>,
    // True if all the logical monitors must always use the same scale.
    // Absence of this means logical monitor scales can differ.
    global_scale_required: Option<bool>,
}

impl From<RawDisplayState> for DisplayState {
    fn from(value: RawDisplayState) -> Self {
        Self {
            serial: value.0,
            monitors: value.1.iter().map(Monitor::from).collect(),
            logical_monitors: value.2.iter().map(LogicalMonitor::from).collect(),
            layout_mode: value
                .3
                .get("layout-mode")
                .and_then(|val| val.downcast_ref::<u32>().ok())
                .and_then(|val| LayoutMode::try_from(val).ok()),
            supports_changing_layout_mode: value
                .3
                .get("supports-changing-layout-mode")
                .and_then(|val| val.downcast_ref().ok()),
            global_scale_required: value
                .3
                .get("global-scale-required")
                .and_then(|val| val.downcast_ref().ok()),
        }
    }
}
