use futures_util::StreamExt;
use strum::{AsRefStr, EnumString};
use zbus::Connection;

use super::DisplayConfigProxy;
use super::color_mode::ColorMode;
use super::config::{CcDisplayConfig, CcDisplayMonitor};
use super::display_settings::DisplaySettings;
use super::monitor_spec::MonitorSpec;

#[derive(Debug)]
pub struct DisplayConfigManager {
    current_config: CcDisplayConfig,
    proxy: DisplayConfigProxy<'static>,

    pub config_type: ConfigType,
}

impl DisplayConfigManager {
    pub async fn new() -> anyhow::Result<Self> {
        let conn = Connection::session().await?;
        let proxy = DisplayConfigProxy::new(&conn).await?;

        let state = proxy.get_current_state().await?;
        let current_config = CcDisplayConfig::from(&state);
        let config_type = Self::detect_config_type(&current_config);

        Ok(Self {
            proxy,
            config_type,
            current_config,
        })
    }

    pub async fn listen_changes<F>(changed: F) -> anyhow::Result<()>
    where
        F: Fn(),
    {
        let conn = Connection::session().await?;
        let proxy = DisplayConfigProxy::new(&conn).await?;
        let mut stream = proxy.receive_monitors_changed().await?;

        while stream.next().await.is_some() {
            changed();
        }

        Ok(())
    }

    pub async fn get_current(&self) -> anyhow::Result<CcDisplayConfig> {
        let state = self.proxy.get_current_state().await?;
        Ok(CcDisplayConfig::from(&state))
    }

    pub async fn refetch(&mut self) -> anyhow::Result<()> {
        let state = self.proxy.get_current_state().await?;

        self.current_config = CcDisplayConfig::from(&state);
        self.config_type = Self::detect_config_type(&self.current_config);

        Ok(())
    }

    pub fn current_config(&self) -> &CcDisplayConfig {
        &self.current_config
    }

    pub fn current_config_mut(&mut self) -> &mut CcDisplayConfig {
        &mut self.current_config
    }

    pub fn monitors(&self) -> &[CcDisplayMonitor] {
        self.current_config.get_monitors()
    }

    pub fn monitor_count(&self) -> usize {
        self.current_config.get_monitors().len()
    }

    pub fn logical_monitor_count(&self) -> usize {
        self.current_config.get_logical_monitors().len()
    }

    pub fn ensure_config_type(&mut self, target: ConfigType) {
        if target == Self::detect_config_type(&self.current_config) {
            self.config_type = target;
            return;
        }

        let old_primary_scale = self
            .current_config
            .get_monitors()
            .iter()
            .find(|monitor| monitor.is_primary())
            .map(|monitor| monitor.get_scale())
            .unwrap_or(-1.0);

        match target {
            ConfigType::Join => {
                self.current_config.set_cloning(false);

                let specs: Vec<MonitorSpec> = self
                    .current_config
                    .get_monitors()
                    .iter()
                    .map(|monitor| monitor.inner.spec.clone())
                    .collect();

                for spec in &specs {
                    let Some(index) = self
                        .current_config
                        .get_monitors()
                        .iter()
                        .position(|monitor| &monitor.inner.spec == spec)
                    else {
                        continue;
                    };

                    let Some(mode) = self.current_config.get_monitors()[index]
                        .get_preferred_mode()
                        .cloned()
                    else {
                        continue;
                    };

                    let active = self.current_config.get_monitors()[index].is_active();

                    let mut scale = if active {
                        self.current_config.get_monitors()[index].get_scale()
                    } else {
                        mode.preferred_scale
                    };

                    if !self.current_config.is_scaled_mode_valid(&mode, scale)
                        && old_primary_scale > 0.0
                        && self
                            .current_config
                            .is_scaled_mode_valid(&mode, old_primary_scale)
                    {
                        scale = old_primary_scale;
                    }

                    let usable = self.current_config.get_monitors()[index].is_usable();

                    self.current_config.set_monitor_active(spec, usable);

                    self.current_config.get_monitors_mut()[index].set_mode(&mode);
                    self.current_config.get_monitors_mut()[index].set_scale(scale);
                }
            }
            ConfigType::Mirror => {
                self.current_config.set_cloning(true);

                let Some(clone_mode) = self
                    .current_config
                    .generate_cloning_modes()
                    .into_iter()
                    .find(|mode| mode.is_preferred())
                else {
                    tracing::warn!("no compatible cloning mode found");
                    return;
                };

                let scale = if old_primary_scale > 0.0
                    && self
                        .current_config
                        .is_scaled_mode_valid(&clone_mode, old_primary_scale)
                {
                    old_primary_scale
                } else {
                    clone_mode.preferred_scale
                };

                let specs: Vec<MonitorSpec> = self
                    .current_config
                    .get_monitors()
                    .iter()
                    .map(|monitor| monitor.inner.spec.clone())
                    .collect();

                for spec in &specs {
                    let Some(index) = self
                        .current_config
                        .get_monitors()
                        .iter()
                        .position(|monitor| &monitor.inner.spec == spec)
                    else {
                        continue;
                    };

                    self.current_config.get_monitors_mut()[index]
                        .set_compatible_clone_mode(&clone_mode);
                    self.current_config.get_monitors_mut()[index].set_scale(scale);
                }
            }
        }

        self.config_type = target;
    }

    pub fn set_primary_monitor(&mut self, index: usize) {
        if let Some(monitor) = self.current_config.get_monitors().get(index) {
            let spec = monitor.inner.spec.clone();
            self.current_config.set_primary(&spec);
        } else {
            tracing::warn!("PrimaryMonitorChanged called with an invalid monitor index");
        }
    }

    pub fn set_monitor_config(&mut self, spec: MonitorSpec, settings: DisplaySettings) {
        if let Some(enabled) = settings.enabled {
            self.current_config.set_monitor_active(&spec, enabled);
        }

        if let Some(hdr) = settings.hdr {
            if let Some(monitor) = self
                .current_config
                .get_monitors_mut()
                .iter_mut()
                .find(|monitor| monitor.inner.spec == spec)
            {
                monitor.set_color_mode(if hdr {
                    ColorMode::BT2100
                } else {
                    ColorMode::Default
                });

                monitor.set_mode(&settings.current_mode);

                if let Some(orientation) = settings.orientation {
                    monitor.set_rotation(orientation);
                }
            }
        } else if let Some(monitor) = self
            .current_config
            .get_monitors_mut()
            .iter_mut()
            .find(|monitor| monitor.inner.spec == spec)
        {
            let _ = monitor.set_mode(&settings.current_mode);

            if let Some(orientation) = settings.orientation {
                monitor.set_rotation(orientation);
            }
        }

        self.current_config
            .set_monitor_scale(&spec, settings.scale.0);
    }

    fn detect_config_type(config: &CcDisplayConfig) -> ConfigType {
        if config.is_cloning() {
            ConfigType::Mirror
        } else {
            ConfigType::Join
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsRefStr, EnumString)]
pub enum ConfigType {
    Join,
    Mirror,
}
