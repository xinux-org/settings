use zbus::Connection;

use crate::ui::display::dbus::ApplyMethod;

use super::{DisplayConfig, DisplayConfigProxy};

#[derive(Debug)]
pub struct DisplayConfigManager {
    conn: Connection,
    current_config: DisplayConfig,
}

impl DisplayConfigManager {
    pub async fn new() -> anyhow::Result<Self> {
        let conn = Connection::session().await?;
        let proxy = DisplayConfigProxy::new(&conn).await?;

        let state = proxy.get_current_state().await?;

        #[cfg(not(debug_assertions))]
        {
            if state.monitors.len() > 1 {
                anyhow::bail!("currently multiple monitors are not supported");
            }
        }

        let current_config = DisplayConfig::from(state);

        Ok(Self {
            conn,
            current_config,
        })
    }

    pub fn get_current_config(&self) -> &DisplayConfig {
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

        let proxy = DisplayConfigProxy::new(&self.conn).await?;
        proxy
            .apply_monitors_config(p.serial, p.method, &p.logical_monitors, p.properties)
            .await?;

        Ok(())
    }
}
