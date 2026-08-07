pub type RawMonitorSpec = (String, String, String, String);

#[derive(Debug)]
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

impl From<RawMonitorSpec> for MonitorSpec {
    fn from(value: RawMonitorSpec) -> Self {
        MonitorSpec {
            connector: value.0,
            vendor: value.1,
            product: value.2,
            serial: value.3,
        }
    }
}

impl From<&RawMonitorSpec> for MonitorSpec {
    fn from(value: &RawMonitorSpec) -> Self {
        Self::from(value.to_owned())
    }
}
