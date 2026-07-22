use std::collections::HashMap;
use display_config::DisplayConfigProxyBlocking;
use zbus::blocking::Connection;
use zbus::zvariant::{OwnedValue};

pub struct MonitorId {
    pub connector: String,
    pub vendor: String,
    pub serial: String,
}

pub struct MonitorMode {
    pub id: String,
    pub width: i32,
    pub height: i32,
    pub refresh_rate: f64,
    pub prefered_scale: f64,
    pub supported_scale: Vec<f64>,
    pub properties: HashMap<String, OwnedValue>
}


pub struct Monitor {
    pub id: MonitorId,
    pub modes: MonitorMode,
    pub properties: HashMap<String, OwnedValue>
}

pub struct LogicalMonitor{
    pub x: i32,
    pub y: i32,
    pub scale: f64,
    pub primary: bool,
    pub monitors: Vec<MonitorId>,
    pub properties: HashMap<String, OwnedValue>
}



fn main() -> zbus::Result<()> {
    let conn = Connection::session()?;
    let proxy = DisplayConfigProxyBlocking::new(&conn)?;

    
    println!("{:#?}", proxy.get_current_state()?);
    Ok(())
}
