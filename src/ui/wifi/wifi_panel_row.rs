use nmrs::WifiSecurity;
use relm4::{
    adw::{self, prelude::*},
    gtk::{self},
    prelude::*,
};

#[derive(Debug)]
pub struct WifiNetwork {
    pub client: nmrs::NetworkManager,
    pub ssid: String,
    pub strength: u8,
    pub connected: bool,
}

#[derive(Debug)]
pub enum NetworkRowMsg {
    Connect(String),
    ClickQr(String)
}

#[derive(Debug)]
pub enum NetworkRowOutput {
    ConnectResult(Result<(), String>),
}

#[relm4::factory(pub)]
impl FactoryComponent for WifiNetwork {
    type Init = WifiNetwork;
    type Input = NetworkRowMsg;
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
                set_icon_name: match self.strength {
                    80..100 => Some("network-wireless-signal-excellent-secure-symbolic"),
                    50..80 => Some("network-wireless-signal-good-secure-symbolic"),
                    25..50 => Some("network-wireless-signal-weak-secure-symbolic"),
                    _ => Some("network-wireless-connected-00-symbolic"),
                },
                set_pixel_size: 16,
            },

            add_suffix = &gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 6,
                #[watch]
                set_visible: self.connected,

                gtk::Button {
                    set_icon_name: "qrscanner-symbolic",
                    add_css_class: "flat",
                    set_valign: gtk::Align::Center,
                    set_tooltip_text: Some("Share Network"),
                    // connect_clicked => NetworkRowMsg::ClickQr
                    connect_clicked[sender, index, ssid = self.ssid.to_owned()] => move |_|
                        sender.input(NetworkRowMsg::ClickQr(
                            ssid.to_string()
                        )
                    )
                },

                gtk::Button {
                    set_icon_name: "settings-symbolic",
                    add_css_class: "flat",
                    set_valign: gtk::Align::Center,
                    set_tooltip_text: Some("Network Options"),
                    
                }
            },

            connect_activated[sender, index, ssid = self.ssid.to_owned()] => move |_|
                sender.input(NetworkRowMsg::Connect(
                    ssid.to_string()
                )
            )
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        init
    }

    fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        match message {
            NetworkRowMsg::Connect(ssid) => {
                let clinet_clone = self.client.clone();
                relm4::spawn(async move {
                    // let connections = clinet_clone.has_saved_connection(&ssid).await.unwrap();
                    let result = clinet_clone
                        // FIXME: this is not open network. nmrs yields error
                        //  ERROR nmrs::core::connection: Fresh connection also failed:
                        // org.freedesktop.NetworkManager.Settings.Connection.InvalidProperty:
                        // 802-11-wireless.ssid: connection does not match access poin
                        //
                        // D-Bus error: org.freedesktop.NetworkManager.Settings.Connection.InvalidProperty:
                        // 802-11-wireless.ssid: connection does not match access point
                        //
                        // Either password should be set via WifiSecurity::WpaPsk or find other way
                        // CLI action to connect wireless network first time works perfect:
                        // nmcli device wifi connect "UIC_Dgov"
                        .connect(ssid.as_ref(), WifiSecurity::Open)
                        .await
                        .map_err(|e| {
                            println!("aaa\n\n\n\n\n\n\n\n\n\n\n\n {}", e);
                            "a".to_string()
                        });
                    let _ = sender.output(NetworkRowOutput::ConnectResult(result));
                });
            },
            NetworkRowMsg::ClickQr(ssid) => {
                println!("Clicked: {ssid}")
            }
        }
    }
}

// async fn connect_network(ssid: &str) -> nmrs::Result<()> {
//     let nm = NetworkManager::new().await?;
//     nm.connect(ssid, WifiSecurity::Open).await
// }
