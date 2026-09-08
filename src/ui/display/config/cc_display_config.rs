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
    monitors: Vec<Arc<CcDisplayMonitor>>,
    monitors: Vec<Arc<RwLock<CcDisplayMonitor>>>,
}

impl From<DisplayState> for CcDisplayConfig {
    fn from(inner: DisplayState) -> Self {
        let mut logical_monitors = inner
            .logical_monitors
            .into_iter()
            .map(CcLogicalMonitor::from)
            .map(Arc::new)
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

        if let Some(builtin) = self.monitors.iter().find(|monitor| monitor.is_builtin())
            && builtin.get_logical_monitor().is_some()
        {
            return DisplayConfigType::Single(Arc::clone(builtin));
        };

        unimplemented!()
    }

    pub fn build_apply_parameters<'a>(&'a self, method: ApplyMethod) -> ApplyMonitorsConfig<'a> {
        let monitors_for_lease = self
            .monitors
            .iter()
            .filter(|monitor| monitor.is_for_lease())
            .collect::<Vec<_>>();

        let logical_monitors = self
            .logical_monitors
            .iter()
            .filter(|&lm| {
                !monitors_for_lease.iter().any(|for_lease| {
                    for_lease
                        .get_logical_monitor()
                        .is_some_and(|for_lease| Arc::ptr_eq(lm, for_lease))
                })
            })
            .map(|lm| lm.into_apply(&self.monitors))
            .collect::<Vec<_>>();

        let monitors_for_lease = monitors_for_lease
            .iter()
            .map(|&monitor| monitor.get_spec())
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
