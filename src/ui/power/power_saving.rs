use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

use crate::ui::power::general_page::PowerSettings;
use crate::ui::power::power_page::PowerMsg;

use crate::ui::power::reusables::{AutoScreenBlack, AutoScreenBlackOutput};
use crate::ui::power::reusables::{DimScreen, DimScreenOutput};

#[derive(Debug)]
pub struct SavingPowerPageView {
    pub settings: PowerSettings,

    /// Automatic Power Saver
    pub auto_power_saver: bool,
    /// Dim screen
    pub idle_dim: bool,
    dim_screen_controller: Controller<DimScreen>,
    /// Automatic Screen Black (uint32 0)
    /// Custom for ComboRow
    pub auto_screen_black: bool,
    /// Automatic Screen Black (uint32 0)
    pub auto_screen_black_delay: u16,
    pub auto_screen_black_controller: Controller<AutoScreenBlack>,

    // Automatic Suspend
    /// On Battery Power
    pub sleep_inactive_battery_type: bool,
    /// While plugged in (ac => Alternating Current)
    pub sleep_inactive_ac_type: bool,

    /// Suspend on battery power timeout
    pub sleep_inactive_battery_timeout: u16,
    /// Suspend on AC timeout
    pub sleep_inactive_ac_timeout: u16,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// ### Currently Unused
/// Used for Automatic Suspend
/// Values: suspend | nothing
pub enum SleepInactiveType {
    Suspend,
    Nothing,
    // Keep other options commented because it's Automatic *Suspend*
    // Blank,
    // Interactive,
    // Hibernate,
    // Logout,
    // Shutdown,
}

#[derive(Debug)]
pub enum PowerSavingMsg {
    SetAutoPowerSaver(bool),
    SetIdleDim(bool),
    SetSleepInactiveBatteryType(bool),
    SetSleepInactiveACType(bool),
    SetAutoScreenBlackEnabled(bool),
    SetAutoScreenBlackDelay(u16),
}

#[relm4::component(pub)]
impl Component for SavingPowerPageView {
    type Init = ();
    type Input = PowerSavingMsg;
    type Output = PowerMsg;
    type CommandOutput = ();

    view! {
        adw::PreferencesPage {
            adw::PreferencesGroup {
                // Dim Screen
                model.dim_screen_controller.widget(),

                // adw::ActionRow {
                //     set_title: "Dim Screen",
                //     set_subtitle: "Reduce screen brightness when the device is inactive",
                //
                //     add_suffix = &gtk::Switch {
                //         set_valign: gtk::Align::Center,
                //         #[watch]
                //         set_active: model.idle_dim,
                //         connect_state_set[sender] => move |_, state| {
                //             sender.input(PowerSavingMsg::SetIdleDim(state));
                //             gtk::glib::Propagation::Proceed
                //         },
                //     },
                // },

                adw::ActionRow {
                    set_title: "Automatic Power Saver",
                    set_subtitle: "Turn on power saver made when battery power is low",

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,
                        #[watch]
                        set_active: model.auto_power_saver,
                        connect_state_set[sender] => move |_, state| {
                            sender.input(PowerSavingMsg::SetAutoPowerSaver(state));
                            gtk::glib::Propagation::Proceed
                        },
                    },
                }
            },

            // Automatic Screen Black
            adw::PreferencesGroup {

                model.auto_screen_black_controller.widget(),
            },

            adw::PreferencesGroup {
                set_title: "Automatic Suspend",

                adw::ActionRow {
                    set_title: "On Battery Power",

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,
                        #[watch]
                        set_active: model.sleep_inactive_battery_type,
                        connect_state_set[sender] => move |_, state| {
                            sender.input(PowerSavingMsg::SetSleepInactiveBatteryType(state));
                            gtk::glib::Propagation::Proceed
                        },
                    },
                },

                adw::ComboRow {
                    #[watch]
                    set_sensitive: model.sleep_inactive_battery_type,

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
                    set_title: "When plugged",

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,
                        #[watch]
                        set_active: model.sleep_inactive_ac_type,
                        connect_state_set[sender] => move |_, state| {
                            sender.input(PowerSavingMsg::SetSleepInactiveACType(state));
                            gtk::glib::Propagation::Proceed
                        },
                    },
                },

