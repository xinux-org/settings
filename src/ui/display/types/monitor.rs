use std::collections::HashMap;

use zbus::zvariant;

use super::color_mode::ColorMode;
use super::display_mode::{DisplayMode, RawDisplayMode};
use super::monitor_spec::{MonitorSpec, RawMonitorSpec};
use super::rgb_range::RgbRange;

pub type RawMonitor = (
    RawMonitorSpec,
    Vec<RawDisplayMode>,
    HashMap<String, zvariant::OwnedValue>,
);

#[derive(Debug)]
pub struct Monitor {
    pub spec: MonitorSpec,
    // available modes
    pub modes: Vec<DisplayMode>,
    // physical width of monitor in millimeters
    pub width_mm: Option<i32>,
    // physical height of monitor in millimeters
    pub height_mm: Option<i32>,
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

impl From<RawMonitor> for Monitor {
    fn from(value: RawMonitor) -> Self {
        Monitor {
            spec: MonitorSpec::from(value.0),
            modes: value.1.iter().map(DisplayMode::from).collect(),
            width_mm: value
                .2
                .get("width-mm")
                .and_then(|val| val.downcast_ref().ok()),
            height_mm: value
                .2
                .get("height-mm")
                .and_then(|val| val.downcast_ref().ok()),
            is_underscanning: value
                .2
                .get("is-underscanning")
                .and_then(|val| val.downcast_ref().ok()),
            max_screen_size: value
                .2
                .get("max-screen-size")
                .and_then(|val| val.downcast_ref().ok()),
            is_builtin: value
                .2
                .get("is-builtin")
                .and_then(|val| val.downcast_ref().ok()),
            display_name: value
                .2
                .get("display-name")
                .and_then(|val| val.downcast_ref().ok()),
            privacy_screen_state: value
                .2
                .get("privacy-screen-state")
                .and_then(|val| val.downcast_ref().ok()),
            min_refresh_rate: value
                .2
                .get("min-refresh-rate")
                .and_then(|val| val.downcast_ref().ok()),
            is_for_lease: value
                .2
                .get("is-for-lease")
                .and_then(|val| val.downcast_ref().ok()),
            color_mode: value
                .2
                .get("color-mode")
                .and_then(|val| val.downcast_ref::<i32>().ok())
                .map(ColorMode::from),
            supported_color_modes: value
                .2
                .get("supported-color-modes")
                .and_then::<Vec<i32>, _>(|val| {
                    let zvariant::Value::Array(arr) = &**val else {
                        return None;
                    };

                    Some(
                        arr.inner()
                            .iter()
                            .filter_map(|v| v.downcast_ref::<i32>().ok())
                            .collect(),
                    )
                })
                .map(|color_modes| color_modes.iter().map(ColorMode::from).collect()),
            rgb_range: value
                .2
                .get("rgb-range")
                .and_then(|val| val.downcast_ref::<i32>().ok())
                .and_then(|val| RgbRange::try_from(val).ok()),
        }
    }
}

impl From<&RawMonitor> for Monitor {
    fn from(value: &RawMonitor) -> Self {
        Self::from(value.to_owned())
    }
}
