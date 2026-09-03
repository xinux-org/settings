use std::sync::Arc;

use crate::ui::display::{
    CcDisplayMonitor, Scale, apply_monitors, logical_monitor::LogicalMonitor,
    monitor_spec::MonitorSpec, transform::Transform,
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
    pub fn get_position(&self) -> (i32, i32) {
        (self.inner.x, self.inner.y)
    }

    pub fn get_scale(&self) -> Scale {
        Scale::from(self.inner.scale)
    }

    pub fn get_transform(&self) -> Transform {
        self.inner.transform
    }

    pub fn has_output(&self, spec: &MonitorSpec) -> bool {
        self.inner.monitors.contains(spec)
    }

    pub fn into_apply(&self, monitors: &[Arc<CcDisplayMonitor>]) -> apply_monitors::LogicalMonitor {
        apply_monitors::LogicalMonitor {
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
                .map(|monitor| apply_monitors::LogicalMonitorOutput {
                    connector: monitor.get_spec().connector.clone(),
                    monitor_mode_id: monitor.get_current_mode().get_id().to_string(),
                    properties: apply_monitors::LogicalMonitorOutputProperties {
                        color_mode: monitor.get_color_mode(),
                        underscanning: monitor.is_underscanning(),
                    },
                })
                .collect(),
        }
    }
}
