use std::sync::{Arc, RwLock};

use crate::ui::display::{CcLogicalMonitor, display_state::DisplayState};

use super::CcDisplayMonitor;

pub enum DisplayConfigType {
    Single(Arc<RwLock<CcDisplayMonitor>>),
}

#[derive(Debug, Clone)]
pub struct CcDisplayConfig {
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

        Self { monitors }
    }
}

impl CcDisplayConfig {
    pub fn get_monitor(&self) -> DisplayConfigType {
        if self.monitors.len() == 1
            && let Some(monitor) = self.monitors.first()
        {
            return DisplayConfigType::Single(Arc::clone(monitor));
        }

        unimplemented!()
    }
}
