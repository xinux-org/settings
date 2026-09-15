use crate::{
    ui::{
        wifi::{
            wifi_panel_row::{NetworkRowOutput, WifiNetwork},
            wifi_qr_dialog::{WifiQrDialog, WifiQrInput},
        },
        window::AppMsg,
    },
    utils::power::get_battery_path,
};
use core::clone::Clone;
use gettextrs::gettext;
use nmrs::NetworkManager;
use relm4::{
    Controller,
    adw::{self, prelude::*},
    factory::FactoryVecDeque,
    gtk::{
        self,
        glib::{self},
    },
    prelude::*,
};
use tracing::debug;

pub struct WifiModel {
    wifi_enabled: bool,
    networks: FactoryVecDeque<WifiNetwork>,
    loading: bool,
    wifi_stack: gtk::Stack,
    wifi_stack_page: WifiStack,
    airplane_mode: bool,
    client: nmrs::NetworkManager,
    active_toggle_task: Option<gtk::glib::JoinHandle<()>>,
    qr_dialog: AsyncController<WifiQrDialog>,
    is_laptop: bool,
}
#[derive(Debug)]
pub enum WifiInput {
    NetworksLoaded(Vec<WifiNetwork>),
    ConnectResult(Result<(), String>),
    ClearNetworksList,
    LoadNetworks,
    ToggleWifi(bool),
    ToggleAirplaneMode(bool),
    ShowQr(String),
}

