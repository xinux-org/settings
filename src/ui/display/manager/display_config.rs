use std::sync::{Arc, RwLock};

use crate::ui::display::dbus::{
    ApplyMethod, ApplyMonitorsConfig, ApplyMonitorsConfigProperties, DisplayState,
    DisplayStateProperties,
};

use super::{DisplayMonitor, LogicalMonitor};

pub enum DisplayConfigType {
    Single(Arc<RwLock<DisplayMonitor>>),
}

#[derive(Debug)]
pub struct DisplayConfig {
    serial: u32,
    properties: DisplayStateProperties,
    monitors: Vec<Arc<RwLock<DisplayMonitor>>>,
}

impl From<DisplayState> for DisplayConfig {
    fn from(inner: DisplayState) -> Self {
        let mut logical_monitors = inner
            .logical_monitors
            .into_iter()
            .map(LogicalMonitor::from)
            .collect::<Vec<_>>();

        let monitors = inner
            .monitors
            .into_iter()
            .map(|m| {
                let lm = logical_monitors
                    .iter()
                    .position(|lm| lm.has_output(&m.spec))
                    .map(|index| logical_monitors.swap_remove(index));

                DisplayMonitor::new(m, lm)
            })
            .map(RwLock::new)
            .map(Arc::new)
            .collect::<Vec<_>>();

        Self {
            monitors,
            serial: inner.serial,
            properties: inner.properties,
        }
    }
}

impl DisplayConfig {
    pub fn get_monitor(&self) -> DisplayConfigType {
        if self.monitors.len() == 1
            && let Some(monitor) = self.monitors.first()
        {
            return DisplayConfigType::Single(Arc::clone(monitor));
        }

        unimplemented!()
    }

    pub fn build_apply_parameters(&self, method: ApplyMethod) -> ApplyMonitorsConfig {
        let read_monitors = self
            .monitors
            .iter()
            .filter_map(|monitor| monitor.read().ok())
            .collect::<Vec<_>>();

        let monitors_for_lease = read_monitors
            .iter()
            .filter(|monitor| monitor.is_for_lease())
            .map(|monitor| monitor.get_spec().clone())
            .collect::<Vec<_>>();

        let logical_monitors = read_monitors
            .iter()
            .filter(|monitor| !monitors_for_lease.contains(monitor.get_spec()))
            .filter_map(|monitor| monitor.get_logical_monitor())
            .map(|lm| lm.into_apply(read_monitors.as_slice()))
            .collect::<Vec<_>>();

        ApplyMonitorsConfig {
            method,
            logical_monitors,
            serial: self.serial,
            properties: ApplyMonitorsConfigProperties {
                monitors_for_lease,
                layout_mode: self.properties.layout_mode,
            },
        }
    }
}
