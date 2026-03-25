use crate::ui::system::system_page::SystemPageMsg;
use relm4::{
    adw::{self, prelude::*},
    gtk::{self},
    prelude::*,
};

#[derive(Debug, Default)]
pub struct SystemDateTimePage {}

#[derive(Debug)]
pub enum SystemDateTimeMsg {}

#[relm4::component(pub)]
impl SimpleComponent for SystemDateTimePage {
    type Init = ();
    type Input = SystemDateTimeMsg;
    type Output = SystemPageMsg;

    view! {
        adw::NavigationPage {
            set_title: "Date & Time",

            adw::ToolbarView {
                set_top_bar_style: adw::ToolbarStyle::Flat,

                add_top_bar = &adw::HeaderBar {},

                adw::PreferencesPage {
                    adw::PreferencesGroup {
                        adw::ActionRow {
                            set_title: "Time format",

                            add_suffix = &gtk::Box {
                                set_spacing: 0,
                                set_halign: gtk::Align::End,
                                set_valign: gtk::Align::Center,
                                add_css_class: "linked",

                                #[name="left"]
                                gtk::ToggleButton {
                                    set_group: Some(&right),
                                    set_label: "24 hours",
                                    set_active: true,
                                },

                                #[name="right"]
                                gtk::ToggleButton {
                                    set_label: "AM / PM",
                                },
                            }
                        },
                    },

                    adw::PreferencesGroup {
                        set_title: "Hour and Calendar",
                        set_description: Some("Yuqori panelda vaqt va qanday boshqarilishini koʻrsating"),

                        adw::SwitchRow {
                            set_title: "Week day",
                        },
                        adw::SwitchRow {
                            set_title: "Date",
                        },
                        adw::SwitchRow {
                            set_title: "0 second",
                        },
                        adw::SwitchRow {
                            set_title: "Week numbers",
                            set_subtitle: "Showed in opened calendar on gnome-shell",
                        },
                    },
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut model = Self {};

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
