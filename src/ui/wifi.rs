use crate::ui::window::AppMsg;
use nmrs::{NetworkManager, WifiSecurity};
use relm4::{
    adw::{self, prelude::*},
    factory::FactoryVecDeque,
    gtk::{
        self,
        glib::{self},
    },
    prelude::*,
};
use tracing::debug;

#[derive(Debug, Clone)]
pub struct WifiNetwork {
    pub ssid: String,
    pub strength: u8,
    pub connected: bool,
}

#[derive(Debug)]
pub enum NetworkRowOutput {
    Connect(String),
}

#[relm4::factory(pub)]
impl FactoryComponent for WifiNetwork {
    type Init = WifiNetwork;
    type Input = ();
    type Output = NetworkRowOutput;
    type CommandOutput = ();
    type ParentWidget = adw::PreferencesGroup;

    view! {
        adw::ActionRow {
            #[watch]
            set_title: &self.ssid,
            #[watch]
            set_subtitle: if self.connected { "Connected" } else { "" },
            set_activatable: true,

            add_prefix = &gtk::Image {
                set_icon_name: Some("network-wireless-symbolic"),
                set_pixel_size: 16,
            },

            add_suffix = &gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 6,
                set_visible: self.connected,

                gtk::Button {
                    set_icon_name: "qr-code-symbolic",
                    add_css_class: "flat",
                    set_valign: gtk::Align::Center,
                },

                gtk::Button {
                    set_icon_name: "emblem-system-symbolic",
                    add_css_class: "flat",
                    set_valign: gtk::Align::Center,
                }
            },

            connect_activated[sender, index, ssid = self.ssid.to_owned()] => move |_| {
                let _ = sender.output(NetworkRowOutput::Connect(
                    ssid.to_string()
                ));
            }
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        init
    }
}

#[derive(Debug)]
pub enum WifiInput {
    LoadNetworks,
    NetworksLoaded(Vec<WifiNetwork>),
    ToggleWifi(bool),
    Connect(String),
    ConnectResult(Result<(), String>),
}

pub struct WifiModel {
    wifi_enabled: bool,
    networks: FactoryVecDeque<WifiNetwork>,
    loading: bool,
}

#[relm4::component(pub)]
impl SimpleComponent for WifiModel {
    type Init = ();
    type Input = WifiInput;
    type Output = AppMsg;

