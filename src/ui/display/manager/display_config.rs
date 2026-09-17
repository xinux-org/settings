use std::rc::Rc;

use crate::ui::display::dbus::DisplayState;

use super::{DisplayMonitor, LogicalMonitor};

pub enum DisplayConfigType {
    Single(Rc<DisplayMonitor>),
}

#[derive(Debug)]
pub struct DisplayConfig {
    monitors: Vec<Rc<DisplayMonitor>>,
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
            .map(Rc::new)
            .collect::<Vec<_>>();

        Self { monitors }
    }
}

impl DisplayConfig {
    pub fn get_monitor(&self) -> DisplayConfigType {
        if self.monitors.len() == 1
            && let Some(monitor) = self.monitors.first()
        {
            return DisplayConfigType::Single(Rc::clone(monitor));
        }

        // This won't happen.
        // Display config manager garrants safety
        // Else cases implements in the future
        unimplemented!()
    }
}
