use zbus::Connection;

use super::DisplayConfigProxy;
use super::config::CcDisplayConfig;

#[derive(Debug)]
pub struct DisplayConfigManager {
    current_config: CcDisplayConfig,
}

impl DisplayConfigManager {
    pub async fn new() -> anyhow::Result<Self> {
        let conn = Connection::session().await?;
        let proxy = DisplayConfigProxy::new(&conn).await?;

        let state = proxy.get_current_state().await?;

        if state.monitors.len() > 1 {
            anyhow::bail!("currently multiple monitors are not supported");
        }

        let current_config = CcDisplayConfig::from(state);

        Ok(Self { current_config })
    }

    pub fn get_current_config(&self) -> &CcDisplayConfig {
        &self.current_config
    }
}
