use enumflags2::{BitFlags, bitflags};
use std::fmt::Display;

use crate::ui::display::display_mode::{DisplayMode, RefreshRateMode};

#[repr(u8)]
#[bitflags]
#[derive(Copy, Clone, Debug, PartialEq)]
enum CcDisplayModeFlags {
    Current = 1 << 1,
    Preferred = 1 << 0,
    Interlaced = 1 << 2,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CcDisplayMode {
    inner: DisplayMode,
    flags: BitFlags<CcDisplayModeFlags>,
}

impl From<DisplayMode> for CcDisplayMode {
    fn from(inner: DisplayMode) -> Self {
        let mut flags = BitFlags::empty();

        if inner.properties.is_current.is_some_and(|x| x) {
            flags |= CcDisplayModeFlags::Current;
        }

        if inner.properties.is_preferred.is_some_and(|x| x) {
            flags |= CcDisplayModeFlags::Preferred;
        }

        if inner.properties.is_interlaced.is_some_and(|x| x) {
            flags |= CcDisplayModeFlags::Interlaced;
        }

        Self { inner, flags }
    }
}

impl CcDisplayMode {
    pub fn get_id(&self) -> &str {
        &self.inner.id
    }

    pub fn is_current(&self) -> bool {
        self.flags.contains(CcDisplayModeFlags::Current)
    }

    pub fn is_preferred(&self) -> bool {
        self.flags.contains(CcDisplayModeFlags::Preferred)
    }

    pub fn is_interlaced(&self) -> bool {
        self.flags.contains(CcDisplayModeFlags::Interlaced)
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
        Scale::from(self.inner.preferred_scale)
    }

    pub fn get_supported_scales(&self) -> Vec<Scale> {
        self.inner
            .supported_scales
            .iter()
            .map(|&scale| Scale::from(scale))
            .collect::<Vec<_>>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

    pub fn get_area(&self) -> i32 {
        self.0 * self.1
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RefreshRate(pub f64);

impl std::ops::Sub for RefreshRate {
    type Output = f64;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Display for RefreshRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:.2} Hz", self.0))
    }
}

impl PartialEq for RefreshRate {
    fn eq(&self, other: &Self) -> bool {
        approx::relative_eq!(self.0, other.0, max_relative = 0.01)
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Scale(pub f64);

impl From<f64> for Scale {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:.0} %", self.0 * 100.0))
    }
}

impl PartialEq for Scale {
    fn eq(&self, other: &Self) -> bool {
        approx::relative_eq!(self.0, other.0, max_relative = 0.01)
    }
}
