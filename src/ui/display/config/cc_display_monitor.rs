use gettextrs::dgettext;
use std::{cmp, fmt::Display, sync::Arc};

use crate::ui::display::{
    RefreshRate, Resolution,
    color_mode::ColorMode,
    monitor::{Monitor, MonitorProperties},
    transform::Transform,
};

use super::{CcDisplayMode, CcLogicalMonitor};

const ROTATIONS: [Transform; 4] = [
    Transform::Normal,
    Transform::Rotate90,
    Transform::Rotate180,
    Transform::Rotate270,
];

#[derive(Debug, Clone)]
pub struct CcDisplayMonitor {
    properties: MonitorProperties,
    modes: Vec<Arc<CcDisplayMode>>,
    logical_monitor: Option<Arc<CcLogicalMonitor>>,
}

impl CcDisplayMonitor {
    pub fn new(monitor: Monitor, logical_monitor: Option<Arc<CcLogicalMonitor>>) -> Self {
        Self {
            logical_monitor,
            properties: monitor.properties,
            modes: monitor
                .modes
                .into_iter()
                .map(CcDisplayMode::from)
                .map(Arc::new)
                .collect::<Vec<_>>(),
        }
    }

    pub fn logical_monitor_for<'a>(
        monitor: &'a Monitor,
        logical_monitors: &'a [Arc<CcLogicalMonitor>],
    ) -> Option<Arc<CcLogicalMonitor>> {
        logical_monitors
            .iter()
            .find(|&lm| lm.has_output(&monitor.spec))
            .map(Arc::clone)
    }

    pub fn get_logical_monitor(&self) -> Option<&Arc<CcLogicalMonitor>> {
        self.logical_monitor.as_ref()
    }

    pub fn is_builtin(&self) -> bool {
        self.properties.is_builtin.unwrap_or(false)
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

    pub fn get_current_mode(&self) -> &Arc<CcDisplayMode> {
        self.modes
            .iter()
            .find(|&mode| mode.is_current())
            .unwrap_or(&self.modes[0])
    }

    pub fn get_modes(&self) -> &[Arc<CcDisplayMode>] {
        &self.modes
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

    pub fn get_supported_refresh_rates(
        &self,
        current_mode: &Arc<CcDisplayMode>,
    ) -> Vec<RefreshRate> {
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

    pub fn get_supported_resolutions(&self) -> Vec<Resolution> {
        let mut resolutions = self
            .modes
            .iter()
            .map(|mode| mode.get_resolution())
            .collect::<Vec<_>>();

        resolutions.dedup();
        resolutions.sort_by_key(|&resolution| cmp::Reverse(resolution));

        resolutions
    }

    pub fn get_orientations(&self) -> Vec<Orientation> {
        ROTATIONS
            .map(|transform| Orientation {
                transform,
                ratio: DisplayRatio::from(self.get_current_mode().get_resolution()),
            })
            .to_vec()
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
