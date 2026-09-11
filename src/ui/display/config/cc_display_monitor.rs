use std::cmp;

use crate::ui::display::{
    color_mode::ColorMode,
    monitor::{Monitor, MonitorProperties},
    transform::Transform,
};

use super::{
    CcDisplayMode, CcLogicalMonitor, DisplayRatio, GetList, GetListVia, Orientation, RefreshRate,
    Resolution, Scale,
};

#[derive(Debug, Clone)]
pub struct CcDisplayMonitor {
    modes: Vec<CcDisplayMode>,
    properties: MonitorProperties,
    logical_monitor: Option<CcLogicalMonitor>,
}

impl CcDisplayMonitor {
    pub fn new(monitor: Monitor, logical_monitor: Option<CcLogicalMonitor>) -> Self {
        Self {
            logical_monitor,
            properties: monitor.properties,
            modes: monitor
                .modes
                .into_iter()
                .map(CcDisplayMode::from)
                .collect::<Vec<_>>(),
        }
    }

    pub fn get_logical_monitor(&self) -> Option<&CcLogicalMonitor> {
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

    pub fn get_current_mode(&self) -> &CcDisplayMode {
        self.modes
            .iter()
            .find(|&mode| mode.is_current())
            .unwrap_or(&self.modes[0])
    }

    pub fn get_mode_by_resolution(&self, resolution: Resolution) -> CcDisplayMode {
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
            .map(|color_modes| color_modes.contains(&ColorMode::BT2100))
            .and_then(|has_hdr| {
                if has_hdr {
                    self.properties
                        .color_mode
                        .map(|color_mode| color_mode == ColorMode::BT2100)
                } else {
                    None
                }
            })
    }

    pub fn is_underscanning(&self) -> Option<bool> {
        self.properties.is_underscanning
    }
}

impl GetList<Orientation> for CcDisplayMonitor {
    fn get_list(&self) -> Vec<Orientation> {
        let rotations = const {
            [
                Transform::Normal,
                Transform::Rotate90,
                Transform::Rotate180,
                Transform::Rotate270,
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

impl GetList<Resolution> for CcDisplayMonitor {
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

impl GetList<RefreshRate> for CcDisplayMonitor {
    fn get_list(&self) -> Vec<RefreshRate> {
        let current_mode = self.get_current_mode();
        self.get_list_via(current_mode)
    }
}

impl GetListVia<RefreshRate, CcDisplayMode> for CcDisplayMonitor {
    fn get_list_via(&self, current_mode: &CcDisplayMode) -> Vec<RefreshRate> {
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

impl GetList<Scale> for CcDisplayMonitor {
    fn get_list(&self) -> Vec<Scale> {
        let current_mode = self.get_current_mode();
        self.get_list_via(current_mode)
    }
}

impl GetListVia<Scale, CcDisplayMode> for CcDisplayMonitor {
    fn get_list_via(&self, current_mode: &CcDisplayMode) -> Vec<Scale> {
        current_mode.get_list()
    }
}
