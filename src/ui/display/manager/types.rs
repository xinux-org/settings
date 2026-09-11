use gettextrs::dgettext;
use std::{cmp, fmt::Display};

use crate::ui::display::dbus::Transform;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisplayRatio {
    Square,
    Portrait,
    Landscape,
}

impl From<Resolution> for DisplayRatio {
    fn from(value: Resolution) -> Self {
        if value.0 > value.1 {
            Self::Landscape
        } else if value.0 < value.1 {
            Self::Portrait
        } else {
            Self::Square
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Orientation {
    ratio: DisplayRatio,
    transform: Transform,
}

impl Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self.ratio {
            DisplayRatio::Landscape => match self.transform {
                Transform::Normal | Transform::Flipped180 => {
                    dgettext("Display rotation", "Landscape")
                }
                Transform::Rotate90 | Transform::Flipped270 => {
                    dgettext("Display rotation", "Portrait Right")
                }
                Transform::Rotate270 | Transform::Flipped90 => {
                    dgettext("Display rotation", "Portrait Left")
                }
                Transform::Rotate180 | Transform::Flipped => {
                    dgettext("Display rotation", "Landscape (flipped)")
                }
            },
            DisplayRatio::Portrait => match self.transform {
                Transform::Normal | Transform::Flipped180 => {
                    dgettext("Display rotation", "Portrait")
                }
                Transform::Rotate90 | Transform::Flipped270 => {
                    dgettext("Display rotation", "Landscape Right")
                }
                Transform::Rotate270 | Transform::Flipped90 => {
                    dgettext("Display rotation", "Landscape Left")
                }
                Transform::Rotate180 | Transform::Flipped => {
                    dgettext("Display rotation", "Portrait (flipped)")
                }
            },
            DisplayRatio::Square => match self.transform {
                Transform::Normal | Transform::Flipped180 => {
                    dgettext("Display rotation", "Upright")
                }
                Transform::Rotate90 | Transform::Flipped270 => {
                    dgettext("Display rotation", "Right")
                }
                Transform::Rotate270 | Transform::Flipped90 => dgettext("Display rotation", "Left"),
                Transform::Rotate180 | Transform::Flipped => {
                    dgettext("Display rotation", "Flipped")
                }
            },
        };

        f.write_str(&label)
    }
}

impl Orientation {
    pub fn new(ratio: DisplayRatio, transform: Transform) -> Self {
        Self { ratio, transform }
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

#[allow(clippy::from_over_into)]
impl Into<f64> for Scale {
    fn into(self) -> f64 {
        self.0
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