                adw::ComboRow {
                    #[watch]
                    set_sensitive: model.sleep_inactive_ac_type,

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

                    #[watch]
                    set_visible: !model.sleep_inactive_battery_type || !model.sleep_inactive_ac_type,

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
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let settings = PowerSettings::new();

        let idle_dim = settings.power.boolean("idle-dim");
        let auto_power_saver = settings.power.boolean("power-saver-profile-on-low-battery");

        let current_delay = settings.session.uint("idle-delay");
        let enabled = current_delay != 0;
        let delay = current_delay as u16;

        let auto_screen_black_controller = AutoScreenBlack::builder()
            .launch((enabled, delay))
            .forward(sender.input_sender(), |out| match out {
                AutoScreenBlackOutput::Toggled(state) => {
                    PowerSavingMsg::SetAutoScreenBlackEnabled(state)
                }
                AutoScreenBlackOutput::Delay(seconds) => {
                    PowerSavingMsg::SetAutoScreenBlackDelay(seconds)
                }
            });

        let sleep_inactive_battery_type = matches!(
            (settings
                .power
                .string("sleep-inactive-battery-type")
                .as_str(),),
            ("suspend",)
        );
        let sleep_inactive_battery_timeout =
            settings.power.int("sleep-inactive-battery-timeout") as u16;

        let sleep_inactive_ac_type = matches!(
            (settings.power.string("sleep-inactive-ac-type").as_str(),),
            ("suspend",)
        );

        let sleep_inactive_ac_timeout = settings.power.int("sleep-inactive-battery-timeout") as u16;

        let dim_screen_controller =
            DimScreen::builder()
                .launch(idle_dim)
                .forward(sender.input_sender(), |out| match out {
                    DimScreenOutput::Toggled(state) => PowerSavingMsg::SetIdleDim(state),
                });

        let model = Self {
            settings,

            auto_power_saver,

            idle_dim,
            dim_screen_controller,

            auto_screen_black: enabled,
            auto_screen_black_delay: delay,
            auto_screen_black_controller,

            sleep_inactive_battery_type,
            sleep_inactive_battery_timeout,

            sleep_inactive_ac_type,
            sleep_inactive_ac_timeout,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            PowerSavingMsg::SetAutoPowerSaver(state) => {
                self.auto_power_saver = state;

                let _ = self
                    .settings
                    .power
                    .set_boolean("power-saver-profile-on-low-battery", state);
            }

            PowerSavingMsg::SetIdleDim(state) => {
                self.idle_dim = state;

                let _ = self.settings.power.set_boolean("idle-dim", state);
            }

            PowerSavingMsg::SetSleepInactiveBatteryType(state) => match state {
                true => {
                    self.sleep_inactive_battery_type = state;

                    let _ = self
                        .settings
                        .power
                        .set_string("sleep-inactive-battery-type", "suspend");
                }
                false => {
                    self.sleep_inactive_battery_type = state;
                    let _ = self
                        .settings
                        .power
                        .set_string("sleep-inactive-battery-type", "nothing");
                }
            },

            PowerSavingMsg::SetSleepInactiveACType(state) => match state {
                true => {
                    self.sleep_inactive_ac_type = state;

                    let _ = self
                        .settings
                        .power
                        .set_string("sleep-inactive-ac-type", "suspend");
                }
                false => {
                    self.sleep_inactive_ac_type = state;

                    let _ = self
                        .settings
                        .power
                        .set_string("sleep-inactive-ac-type", "nothing");
                }
            },
            PowerSavingMsg::SetAutoScreenBlackEnabled(state) => {
                self.auto_screen_black = state;

                let value = if state {
                    self.auto_screen_black_delay as u32
                } else {
                    0
                };

                let _ = self.settings.session.set_uint("idle-delay", value);
            }

            PowerSavingMsg::SetAutoScreenBlackDelay(d) => {
                self.auto_screen_black_delay = d;

                if self.auto_screen_black {
                    let _ = self.settings.session.set_uint("idle-delay", d as u32);
                }
            }
        }
    }
}
