use serde::{Deserialize, Serialize};
use zbus::zvariant::{DeserializeDict, Type};

pub fn approx_equal(a: f64, b: f64) -> bool {
    approx::abs_diff_eq!(a, b, epsilon = 0.01)
}

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
#[zvariant(signature = "s", rename_all = "kebab-case")]
pub enum RefreshRateMode {
    #[default]
    Fixed,
    Variable,
}

impl DisplayMode {
    pub fn new_virtual(
        width: i32,
        height: i32,
        preferred_scale: f64,
        supported_scales: Vec<f64>,
    ) -> Self {
        Self {
            width,
            height,
            preferred_scale,
            supported_scales,
            id: String::new(),
            refresh_rate: 0.0,
            properties: DisplayModeProperties::default(),
        }
    }

    pub fn is_clone_mode(&self) -> bool {
        self.id.is_empty()
    }

    pub fn get_resolution(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    pub fn get_freq(&self) -> i32 {
        self.refresh_rate.round() as i32
    }

    pub fn get_freq_f(&self) -> f64 {
        self.refresh_rate
    }

    pub fn same_resolution(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }

    pub fn is_interlaced(&self) -> bool {
        self.properties.is_interlaced.unwrap_or(false)
    }

    pub fn is_preferred(&self) -> bool {
        self.properties.is_preferred.unwrap_or(false)
    }

    pub fn is_current(&self) -> bool {
        self.properties.is_current.unwrap_or(false)
    }

    pub fn get_refresh_rate_mode(&self) -> RefreshRateMode {
        self.properties.refresh_rate_mode.unwrap_or_default()
    }

    pub fn get_supported_scales(&self) -> &[f64] {
        &self.supported_scales
    }

    pub fn get_preferred_scale(&self) -> f64 {
        self.preferred_scale
    }

    pub fn is_supported_scale(&self, scale: f64) -> bool {
        self.supported_scales
            .iter()
            .any(|&sup_scale| approx_equal(sup_scale, scale))
    }
}
