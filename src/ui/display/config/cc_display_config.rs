use std::sync::Arc;

use crate::ui::display::{CcLogicalMonitor, display_state::DisplayState};

use super::CcDisplayMonitor;

pub enum DisplayConfigType {
    Single(Arc<CcDisplayMonitor>),
}

#[derive(Debug, Clone)]
pub struct CcDisplayConfig {
    monitors: Vec<Arc<CcDisplayMonitor>>,
    logical_monitors: Vec<Arc<CcLogicalMonitor>>,
}

impl From<DisplayState> for CcDisplayConfig {
    fn from(inner: DisplayState) -> Self {
        let logical_monitors = inner
            .logical_monitors
            .into_iter()
            .map(CcLogicalMonitor::from)
            .map(Arc::new)
            .collect::<Vec<_>>();

        let monitors = inner
            .monitors
            .into_iter()
            .map(|m| {
                let lm = CcDisplayMonitor::logical_monitor_for(&m, &logical_monitors);
                CcDisplayMonitor::new(m, lm)
            })
            .map(Arc::new)
            .collect::<Vec<_>>();

        Self {
            monitors,
            logical_monitors,
        }
    }
}

impl CcDisplayConfig {
    pub fn get_monitor(&self) -> DisplayConfigType {
        if self.monitors.len() == 1
            && self.logical_monitors.len() == 1
            && let Some(monitor) = self.monitors.first()
            && let Some(logical_monitor) = self.logical_monitors.first()
            && monitor
                .get_logical_monitor()
                .is_some_and(|lm| Arc::ptr_eq(lm, logical_monitor))
        {
            return DisplayConfigType::Single(Arc::clone(monitor));
        }

        unimplemented!()
    }
}
