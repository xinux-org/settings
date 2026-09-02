use zbus::Connection;

use super::DisplayConfigProxy;
use super::config::CcDisplayConfig;

#[derive(Debug)]
pub struct DisplayConfigManager {
    current_config: CcDisplayConfig,
    #[allow(dead_code)]
    proxy: DisplayConfigProxy<'static>,
}

impl DisplayConfigManager {
    pub async fn new() -> anyhow::Result<Self> {
        let conn = Connection::session().await?;
        let proxy = DisplayConfigProxy::new(&conn).await?;

        if proxy.has_external_monitor().await? {
            anyhow::bail!("current multiple monitors are not supported")
        }

        let state = proxy.get_current_state().await?;
        let current_config = CcDisplayConfig::from(state);

        Ok(Self {
            proxy,
            current_config,
        })
    }

    pub fn get_current_config(&self) -> &CcDisplayConfig {
        &self.current_config
    }
}
