use std::collections::HashMap;

use zbus::zvariant;

pub type RawDisplayMode = (
    String,
    i32,
    i32,
    f64,
    f64,
    Vec<f64>,
    HashMap<String, zvariant::OwnedValue>,
);

#[derive(Debug)]
pub struct DisplayMode {
    // mode ID
    pub id: String,
    // width in physical pixels
    pub width: i32,
    // height in physical pixels
    pub height: i32,
    // refresh rate
    pub refresh_rate: f64,
    // scale preferred as per calculations
    pub preferred_scale: f64,
    // scales supported by this mode
    pub supported_scales: Vec<f64>,
    // the mode is currently active mode
    pub is_current: Option<bool>,
    // the mode is the preferred mode
    pub is_preferred: Option<bool>,
    // the mode is an interlaced mode
    pub is_interlaced: Option<bool>,
    // the refresh rate mode, either "variable" or "fixed" (absence of this means "fixed")
    pub refresh_rate_mode: Option<String>,
}

impl From<RawDisplayMode> for DisplayMode {
    fn from(value: RawDisplayMode) -> Self {
        Self {
            id: value.0,
            width: value.1,
            height: value.2,
            refresh_rate: value.3,
            preferred_scale: value.4,
            supported_scales: value.5,
            is_current: value
                .6
                .get("is-current")
                .and_then(|val| val.downcast_ref().ok()),
            is_preferred: value
                .6
                .get("is-preferred")
                .and_then(|val| val.downcast_ref().ok()),
            is_interlaced: value
                .6
                .get("is-preferred")
                .and_then(|val| val.downcast_ref().ok()),
            refresh_rate_mode: value
                .6
                .get("refresh-rate-mode")
                .and_then(|val| val.downcast_ref().ok()),
        }
    }
}

impl From<&RawDisplayMode> for DisplayMode {
    fn from(value: &RawDisplayMode) -> Self {
        Self::from(value.to_owned())
    }
}