#[derive(Debug)]
pub enum WifiStack {
    WifiOn,
    WifiOff,
    Airplane,
}

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for WifiModel {
    type Init = ();
    type Input = WifiInput;
    type Output = AppMsg;

    view! {
      // FIXME: if PC has no wifi interface, hide this section from windows.rs
        #[root]
        adw::ToolbarView {
            set_top_bar_style: adw::ToolbarStyle::Flat,
            add_top_bar = &adw::HeaderBar {
                #[wrap(Some)]
                set_title_widget = &adw::WindowTitle {
                    set_title: &gettext("Wi-Fi"),
                    set_subtitle: &gettext("this is subtit"),
                }
            },
            adw::PreferencesPage {
                adw::PreferencesGroup {
                    adw::SwitchRow {
                        set_title: &gettext("Wi-Fi"),
                        set_activatable: true,
                        #[watch]
                        set_active: model.wifi_enabled,
                        connect_active_notify[sender] => move |row| {
                            sender.input(WifiInput::ToggleWifi(row.is_active()));
                        }
                    }
                },
                adw::PreferencesGroup {
                    adw::ActionRow {
                        set_title: &gettext("Saved Networks"),
                        set_activatable: true,
                        add_suffix = &gtk::Image {
                            set_icon_name: Some("go-next-symbolic"),
                            set_pixel_size: 16,
                        }
                    },
                    adw::ActionRow {
                        set_title: &gettext("Connect to Hidden Network..."),
                        set_activatable: true,
                        add_suffix = &gtk::Image {
                            set_icon_name: Some("go-next-symbolic"),
                            set_pixel_size: 16,
                        }
                    },
                    adw::ActionRow {
                        set_title: &gettext("Turn On Wi-Fi Hotspot..."),
                        set_activatable: true,
                        add_suffix = &gtk::Image {
                            set_icon_name: Some("go-next-symbolic"),
                            set_pixel_size: 16,
                        }
                    }
                },
                adw::PreferencesGroup {
                    // show only in laptop!
                    set_visible: model.is_laptop,
                    adw::SwitchRow {
                        set_title: &gettext("Airplane Mode"),
                        set_subtitle: &gettext("Disables Wi-Fi, Bluetooth and mobile broadband"),
                        set_use_underline: true,
                        #[watch]
                        set_active: model.airplane_mode,
                        connect_active_notify[sender] => move |row| {
                            let is_active = row.is_active();
                            sender.input(WifiInput::ToggleAirplaneMode(is_active));
                        }
                    }
                },
                adw::PreferencesGroup {
                    #[name(wifi_stack)]
                    gtk::Stack {
                        set_transition_type: gtk::StackTransitionType::Crossfade,
                        set_hhomogeneous: false,
                        set_vhomogeneous: false,
                        // donʻt translate
                        add_named: (&wifi_off, Some("wifi-off")),
                        add_named: (&wifi_connections, Some("wifi-connections")),
                        add_named: (&wifi_connections, Some("airplane-mode")),
                        #[watch]
                        set_visible_child_name: match model.wifi_stack_page {
                          // donʻt translate
                            WifiStack::WifiOn => "wifi-connections",
                            WifiStack::WifiOff => "wifi-off",
                            WifiStack::Airplane => "airplane-mode",
                        },
                    },
                }
            }
        },
        wifi_connections = adw::PreferencesGroup {
            #[local_ref]
            networks_group -> adw::PreferencesGroup {
                #[watch]
                set_title: if !model.loading { "Visible Networks" } else { "" },
                gtk::Box {
                    set_hexpand: true,
                    set_halign: gtk::Align::Start,
                    set_spacing: 6,
                    set_margin_bottom: 12,
                    #[watch]
                    set_visible: model.loading,

                    #[name(list_label)]
                    gtk::Label {
                        set_label: &gettext("Visible Networks"),
                        set_xalign: 0.0,
                        add_css_class: "heading",
                    },
                    #[name(spinner)]
                    adw::Spinner {},
                }
            }
        },
        wifi_off = &adw::StatusPage {
            set_icon_name: Some("network-wireless-disabled-symbolic"),
            set_title: &gettext("Wi-Fi Off"),
            set_description: Some("Turn on to use Wi-Fi"),
        },
        airplane = &adw::StatusPage {
            set_icon_name: Some("airplane-mode-symbolic"),
            set_title: &gettext("Airplane Mode On"),
            set_description: Some("Turn off to use Wi-Fi"),
        },
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let networks = FactoryVecDeque::builder()
            .launch(adw::PreferencesGroup::new())
            .forward(sender.input_sender(), |msg| match msg {
                NetworkRowOutput::ConnectResult(result) => WifiInput::ConnectResult(result),
                NetworkRowOutput::ShowQr(ssid) => WifiInput::ShowQr(ssid),
            });

        let nm = NetworkManager::new()
            .await
            .expect("cannot connect to NetworkManager in is_wifi_enabled");

        let wifi_stack_page = if is_wifi_enabled(&nm).await {
            WifiStack::WifiOn
        } else {
            WifiStack::WifiOff
        };

        let qr_dialog = WifiQrDialog::builder()
            .launch(root.clone().upcast::<gtk::Widget>())
            .detach();

        // FIXME: get initial values instead of hardcode
        let mut model = Self {
            wifi_enabled: is_wifi_enabled(&nm).await,
            networks,
            loading: true,
            wifi_stack: gtk::Stack::new(),
            wifi_stack_page,
            airplane_mode: false,
            client: nm,
            active_toggle_task: None,
            qr_dialog,
            is_laptop: !get_battery_path().is_empty(),
        };
        let networks_group = model.networks.widget();

        if model.wifi_enabled {
            sender.input(WifiInput::LoadNetworks);
        }

        let widgets = view_output!();
        let wifi_stack = widgets.wifi_stack.clone();
        model.wifi_stack = wifi_stack;

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, message: Self::Input, sender: AsyncComponentSender<Self>) {
        match message {
            WifiInput::LoadNetworks => {
                if self.wifi_enabled && !self.airplane_mode {
                    sender.input(WifiInput::ClearNetworksList);

                    match load_networks(&self.client).await {
                        Ok(nets) => sender.input(WifiInput::NetworksLoaded(nets)),
                        Err(e) => {
                            eprintln!("nmrs error: {e}");
                        }
                    }
                }
            }
            WifiInput::NetworksLoaded(nets) => {
                self.loading = false;
                let _: Vec<_> = nets
                    .into_iter()
                    .map(|n| self.networks.guard().push_back(n))
                    .collect();
            }
            WifiInput::ToggleWifi(on) => {
                self.wifi_enabled = on;
                // drop spawned load_networks async task and run fresh task
                if let Some(handle) = self.active_toggle_task.take() {
                    handle.abort();
                }

                // Immediate UI cleanup
                if on {
                    self.loading = true;
                    self.wifi_stack_page = WifiStack::WifiOn;
                } else {
                    sender.input(WifiInput::ClearNetworksList);
                    self.wifi_stack_page = WifiStack::WifiOff;
                }

                // spawn task on background and let finish WifiInput::ToggleWifi
                let clinet_clone = self.client.clone();
                let handle = relm4::spawn_local(async move {
                    if let Err(e) = set_wifi_enabled(clinet_clone, on).await {
                        debug!("Could not toggle Wi-Fi: {e}");
                        return;
                    }
                    if on {
                        glib::timeout_future(std::time::Duration::from_secs(5)).await;
                        sender.input(WifiInput::LoadNetworks);
                    }
                });
                self.active_toggle_task = Some(handle);
            }
            WifiInput::ConnectResult(res) => match res {
                Ok(_) => {
                    debug!("Connected successfully");
                    sender.input(WifiInput::LoadNetworks);
                }
                Err(e) => eprintln!("Connection failed: {e}"),
            },
            WifiInput::ClearNetworksList => self.networks.guard().clear(),
            WifiInput::ShowQr(ssid) => {
                let (password, security) = get_wifi_credentials(&ssid).await;
                self.qr_dialog.emit(WifiQrInput::Show {
                    ssid,
                    password,
                    security,
                });
            }
            WifiInput::ToggleAirplaneMode(_on) => {}
        }
    }
}

