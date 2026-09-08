use std::sync::{Arc, RwLock};

use crate::ui::display::{
    CcLogicalMonitor,
    apply_monitors::{ApplyMethod, ApplyMonitorsConfig, ApplyMonitorsConfigProperties},
    display_state::{DisplayState, DisplayStateProperties},
};

use super::CcDisplayMonitor;

pub enum DisplayConfigType {
    Single(Arc<RwLock<CcDisplayMonitor>>),
}

#[derive(Debug, Clone)]
pub struct CcDisplayConfig {
    serial: u32,
    properties: DisplayStateProperties,
    monitors: Vec<Arc<RwLock<CcDisplayMonitor>>>,
}

impl From<DisplayState> for CcDisplayConfig {
    fn from(inner: DisplayState) -> Self {
        let mut logical_monitors = inner
            .logical_monitors
            .into_iter()
            .map(CcLogicalMonitor::from)
            .collect::<Vec<_>>();

        let monitors = inner
            .monitors
            .into_iter()
            .map(|m| {
                let lm = logical_monitors
                    .iter()
                    .position(|lm| lm.has_output(&m.spec))
                    .map(|index| logical_monitors.swap_remove(index));

                CcDisplayMonitor::new(m, lm)
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

impl CcDisplayConfig {
    pub fn get_monitor(&self) -> DisplayConfigType {
        if self.monitors.len() == 1
            && let Some(monitor) = self.monitors.first()
        {
            return DisplayConfigType::Single(Arc::clone(monitor));
        }

        if let Some(index) = self
            .monitors
            .iter()
            .enumerate()
            .filter_map(|(i, m)| m.read().ok().map(|m| (i, m)))
            .find(|(_, m)| m.is_builtin())
            .map(|(i, _)| i)
        {
            return DisplayConfigType::Single(Arc::clone(&self.monitors[index]));
        };

        unimplemented!()
    }

    pub fn build_apply_parameters<'a>(&'a self, method: ApplyMethod) -> ApplyMonitorsConfig<'a> {
        let read_monitors = self
            .monitors
            .iter()
            .filter_map(|monitor| monitor.read().ok())
            .collect::<Vec<_>>();

        let logical_monitors = read_monitors
            .iter()
            // .filter(|monitor| !monitors_for_lease.contains(&monitor))
            .filter_map(|monitor| monitor.get_logical_monitor())
            .map(|lm| lm.into_apply(read_monitors.as_slice()))
            .collect::<Vec<_>>();

        ApplyMonitorsConfig {
            method,
            logical_monitors,
            serial: self.serial,
            properties: ApplyMonitorsConfigProperties {
                monitors_for_lease: vec![],
                layout_mode: self.properties.layout_mode,
            },
        }
    }
}
