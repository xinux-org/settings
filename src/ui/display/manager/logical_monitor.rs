use std::sync::RwLockReadGuard;

use crate::ui::display::{DisplayMonitor, dbus};

use super::Scale;

#[derive(Debug, Clone, PartialEq)]
pub struct LogicalMonitor {
    inner: dbus::LogicalMonitor,
}

impl From<dbus::LogicalMonitor> for LogicalMonitor {
    fn from(inner: dbus::LogicalMonitor) -> Self {
        Self { inner }
    }
}

impl LogicalMonitor {
    pub fn get_scale(&self) -> Scale {
        Scale::from(self.inner.scale)
    }

    pub fn get_transform(&self) -> dbus::Transform {
        self.inner.transform
    }

    pub fn has_output(&self, spec: &dbus::MonitorSpec) -> bool {
        self.inner.monitors.contains(spec)
    }

    pub fn set_scale(&mut self, scale: Scale) {
        self.inner.scale = scale.into();
    }

    pub fn into_apply(
        &self,
        monitors: &[RwLockReadGuard<'_, DisplayMonitor>],
    ) -> dbus::ApplyLogicalMonitor {
        dbus::ApplyLogicalMonitor {
            x: self.inner.x,
            y: self.inner.y,
            scale: self.inner.scale,
            transform: self.inner.transform,
            is_primary: self.inner.is_primary,
            outputs: self
                .inner
                .monitors
                .iter()
                .filter_map(|monitor_spec| {
                    monitors
                        .iter()
                        .find(|&monitor| monitor.get_spec() == monitor_spec)
                })
                .map(|monitor| dbus::LogicalMonitorOutput {
                    connector: monitor.get_spec().connector.clone(),
                    monitor_mode_id: monitor.get_current_mode().get_id().to_string(),
                    properties: dbus::LogicalMonitorOutputProperties {
                        color_mode: monitor.get_color_mode(),
                        underscanning: monitor.is_underscanning(),
                    },
                })
                .collect(),
        }
    }
}
