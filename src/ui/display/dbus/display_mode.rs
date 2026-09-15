use serde::{Deserialize, Serialize};
use zbus::zvariant::{DeserializeDict, Type};

#[derive(Deserialize, Type, Debug, Clone, PartialEq)]
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
    pub properties: DisplayModeProperties,
}

#[derive(DeserializeDict, Type, Debug, Clone, PartialEq, Default)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct DisplayModeProperties {
    // the mode is currently active mode
    // (mutter only emits this key when true)
    pub is_current: Option<bool>,
    // the mode is the preferred mode
    // (mutter only emits this key when true)
    pub is_preferred: Option<bool>,
    // the mode is an interlaced mode
    // (mutter only emits this key when true)
    pub is_interlaced: Option<bool>,
    // the refresh rate mode, either "variable" or "fixed" (absence of this means "fixed")
    pub refresh_rate_mode: Option<RefreshRateMode>,
}

#[derive(Deserialize, Serialize, Type, Default, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[zvariant(signature = "s")]
pub enum RefreshRateMode {
    #[default]
    Fixed,
    Variable,
}
