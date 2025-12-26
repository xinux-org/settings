use crate::app::AppMsg;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct WifiModel {}

#[relm4::component(pub)]
impl SimpleComponent for WifiModel {
    type Init = ();
    type Input = ();
    type Output = AppMsg;

    view! {
        adw::PreferencesGroup {
            adw::HeaderBar {},

            adw::PreferencesPage {

            adw::PreferencesGroup {
                adw::ActionRow {
                    set_title: "Wi-Fi",
                    set_activatable: true,

                    add_suffix = &gtk::Switch {
                        set_active: true,
                        set_valign: gtk::Align::Center,
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
                adw::ActionRow {
                    set_title: "Airplane Mode",
                    set_subtitle: "Disables Wi-Fi, Bluetooth and mobile broadband",
                    set_activatable: true,

                    add_suffix = &gtk::Switch {
                        set_active: false,
                        set_valign: gtk::Align::Center,
                    }
                }
            },

            adw::PreferencesGroup {
                set_title: "Visible Networks",

                adw::ActionRow {
                    set_title: "HuaweiNetwork",
                    set_subtitle: "Connected",
                    set_activatable: true,

                    add_prefix = &gtk::Image {
                        set_icon_name: Some("network-wireless-symbolic"),
                        set_pixel_size: 16,
                    },

                    add_suffix = &gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 6,

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
                    }
                },

                adw::ActionRow {
                    set_title: "Jamshidbek",
                    set_activatable: true,

                    add_prefix = &gtk::Image {
                        set_icon_name: Some("network-wireless-symbolic"),
                        set_pixel_size: 16,
                    },

                    add_suffix = &gtk::Button {
                        set_icon_name: "emblem-system-symbolic",
                        add_css_class: "flat",
                        set_valign: gtk::Align::Center,
                    }
                },
            }
        }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
