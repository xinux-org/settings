use serde::Serialize;
use zbus::zvariant::{SerializeDict, Type};

use super::{ColorMode, LayoutMode, MonitorSpec, Transform};

#[derive(Serialize, Type, Debug)]
pub enum ApplyMethod {
    Verify = 0,
    Temporary = 1,
    Persistent = 2,
}

#[derive(Serialize, Type, Debug)]
pub struct ApplyMonitorsConfig {
    pub serial: u32,
    pub method: ApplyMethod,
    pub logical_monitors: Vec<ApplyLogicalMonitor>,
    pub properties: ApplyMonitorsConfigProperties,
}

#[derive(Serialize, Type, Debug)]
pub struct ApplyLogicalMonitor {
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
pub struct ApplyMonitorsConfigProperties {
    pub layout_mode: Option<LayoutMode>,
    pub monitors_for_lease: Vec<MonitorSpec>,
}
