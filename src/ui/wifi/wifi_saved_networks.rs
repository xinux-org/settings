use crate::ui::wifi::components::wifi_panel_row::WifiNetwork;
use gettextrs::gettext;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug)]
pub struct WifiSavedNetworkModel {
    networks: FactoryVecDeque<WifiNetwork>,
    wifi_saved_networks_list: Option<Vec<String>>,
}

impl WifiSavedNetworkModel {
    async fn load_networks(client: &nmrs::NetworkManager) -> Vec<WifiNetwork> {
        let current = client.current_ssid().await;
        let raw = client.list_saved_connections().await;

        let mut networks: Vec<WifiNetwork> = raw
            .iter()
            .flatten()
            .filter(|n| !n.is_empty())
            .map(|n| WifiNetwork {
                client: client.clone(),
                connected: false,
                strength: 0,
                ssid: n.to_string(),
            })
            .collect();

        networks.sort_by(|a, b| b.strength.cmp(&a.strength));

        networks
    }
}
#[relm4::component(pub)]
impl SimpleComponent for WifiSavedNetworkModel {
    type Init = Option<Vec<String>>;
    type Input = ();
    type Output = ();

    view! {
        #[name(saved_networks_dialog)]
        adw::Dialog {
            set_title: &gettext("Saved Wi-Fi Networks"),
            set_content_width: 500,
            set_content_height: 600,

            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar,

                #[name(saved_networks_toast_overlay)]
                #[wrap(Some)]
                set_content = &adw::PreferencesPage {
                    #[local_ref]
                    networks_group -> adw::PreferencesGroup {
                    },
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let networks = FactoryVecDeque::builder()
            .launch(adw::PreferencesGroup::new())
            .detach();

        let model = Self {
            networks,
            wifi_saved_networks_list: init,
        };

        let networks_group = model.networks.widget();
        // let mut networks: Vec<WifiNetwork> = raw
        //     .into_iter()
        //     .filter(|n| !n.ssid.trim().is_empty()) // remove unnamed networks
        //     .filter(|n| n.ssid.ne("<Hidden Network>")) // remove hidden networks
        //     .filter(|n| seen.insert(n.ssid.clone())) // deduplicate by SSID
        //     .map(|n| WifiNetwork {
        //         client: client.clone(),
        //         connected: current.as_deref() == Some(&n.ssid),
        //         strength: n.strength.unwrap_or(0),
        //         ssid: n.ssid,
        //     })
        //     .collect();

        // let _: Vec<_> = model
        //     .wifi_saved_networks_list
        //     .into_iter()
        //     .map(|n| model.networks.guard().push_back(n))
        //     .collect();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
