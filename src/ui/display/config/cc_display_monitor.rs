use std::fmt;

use crate::ui::display::color_mode::ColorMode;
use crate::ui::display::display_mode::{DisplayMode, RefreshRateMode, approx_equal};
use crate::ui::display::monitor::{Geometry, Monitor, compare_mode_preference};
use crate::ui::display::transform::Transform;

use super::LogicalMonitorHandle;

#[derive(Debug, Clone)]
pub struct CcDisplayMonitor {
    pub inner: Monitor,

    is_usable: bool,
    logical_monitor: Option<LogicalMonitorHandle>,
}

impl PartialEq for CcDisplayMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner && self.is_usable == other.is_usable
    }
}

impl fmt::Display for CcDisplayMonitor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.get_output_ui_name())
    }
}

impl CcDisplayMonitor {
    pub(crate) fn new(monitor: Monitor, logical_monitor: Option<LogicalMonitorHandle>) -> Self {
        Self {
            inner: monitor,
            is_usable: true,
            logical_monitor,
        }
    }

    pub fn logical_monitor(&self) -> Option<LogicalMonitorHandle> {
        self.logical_monitor.clone()
    }

    pub fn get_display_name(&self) -> &str {
        self.inner.get_display_name()
    }

    pub fn is_active(&self) -> bool {
        self.logical_monitor.is_some()
    }

    pub fn get_vendor_name(&self) -> &str {
        self.inner.get_vendor_name()
    }

    pub fn get_product_name(&self) -> &str {
        self.inner.get_product_name()
    }

    pub fn get_product_serial(&self) -> &str {
        self.inner.get_product_serial()
    }

    pub fn get_connector_name(&self) -> &str {
        self.inner.get_connector_name()
    }

    pub fn get_rotation(&self) -> Transform {
        self.logical_monitor
            .as_ref()
            .and_then(|lm| lm.lock().ok().map(|lm| lm.rotation()))
            .unwrap_or(Transform::Normal)
    }

    pub fn set_rotation(&mut self, r: Transform) {
        if let Some(lm) = &self.logical_monitor
            && let Ok(mut lm) = lm.lock()
        {
            lm.set_rotation(r);
        }
    }

    pub fn get_physical_size(&self) -> (i32, i32) {
        self.inner.get_physical_size()
    }

    pub fn is_builtin(&self) -> bool {
        self.inner.is_builtin()
    }

    pub fn is_primary(&self) -> bool {
        self.logical_monitor
            .as_ref()
            .and_then(|lm| lm.lock().ok().map(|lm| lm.is_primary()))
            .unwrap_or(false)
    }

    pub fn set_primary(&mut self, primary: bool) {
        if let Some(lm) = &self.logical_monitor
            && let Ok(mut lm) = lm.lock()
        {
            lm.set_primary(primary);
        }
    }

    pub fn supports_variable_refresh_rate(&self) -> bool {
        self.inner.supports_variable_refresh_rate()
    }

    pub fn get_supported_color_modes(&self) -> &[ColorMode] {
        self.inner.get_supported_color_modes()
    }

    pub fn supports_color_mode(&self, color_mode: ColorMode) -> bool {
        self.inner.supports_color_mode(color_mode)
    }

    pub fn get_color_mode(&self) -> ColorMode {
        self.inner.get_color_mode()
    }

    pub fn set_color_mode(&mut self, color_mode: ColorMode) {
        if self.inner.supports_color_mode(color_mode) {
            self.inner.properties.color_mode = Some(color_mode);
        }
    }

    pub fn supports_underscanning(&self) -> bool {
        self.inner.supports_underscanning()
    }

    pub fn get_underscanning(&self) -> bool {
        self.inner.get_underscanning()
    }

    pub fn set_underscanning(&mut self, underscanning: bool) {
        if self.inner.supports_underscanning() {
            self.inner.properties.is_underscanning = Some(underscanning);
        }
    }

    pub fn get_geometry(&self) -> Geometry {
        let (x, y) = self
            .logical_monitor
            .as_ref()
            .and_then(|lm| lm.lock().ok().map(|lm| (lm.x(), lm.y())))
            .unwrap_or((-1, -1));

        let (width, height) = self
            .try_get_optimal_mode()
            .map(|mode| (mode.width, mode.height))
            .unwrap_or_else(|| {
                tracing::warn!("Monitor at {} has no modes?", self.inner.spec.connector);
                (-1, -1)
            });

        Geometry {
            x,
            y,
            width,
            height,
        }
    }

    pub fn get_min_freq(&self) -> i32 {
        self.inner.get_min_freq()
    }

