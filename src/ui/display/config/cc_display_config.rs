use std::sync::Arc;

use crate::ui::display::display_mode::{DisplayMode, approx_equal};
use crate::ui::display::display_state::DisplayState;
use crate::ui::display::layout_mode::LayoutMode;
use crate::ui::display::logical_monitor::LogicalMonitor;
use crate::ui::display::monitor_spec::MonitorSpec;
use crate::ui::display::transform::Transform;

use super::{CcDisplayMonitor, CcLogicalMonitor, LogicalMonitorHandle, logical_handle};

#[derive(Debug, Clone)]
pub struct CcDisplayConfig {
    pub serial: u32,
    pub min_width: i32,
    pub min_height: i32,
    pub global_scale_required: bool,
    pub monitors: Vec<CcDisplayMonitor>,
    pub layout_mode: Option<LayoutMode>,
    pub supports_changing_layout_mode: bool,
    logical_monitors: Vec<LogicalMonitorHandle>,
}

impl From<&DisplayState> for CcDisplayConfig {
    fn from(state: &DisplayState) -> Self {
        let logical_monitors = state
            .logical_monitors
            .iter()
            .filter(|logical| !logical.monitors.is_empty())
            .cloned()
            .map(logical_handle)
            .collect::<Vec<_>>();

        let monitors = state
            .monitors
            .iter()
            .cloned()
            .map(|monitor| {
                let logical = logical_monitors
                    .iter()
                    .find(|logical| {
                        logical
                            .lock()
                            .ok()
                            .is_some_and(|logical| logical.inner().monitors.contains(&monitor.spec))
                    })
                    .cloned();

                CcDisplayMonitor::new(monitor, logical)
            })
            .collect();

        let mut config = Self {
            monitors,
            min_width: 0,
            min_height: 0,
            logical_monitors,
            serial: state.serial,
            layout_mode: state.properties.layout_mode,
            global_scale_required: state.properties.global_scale_required.unwrap_or(false),
            supports_changing_layout_mode: state
                .properties
                .supports_changing_layout_mode
                .unwrap_or(false),
        };

        config.filter_invalid_scaled_modes();
        config.ensure_primary();

        config
    }
}

impl CcDisplayConfig {
    pub fn get_monitors(&self) -> &[CcDisplayMonitor] {
        &self.monitors
    }

    pub fn get_monitors_mut(&mut self) -> &mut [CcDisplayMonitor] {
        &mut self.monitors
    }

    pub fn get_logical_monitors(&self) -> &[LogicalMonitorHandle] {
        &self.logical_monitors
    }

    pub fn logical_snapshots(&self) -> Vec<LogicalMonitor> {
        self.logical_monitors
            .iter()
            .filter_map(|handle| handle.lock().ok().map(|logical| logical.snapshot()))
            .collect()
    }

    pub fn count_useful_monitors(&self) -> usize {
        self.monitors
            .iter()
            .filter(|monitor| monitor.is_useful())
            .count()
    }

    pub fn set_monitor_active(&mut self, spec: &MonitorSpec, active: bool) {
        let Some(index) = self
            .monitors
            .iter()
            .position(|monitor| monitor.inner.spec == *spec)
        else {
            return;
        };

        if active == self.monitors[index].is_active() {
            return;
        }

        if !active {
            self.monitors[index].set_logical_monitor(None);
        } else {
            if self.monitors[index].get_current_mode().is_none() {
                let mode = self.monitors[index]
                    .get_preferred_mode()
                    .or_else(|| self.monitors[index].inner.modes.first())
                    .cloned();

                if let Some(mode) = mode {
                    self.monitors[index].set_mode(&mode);
                }
            }

            let scale = self.monitors[index]
                .try_get_optimal_mode()
                .map(|mode| mode.preferred_scale)
                .unwrap_or(1.0);

            let mut logical =
                LogicalMonitor::new(0, 0, scale, Transform::Normal, false, vec![spec.clone()]);

            logical.x = self.rightmost_x();

            let handle = logical_handle(logical);

            self.monitors[index].set_logical_monitor(Some(handle.clone()));
            self.logical_monitors.push(handle);
        }

        self.ensure_primary();
    }

