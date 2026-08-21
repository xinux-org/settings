use std::fmt::Display;

use serde::Deserialize;
use zbus::zvariant::{DeserializeDict, Type};

use super::color_mode::ColorMode;
use super::display_mode::DisplayMode;
use super::logical_monitor::LogicalMonitor;
use super::monitor_spec::MonitorSpec;
use super::rgb_range::RgbRange;

pub const KNOWN_DIAGONALS: [f32; 3] = [12.1, 13.3, 15.6];

#[derive(Debug)]
pub struct Geometry {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Deserialize, Type, Debug, Clone, PartialEq)]
pub struct Monitor {
    pub spec: MonitorSpec,
    // available modes
    pub modes: Vec<DisplayMode>,
    pub properties: MonitorProperties,
}

#[derive(DeserializeDict, Type, Debug, Clone, PartialEq)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct MonitorProperties {
    // physical width of monitor in millimeters
    pub width_mm: Option<u32>,
    // physical height of monitor in millimeters
    pub height_mm: Option<u32>,
    // whether underscanning is enabled (absence of this means underscanning not being supported)
    pub is_underscanning: Option<bool>,
    // the maximum size a screen may have (absence of this means unlimited screen size)
    pub max_screen_size: Option<(i32, i32)>,
    // whether the monitor is built in, e.g. a laptop panel (absence of this means it is not built in)
    pub is_builtin: Option<bool>,
    // a human readable display name of the monitor
    pub display_name: Option<String>,
    // the state of the privacy screen (absence of this means it is not being supported)
    // first value indicates whether it's enabled and
    // second value whether it's hardware locked (and so can't be changed via gsettings)
    pub privacy_screen_state: Option<(bool, bool)>,
    // minimum refresh rate of monitor when Variable Refresh Rate is active (absence of this means unknown)
    pub min_refresh_rate: Option<i32>,
    // whether the monitor is for lease or not
    pub is_for_lease: Option<bool>,
    // current color mode
    pub color_mode: Option<ColorMode>,
    // list of supported color modes
    pub supported_color_modes: Option<Vec<ColorMode>>,
    // current RGB quantization range
    pub rgb_range: Option<RgbRange>,
}

impl Display for Monitor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.get_output_ui_name()))
    }
}

impl Monitor {
    pub fn get_optimal_mode(&self) -> &DisplayMode {
        self.try_get_optimal_mode().unwrap()
    }

    pub fn try_get_optimal_mode(&self) -> Option<&DisplayMode> {
        self.get_current_mode()
            .or_else(|| self.get_preferred_mode())
            .or_else(|| self.modes.first())
    }

    pub fn get_current_mode(&self) -> Option<&DisplayMode> {
        self.modes
            .iter()
            .find(|m| m.properties.is_current.is_some_and(|val| val))
    }

    pub fn get_preferred_mode(&self) -> Option<&DisplayMode> {
        self.modes
            .iter()
            .find(|m| m.properties.is_preferred.is_some_and(|val| val))
    }

    pub fn get_output_ui_name(&self) -> String {
        if let Some(size) = self.make_display_size_string() {
            return format!("{} ({})", self.get_display_name(), size);
        }

        self.get_display_name().to_string()
    }

    pub fn get_physical_size(&self) -> Option<u32> {
        self.properties.width_mm.or(self.properties.height_mm)
    }

    pub fn get_display_name(&self) -> &String {
        if let Some(display_name) = &self.properties.display_name {
            return display_name;
        }

        &self.spec.connector
    }

    pub fn get_geometry(&self, logical_monitor: Option<&LogicalMonitor>) -> Geometry {
        let (x, y) = match logical_monitor {
            Some(lm) => (lm.x, lm.y),
            None => (-1, -1),
        };

        let mode = self.try_get_optimal_mode();

        let (width, height) = match mode {
            Some(mode) => (mode.width, mode.height),
            None => {
                tracing::warn!("Monitor at {} has no modes?", self.spec.connector);

                (-1, -1)
            }
        };

        Geometry {
            x,
            y,
            width,
            height,
        }
    }

    fn diagonal_to_str(&self, d: f32) -> String {
        for known_diagonal in KNOWN_DIAGONALS {
            let delta = (known_diagonal - d).abs();

            if delta < 0.1 {
                return format!("{}\"", known_diagonal);
            }
        }

        format!("{}\"", d + 0.5)
    }

    fn make_display_size_string(&self) -> Option<String> {
        if let (Some(width), Some(height)) = (self.properties.width_mm, self.properties.height_mm) {
            if width > 0 && height > 0 {
                let d = (width * height + height * height).isqrt() as f32;

                return Some(self.diagonal_to_str(d / 25.4));
            }

            return None;
        }

        None
    }
}
