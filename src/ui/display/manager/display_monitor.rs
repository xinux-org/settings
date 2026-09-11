use std::cmp;

use crate::ui::display::dbus;

use super::{
    DisplayMode, LogicalMonitor, DisplayRatio, GetList, GetListVia, Orientation, RefreshRate,
    Resolution, Scale,
};

#[derive(Debug, Clone)]
pub struct DisplayMonitor {
    modes: Vec<DisplayMode>,
    properties: dbus::MonitorProperties,
    logical_monitor: Option<LogicalMonitor>,
}

impl DisplayMonitor {
    pub fn new(monitor: dbus::Monitor, logical_monitor: Option<LogicalMonitor>) -> Self {
        Self {
            logical_monitor,
            properties: monitor.properties,
            modes: monitor
                .modes
                .into_iter()
                .map(DisplayMode::from)
                .collect::<Vec<_>>(),
        }
    }

    pub fn get_logical_monitor(&self) -> Option<&LogicalMonitor> {
        self.logical_monitor.as_ref()
    }

    pub fn get_orientation(&self) -> Orientation {
        Orientation::new(
            DisplayRatio::from(self.get_current_mode().get_resolution()),
            self.get_logical_monitor()
                .map(|lm| lm.get_transform())
                .unwrap_or_default(),
        )
    }

    pub fn get_current_mode(&self) -> &DisplayMode {
        self.modes
            .iter()
            .find(|&mode| mode.is_current())
            .unwrap_or(&self.modes[0])
    }

    pub fn get_mode_by_resolution(&self, resolution: Resolution) -> DisplayMode {
        self.modes
            .iter()
            .find(|&mode| mode.get_resolution() == resolution)
            .unwrap_or(&self.modes[0])
            .clone()
    }

    pub fn is_hdr(&self) -> Option<bool> {
        self.properties
            .supported_color_modes
            .as_ref()
            .map(|color_modes| color_modes.contains(&dbus::ColorMode::BT2100))
            .and_then(|has_hdr| {
                if has_hdr {
                    self.properties
                        .color_mode
                        .map(|color_mode| color_mode == dbus::ColorMode::BT2100)
                } else {
                    None
                }
            })
    }

    pub fn is_underscanning(&self) -> Option<bool> {
        self.properties.is_underscanning
    }
}

impl GetList<Orientation> for DisplayMonitor {
    fn get_list(&self) -> Vec<Orientation> {
        let rotations = const {
            [
                dbus::Transform::Normal,
                dbus::Transform::Rotate90,
                dbus::Transform::Rotate180,
                dbus::Transform::Rotate270,
            ]
        };

        rotations
            .map(|transform| {
                Orientation::new(
                    DisplayRatio::from(self.get_current_mode().get_resolution()),
                    transform,
                )
            })
            .to_vec()
    }
}

impl GetList<Resolution> for DisplayMonitor {
    fn get_list(&self) -> Vec<Resolution> {
        let mut resolutions = self
            .modes
            .iter()
            .map(|mode| mode.get_resolution())
            .collect::<Vec<_>>();

        resolutions.dedup();
        resolutions.sort_by_key(|&resolution| cmp::Reverse(resolution));

        resolutions
    }
}

impl GetList<RefreshRate> for DisplayMonitor {
    fn get_list(&self) -> Vec<RefreshRate> {
        let current_mode = self.get_current_mode();
        self.get_list_via(current_mode)
    }
}

impl GetListVia<RefreshRate, DisplayMode> for DisplayMonitor {
    fn get_list_via(&self, current_mode: &DisplayMode) -> Vec<RefreshRate> {
        let mut refresh_rates = self
            .modes
            .iter()
            .filter(|mode| mode.get_resolution() == current_mode.get_resolution())
            .filter(|mode| mode.get_refresh_rate_mode() == current_mode.get_refresh_rate_mode())
            .map(|mode| mode.get_refresh_rate())
            .collect::<Vec<_>>();

        refresh_rates.dedup();
        refresh_rates.sort_by_key(|&refresh_rate| cmp::Reverse(refresh_rate));

        refresh_rates
    }
}

impl GetList<Scale> for DisplayMonitor {
    fn get_list(&self) -> Vec<Scale> {
        let current_mode = self.get_current_mode();
        self.get_list_via(current_mode)
    }
}

impl GetListVia<Scale, DisplayMode> for DisplayMonitor {
    fn get_list_via(&self, current_mode: &DisplayMode) -> Vec<Scale> {
        current_mode.get_list()
    }
}
