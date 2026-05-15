use crate::ui::{
    wifi::wifi_panel_row::{NetworkRowOutput, WifiNetwork},
    window::AppMsg,
};
use futures_util::stream::StreamExt;
use nmrs::NetworkManager;
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
use zbus::{Connection, proxy};

#[proxy(
    interface = "org.freedesktop.NetworkManager",
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager"
)]

trait NetworkManagerInterface {
    #[zbus(property)]
    fn wireless_enabled(&self) -> zbus::Result<bool>;
}
// #[proxy(
//     interface = "org.gnome.SettingsDaemon.Rfkill",
//     default_service = "org.gnome.SettingsDaemon.Rfkill",
//     default_path = "/org/gnome/SettingsDaemon/Rfkill"
// )]
// trait Rfkill {
//     /// Read the AirplaneMode property
//     #[zbus(property)]
//     fn airplane_mode(&self) -> zbus::Result<bool>;

//     /// Set the AirplaneMode property
//     #[zbus(property)]
//     fn set_airplane_mode(&self, value: bool) -> zbus::Result<()>;
// }
//
pub struct WifiModel {
    wifi_enabled: bool,
    networks: FactoryVecDeque<WifiNetwork>,
    loading: bool,
    wifi_stack: gtk::Stack,
    wifi_stack_page: WifiStack,
    airplane_mode: bool,
    client: nmrs::NetworkManager,
    active_toggle_task: Option<gtk::glib::JoinHandle<()>>,
    // Store the proxy to call methods later
    // proxy: Option<RfkillProxy<'static>>,
}

#[derive(Debug)]
pub enum WifiInput {
    NetworksLoaded(Vec<WifiNetwork>),
    ConnectResult(Result<(), String>),
    ClearNetworksList,
    LoadNetworks,
    ToggleWifi(bool),
    ToggleAirplaneMode(bool),
    HandleWifiState(bool),
    // Received update from System D-Bus
    // AirplaneModeChanged(bool),
    // ProxyInitialized(RfkillProxy<'static>),
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
                    set_title: "Wi-Fi",
                    set_subtitle: "this is subtit",
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
                    // FIXME: show only in laptop!
                    adw::SwitchRow {
                        set_title: "Airplane Mode",
                        set_subtitle: "Disables Wi-Fi, Bluetooth and mobile broadband",
                        set_use_underline: true,
                        // #[watch]
                        // set_active: model.airplane_mode,
                        // connect_active_notify[sender] => move |row| {
                        //     let is_active = row.is_active();
                        //     sender.input(WifiInput::ToggleAirplaneMode(is_active));
                        // }
                    }
                },
                adw::PreferencesGroup {
                    #[name(wifi_stack)]
                    gtk::Stack {
                        set_transition_type: gtk::StackTransitionType::Crossfade,
                        set_hhomogeneous: false,
                        set_vhomogeneous: false,
                        #[watch]
                        set_visible_child_name: match model.wifi_stack_page {
                          // donʻt translate
                          WifiStack::WifiOn => "wifi-connections",
                          WifiStack::WifiOff => "wifi-off",
                          WifiStack::Airplane => "airplane-mode",
                        },
                        // donʻt translate
                        add_named: (&wifi_off, Some("wifi-off")),
                        add_named: (&wifi_connections, Some("wifi-connections")),
                        add_named: (&wifi_connections, Some("airplane-mode")),
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
                    set_label: "Visible Networks",
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
            set_title: "Wi-Fi Off",
            set_description: Some("Turn on to use Wi-Fi"),
        },
        airplane = &adw::StatusPage {
            set_icon_name: Some("airplane-mode-symbolic"),
            set_title: "Airplane Mode On",
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
            });

        let nm = NetworkManager::new()
            .await
            .expect("cannot connect to NetworkManager in is_wifi_enabled");

        let wifi_stack_page = if is_wifi_enabled(&nm).await {
            WifiStack::WifiOn
        } else {
            WifiStack::WifiOff
        };
        // 1. Monitor network changes (e.g., signal strength, SSID)
        let clinet_clone = nm.clone();

        // FIXME: get initial values instead of hardcode
        let mut model = Self {
            wifi_enabled: is_wifi_enabled(&nm).await,
            networks,
            loading: true,
            wifi_stack: gtk::Stack::new(),
            wifi_stack_page, // fixme with airplane mode
            airplane_mode: false,
            client: nm,
            active_toggle_task: None,
            // proxy: None,
        };
        let networks_group = model.networks.widget();

        if model.wifi_enabled {
            sender.input(WifiInput::LoadNetworks);
        }

        let widgets = view_output!();
        let wifi_stack = widgets.wifi_stack.clone();
        model.wifi_stack = wifi_stack;

        let sender_clone = sender.clone();
        relm4::spawn_local(async move {
            let connection = Connection::system().await.unwrap();
            let proxy = NetworkManagerInterfaceProxy::new(&connection)
                .await
                .unwrap();

            // zbus generates 'receive_<prop>_changed' automatically
            let mut wireless_enabled_changed = proxy.receive_wireless_enabled_changed().await;
            while let Some(change) = wireless_enabled_changed.next().await {
                if let Ok(enable) = change.get().await {
                    sender_clone.input(WifiInput::ToggleWifi(enable));
                    sender_clone.input(WifiInput::HandleWifiState(enable));
                }
            }
        });

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, message: Self::Input, sender: AsyncComponentSender<Self>) {
        match message {
            WifiInput::LoadNetworks => {
                if self.wifi_enabled {
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
            // The user clicked a button to turn Wi-Fi on/off
            WifiInput::ToggleWifi(enable) => {
                self.wifi_enabled = enable;
                // Immediate UI cleanup
                match enable {
                    true => {
                        self.loading = true;
                        self.wifi_stack_page = WifiStack::WifiOn;
                    }
                    false => {
                        self.loading = false;
                        sender.input(WifiInput::ClearNetworksList);
                        self.wifi_stack_page = WifiStack::WifiOff;
                    }
                }
            }
            WifiInput::HandleWifiState(enable) => {
                // drop spawned load_networks async task and run fresh task
                if let Some(handle) = self.active_toggle_task.take() {
                    handle.abort();
                }
                // spawn task on background and let finish WifiInput::ToggleWifi
                let clinet_clone = self.client.clone();
                let handle = relm4::spawn_local(async move {
                    if let Err(e) = set_wifi_enabled(clinet_clone, enable).await {
                        debug!("Could not toggle Wi-Fi: {e}");
                        return;
                    }
                    if enable {
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
            WifiInput::ToggleAirplaneMode(on) => {
                // if let Some(ref proxy) = self.proxy {
                //     // we let the D-Bus stream (above) tell us when it's done.
                //     let p = proxy.clone();
                //     relm4::spawn_local(async move {
                //         let _ = p.set_airplane_mode(on).await;
                //     });
                // }
            } // WifiInput::AirplaneModeChanged(on) => {
              //     self.airplane_mode = on;
              //     if !on {
              //         self.wifi_stack = WifiStack::Airplane;
              //     }
              // }
              // WifiInput::ProxyInitialized(proxy) => {
              //     self.proxy = Some(proxy);
              // }
        }
    }
}

async fn is_wifi_enabled(client: &nmrs::NetworkManager) -> bool {
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

    networks.sort_by(|a, b| b.strength.cmp(&a.strength));

    Ok(networks)
}

async fn set_wifi_enabled(client: nmrs::NetworkManager, enabled: bool) -> nmrs::Result<()> {
    // Control WiFi
    client.set_wifi_enabled(enabled).await?; // Disable WiFi
    Ok(())
}
