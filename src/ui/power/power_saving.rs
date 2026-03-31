use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

use crate::ui::power::power_page::PowerMsg;

#[derive(Debug)]
pub struct SavingPowerPageView {}

#[relm4::component(pub)]
impl Component for SavingPowerPageView {
    type Init = ();
    type Input = ();
    type Output = PowerMsg;
    type CommandOutput = ();

    view! {
        adw::PreferencesPage {
            adw::PreferencesGroup {
                adw::SwitchRow {
                    set_title: "Automic Power Saver",
                    set_subtitle: "Turn on power saver made when battery power is low",
                }
            },

            adw::PreferencesGroup {
                adw::SwitchRow {
                    set_title: "Automatic Screen Blank",
                    set_subtitle: "Turn the screen off after a period of inactivity",
                },

                adw::ComboRow {
                    set_title: "Delay",
                    set_model: Some(&gtk::StringList::new(&[
                        "1 minute",
                        "2 minute",
                        "3 minute",
                        "4 minute",
                        "5 minute",
                        "8 minute",
                        "10 minute",
                        "12 minute",
                        "15 minute",
                    ])),
                }
            },

            adw::PreferencesGroup {
                set_title: "Automatic Suspend",

                adw::SwitchRow {
                    set_title: "On Battery Power",
                },

                adw::ComboRow {
                    set_title: "Delay",
                    set_model: Some(&gtk::StringList::new(&[
                        "15 minute",
                        "20 minute",
                        "25 minute",
                        "30 minute",
                        "45 minute",
                        "1 hour",
                        "1 hour 20 minute",
                        "1 hour 30 minute",
                        "1 hour 40 minute",
                        "2 hours",
                    ])),
                }
            },

            adw::PreferencesGroup {
                adw::SwitchRow {
                    set_title: "When plugged",
                },

                adw::ComboRow {
                    set_title: "Delay",
                    set_model: Some(&gtk::StringList::new(&[
                        "15 minute",
                        "20 minute",
                        "25 minute",
                        "30 minute",
                        "45 minute",
                        "1 hour",
                        "1 hour 20 minute",
                        "1 hour 30 minute",
                        "1 hour 40 minute",
                        "2 hours",
                    ])),
                }
            },

            adw::PreferencesGroup {
                adw::ActionRow {
                    set_title: "Disabling automatic suspend will result in higher power consumption. It is recomended to keep automatic suspend enabled.",

                    add_prefix = &gtk::Image {
                        set_icon_name: Some("info-outline"),
                        set_pixel_size: 24,
                    }
                }
            }
        },
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

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
    }
}
