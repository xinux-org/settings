use crate::ui::display::dbus;

use super::Scale;

#[derive(Debug, PartialEq)]
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
}
