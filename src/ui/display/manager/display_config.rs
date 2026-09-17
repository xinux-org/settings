use std::sync::Arc;

use crate::ui::display::dbus::DisplayState;

use super::{DisplayMonitor, LogicalMonitor};

pub enum DisplayConfigType {
    Single(Arc<DisplayMonitor>),
}

#[derive(Debug)]
pub struct DisplayConfig {
    monitors: Vec<Arc<DisplayMonitor>>,
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
            .map(Arc::new)
            .collect::<Vec<_>>();

        Self { monitors }
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
}
