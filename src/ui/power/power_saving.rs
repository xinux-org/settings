use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

use gettextrs::gettext;

use crate::ui::power::general_page::PowerSettings;
use crate::ui::power::power_page::PowerMsg;

use crate::ui::power::reusables::{AutoScreenBlank, AutoScreenBlankOutput};
use crate::ui::power::reusables::{AutomaticSuspend, AutomaticSuspendOutput};
use crate::ui::power::reusables::{DimScreen, DimScreenOutput};

use crate::utils::power::SUSPEND_DELAY_VALUES;

#[derive(Debug)]
pub struct SavingPowerPageView {
    pub settings: PowerSettings,

    /// Automatic Power Saver
    pub auto_power_saver: bool,
    /// Dim screen
    pub idle_dim: bool,
    dim_screen_controller: Controller<DimScreen>,
    pub auto_screen_black_controller: Controller<AutoScreenBlank>,

    // Automatic Suspend
    /// On Battery Power
    pub sleep_inactive_battery_type: bool,
    /// While plugged in (ac => Alternating Current)
    pub sleep_inactive_ac_type: bool,

    /// Suspend on battery power timeout
    pub sleep_inactive_battery_timeout: u16,
    /// Suspend on AC timeout
    pub sleep_inactive_ac_timeout: u16,
    pub automatic_suspend_controller: Controller<AutomaticSuspend>,
    pub automatic_suspend_controller_battery: Controller<AutomaticSuspend>,
}

#[derive(Debug)]
pub enum PowerSavingMsg {
    SetAutoPowerSaver(bool),
    SetIdleDim(bool),
    SetSleepInactiveBatteryType(bool),
    SetSleepInactiveACType(bool),
    SetSleepInactiveACTimeout(u16),
    Noop,
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
                add: model.auto_screen_black_controller.widget(),
            },

            adw::PreferencesGroup {
                set_title: "Automatic Suspend",

                // On Battery Power
                add: model.automatic_suspend_controller_battery.widget(),
            },

            adw::PreferencesGroup {
                // When Plugged In
                add: model.automatic_suspend_controller.widget(),
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
        let suspend_delay_labels: Vec<String> = [
            gettext("15 minute"),
            gettext("20 minute"),
            gettext("25 minute"),
            gettext("30 minute"),
            gettext("45 minute"),
            gettext("1 hour"),
            gettext("1 hour 20 minute"),
            gettext("1 hour 30 minute"),
            gettext("1 hour 40 minute"),
            gettext("2 hours"),
        ]
        .to_vec();

        let settings = PowerSettings::new();

        let idle_dim = settings.power.boolean("idle-dim");
        let auto_power_saver = settings.power.boolean("power-saver-profile-on-low-battery");

        let auto_screen_black_controller = AutoScreenBlank::builder()
            .launch(settings.to_owned())
            .forward(sender.input_sender(), |out| match out {
                // we do not need child and parent relationship in this case
                AutoScreenBlankOutput::Noop => PowerSavingMsg::Noop,
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

        let automatic_suspend_controller = AutomaticSuspend::builder()
            .launch((
                "When Plugged In".to_string(),
                "ac".to_string(),
                settings.to_owned(),
                // suspend_delay_labels.clone(),
                SUSPEND_DELAY_VALUES.to_vec(),
            ))
            .forward(sender.input_sender(), |out| match out {
                AutomaticSuspendOutput::Noop => PowerSavingMsg::Noop,
            });

        let automatic_suspend_controller_battery = AutomaticSuspend::builder()
            .launch((
                "On Battery Power".to_string(),
                "ac".to_string(),
                settings.to_owned(),
                // suspend_delay_labels.clone(),
                SUSPEND_DELAY_VALUES.to_vec(),
            ))
            .forward(sender.input_sender(), |out| match out {
                AutomaticSuspendOutput::Noop => PowerSavingMsg::Noop,
            });

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
            auto_screen_black_controller,

            sleep_inactive_battery_type,
            sleep_inactive_battery_timeout,

            sleep_inactive_ac_type,
            sleep_inactive_ac_timeout,
            automatic_suspend_controller,
            automatic_suspend_controller_battery,
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
            PowerSavingMsg::SetSleepInactiveACTimeout(_seconds) => {}
            PowerSavingMsg::Noop => {}
        }
    }
}
