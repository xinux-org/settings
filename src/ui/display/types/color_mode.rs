use serde::Deserialize;
use zbus::zvariant::Type;

#[derive(Deserialize, Type, Debug, Copy, Clone, PartialEq)]
pub enum ColorMode {
    Default = 0,
    BT2100 = 1,
    SDRNative = 2,
}
