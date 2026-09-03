use serde::Serialize;
use zbus::zvariant::{SerializeDict, Type};

use crate::ui::display::transform::Transform;

use super::color_mode::ColorMode;
use super::layout_mode::LayoutMode;
use super::monitor_spec::MonitorSpec;

#[derive(Serialize, Type, Debug)]
pub enum ApplyMethod {
    Verify = 0,
    Temporary = 1,
    Persistent = 2,
}

#[derive(Serialize, Type, Debug)]
pub struct ApplyMonitorsConfig<'a> {
    pub serial: u32,
    pub method: ApplyMethod,
    pub logical_monitors: Vec<LogicalMonitor>,
    pub properties: ApplyMonitorsConfigProperties<'a>,
}

#[derive(Serialize, Type, Debug)]
pub struct LogicalMonitor {
    pub x: i32,
    pub y: i32,
    pub scale: f64,
    pub transform: Transform,
    pub is_primary: bool,
    pub outputs: Vec<LogicalMonitorOutput>,
}

#[derive(Serialize, Type, Debug)]
pub struct LogicalMonitorOutput {
    pub connector: String,
    pub monitor_mode_id: String,
    pub properties: LogicalMonitorOutputProperties,
}

#[derive(SerializeDict, Type, Debug)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct LogicalMonitorOutputProperties {
    pub color_mode: ColorMode,
    pub underscanning: Option<bool>,
}

#[derive(SerializeDict, Type, Debug)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct ApplyMonitorsConfigProperties<'a> {
    pub layout_mode: Option<LayoutMode>,
    pub monitors_for_lease: Vec<&'a MonitorSpec>,
}
