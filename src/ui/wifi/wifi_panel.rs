use crate::ui::{
    wifi::wifi_panel_row::{NetworkRowOutput, WifiNetwork},
    window::AppMsg,
};
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

pub struct WifiModel {
    wifi_enabled: bool,
    networks: FactoryVecDeque<WifiNetwork>,
    loading: bool,
    many: gtk::Stack,
    wifi_stack: WifiStack,
    airplane_mode: bool,
    // Store the proxy to call methods later
    // proxy: Option<RfkillProxy<'static>>,
}

// use zbus::proxy;

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

#[derive(Debug)]
pub enum WifiInput {
    NetworksLoaded(Vec<WifiNetwork>),
    ConnectResult(Result<(), String>),
    ClearNetworksList,
    LoadNetworks,
    ToggleWifi(bool),
    ToggleAirplaneMode(bool),
    // Received update from System D-Bus
    // AirplaneModeChanged(bool),
    // ProxyInitialized(RfkillProxy<'static>),
}

#[derive(Debug)]
enum WifiStack {
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
                    // FIXME: only in laptop!
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

                // FIXME: use StackPage use display multiple modes
                // source: https://github.com/GNOME/gnome-control-center/blob/main/panels/network/cc-wifi-panel.blp#L42-L167
                adw::PreferencesGroup {
                    #[name(many)]
                    gtk::Stack {
                        set_transition_type: gtk::StackTransitionType::Crossfade,
                        set_hhomogeneous: false,
                        set_vhomogeneous: false,
                        #[watch]
                        set_visible_child_name: match model.wifi_stack {
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

        // FIXME: get initial values instead of hardcode
        let mut model = Self {
            wifi_enabled: true,
            networks,
            loading: true,
            many: gtk::Stack::new(),
            wifi_stack: WifiStack::WifiOn, // fixme
            airplane_mode: false,
            // proxy: None,
        };

        let networks_group = model.networks.widget();
        let widgets = view_output!();

        // FIXME: change it to toggle wifi
        sender.input(WifiInput::LoadNetworks);

        // please improve logic
        let many = widgets.many.clone();
        many.set_visible_child_name("wifi-connections");
        model.many = many;

        // let sender_clone = sender.clone();
        // relm4::spawn_local(async move {
        //     let connection = zbus::Connection::session().await.unwrap();
        //     let proxy = RfkillProxy::new(&connection).await.unwrap();
        //     sender_clone.input(WifiInput::ProxyInitialized(proxy.clone()));

        //     // initial state
        //     if let Ok(on) = proxy.airplane_mode().await {
        //         sender_clone.input(WifiInput::AirplaneModeChanged(on));
        //     }

        //     // zbus generates 'receive_<prop>_changed' automatically
        //     let mut stream = proxy.receive_airplane_mode_changed().await;
        //     while let Some(update) = futures_util::StreamExt::next(&mut stream).await {
        //         if let Ok(on) = update.get().await {
        //             sender_clone.input(WifiInput::AirplaneModeChanged(on));
        //         }
        //     }
        // });

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, message: Self::Input, sender: AsyncComponentSender<Self>) {
        match message {
            WifiInput::LoadNetworks => {
                if self.wifi_enabled && !self.airplane_mode {
                    sender.input(WifiInput::ClearNetworksList);
                    // glib::timeout_future(std::time::Duration::from_secs(5)).await;

                    match load_networks().await {
                        Ok(nets) => sender.input(WifiInput::NetworksLoaded(nets)),
                        Err(e) => {
                            eprintln!("nmrs error: {e}");
                            return;
                        }
                    }
                }
            }
            WifiInput::NetworksLoaded(nets) => {
                self.loading = false;
                let _: Vec<_> = nets
                    .into_iter()
                    .filter(|net| net.ssid.ne("<Hidden Network>"))
                    .map(|n| self.networks.guard().push_back(n))
                    .collect();
            }
            WifiInput::ToggleWifi(on) => {
                // Currently if you turn of/on wifi toggle so many times,
                // It also loads network many times giving not good experience
                self.wifi_enabled = on;
                // if let Err(e) = set_wifi_enabled(on).await {
                //     debug!("Could not toggle Wi-Fi: {e}");
                //     return;
                // }

                // Immediate UI cleanup
                if !on {
                    sender.input(WifiInput::ClearNetworksList);
                    self.wifi_stack = WifiStack::WifiOff;
                } else {
                    self.loading = true;
                    self.wifi_stack = WifiStack::WifiOn;

                    // glib::timeout_future(std::time::Duration::from_secs(5)).await;
                    // sender.input(WifiInput::LoadNetworks);
                }
                // Returns itʻs status when finished on backgroud without
                // depending WifiInput::ToggleWifi
                relm4::spawn_local(async move {
                    if let Err(e) = set_wifi_enabled(on).await {
                        debug!("Could not toggle Wi-Fi: {e}");
                        return;
                    }
                    if on {
                        glib::timeout_future(std::time::Duration::from_secs(5)).await;
                        sender.input(WifiInput::LoadNetworks);
                    }
                });

                // if let Err(e) = set_wifi_enabled(on).await {
                //     debug!("Could not toggle Wi-Fi: {e}");
                //     return;
                // }
                // if on {
                //     glib::timeout_future(std::time::Duration::from_secs(5)).await;
                //     sender.input(WifiInput::LoadNetworks);
                // }
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

async fn is_wifi_enabled() -> bool {
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