    view! {
        adw::ToolbarView {
            set_top_bar_style: adw::ToolbarStyle::Flat,

            add_top_bar = &adw::HeaderBar {
                #[wrap(Some)]
                set_title_widget = &adw::WindowTitle {
                    set_title: "Wi-Fi",
                }
            },

            adw::PreferencesPage {
                adw::PreferencesGroup {
                    adw::SwitchRow {
                        set_title: "Wi-Fi",
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
                        set_title: "Saved Networks",
                        set_activatable: true,
                        add_suffix = &gtk::Image {
                            set_icon_name: Some("go-next-symbolic"),
                            set_pixel_size: 16,
                        }
                    },
                    adw::ActionRow {
                        set_title: "Connect to Hidden Network...",
                        set_activatable: true,
                        add_suffix = &gtk::Image {
                            set_icon_name: Some("go-next-symbolic"),
                            set_pixel_size: 16,
                        }
                    },
                    adw::ActionRow {
                        set_title: "Turn On Wi-Fi Hotspot...",
                        set_activatable: true,
                        add_suffix = &gtk::Image {
                            set_icon_name: Some("go-next-symbolic"),
                            set_pixel_size: 16,
                        }
                    }
                },

                adw::PreferencesGroup {
                    adw::SwitchRow {
                        set_title: "Airplane Mode",
                        set_subtitle: "Disables Wi-Fi, Bluetooth and mobile broadband",
                    }
                },

                #[local_ref]
                networks_group -> adw::PreferencesGroup {
                    set_title: "Visible Networks",
                    #[watch]
                    set_description: if model.loading { Some("Scanning...") } else { None },
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let networks = FactoryVecDeque::builder()
            .launch(adw::PreferencesGroup::new())
            .forward(sender.input_sender(), |msg| match msg {
                NetworkRowOutput::Connect(ssid) => WifiInput::Connect(ssid),
            });

        let model = Self {
            wifi_enabled: true,
            networks,
            loading: true,
        };

        let networks_group = model.networks.widget();
        let widgets = view_output!();

        sender.input(WifiInput::LoadNetworks);
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            WifiInput::LoadNetworks => {
                self.loading = true;
                relm4::spawn_local(async move {
                    match load_networks().await {
                        Ok(nets) => sender.input(WifiInput::NetworksLoaded(nets)),
                        Err(e) => {
                            eprintln!("nmrs error: {e}");
                            sender.input(WifiInput::NetworksLoaded(vec![]));
                        }
                    }
                });
            }

            WifiInput::NetworksLoaded(nets) => {
                self.loading = false;

                let mut guard = self.networks.guard();
                guard.clear();
                for net in nets {
                    guard.push_back(net);
                }
            }

            WifiInput::ToggleWifi(on) => {
                self.wifi_enabled = on;

                if !on {
                    let mut guard = self.networks.guard();
                    guard.clear();
                }
                relm4::spawn_local(async move {
                    if let Err(e) = set_wifi_enabled(on).await {
                        debug!("Could not toggle Wi-Fi: {e}");
                    }
                    if on {
                        glib::timeout_future(std::time::Duration::from_secs(3)).await;
                    }
                    sender.input(WifiInput::LoadNetworks);
                });
            }

            WifiInput::Connect(ssid) => {
                relm4::spawn_local(async move {
                    let result = connect_network(&ssid).await.map_err(|e| e.to_string());
                    sender.input(WifiInput::ConnectResult(result));
                });
            }

            WifiInput::ConnectResult(res) => match res {
                Ok(_) => {
                    debug!("Connected successfully");
                    sender.input(WifiInput::LoadNetworks);
                }
                Err(e) => eprintln!("Connection failed: {e}"),
            },
        }
    }
}

async fn is_wifi_enabled() -> bool {
    use zbus::Connection;
    use zbus::proxy;

    #[proxy(
        interface = "org.freedesktop.NetworkManager",
        default_service = "org.freedesktop.NetworkManager",
        default_path = "/org/freedesktop/NetworkManager"
    )]
    trait NetworkManagerDBus {
        #[zbus(property)]
        fn wireless_enabled(&self) -> zbus::Result<bool>;
    }

    let Ok(conn) = Connection::system().await else {
        return false;
    };
    let Ok(proxy) = NetworkManagerDBusProxy::new(&conn).await else {
        return false;
    };
    proxy.wireless_enabled().await.unwrap_or(false)
}

async fn load_networks() -> nmrs::Result<Vec<WifiNetwork>> {
    let nm = NetworkManager::new().await?;

    if !is_wifi_enabled().await {
        return Ok(vec![]);
    }

    let current = nm.current_ssid().await;
    let raw = nm.list_networks().await?;

    let mut seen = std::collections::HashSet::new();

    let mut networks: Vec<WifiNetwork> = raw
        .into_iter()
        .filter(|n| !n.ssid.trim().is_empty()) // remove unnamed networks
        .filter(|n| seen.insert(n.ssid.clone())) // deduplicate by SSID
        .map(|n| WifiNetwork {
            connected: current.as_deref() == Some(&n.ssid),
            strength: n.strength.unwrap_or(0),
            ssid: n.ssid,
        })
        .collect();

    networks.sort_by(|a, b| b.strength.cmp(&a.strength));

    Ok(networks)
}

async fn set_wifi_enabled(enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
    use zbus::Connection;
    use zbus::proxy;

    #[proxy(
        interface = "org.freedesktop.NetworkManager",
        default_service = "org.freedesktop.NetworkManager",
        default_path = "/org/freedesktop/NetworkManager"
    )]
    trait NetworkManagerDBus {
        #[zbus(property)]
        fn set_wireless_enabled(&self, enabled: bool) -> zbus::Result<()>;
    }

    let conn = Connection::system().await?;
    let proxy = NetworkManagerDBusProxy::new(&conn).await?;
    proxy.set_wireless_enabled(enabled).await?;
    Ok(())
}

async fn connect_network(ssid: &str) -> nmrs::Result<()> {
    let nm = NetworkManager::new().await?;
    nm.connect(ssid, WifiSecurity::Open).await
}
