use std::convert::identity;

use crate::ui::system::system_about::SystemAboutPage;
use crate::ui::system::system_l10n::SystemRegionLanguagePage;
use crate::ui::window::AppMsg;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug)]
pub struct SystemPageModel {
    navigation: adw::NavigationView,
    system_about: Controller<SystemAboutPage>,
    system_l10n: Controller<SystemRegionLanguagePage>,
}

#[derive(Debug)]
pub enum SystemPageMsg {
    OpenSystemAboutPage,
    OpenSystemRegionLanguagePage,
    Rebuild(String, String, String), // single line nix path, argument and value
}

#[relm4::component(pub)]
impl SimpleComponent for SystemPageModel {
    type Init = ();
    type Input = SystemPageMsg;
    type Output = AppMsg;

    view! {
      #[name = "navigation"]
      adw::NavigationView {
          add = &adw::NavigationPage {
              set_title: "System",

              adw::ToolbarView {
                  set_top_bar_style: adw::ToolbarStyle::Flat,

                  add_top_bar = &adw::HeaderBar {},

                  adw::PreferencesPage {
                      adw::PreferencesGroup {
                          adw::ActionRow {
                              set_title: "Region and Language",
                              set_subtitle: "System language and localization",
                              set_activatable: true,

                              add_prefix = &gtk::Image {
                                  set_icon_name: Some("emoji-flags-symbolic"),
                                  set_pixel_size: 16
                              },

                              add_suffix = &gtk::Image {
                                  set_icon_name: Some("go-next-symbolic"),
                                  set_pixel_size: 16,
                              },

                              connect_activated => SystemPageMsg::OpenSystemRegionLanguagePage,
                          },

                          adw::ActionRow {
                              set_title: "Date and Time",
                              set_subtitle: "Time zone and clock settings",
                              set_activatable: true,

                              add_prefix = &gtk::Image {
                                  set_icon_name: Some("preferences-system-time-symbolic"),
                                  set_pixel_size: 16
                              },

                              add_suffix = &gtk::Image {
                                  set_icon_name: Some("go-next-symbolic"),
                                  set_pixel_size: 16,
                              }
                          },

                          adw::ActionRow {
                              set_title: "Users",
                              set_subtitle: "Add and remove accounts, change password",
                              set_activatable: true,

                              add_prefix = &gtk::Image {
                                  set_icon_name: Some("org.gnome.Settings-users-symbolic"),
                                  set_pixel_size: 16
                              },

                              add_suffix = &gtk::Image {
                                  set_icon_name: Some("go-next-symbolic"),
                                  set_pixel_size: 16,
                              }
                          },

                          adw::ActionRow {
                              set_title: "Secure Shell",
                              set_subtitle: "SSH network access",
                              set_activatable: true,

                              add_prefix = &gtk::Image {
                                  set_icon_name: Some("org.gnome.Settings-secure-shell-symbolic"),
                                  set_pixel_size: 16
                              },

                              add_suffix = &gtk::Image {
                                  set_icon_name: Some("go-next-symbolic"),
                                  set_pixel_size: 16,
                              }
                          },

                          adw::ActionRow {
                              set_title: "About",
                              set_subtitle: "Hardware details and software versions",
                              set_activatable: true,

                              add_prefix = &gtk::Image {
                                  set_icon_name: Some("dialog-warning-symbolicc"),
                                  set_pixel_size: 16
                              },

                              add_suffix = &gtk::Image {
                                  set_icon_name: Some("go-next-symbolic"),
                                  set_pixel_size: 16,
                              },

                              connect_activated => SystemPageMsg::OpenSystemAboutPage,
                          },
                      },
                  }
              }
          }
      }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let system_about = SystemAboutPage::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let system_l10n = SystemRegionLanguagePage::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let mut model = Self {
            navigation: adw::NavigationView::new(),
            system_about,
            system_l10n,
        };

        let widgets = view_output!();
        model.navigation = widgets.navigation.clone();

        ComponentParts { model, widgets }
    }
    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            SystemPageMsg::OpenSystemAboutPage => {
                let page = self.system_about.widget();
                self.navigation.push(page);
            }
            SystemPageMsg::OpenSystemRegionLanguagePage => {
                let page = self.system_l10n.widget();
                self.navigation.push(page);
            }
            SystemPageMsg::Rebuild(relative_config_path, argument, value) => {
                sender.output(AppMsg::Rebuild(relative_config_path, argument, value));
            }
        }
    }
}
