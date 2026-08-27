use std::fmt::Display;

use serde::Deserialize;
use zbus::zvariant::{DeserializeDict, Type};

use super::color_mode::ColorMode;
use super::display_mode::{DisplayMode, RefreshRateMode};
use super::monitor_spec::MonitorSpec;
use super::rgb_range::RgbRange;

pub const KNOWN_DIAGONALS: [f64; 3] = [12.1, 13.3, 15.6];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(DeserializeDict, Type, Debug, Clone, PartialEq, Default)]
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
        f.write_str(&self.get_output_ui_name())
    }
}

// TODO: move into CcMonitorConfig
pub(crate) fn compare_mode_preference(
    left: &&DisplayMode,
    right: &&DisplayMode,
) -> std::cmp::Ordering {
    left.refresh_rate
        .partial_cmp(&right.refresh_rate)
        .unwrap_or(std::cmp::Ordering::Equal)
        .then_with(|| right.is_interlaced().cmp(&left.is_interlaced()))
}

impl Monitor {
    pub fn get_display_name(&self) -> &str {
        match &self.properties.display_name {
            Some(display_name) if !display_name.is_empty() => display_name,
            _ => &self.spec.connector,
        }
    }

    pub fn get_vendor_name(&self) -> &str {
        &self.spec.vendor
    }

    pub fn get_product_name(&self) -> &str {
        &self.spec.product
    }

    pub fn get_product_serial(&self) -> &str {
        &self.spec.serial
    }

    pub fn get_connector_name(&self) -> &str {
        &self.spec.connector
    }

    pub fn get_physical_size(&self) -> (i32, i32) {
        (
            self.properties.width_mm.unwrap_or(0).min(i32::MAX as u32) as i32,
            self.properties.height_mm.unwrap_or(0).min(i32::MAX as u32) as i32,
        )
    }

    pub fn diagonal_to_str(d: f64) -> String {
        for &known_diagonal in &KNOWN_DIAGONALS {
            let delta = (known_diagonal - d).abs();

            if delta < 0.1 {
                return format!("{:.1}\"", known_diagonal);
            }
        }

        format!("{}\"", (d + 0.5) as i32)
    }

    pub fn make_display_size_string(&self) -> Option<String> {
        let (w, h) = self.get_physical_size();

        if w > 0 && h > 0 {
            let d = ((w as f64).powi(2) + (h as f64).powi(2)).sqrt() / 25.4;

            Some(Self::diagonal_to_str(d))
        } else {
            None
        }
    }

    pub fn get_output_ui_name(&self) -> String {
        if let Some(size) = self.make_display_size_string() {
            format!("{} ({})", self.get_display_name(), size)
        } else {
            self.get_display_name().to_string()
        }
    }

    pub fn is_builtin(&self) -> bool {
        self.properties.is_builtin.unwrap_or(false)
    }

    pub fn supports_variable_refresh_rate(&self) -> bool {
        self.properties.min_refresh_rate.is_some_and(|r| r > 0)
            || self
                .modes
                .iter()
                .any(|m| m.get_refresh_rate_mode() == RefreshRateMode::Variable)
    }

    pub fn get_min_freq(&self) -> i32 {
        self.properties.min_refresh_rate.unwrap_or(0)
    }

    pub fn get_supported_color_modes(&self) -> &[ColorMode] {
        self.properties
            .supported_color_modes
            .as_deref()
            .unwrap_or(&[])
    }

    pub fn supports_color_mode(&self, color_mode: ColorMode) -> bool {
        self.get_supported_color_modes().contains(&color_mode)
    }

    pub fn get_color_mode(&self) -> ColorMode {
        self.properties.color_mode.unwrap_or(ColorMode::Default)
    }

    pub fn supports_underscanning(&self) -> bool {
        self.properties.is_underscanning.is_some()
    }

    pub fn get_underscanning(&self) -> bool {
        self.properties.is_underscanning.unwrap_or(false)
    }

    pub fn is_for_lease(&self) -> bool {
        self.properties.is_for_lease.unwrap_or(false)
    }
}
