use gettextrs::dgettext;
use std::{cmp, fmt::Display};

use crate::ui::display::{
    GetListVia, RefreshRate, Resolution, Scale,
    color_mode::ColorMode,
    monitor::{Monitor, MonitorProperties},
    monitor_spec::MonitorSpec, transform::Transform,
};

use super::{CcDisplayMode, CcLogicalMonitor, GetList};

const ROTATIONS: [Transform; 4] = [
    Transform::Normal,
    Transform::Rotate90,
    Transform::Rotate180,
    Transform::Rotate270,
];

#[derive(Debug, Clone, PartialEq)]
pub struct CcDisplayMonitor {
    spec: MonitorSpec,
    modes: Vec<CcDisplayMode>,
    properties: MonitorProperties,
    logical_monitor: Option<CcLogicalMonitor>,
}

impl CcDisplayMonitor {
    pub fn new(monitor: Monitor, logical_monitor: Option<CcLogicalMonitor>) -> Self {
        Self {
            logical_monitor,
            spec: monitor.spec,
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
        Orientation {
            ratio: DisplayRatio::from(self.get_current_mode().get_resolution()),
            transform: self
                .get_logical_monitor()
                .map(|lm| lm.get_transform())
                .unwrap_or_default(),
        }
    }

    pub fn get_current_mode(&self) -> &CcDisplayMode {
        self.modes
            .iter()
            .find(|&mode| mode.is_current())
            .unwrap_or(&self.modes[0])
    }

    pub fn get_modes(&self) -> &[CcDisplayMode] {
        &self.modes
    }

    pub fn get_mode_by_resolution(&self, resolution: Resolution) -> CcDisplayMode {
        self.modes
            .iter()
            .find(|&mode| mode.get_resolution() == resolution)
            .unwrap_or(&self.modes[0])
            .clone()
    }

    pub fn get_color_mode(&self) -> ColorMode {
        self.properties.color_mode.unwrap_or_default()
    }

    pub fn get_spec(&self) -> &MonitorSpec {
        &self.spec
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

    pub fn is_builtin(&self) -> bool {
        self.properties.is_builtin.unwrap_or_default()
    }

    pub fn is_for_lease(&self) -> bool {
        self.properties.is_for_lease.unwrap_or_default()
    }

    pub fn is_underscanning(&self) -> Option<bool> {
        self.properties.is_underscanning
    }

    pub fn set_current_mode(&mut self, resolution: Resolution) {
        self.modes.iter_mut().for_each(|mode| {
            mode.set_current(mode.get_resolution() == resolution);
        });
    }

    pub fn set_color_mode(&mut self, is_hdr: bool) {
        if self
            .properties
            .supported_color_modes
            .as_ref()
            .is_some_and(|color_modes| color_modes.contains(&ColorMode::BT2100))
        {
            self.properties.color_mode = Some(if is_hdr {
                ColorMode::BT2100
            } else {
                ColorMode::Default
            });
        }
    }

    pub fn set_refresh_rate(&mut self, refresh_rate: RefreshRate) {
        let current_mode = self.get_current_mode().clone();

        self.modes.iter_mut().for_each(|mode| {
            if mode.is_compatible(&current_mode) {
                mode.set_current(mode.get_refresh_rate() == refresh_rate);
            }
        });
    }

    pub fn set_scale(&mut self, scale: Scale) {
        if let Some(lm) = self.logical_monitor.as_mut() {
            lm.set_scale(scale)
        }
    }

    pub fn set_underscanning(&mut self, is_underscanning: bool) {
        if self.properties.is_underscanning.is_some() {
            self.properties.is_underscanning = Some(is_underscanning);
        }
    }
}

impl GetList<Orientation> for CcDisplayMonitor {
    fn get_list(&self) -> Vec<Orientation> {
        ROTATIONS
            .map(|transform| Orientation {
                transform,
                ratio: DisplayRatio::from(self.get_current_mode().get_resolution()),
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
            .filter(|mode| mode.is_compatible(current_mode))
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisplayRatio {
    Square,
    Portrait,
    Landscape,
}

impl From<Resolution> for DisplayRatio {
    fn from(value: Resolution) -> Self {
        if value.0 > value.1 {
            Self::Landscape
        } else if value.0 < value.1 {
            Self::Portrait
        } else {
            Self::Square
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Orientation {
    ratio: DisplayRatio,
    transform: Transform,
}

impl Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self.ratio {
            DisplayRatio::Landscape => match self.transform {
                Transform::Normal | Transform::Flipped180 => {
                    dgettext("Display rotation", "Landscape")
                }
                Transform::Rotate90 | Transform::Flipped270 => {
                    dgettext("Display rotation", "Portrait Right")
                }
                Transform::Rotate270 | Transform::Flipped90 => {
                    dgettext("Display rotation", "Portrait Left")
                }
                Transform::Rotate180 | Transform::Flipped => {
                    dgettext("Display rotation", "Landscape (flipped)")
                }
            },
            DisplayRatio::Portrait => match self.transform {
                Transform::Normal | Transform::Flipped180 => {
                    dgettext("Display rotation", "Portrait")
                }
                Transform::Rotate90 | Transform::Flipped270 => {
                    dgettext("Display rotation", "Landscape Right")
                }
                Transform::Rotate270 | Transform::Flipped90 => {
                    dgettext("Display rotation", "Landscape Left")
                }
                Transform::Rotate180 | Transform::Flipped => {
                    dgettext("Display rotation", "Portrait (flipped)")
                }
            },
            DisplayRatio::Square => match self.transform {
                Transform::Normal | Transform::Flipped180 => {
                    dgettext("Display rotation", "Upright")
                }
                Transform::Rotate90 | Transform::Flipped270 => {
                    dgettext("Display rotation", "Right")
                }
                Transform::Rotate270 | Transform::Flipped90 => dgettext("Display rotation", "Left"),
                Transform::Rotate180 | Transform::Flipped => {
                    dgettext("Display rotation", "Flipped")
                }
            },
        };

        f.write_str(&label)
    }
}
