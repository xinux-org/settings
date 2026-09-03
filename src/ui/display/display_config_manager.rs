use zbus::Connection;

use super::{DisplayConfigProxy, apply_monitors::ApplyMethod, config::CcDisplayConfig};

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

    pub async fn config_is_applicable(&self) -> anyhow::Result<()> {
        self.apply(ApplyMethod::Verify).await
    }

    pub async fn config_apply(&self) -> anyhow::Result<()> {
        self.apply(ApplyMethod::Persistent).await
    }

    async fn apply(&self, method: ApplyMethod) -> anyhow::Result<()> {
        let p = self.current_config.build_apply_parameters(method);

        self.proxy
            .apply_monitors_config(p.serial, p.method, &p.logical_monitors, p.properties)
            .await?;

        Ok(())
    }
}
