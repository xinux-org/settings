use serde::Deserialize;
use zbus::zvariant::Type;

#[derive(Deserialize, Type, Debug, Copy, Clone, PartialEq, Eq)]
pub enum RgbRange {
    Auto = 1,
    Full = 2,
    Limited = 3,
}
