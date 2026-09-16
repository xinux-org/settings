use serde::Deserialize;
use zbus::zvariant::{DeserializeDict, Type};

use super::{LogicalMonitor, Monitor};

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

// current layout mode represents the way logical monitors are laid out on the screen
#[derive(Deserialize, Type, Debug, Copy, Clone, PartialEq, Eq)]
pub enum LayoutMode {
    // With logical mode, the dimension of a logical monitor is the dimension
    // of the monitor mode, divided by the logical monitor scale.
    Logical = 1,
    // With physical layout mode, each logical monitor has the same dimensions
    // as the monitor modes of the associated monitors assigned to it, no
    // matter what scale is in use.
    Physical = 2,
}
