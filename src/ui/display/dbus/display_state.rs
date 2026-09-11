use serde::Deserialize;
use zbus::zvariant::{DeserializeDict, Type};

use super::layout_mode::LayoutMode;
use super::logical_monitor::LogicalMonitor;
use super::monitor::Monitor;

#[derive(Deserialize, Type, Debug, Clone)]
pub struct DisplayState {
    // configuration serial
    pub serial: u32,
    // available monitors
    pub monitors: Vec<Monitor>,
    // current logical monitor configuration
    pub logical_monitors: Vec<LogicalMonitor>,
    pub properties: DisplayStateProperties,
}

#[derive(DeserializeDict, Type, Debug, Clone)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct DisplayStateProperties {
    // Represents in what way logical monitors are laid out on the screen.
    // The layout mode can be either of the ones listed below.
    // Absence of this property means the layout mode cannot be changed,
    // and that "logical" mode is assumed to be used.
    pub layout_mode: Option<LayoutMode>,
    // True if the layout mode can be changed.
    // Absence of this means the layout mode cannot be changed.
    pub supports_changing_layout_mode: Option<bool>,
    // True if all the logical monitors must always use the same scale.
    // Absence of this means logical monitor scales can differ.
    pub global_scale_required: Option<bool>,
}
