use crate::ui::display::{
    Scale, logical_monitor::LogicalMonitor, monitor_spec::MonitorSpec, transform::Transform,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CcLogicalMonitor {
    inner: LogicalMonitor,
}

impl From<LogicalMonitor> for CcLogicalMonitor {
    fn from(inner: LogicalMonitor) -> Self {
        Self { inner }
    }
}

impl CcLogicalMonitor {
    pub fn get_scale(&self) -> Scale {
        Scale::from(self.inner.scale)
    }

    pub fn get_transform(&self) -> Transform {
        self.inner.transform
    }

    pub fn has_output(&self, spec: &MonitorSpec) -> bool {
        self.inner.monitors.contains(spec)
    }
}