    pub fn set_primary(&mut self, spec: &MonitorSpec) {
        let Some(target) = self
            .monitors
            .iter()
            .find(|monitor| monitor.inner.spec == *spec)
            .and_then(CcDisplayMonitor::logical_monitor)
        else {
            return;
        };

        for handle in &self.logical_monitors {
            let is_target = Arc::ptr_eq(handle, &target);

            if let Ok(mut logical) = handle.lock() {
                logical.set_primary(is_target);
            }
        }
    }

    pub fn set_monitor_scale(&mut self, spec: &MonitorSpec, scale: f64) {
        let Some(index) = self
            .monitors
            .iter()
            .position(|monitor| monitor.inner.spec == *spec)
        else {
            return;
        };

        if !self.is_scaled_mode_valid_for_monitor(index, scale) {
            return;
        }

        if self.global_scale_required {
            for monitor in &mut self.monitors {
                if monitor.is_active() {
                    monitor.set_scale(scale);
                }
            }
        } else {
            self.monitors[index].set_scale(scale);
        }
    }

    pub fn is_cloning(&self) -> bool {
        self.monitors
            .iter()
            .filter(|monitor| monitor.is_active())
            .count()
            > 1
            && self.logical_monitors.len() == 1
    }

    pub fn set_cloning(&mut self, clone: bool) {
        if clone && !self.is_cloning() {
            let specs: Vec<_> = self
                .monitors
                .iter()
                .map(|monitor| monitor.inner.spec.clone())
                .collect();

            for spec in &specs {
                self.set_monitor_active(spec, true);
            }

            let primary = self.primary_handle().is_some();
            let handle = logical_handle(LogicalMonitor::new(
                0,
                0,
                1.0,
                Transform::Normal,
                primary,
                specs,
            ));

            for monitor in &mut self.monitors {
                monitor.set_logical_monitor(Some(handle.clone()));
            }

            self.logical_monitors = vec![handle];
        } else if !clone && self.is_cloning() {
            let active: Vec<_> = self
                .monitors
                .iter()
                .filter(|monitor| monitor.is_active())
                .map(|monitor| monitor.inner.spec.clone())
                .collect();

            self.logical_monitors.clear();

            for spec in active {
                let index = self
                    .monitors
                    .iter()
                    .position(|monitor| monitor.inner.spec == spec)
                    .expect("active monitor disappeared");

                let scale = self.monitors[index]
                    .try_get_optimal_mode()
                    .map(|mode| mode.preferred_scale)
                    .unwrap_or(1.0);

                let primary = self.monitors[index].is_primary();

                let handle = logical_handle(LogicalMonitor::new(
                    0,
                    0,
                    scale,
                    Transform::Normal,
                    primary,
                    vec![spec],
                ));

                self.monitors[index].set_logical_monitor(Some(handle.clone()));
                self.logical_monitors.push(handle);
            }

            self.make_linear();
        }

        self.ensure_primary();
    }

    pub fn make_linear(&mut self) {
        let mut handles = self.logical_monitors.clone();

        handles.sort_by_key(|handle| {
            handle
                .lock()
                .ok()
                .map(|logical| (!logical.is_primary(), logical.x()))
                .unwrap_or((true, i32::MAX))
        });

        let mut x = 0;

        for handle in handles {
            let width = handle
                .lock()
                .ok()
                .map(|logical| self.logical_width(logical.inner()))
                .unwrap_or(0);

            if let Ok(mut logical) = handle.lock() {
                logical.set_position(x, 0);
            }

            x += width;
        }
    }

    pub fn set_mode_on_all_outputs(&mut self, clone_mode: &DisplayMode) {
        for monitor in &mut self.monitors {
            monitor.set_compatible_clone_mode(clone_mode);
            monitor.set_position(0, 0);
        }
    }

