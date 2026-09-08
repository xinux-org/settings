use std::{cmp, fmt::Display};

use crate::ui::display::display_mode::{DisplayMode, RefreshRateMode};

use super::GetList;

#[derive(Debug, Clone, PartialEq)]
pub struct CcDisplayMode {
    inner: DisplayMode,
}

impl From<DisplayMode> for CcDisplayMode {
    fn from(inner: DisplayMode) -> Self {
        Self { inner }
    }
}

impl CcDisplayMode {
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

    pub fn get_refresh_rate_mode(&self) -> RefreshRateMode {
        self.inner.properties.refresh_rate_mode.unwrap_or_default()
    }

    pub fn get_preferred_scale(&self) -> Scale {
        println!("{:?}", self.inner.preferred_scale);

        Scale::from(self.inner.preferred_scale)
    }
}

impl GetList<Scale> for CcDisplayMode {
    fn get_list(&self) -> Vec<Scale> {
        self.inner
            .supported_scales
            .iter()
            .map(|&scale| Scale::from(scale))
            .collect::<Vec<_>>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Resolution(pub i32, pub i32);

impl Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(aspect) = self.get_aspect_ratio() {
            return f.write_fmt(format_args!("{} × {} ({})", self.0, self.1, aspect));
        }

        f.write_fmt(format_args!("{} × {}", self.0, self.1))
    }
}

impl Resolution {
    pub fn get_aspect_ratio(&self) -> Option<&str> {
        let ratio = if self.0 > self.1 {
            self.0 * 10 / self.1
        } else {
            self.1 * 10 / self.0
        };

        match ratio {
            10 => Some("1:1"),
            12 => Some("5:4"),
            13 => Some("4:3"),
            15 => Some("3:2"),
            16 => Some("16:10"),
            17 => Some("16:9"),
            18 => Some("9:5"),
            23 => Some("21:9"),
            35 => Some("32:9"),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RefreshRate(pub f64);

impl Display for RefreshRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:.2} Hz", self.0))
    }
}

impl Eq for RefreshRate {}

impl PartialOrd for RefreshRate {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.cmp(other).into()
    }
}

impl Ord for RefreshRate {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self.0 > other.0 {
            cmp::Ordering::Greater
        } else if self.0 < other.0 {
            cmp::Ordering::Less
        } else {
            cmp::Ordering::Equal
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Scale(pub f64);

impl From<f64> for Scale {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:.0} %", (self.0 * 100.0).trunc()))
    }
}

impl Default for Scale {
    fn default() -> Self {
        Self(1.0)
    }
}
