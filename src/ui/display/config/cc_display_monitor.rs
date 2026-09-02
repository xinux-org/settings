use gettextrs::dgettext;
use std::{cmp::Ordering, fmt::Display, sync::Arc};

use crate::ui::display::{
    RefreshRate, Resolution,
    color_mode::ColorMode,
    display_mode::RefreshRateMode,
    monitor::{Monitor, MonitorProperties},
    monitor_spec::MonitorSpec,
    transform::Transform,
};

use super::{CcDisplayMode, CcLogicalMonitor};

pub const KNOWN_DIAGONALS: [f64; 3] = [12.1, 13.3, 15.6];

const ROTATIONS: [Transform; 4] = [
    Transform::Normal,
    Transform::Rotate90,
    Transform::Rotate180,
    Transform::Rotate270,
];

#[derive(Debug, Clone)]
pub struct CcDisplayMonitor {
    spec: MonitorSpec,
    properties: MonitorProperties,
    modes: Vec<Arc<CcDisplayMode>>,
    current_mode: Arc<CcDisplayMode>,
    logical_monitor: Option<Arc<CcLogicalMonitor>>,
}

impl CcDisplayMonitor {
    pub fn new(monitor: Monitor, logical_monitor: Option<Arc<CcLogicalMonitor>>) -> Self {
        let modes = monitor
            .modes
            .into_iter()
            .map(CcDisplayMode::from)
            .map(Arc::new)
            .collect::<Vec<_>>();

        let current_mode = modes
            .iter()
            .find(|&mode| mode.is_current())
            .unwrap_or(&modes[0]);

        Self {
            logical_monitor,
            spec: monitor.spec,
            properties: monitor.properties,
            current_mode: Arc::clone(current_mode),
            modes,
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

    pub fn get_geometry(&self) -> Geometry {
        let (x, y) = self
            .logical_monitor
            .as_ref()
            .map(|lm| lm.get_position())
            .unwrap_or((-1, -1));

        let Resolution(width, height) = self.current_mode.get_resolution();

        Geometry {
            x,
            y,
            width,
            height,
        }
    }

    pub fn get_orientation(&self) -> Option<Orientation> {
        self.logical_monitor
            .as_ref()
            .map(|lm| lm.get_transform())
            .map(|transform| Orientation {
                transform,
                ratio: DisplayRatio::from(self.get_geometry()),
            })
    }

    pub fn get_display_name(&self) -> &str {
        match &self.properties.display_name {
            Some(display_name) if !display_name.is_empty() => display_name,
            _ => &self.spec.connector,
        }
    }

    pub fn get_output_ui_name(&self) -> String {
        let display_name = self.get_display_name();

        self.make_display_size_string()
            .map(|size| format!("{display_name} ({size})"))
            .unwrap_or(display_name.to_string())
    }

    pub fn get_current_mode(&self) -> &Arc<CcDisplayMode> {
        &self.current_mode
    }

    pub fn get_modes(&self) -> &[Arc<CcDisplayMode>] {
        &self.modes
    }

    fn make_display_size_string(&self) -> Option<String> {
        match (self.properties.width_mm, self.properties.height_mm) {
            (Some(w), Some(h)) if w > 0 && h > 0 => {
                let d: f64 = (w.pow(2) + h.pow(2)).into();

                Some(Self::diagonal_to_str(d / 25.4))
            }
            _ => None,
        }
    }

    fn diagonal_to_str(d: f64) -> String {
        KNOWN_DIAGONALS
            .iter()
            .find(|&known_d| (*known_d - d).abs() < 0.1)
            .map(|&known_d| format!("{:.1}\"", known_d))
            .unwrap_or_else(|| format!("{:.0}\"", d + 0.5))
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
        let mut compatible_modes = self
            .modes
            .iter()
            .filter(|mode| mode.get_resolution() == current_mode.get_resolution())
            .filter(|mode| mode.get_refresh_rate_mode() == current_mode.get_refresh_rate_mode())
            .collect::<Vec<_>>();

        compatible_modes.sort_by(|a, b| {
            if a.get_refresh_rate_mode() != b.get_refresh_rate_mode() {
                if a.get_refresh_rate_mode() == RefreshRateMode::Variable {
                    return Ordering::Less;
                } else {
                    return Ordering::Greater;
                }
            }

            let delta = (b.get_refresh_rate() - a.get_refresh_rate()) * 1000.0;

            if delta > 0.0 {
                Ordering::Greater
            } else if delta < 0.0 {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
        compatible_modes.dedup_by(|a, b| a.get_refresh_rate() == b.get_refresh_rate());

        compatible_modes
            .into_iter()
            .map(|mode| mode.get_refresh_rate())
            .collect::<Vec<_>>()
    }

    pub fn get_supported_resolutions(&self) -> Vec<Resolution> {
        let mut resolutions = self
            .modes
            .iter()
            .map(|mode| mode.get_resolution())
            .collect::<Vec<_>>();

        resolutions.dedup();
        resolutions.sort_by_key(|resolution| std::cmp::Reverse(resolution.get_area()));

        resolutions
    }

    pub fn get_orientations(&self) -> Vec<Orientation> {
        ROTATIONS
            .map(|transform| Orientation {
                transform,
                ratio: DisplayRatio::from(self.get_geometry()),
            })
            .to_vec()
    }
}

pub struct Geometry {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisplayRatio {
    Square,
    Portrait,
    Landscape,
}

impl From<Geometry> for DisplayRatio {
    fn from(value: Geometry) -> Self {
        if value.width > value.height {
            Self::Landscape
        } else if value.width < value.height {
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
