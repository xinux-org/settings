use zbus::Connection;

use super::{DisplayConfig, DisplayConfigProxy};

#[derive(Debug)]
pub struct DisplayConfigManager {
    current_config: DisplayConfig,
}

impl DisplayConfigManager {
    pub async fn new() -> anyhow::Result<Self> {
        let conn = Connection::session().await?;
        let proxy = DisplayConfigProxy::new(&conn).await?;

        let state = proxy.get_current_state().await?;

        let current_config = DisplayConfig::from(state);

        Ok(Self { current_config })
    }

    pub fn get_current_config(&self) -> &DisplayConfig {
        &self.current_config
    }
}
