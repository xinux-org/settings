use crate::ui::display::display_mode::{DisplayMode, RefreshRateMode, approx_equal};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CcDisplayModeFlags {
    Current = 1 << 1,
    Preferred = 1 << 0,
    Interlaced = 1 << 2,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CcDisplayMode {
    pub inner: DisplayMode,
}

impl CcDisplayMode {
    pub fn new(mode: DisplayMode) -> Self {
        Self { inner: mode }
    }

    pub fn new_virtual(
        width: i32,
        height: i32,
        preferred_scale: f64,
        supported_scales: Vec<f64>,
    ) -> Self {
        Self {
            inner: DisplayMode::new_virtual(width, height, preferred_scale, supported_scales),
        }
    }

    pub fn is_clone_mode(&self) -> bool {
        self.inner.is_clone_mode()
    }

    pub fn get_resolution(&self) -> (i32, i32) {
        self.inner.get_resolution()
    }

    pub fn get_freq(&self) -> i32 {
        self.inner.get_freq()
    }

    pub fn get_freq_f(&self) -> f64 {
        self.inner.get_freq_f()
    }

    pub fn is_interlaced(&self) -> bool {
        self.inner.is_interlaced()
    }

    pub fn is_preferred(&self) -> bool {
        self.inner.is_preferred()
    }

    pub fn is_current(&self) -> bool {
        self.inner.is_current()
    }

    pub fn get_refresh_rate_mode(&self) -> RefreshRateMode {
        self.inner.get_refresh_rate_mode()
    }

    pub fn get_supported_scales(&self) -> &[f64] {
        self.inner.get_supported_scales()
    }

    pub fn get_preferred_scale(&self) -> f64 {
        self.inner.get_preferred_scale()
    }

    pub fn is_supported_scale(&self, scale: f64) -> bool {
        self.inner.is_supported_scale(scale)
    }

    pub fn inner(&self) -> &DisplayMode {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut DisplayMode {
        &mut self.inner
    }

    pub fn into_inner(self) -> DisplayMode {
        self.inner
    }
}

pub fn cc_display_same_scale(a: f64, b: f64) -> bool {
    approx_equal(a, b)
}