async fn is_wifi_enabled(client: &nmrs::NetworkManager) -> bool {
    // Control WiFi
    client
        .wifi_enabled()
        .await
        .expect("cannot get whather wifi_enabled")
}

async fn load_networks(client: &nmrs::NetworkManager) -> nmrs::Result<Vec<WifiNetwork>> {
    let current = client.current_ssid().await;
    let raw = client.list_networks().await?;
    let mut seen = std::collections::HashSet::new();

    let mut networks: Vec<WifiNetwork> = raw
        .into_iter()
        .filter(|n| !n.ssid.trim().is_empty()) // remove unnamed networks
        .filter(|n| n.ssid.ne("<Hidden Network>")) // remove hidden networks
        .filter(|n| seen.insert(n.ssid.clone())) // deduplicate by SSID
        .map(|n| WifiNetwork {
            client: client.clone(),
            connected: current.as_deref() == Some(&n.ssid),
            strength: n.strength.unwrap_or(0),
            ssid: n.ssid,
        })
        .collect();

    networks.sort_by_key(|b| std::cmp::Reverse(b.strength));

    Ok(networks)
}

async fn set_wifi_enabled(client: nmrs::NetworkManager, enabled: bool) -> nmrs::Result<()> {
    // Control WiFi
    client.set_wifi_enabled(enabled).await?; // Disable WiFi
    Ok(())
}

type NMSettingsMap = std::collections::HashMap<
    String,
    std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
>;

#[zbus::proxy(
    interface = "org.freedesktop.NetworkManager.Settings",
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager/Settings"
)]
trait NMSettings {
    fn list_connections(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;
}

#[zbus::proxy(
    interface = "org.freedesktop.NetworkManager.Settings.Connection",
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager/Settings/1"
)]
trait NMConnection {
    fn get_settings(&self) -> zbus::Result<NMSettingsMap>;
    fn get_secrets(&self, setting_name: &str) -> zbus::Result<NMSettingsMap>;
}

async fn get_wifi_credentials(ssid: &str) -> (Option<String>, String) {
    get_wifi_credentials_inner(ssid)
        .await
        .unwrap_or((None, "nopass".to_string()))
}

async fn get_wifi_credentials_inner(ssid: &str) -> zbus::Result<(Option<String>, String)> {
    let conn = zbus::Connection::system().await?;
    let nm_settings = NMSettingsProxy::new(&conn).await?;
    let paths = nm_settings.list_connections().await?;

    for path in paths {
        let nm_conn = NMConnectionProxy::builder(&conn)
            .path(path)?
            .build()
            .await?;

        let Ok(map) = nm_conn.get_settings().await else {
            continue;
        };

        let Some(wifi_section) = map.get("802-11-wireless") else {
            continue;
        };
        let Some(ssid_val) = wifi_section.get("ssid") else {
            continue;
        };
        let Some(conn_ssid) = nm_value_as_ssid(ssid_val) else {
            continue;
        };
        if conn_ssid != ssid {
            continue;
        }

        let security = map
            .get("802-11-wireless-security")
            .and_then(|sec| sec.get("key-mgmt"))
            .and_then(nm_value_as_str)
            .map(|km| match km.as_str() {
                "wpa-psk" | "wpa-eap" | "wpa-eap-suite-b-192" => "WPA",
                "none" => "WEP",
                _ => "nopass",
            })
            .unwrap_or("nopass")
            .to_string();

        let password = nm_conn
            .get_secrets("802-11-wireless-security")
            .await
            .ok()
            .and_then(|secrets| {
                secrets
                    .get("802-11-wireless-security")
                    .and_then(|sec| sec.get("psk"))
                    .and_then(nm_value_as_str)
                    .filter(|s| !s.is_empty())
            });

        return Ok((password, security));
    }

    Ok((None, "nopass".to_string()))
}

fn nm_value_as_ssid(val: &zbus::zvariant::OwnedValue) -> Option<String> {
    if let zbus::zvariant::Value::Array(arr) = &**val {
        let bytes: Vec<u8> = arr
            .iter()
            .filter_map(|v| {
                if let zbus::zvariant::Value::U8(b) = v {
                    Some(*b)
                } else {
                    None
                }
            })
            .collect();
        String::from_utf8(bytes).ok()
    } else {
        None
    }
}

fn nm_value_as_str(val: &zbus::zvariant::OwnedValue) -> Option<String> {
    if let zbus::zvariant::Value::Str(s) = &**val {
        Some(s.as_str().to_string())
    } else {
        None
    }
}
