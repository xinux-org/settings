use relm4::{
    adw::prelude::*,
    gtk::{self},
    prelude::*,
};

use crate::ui::power::general_page::{
    PowerSettings, SCREEN_BLACK_DELAY_LABELS, SCREEN_BLACK_DELAY_VALUES, SUSPEND_DELAY_TIMEOUT,
};

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
    session_settings: gtk::gio::Settings,
    enabled: bool,
    delay: u32,
}

#[derive(Debug)]
pub enum AutoScreenBlackMsg {
    Toggle(bool),
    Delay(u32),
}

#[derive(Debug)]
pub enum AutoScreenBlackOutput {
    Noop,
}

const BLANK_SCREEN_DEFAULT: u32 = 300;

#[relm4::component(pub)]
impl Component for AutoScreenBlack {
    type Init = PowerSettings;
    type Input = AutoScreenBlackMsg;
    type Output = AutoScreenBlackOutput;
    type CommandOutput = ();

    view! {
        #[name(blank_screen_group)]
        adw::PreferencesGroup {
            #[name(blank_screen_switch_row)]
            adw::SwitchRow {
                set_title: "Automatic Screen Blank",
                set_subtitle: "Turn the screen off after a period of inactivity",
                #[watch]
                set_active: model.enabled,

                connect_active_notify[sender] => move |row| {
                    sender.input(AutoScreenBlackMsg::Toggle(row.is_active()));
                }
            },

            #[name(blank_screen_delay_row)]
            adw::ComboRow {
                set_title: "Delay",
                #[watch]
                set_sensitive: model.enabled,
                set_model: Some(&gtk::StringList::new(&SCREEN_BLACK_DELAY_LABELS)),

                // example: https://github.com/blissd/fotema/blob/main/src/app/components/preferences.rs#L127-L130
                connect_selected_item_notify[sender] => move |row| {
                    sender.input(AutoScreenBlackMsg::Delay(row.selected()));
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let session_settings = init.session;
        let current_delay = session_settings.uint("idle-delay");

        let (enabled, delay) = current_delay
            .eq(&0)
            .then(|| (false, BLANK_SCREEN_DEFAULT))
            .unwrap_or((true, current_delay));

        let model = Self {
            session_settings,
            enabled,
            delay,
        };

        sender.input(AutoScreenBlackMsg::Toggle(model.enabled));
        // sender.input(AutoScreenBlackMsg::Delay(delay));

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            AutoScreenBlackMsg::Toggle(state) => {
                // FIXME: https://gitlab.gnome.org/GNOME/gnome-control-center/-/blob/main/panels/power/cc-power-panel.c?ref_type=heads#L740
                if state {
                    if self.delay == 0 {
                        sender.input(AutoScreenBlackMsg::Delay(3600));
                    }
                } else {
                    sender.input(AutoScreenBlackMsg::Delay(0));
                }
                self.enabled = state;
            }
            AutoScreenBlackMsg::Delay(seconds) => {
                self.delay = seconds;
                let _ = self.session_settings.set_uint("idle-delay", self.delay);
            }
        }
    }
}

// Automatic Suspend / When Plugged In
#[derive(Debug)]
pub struct AutomaticSuspend {
    suspend_text: String,
    enabled: bool,
    delay: u16,
}

#[derive(Debug)]
pub enum AutomaticSuspendMsg {
    Toggle(bool),
    Delay(u16),
}

#[derive(Debug)]
pub enum AutomaticSuspendOutput {
    Toggled(bool),
    Delay(u16),
}

#[relm4::component(pub)]
impl Component for AutomaticSuspend {
    type Init = (String, bool, u16);
    type Input = AutomaticSuspendMsg;
    type Output = AutomaticSuspendOutput;
    type CommandOutput = ();

    view! {
        #[root]
        adw::ActionRow {
            set_title: model.suspend_text.as_str(),

            add_suffix = &gtk::Switch {
                set_valign: gtk::Align::Center,
                #[watch]
                set_active: model.enabled,
                connect_state_set[sender] => move |_, state| {
                    sender.input(AutomaticSuspendMsg::Toggle(state));
                    gtk::glib::Propagation::Proceed
                },
            },
        },


        adw::ComboRow {
            #[watch]
            set_sensitive: model.enabled,

            set_title: "Delay",
            set_model: Some(&gtk::StringList::new(&SUSPEND_DELAY_TIMEOUT)),
        }

    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            suspend_text: init.0,
            enabled: init.1,
            delay: init.2,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            AutomaticSuspendMsg::Toggle(state) => {
                self.enabled = state;

                sender
                    .output(AutomaticSuspendOutput::Toggled(state))
                    .unwrap()
            }
            AutomaticSuspendMsg::Delay(seconds) => {
                self.delay = seconds;

                sender
                    .output(AutomaticSuspendOutput::Delay(seconds))
                    .unwrap()
            }
        }
    }
}
