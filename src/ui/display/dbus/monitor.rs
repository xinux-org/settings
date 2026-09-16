use serde::Deserialize;
use zbus::zvariant::{DeserializeDict, Type};

use super::DisplayMode;

#[derive(Deserialize, Type, Debug, Copy, Clone, PartialEq, Eq)]
pub enum RgbRange {
    Auto = 1,
    Full = 2,
    Limited = 3,
}

#[derive(Deserialize, Type, Debug, Copy, Clone, PartialEq, Eq)]
pub enum ColorMode {
    Default = 0,
    BT2100 = 1,
    SDRNative = 2,
}

#[derive(Deserialize, Type, Debug, Clone, Eq, PartialEq)]
pub struct MonitorSpec {
    // connector name (e.g. HDMI-1, DP-1, etc)
    pub connector: String,
    // vendor name
    pub vendor: String,
    // product name
    pub product: String,
    // product serial
    pub serial: String,
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
