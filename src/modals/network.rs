use crate::app::AppMsg;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct NetworkModel {}

#[relm4::component(pub)]
impl SimpleComponent for NetworkModel {
    type Init = ();
    type Input = ();
    type Output = AppMsg;

    view! {
      adw::PreferencesGroup{
        adw::HeaderBar {},
        adw::PreferencesPage {
            adw::PreferencesGroup {
              set_title: "Wired",

              #[wrap(Some)]
              set_header_suffix = &gtk::Button {
                set_icon_name: "list-add-symbolic",
                add_css_class: "flat",
                set_valign: gtk::Align::Center,
              },

                adw::ActionRow {
                    set_title: "Cable unplugged",
                    set_activatable: true,

                    add_suffix = &gtk::Switch {
                        set_active: true,
                        set_valign: gtk::Align::Center,
                    },
                    add_suffix = &gtk::Button {
                      set_icon_name: "emblem-system-symbolic",
                      add_css_class: "flat",
                      set_valign: gtk::Align::Center,
                    }
                }
            },

            adw::PreferencesGroup {
              set_title: "VPN",

              #[wrap(Some)]
              set_header_suffix = &gtk::Button {
                set_icon_name: "list-add-symbolic",
                add_css_class: "flat",
                set_valign: gtk::Align::Center,
              },

                adw::ActionRow {
                    set_title: "Not set up",
                    set_activatable: true,
                }
            },

            adw::PreferencesGroup {
              set_title: "Proxy",
                adw::ActionRow {
                  add_prefix = &gtk::Image {
                    set_icon_name: Some("network-proxy-server-symbolic"),
                    set_pixel_size: 16,
                  },
                  set_title: "Proxy",
                  set_activatable: true,

                  add_suffix = &gtk::Image {
                      set_icon_name: Some("go-next-symbolic"),
                      set_pixel_size: 16,
                  }
                }
            },
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
