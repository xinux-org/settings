use relm4::{
    adw::prelude::*,
    gtk::{self},
    prelude::*,
};

use crate::ui::power::general_page::PowerSettings;
use crate::utils::power::{
    SCREEN_BLANK_DELAY_LABELS, SCREEN_BLANK_DELAY_VALUES, SUSPEND_DELAY_LABELS,
    SUSPEND_DELAY_VALUES,
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
pub struct AutoScreenBlank {
    session_settings: gtk::gio::Settings,
    enabled: bool,
    delay: u32,
}

#[derive(Debug)]
pub enum AutoScreenBlankMsg {
    Toggle(bool, u32),
    Delay(u32),
}

#[derive(Debug)]
pub enum AutoScreenBlankOutput {
    Noop,
}

const BLANK_SCREEN_DEFAULT: u32 = 300;

#[relm4::component(pub)]
impl Component for AutoScreenBlank {
    type Init = PowerSettings;
    type Input = AutoScreenBlankMsg;
    type Output = AutoScreenBlankOutput;
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

                connect_active_notify[sender, blank_screen_delay_row] => move |row| {
                    sender.input(AutoScreenBlankMsg::Toggle(row.is_active(), blank_screen_delay_row.selected()));
                }
            },

            #[name(blank_screen_delay_row)]
            adw::ComboRow {
                set_title: "Delay",
                #[watch]
                set_sensitive: model.enabled,
                set_model: Some(&gtk::StringList::new(&SCREEN_BLANK_DELAY_LABELS)),

                // example: https://github.com/blissd/fotema/blob/main/src/app/components/preferences.rs#L127-L130
                connect_selected_item_notify[sender] => move |row| {
                    sender.input(AutoScreenBlankMsg::Delay(row.selected()));
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

        // sender.input(AutoScreenBlankMsg::Toggle(model.enabled, 0));
        // sender.input(AutoScreenBlankMsg::Delay(delay));

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            AutoScreenBlankMsg::Toggle(state, index) => {
                // FIXME: https://gitlab.gnome.org/GNOME/gnome-control-center/-/blob/main/panels/power/cc-power-panel.c?ref_type=heads#L740
                // true
                if state {
                    sender.input(AutoScreenBlankMsg::Delay(index));

                    // unwrap is used here because index is always in range, this should NOT fail
                    self.delay = *SCREEN_BLANK_DELAY_VALUES.get(index as usize).unwrap();
                // false
                } else {
                    sender.input(AutoScreenBlankMsg::Delay(
                        SCREEN_BLANK_DELAY_VALUES.len() as u32
                    ));
                }
                self.enabled = state;
            }
            AutoScreenBlankMsg::Delay(index) => {
                let seconds = match SCREEN_BLANK_DELAY_VALUES.get(index as usize) {
                    Some(val) => *val,
                    None => 0,
                };

                let _ = self.session_settings.set_uint("idle-delay", seconds);
            }
        }
    }
}

// Automatic Suspend / When Plugged In
#[derive(Debug)]
pub struct AutomaticSuspend {
    power_settings: gtk::gio::Settings,
    suspend_text: String,
    enabled: bool,
    delay: u32,
}

#[derive(Debug)]
pub enum AutomaticSuspendMsg {
    Toggle(bool),
    Delay(u32),
}

#[derive(Debug)]
pub enum AutomaticSuspendOutput {
    Noop,
}

#[relm4::component(pub)]
impl Component for AutomaticSuspend {
    type Init = (String, PowerSettings);
    type Input = AutomaticSuspendMsg;
    type Output = AutomaticSuspendOutput;
    type CommandOutput = ();

    view! {
        #[root]
        adw::PreferencesGroup {
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
                set_model: Some(&gtk::StringList::new(&SUSPEND_DELAY_LABELS)),

                connect_selected_item_notify[sender] => move |row| {
                    sender.input(AutomaticSuspendMsg::Delay(row.selected()));
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let power_settings = init.1.power;

        let sleep_inactive_ac_type = power_settings.string("sleep-inactive-ac-type");
        let enabled = sleep_inactive_ac_type.as_str() == "suspend";
        let delay = power_settings.int("sleep-inactive-ac-timeout") as u32;

        let model = Self {
            suspend_text: init.0,
            power_settings,
            enabled,
            delay,
        };

        // sender.input(AutoScreenBlankMsg::Toggle(model.enabled, 0));
        // sender.input(AutoScreenBlankMsg::Delay(delay));

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            AutomaticSuspendMsg::Toggle(state) => {
                self.enabled = state;

                let status = if state { "suspend" } else { "nothing" };

                let _ = self
                    .power_settings
                    .set_string("sleep-inactive-ac-type", status);
            }

            AutomaticSuspendMsg::Delay(index) => {
                let seconds = match SUSPEND_DELAY_VALUES.get(index as usize) {
                    Some(val) => *val,
                    None => 0,
                };

                println!("Seconds: {:?}\nIndex: {:?}\n\n\n\n\n", seconds, index);
                let _ = self
                    .power_settings
                    .set_int("sleep-inactive-ac-timeout", seconds as i32);
            }
        }
    }
}