    pub fn get_modes(&self) -> &[DisplayMode] {
        &self.inner.modes
    }

    pub fn get_current_mode(&self) -> Option<&DisplayMode> {
        self.inner.modes.iter().find(|m| m.is_current())
    }

    pub fn get_preferred_mode(&self) -> Option<&DisplayMode> {
        self.inner.modes.iter().find(|m| m.is_preferred())
    }

    pub fn try_get_optimal_mode(&self) -> Option<&DisplayMode> {
        self.get_current_mode()
            .or_else(|| self.get_preferred_mode())
            .or_else(|| self.get_modes().first())
    }

    pub fn get_optimal_mode(&self) -> &DisplayMode {
        self.try_get_optimal_mode().expect("display mode not found")
    }

    pub fn get_closest_mode(
        &self,
        width: i32,
        height: i32,
        refresh_rate: f64,
        refresh_rate_mode: RefreshRateMode,
        is_interlaced: bool,
    ) -> Option<&DisplayMode> {
        let similar = self.inner.modes.iter().filter(|mode| {
            mode.width == width
                && mode.height == height
                && mode.get_refresh_rate_mode() == refresh_rate_mode
        });
        similar
            .clone()
            .find(|mode| {
                approx_equal(mode.refresh_rate, refresh_rate)
                    && mode.is_interlaced() == is_interlaced
            })
            .or_else(|| similar.max_by(compare_mode_preference))
    }

    pub fn get_compatible_clone_mode(
        &self,
        clone_width: i32,
        clone_height: i32,
    ) -> Option<&DisplayMode> {
        self.inner
            .modes
            .iter()
            .filter(|mode| mode.width == clone_width && mode.height == clone_height)
            .max_by(compare_mode_preference)
    }

    pub fn get_scale(&self) -> f64 {
        self.logical_monitor
            .as_ref()
            .and_then(|lm| lm.lock().ok().map(|lm| lm.scale()))
            .unwrap_or(1.0)
    }

    pub fn set_scale(&mut self, scale: f64) {
        if let Some(mode) = self.try_get_optimal_mode()
            && mode.is_supported_scale(scale)
            && let Some(lm) = &self.logical_monitor
            && let Ok(mut lm) = lm.lock()
        {
            lm.set_scale(scale);
        }
    }

    pub fn set_compatible_clone_mode(&mut self, clone_mode: &DisplayMode) -> Option<&DisplayMode> {
        let best = self
            .get_compatible_clone_mode(clone_mode.width, clone_mode.height)?
            .clone();

        self.set_mode(&best)
    }

    pub fn set_mode(&mut self, target_mode: &DisplayMode) -> Option<&DisplayMode> {
        let closest_id = self
            .get_closest_mode(
                target_mode.width,
                target_mode.height,
                target_mode.refresh_rate,
                target_mode.get_refresh_rate_mode(),
                target_mode.is_interlaced(),
            )
            .map(|m| m.id.clone())?;

        for mode in &mut self.inner.modes {
            mode.properties.is_current = Some(mode.id == closest_id);
        }

        if let Some(lm) = &self.logical_monitor
            && let Some(current_mode) = self.inner.modes.iter().find(|m| m.id == closest_id)
            && !current_mode.is_supported_scale(self.get_scale())
            && let Ok(mut lm) = lm.lock()
        {
            lm.set_scale(current_mode.preferred_scale);
        }

        self.inner.modes.iter().find(|m| m.id == closest_id)
    }

    pub fn set_refresh_rate_mode(
        &mut self,
        refresh_rate_mode: RefreshRateMode,
    ) -> Option<&DisplayMode> {
        let optimal = self.try_get_optimal_mode()?.clone();
        let target = self
            .get_closest_mode(
                optimal.width,
                optimal.height,
                optimal.refresh_rate,
                refresh_rate_mode,
                optimal.is_interlaced(),
            )?
            .clone();

        self.set_mode(&target)
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        if let Some(lm) = &self.logical_monitor
            && let Ok(mut lm) = lm.lock()
        {
            lm.set_position(x, y);
        }
    }

    pub fn is_useful(&self) -> bool {
        self.is_usable && self.is_active()
    }

    pub fn is_usable(&self) -> bool {
        self.is_usable
    }

    pub fn set_usable(&mut self, is_usable: bool) {
        self.is_usable = is_usable;
    }

    pub fn get_output_ui_name(&self) -> String {
        self.inner.get_output_ui_name()
    }

    pub(crate) fn set_logical_monitor(&mut self, logical_monitor: Option<LogicalMonitorHandle>) {
        self.logical_monitor = logical_monitor;
    }
}
