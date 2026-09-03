use serde::{Deserialize, Serialize};
use zbus::zvariant::Type;

#[derive(Serialize, Deserialize, Type, Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum ColorMode {
    #[default]
    Default = 0,
    BT2100 = 1,
    SDRNative = 2,
}
