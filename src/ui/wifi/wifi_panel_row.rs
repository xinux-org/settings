use nmrs::{NetworkManager, WifiSecurity};
use relm4::{
    adw::{self, prelude::*},
    gtk::{self},
    prelude::*,
};

#[derive(Debug)]
pub struct WifiNetwork {
    pub ssid: String,
    pub strength: u8,
    pub connected: bool,
}

#[derive(Debug)]
pub enum NetworkRowMsg {
    Connect(String),
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
                },

                gtk::Button {
                    set_icon_name: "settings-symbolic",
                    add_css_class: "flat",
                    set_valign: gtk::Align::Center,
                    set_tooltip_text: Some("Network Options"),
                }
            },

            connect_activated[sender, index, ssid = self.ssid.to_owned()] => move |_| {
                let _ = sender.input(NetworkRowMsg::Connect(
                    ssid.to_string()
                ));
            }
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        init
    }

    fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        match message {
            NetworkRowMsg::Connect(ssid) => {
                relm4::spawn_local(async move {
                    let result = connect_network(&ssid).await.map_err(|e| e.to_string());
                    let _ = sender.output(NetworkRowOutput::ConnectResult(result));
                });
            }
        }
    }
}

async fn connect_network(ssid: &str) -> nmrs::Result<()> {
    let nm = NetworkManager::new().await?;
    nm.connect(ssid, WifiSecurity::Open).await
}
