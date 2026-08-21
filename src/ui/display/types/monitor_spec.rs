use serde::{Deserialize, Serialize};
use zbus::zvariant::Type;

#[derive(Serialize, Deserialize, Type, Debug, Clone, Eq, PartialEq)]
pub struct MonitorSpec {
    // connector name (e.g. HDMI-1, DP-1, etc)
    pub connector: String,
    // vendor name
    pub vendor: String,
    // product name
    pub product: String,
    // product serial
    pub serial: String,
}
