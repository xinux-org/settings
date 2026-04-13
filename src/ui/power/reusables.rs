use relm4::{
    adw::prelude::*,
    gtk::{self},
    prelude::*,
};

use crate::ui::power::general_page::SCREEN_BLACK_DELAY_TIMEOUT;

#[derive(Debug)]
pub struct DimScreen {
    idle_dim: bool,
}

#[derive(Debug)]
pub enum DimScreenMsg {
    Toggle(bool),
}

#[derive(Debug)]
pub enum DimScreenOutput {
    Toggled(bool),
}

#[relm4::component(pub)]
impl Component for DimScreen {
    type Init = bool;
    type Input = DimScreenMsg;
    type Output = DimScreenOutput;
    type CommandOutput = ();

    view! {
            adw::PreferencesGroup {
                set_title: "Power Saving",

                adw::PreferencesGroup {
                    adw::ActionRow {
                        set_title: "Dim Screen",
                        set_subtitle: "Reduce screen brightness when the device is inactive",

                        add_suffix = &gtk::Switch {
                            set_valign: gtk::Align::Center,
                            #[watch]
                            set_active: model.idle_dim,
                            connect_state_set[sender] => move |_, state| {
                                sender.input(DimScreenMsg::Toggle(state));
                                gtk::glib::Propagation::Proceed
                            },
                        },
                    },
                },
            },
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            // In case there is no battery
            idle_dim: init,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            DimScreenMsg::Toggle(state) => {
                self.idle_dim = state;

                sender.output(DimScreenOutput::Toggled(state)).unwrap()
            }
        }
    }
}

// Automatic Screen Black
#[derive(Debug)]
pub struct AutoScreenBlack {
    enabled: bool,
    delay: u16,
}

#[derive(Debug)]
pub enum AutoScreenBlackMsg {
    Toggle(bool),
    Delay(u16),
}

#[derive(Debug)]
pub enum AutoScreenBlackOutput {
    Toggled(bool),
    Delay(u16),
}

#[relm4::component(pub)]
impl Component for AutoScreenBlack {
    type Init = (bool, u16);
    type Input = AutoScreenBlackMsg;
    type Output = AutoScreenBlackOutput;
    type CommandOutput = ();

    view! {
            adw::PreferencesGroup {

                adw::ActionRow {
                    set_title: "Automatic Screen Blank",
                    set_subtitle: "Turn the screen off after a period of inactivity",

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,
                        #[watch]
                        set_active: model.enabled,
                        connect_state_set[sender] => move |_, state| {
                            sender.input(AutoScreenBlackMsg::Toggle(state));
                            gtk::glib::Propagation::Proceed
                        },
                    },
                },

                adw::ComboRow {
                    #[watch]
                    set_sensitive: model.enabled,

                    set_title: "Delay",
                    set_model: Some(&gtk::StringList::new(&SCREEN_BLACK_DELAY_TIMEOUT)),
                }
            },
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            enabled: init.0,
            delay: init.1,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            AutoScreenBlackMsg::Toggle(state) => {
                self.enabled = state;
                sender
                    .output(AutoScreenBlackOutput::Toggled(state))
                    .unwrap()
            }

            AutoScreenBlackMsg::Delay(seconds) => {
                self.delay = seconds;
                sender
                    .output(AutoScreenBlackOutput::Delay(seconds))
                    .unwrap();
            }
        }
    }
}
