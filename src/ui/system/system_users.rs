use crate::ui::system::system_page::SystemPageMsg;
use gettextrs::gettext;
use relm4::{
    adw::{self, prelude::*},
    gtk::{self, gio},
    prelude::*,
};

#[derive(Debug, Default)]
pub struct SystemUsersPage {}

#[derive(Debug)]
pub enum SystemUsersMsg {}

#[relm4::component(pub)]
impl SimpleComponent for SystemUsersPage {
    type Init = ();
    type Input = SystemUsersMsg;
    type Output = SystemPageMsg;

    view! {
        adw::NavigationPage {
            set_title: &gettext("Users"),

            adw::ToolbarView {
                set_top_bar_style: adw::ToolbarStyle::Flat,

                add_top_bar = &adw::HeaderBar {},

                adw::PreferencesPage {
                    adw::PreferencesGroup {
                        adw::ActionRow {
                            set_title: &gettext("Time format"),
                            set_use_underline: true,

                            #[name(time_format_toggle_group)]
                            add_suffix = &adw::ToggleGroup {
                                set_valign: gtk::Align::Center,
                                set_homogeneous: true,

                                add = adw::Toggle {
                                    set_label: Some(&gettext("24-hour")),
                                    set_name: Some("24h"), // donʻt trans
                                    set_use_underline: true,
                                },

                            },
                        },
                    },
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {}
}
