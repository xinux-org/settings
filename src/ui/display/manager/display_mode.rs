use crate::ui::display::dbus;

use super::{GetList, RefreshRate, Resolution, Scale};

#[derive(Debug, Clone, PartialEq)]
pub struct DisplayMode {
    inner: dbus::DisplayMode,
}

impl From<dbus::DisplayMode> for DisplayMode {
    fn from(inner: dbus::DisplayMode) -> Self {
        Self { inner }
    }
}

impl DisplayMode {
    pub fn get_id(&self) -> &str {
        &self.inner.id
    }

    pub fn is_current(&self) -> bool {
        self.inner.properties.is_current.unwrap_or_default()
    }

    pub fn set_current(&mut self, is: bool) {
        self.inner.properties.is_current = Some(is);
    }

    pub fn is_preferred(&self) -> bool {
        self.inner.properties.is_preferred.unwrap_or_default()
    }

    pub fn is_interlaced(&self) -> bool {
        self.inner.properties.is_interlaced.unwrap_or_default()
    }

    pub fn get_resolution(&self) -> Resolution {
        Resolution(self.inner.width, self.inner.height)
    }

    pub fn get_refresh_rate(&self) -> RefreshRate {
        RefreshRate(self.inner.refresh_rate)
    }

    pub fn get_refresh_rate_mode(&self) -> dbus::RefreshRateMode {
        self.inner.properties.refresh_rate_mode.unwrap_or_default()
    }

    pub fn get_preferred_scale(&self) -> Scale {
        Scale::from(self.inner.preferred_scale)
    }
}

impl GetList<Scale> for DisplayMode {
    fn get_list(&self) -> Vec<Scale> {
        self.inner
            .supported_scales
            .iter()
            .map(|&scale| Scale::from(scale))
            .collect::<Vec<_>>()
    }
}
