use serde::{Deserialize, Serialize};
use zbus::zvariant::Type;

#[derive(Serialize, Deserialize, Type, Debug, Copy, Clone, PartialEq, Eq)]
pub enum ColorMode {
    Default = 0,
    BT2100 = 1,
    SDRNative = 2,
}