    pub fn generate_cloning_modes(&self) -> Vec<DisplayMode> {
        let Some(base) = self.monitors.iter().find(|monitor| monitor.is_active()) else {
            return vec![];
        };

        let mut modes = Vec::new();

        for mode in base.get_modes() {
            let mut scales = mode.supported_scales.clone();

            let compatible = self.monitors.iter().all(|monitor| {
                let Some(other) = monitor.get_modes().iter().find(|other| {
                    other.width == mode.width
                        && other.height == mode.height
                        && other.is_interlaced() == mode.is_interlaced()
                }) else {
                    return false;
                };

                scales.retain(|scale| other.is_supported_scale(*scale));

                !scales.is_empty()
            });

            if compatible {
                modes.push(DisplayMode::new_virtual(
                    mode.width,
                    mode.height,
                    mode.preferred_scale,
                    scales,
                ));
            }
        }
        if let Some((index, _)) = modes
            .iter()
            .enumerate()
            .max_by_key(|(_, mode)| (mode.width * mode.height, !mode.is_interlaced()))
        {
            modes[index].properties.is_preferred = Some(true);
        }
        modes
    }

    pub fn is_layout_logical(&self) -> bool {
        self.layout_mode.unwrap_or(LayoutMode::Logical) == LayoutMode::Logical
    }

    pub fn is_single(&self) -> bool {
        self.monitors.len() == 1 && self.logical_monitors.len() == 1
    }

    pub fn is_multiple(&self) -> bool {
        self.monitors.len() > 1
    }

    pub fn is_scaled_mode_valid(&self, mode: &DisplayMode, scale: f64) -> bool {
        !self.is_cloning()
            || self
                .monitors
                .iter()
                .filter(|monitor| monitor.is_active())
                .all(|monitor| {
                    monitor.get_modes().iter().any(|other| {
                        other.width == mode.width
                            && other.height == mode.height
                            && other.is_supported_scale(scale)
                    })
                })
    }

    fn primary_handle(&self) -> Option<LogicalMonitorHandle> {
        self.logical_monitors
            .iter()
            .find(|handle| {
                handle
                    .lock()
                    .ok()
                    .is_some_and(|logical| logical.is_primary())
            })
            .cloned()
    }

    fn ensure_primary(&mut self) {
        if self.primary_handle().is_none()
            && let Some(handle) = self.logical_monitors.first()
            && let Ok(mut logical) = handle.lock()
        {
            logical.set_primary(true);
        }
    }

    fn rightmost_x(&self) -> i32 {
        self.logical_snapshots()
            .iter()
            .map(|logical| logical.x + self.logical_width(logical))
            .max()
            .unwrap_or(0)
    }

    fn logical_width(&self, logical: &LogicalMonitor) -> i32 {
        logical
            .monitors
            .first()
            .and_then(|spec| {
                self.monitors
                    .iter()
                    .find(|monitor| monitor.inner.spec == *spec)
            })
            .and_then(|monitor| {
                let mode = monitor.try_get_optimal_mode()?;
                Some(
                    CcLogicalMonitor::new(logical.clone())
                        .get_logical_dimensions(mode.width, mode.height, self.is_layout_logical())
                        .0,
                )
            })
            .unwrap_or(0)
    }

    fn is_scaled_mode_valid_for_monitor(&self, index: usize, scale: f64) -> bool {
        self.monitors[index]
            .try_get_optimal_mode()
            .is_some_and(|mode| self.is_scaled_mode_valid(mode, scale))
    }

    fn filter_invalid_scaled_modes(&mut self) {
        for monitor in &mut self.monitors {
            for mode in &mut monitor.inner.modes {
                let (width, height, current, preferred, preferred_scale) = (
                    mode.width,
                    mode.height,
                    mode.is_current(),
                    mode.is_preferred(),
                    mode.preferred_scale,
                );

                mode.supported_scales.retain(|scale| {
                    let scaled_width = (width as f64 / scale).round() as i32;
                    let scaled_height = (height as f64 / scale).round() as i32;
                    (scaled_width.max(scaled_height) >= self.min_width
                        && scaled_width.min(scaled_height) >= self.min_height)
                        || current
                        || preferred
                        || approx_equal(*scale, preferred_scale)
                });
            }
        }
    }
}
