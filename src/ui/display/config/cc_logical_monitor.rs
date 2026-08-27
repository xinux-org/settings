use std::sync::{Arc, Mutex};

use crate::ui::display::logical_monitor::LogicalMonitor;
use crate::ui::display::transform::Transform;

#[derive(Debug, Clone, PartialEq)]
pub struct CcLogicalMonitor {
    inner: LogicalMonitor,
}

pub type LogicalMonitorHandle = Arc<Mutex<CcLogicalMonitor>>;

impl CcLogicalMonitor {
    pub fn new(inner: LogicalMonitor) -> Self {
        Self { inner }
    }

    pub fn inner(&self) -> &LogicalMonitor {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut LogicalMonitor {
        &mut self.inner
    }

    pub fn snapshot(&self) -> LogicalMonitor {
        self.inner.clone()
    }

    pub fn x(&self) -> i32 {
        self.inner.x
    }

    pub fn y(&self) -> i32 {
        self.inner.y
    }

    pub fn scale(&self) -> f64 {
        self.inner.scale
    }

    pub fn rotation(&self) -> Transform {
        self.inner.transform
    }

    pub fn is_primary(&self) -> bool {
        self.inner.is_primary
    }

    pub fn is_rotated(&self) -> bool {
        self.inner.transform.is_rotated()
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.inner.x = x;
        self.inner.y = y;
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.inner.scale = scale;
    }

    pub fn set_rotation(&mut self, rotation: Transform) {
        self.inner.transform = rotation;
    }

    pub fn set_primary(&mut self, primary: bool) {
        self.inner.is_primary = primary;
    }

    pub fn is_empty(&self) -> bool {
        self.inner.monitors.is_empty()
    }

    pub fn get_transformed_resolution(&self, width: i32, height: i32) -> (i32, i32) {
        self.inner.transform.transform_dimensions(width, height)
    }

    pub fn get_logical_dimensions(
        &self,
        mode_width: i32,
        mode_height: i32,
        is_layout_logical: bool,
    ) -> (i32, i32) {
        let (width, height) = self.get_transformed_resolution(mode_width, mode_height);

        if is_layout_logical && self.inner.scale > 0.0 {
            (
                (width as f64 / self.inner.scale).round() as i32,
                (height as f64 / self.inner.scale).round() as i32,
            )
        } else {
            (width, height)
        }
    }
}

pub(crate) fn logical_handle(inner: LogicalMonitor) -> LogicalMonitorHandle {
    Arc::new(Mutex::new(CcLogicalMonitor::new(inner)))
}
