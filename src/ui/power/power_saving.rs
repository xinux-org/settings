use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

use crate::ui::power::power_page::PowerMsg;

#[derive(Debug)]
pub struct SavingPowerPageView {
    /// Automatic Power Saver
    pub auto_power_saver: bool,
    /// Dim screen
    pub idle_dim: bool,
    /// Automatic Screen Black (uint32 0)
    /// Custom for ComboRow
    pub auto_screen_black: bool,
    /// Automatic Screen Black (uint32 0)
    pub idle_delay: String,

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
    SetAutoScreenBlack(bool),
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
                adw::ActionRow {
                    set_title: "Dim Screen",
                    set_subtitle: "Reduce screen brightness when the device is inactive",

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,
                        #[watch]
                        set_active: model.idle_dim,
                        connect_state_set[sender] => move |_, state| {
                            sender.input(PowerSavingMsg::SetIdleDim(state));
                            gtk::glib::Propagation::Proceed
                        },
                    },
                },

                adw::ActionRow {
                    set_title: "Automic Power Saver",
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

            adw::PreferencesGroup {
                adw::ActionRow {
                    set_title: "Automatic Screen Blank",
                    set_subtitle: "Turn the screen off after a period of inactivity",

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,
                        #[watch]
                        set_active: model.auto_screen_black,
                        connect_state_set[sender] => move |_, state| {
                            sender.input(PowerSavingMsg::SetAutoScreenBlack(state));
                            gtk::glib::Propagation::Proceed
                        },
                    },
                },

                adw::ComboRow {
                    #[watch]
                    set_sensitive: model.auto_screen_black,

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
        let model = Self {
            auto_power_saver: dconf_rs::get_boolean(
                "/org/gnome/settings-daemon/plugins/power/power-saver-profile-on-low-battery",
            )
            .unwrap(),

            idle_dim: dconf_rs::get_boolean("/org/gnome/settings-daemon/plugins/power/idle-dim")
                .unwrap(),

            auto_screen_black: true,
            idle_delay: dconf_rs::get_string("/org/gnome/desktop/session/idle-delay").unwrap(),

            sleep_inactive_battery_type: matches!(
                (dconf_rs::get_string(
                    "/org/gnome/settings-daemon/plugins/power/sleep-inactive-battery-type",
                )
                .unwrap()
                .as_str(),),
                ("suspend",)
            ),

            sleep_inactive_battery_timeout: dconf_rs::get_int(
                "/org/gnome/settings-daemon/plugins/power/sleep-inactive-battery-timeout",
            )
            .unwrap_or(0) as u16,

            sleep_inactive_ac_type: matches!(
                (dconf_rs::get_string(
                    "/org/gnome/settings-daemon/plugins/power/sleep-inactive-ac-type",
                )
                .unwrap()
                .as_str(),),
                ("suspend",)
            ),

            sleep_inactive_ac_timeout: dconf_rs::get_int(
                "/org/gnome/settings-daemon/plugins/power/sleep-inactive-battery-timeout",
            )
            .unwrap_or(0) as u16,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            PowerSavingMsg::SetAutoPowerSaver(state) => {
                self.auto_power_saver = state;

                let _ = dconf_rs::set_boolean(
                    "/org/gnome/settings-daemon/plugins/power/power-saver-profile-on-low-battery",
                    state,
                );
            }
            PowerSavingMsg::SetIdleDim(state) => {
                self.idle_dim = state;

                let _ = dconf_rs::set_boolean(
                    "/org/gnome/settings-daemon/plugins/power/idle-dim",
                    state,
                );
            }

            PowerSavingMsg::SetSleepInactiveBatteryType(state) => match state {
                true => {
                    self.sleep_inactive_battery_type = state;

                    let _ = dconf_rs::set_string(
                        "/org/gnome/settings-daemon/plugins/power/sleep-inactive-battery-type",
                        "suspend",
                    );
                }
                false => {
                    self.sleep_inactive_battery_type = state;

                    let _ = dconf_rs::set_string(
                        "/org/gnome/settings-daemon/plugins/power/sleep-inactive-battery-type",
                        "nothing",
                    );
                }
            },

            PowerSavingMsg::SetSleepInactiveACType(state) => match state {
                true => {
                    self.sleep_inactive_ac_type = state;

                    let _ = dconf_rs::set_string(
                        "/org/gnome/settings-daemon/plugins/power/sleep-inactive-ac-type",
                        "suspend",
                    );
                }
                false => {
                    self.sleep_inactive_ac_type = state;

                    let _ = dconf_rs::set_string(
                        "/org/gnome/settings-daemon/plugins/power/sleep-inactive-ac-type",
                        "nothing",
                    );
                }
            },
            PowerSavingMsg::SetAutoScreenBlack(state) => {
                self.auto_screen_black = state;
            }
        }
    }
}
