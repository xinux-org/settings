use crate::ui::power::{battery_row::BatteryModel, power_page::PowerMsg};
use ppd::PpdProxyBlocking;
use regex::Regex;
use relm4::{
    adw::prelude::*,
    gtk::{self},
    prelude::*,
};
use relm4_components::simple_adw_combo_row::SimpleComboRow;
use std::process::{Command, Stdio};
use std::{fmt, fs, path::Path, sync::Arc};
use zbus::blocking::Connection;

use crate::ui::power::reusables::{AutoScreenBlank, AutoScreenBlankOutput};
use crate::ui::power::reusables::{AutomaticSuspend, AutomaticSuspendOutput};
use crate::ui::power::reusables::{DimScreen, DimScreenOutput};

use gtk::gio::Settings;

use crate::utils::power::{POWER_BUTTON_ACTIONS, SUSPEND_DELAY_LABELS, SUSPEND_DELAY_VALUES};

#[derive(Debug, Clone)]
pub struct PowerSettings {
    pub session: Settings,
    pub power: Settings,
    pub interface: Settings,
}

impl PowerSettings {
    pub fn new() -> Self {
        Self {
            session: Settings::new("org.gnome.desktop.session"),
            power: Settings::new("org.gnome.settings-daemon.plugins.power"),
            interface: Settings::new("org.gnome.desktop.interface"),
        }
    }
}

/// Possible actions for power button(usually turn on/off)
/// - Power Off - which is sometimes stated as Interactive will prompt you to decide if you really to turn off your device.
/// - Hibernate - turns of the device after copying the current state of running applications from RAM to SWAP(if configured)
/// - Suspend - does NOT turn off the device, instead, it switches to sleep mode or low power consumption mode keeping the applications open and running.
/// - Nothing - the name is self explanatory.

impl fmt::Display for PowerMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            PowerMode::Performance => "performance",
            PowerMode::Balanced => "balanced",
            PowerMode::PowerSaver => "power-saver",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug)]
#[tracker::track]
pub struct GeneralPowerPageView {
    #[tracker::do_not_track]
    pub settings: PowerSettings,

    pub power_mode: PowerMode,
    pub charging_mode: ChargingMode,

    pub show_battery_percentage: bool,
    pub show_batteries: bool,

    pub power_button_action: u32,
    pub battery_label_text: String,

    #[tracker::do_not_track]
    pub power_button_action_row: Controller<SimpleComboRow<&'static str>>,

    #[tracker::do_not_track]
    batteries: FactoryVecDeque<BatteryModel>,

    #[tracker::do_not_track]
    pub ppd: Arc<PpdProxyBlocking<'static>>,

    // Power Saving Options
    /// Dim screen
    pub idle_dim: bool,
    #[tracker::do_not_track]
    pub dim_screen_controller: Controller<DimScreen>,
    #[tracker::do_not_track]
    pub auto_screen_black_controller: Controller<AutoScreenBlank>,
    #[tracker::do_not_track]
    pub automatic_suspend_controller: Controller<AutomaticSuspend>,

    // Automatic Suspend
    /// While plugged in (ac => Alternating Current)
    pub sleep_inactive_ac_type: bool,
    /// Suspend on AC timeout
    pub sleep_inactive_ac_timeout: u16,
}

#[derive(Debug)]
pub enum GeneralPowerPageViewMsg {
    SetPowerMode(PowerMode),
    SetChargingMode(ChargingMode),
    ToggleBatteryPercentage(bool),
    SelectPowerButtonAction(usize),

    // Automatic Suspend
    SetIdleDim(bool),
    SetSleepInactiveACType(bool),

    // no operation needed.
    // we do it just to avoit type Output
    // in child component handling
    Noop,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerMode {
    Performance, // performance
    Balanced,    // balanced
    PowerSaver,  // power-saver
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChargingMode {
    Preserve,    // with a threshold
    Maximize,    // 100% without a threshold
    Unsupported, // couldn't find the threshold file
}

#[relm4::component(pub)]
impl Component for GeneralPowerPageView {
    type Init = ();
    type Input = GeneralPowerPageViewMsg;
    type Output = PowerMsg;
    type CommandOutput = ();

    view! {
        adw::PreferencesPage {
            #[name(battery_section)]
            adw::PreferencesGroup {
                #[watch]
                set_visible: model.show_batteries,
                set_title: model.battery_label_text.as_str(),

                #[local_ref]
                battery_list -> gtk::ListBox {
                    set_selection_mode: gtk::SelectionMode::None,
                    add_css_class: "boxed-list",
                },
            },

            adw::PreferencesGroup {
                set_title: "Battery Charging",
                set_visible: model.charging_mode != ChargingMode::Unsupported,

                adw::ActionRow {
                    set_title: "Maximize Charge",
                    set_subtitle: "Uses all battery capacity. Degrades batteries more quickly.",
                    set_activatable: true,

                    set_activatable_widget: Some(&activatable_maximize),

                    #[name = "activatable_maximize"]
                    add_prefix = &gtk::CheckButton {
                        set_group: Some(&activatable_preserve),

                        #[watch]
                        set_active: model.charging_mode == ChargingMode::Maximize,
                        connect_toggled[sender] => move |btn| {
                            if btn.is_active() {
                                sender.input(GeneralPowerPageViewMsg::SetChargingMode(ChargingMode::Maximize));
                            }
                        },
                    },
                },

                adw::ActionRow {
                    set_title: "Preserve Battery Health",
                    set_subtitle: "Increases battery longevity by maintaining lower charge levels.",
                    set_activatable: true,

                    set_activatable_widget: Some(&activatable_preserve),

                    #[name = "activatable_preserve"]
                    add_prefix = &gtk::CheckButton {

                        #[watch]
                        set_active: model.charging_mode == ChargingMode::Preserve,
                        connect_toggled[sender] => move |btn| {
                            if btn.is_active() {
                                sender.input(GeneralPowerPageViewMsg::SetChargingMode(ChargingMode::Preserve));
                            }
                        },
                    },
                },
            },

            adw::PreferencesGroup {
                set_title: "Power Mode",

                adw::ActionRow {
                    set_title: "Performance",
                    set_subtitle: "High performance and power usage",
                    set_activatable: true,

                    set_activatable_widget: Some(&activatable_performance),

                    #[name = "activatable_performance"]
                    add_prefix = &gtk::CheckButton {
                        set_group: Some(&activatable_balanced),

                        #[watch]
                        set_active: model.power_mode == PowerMode::Performance,
                        connect_toggled[sender] => move |btn| {
                            if btn.is_active() {
                                sender.input(GeneralPowerPageViewMsg::SetPowerMode(PowerMode::Performance));
                            }
                        },
                    },
                },

                adw::ActionRow {
                    set_title: "Balanced",
                    set_subtitle: "Standard performance and power usage",
                    set_activatable: true,

                    set_activatable_widget: Some(&activatable_balanced),

                    #[name = "activatable_balanced"]
                    add_prefix = &gtk::CheckButton {
                        set_group: Some(&activatable_powersaver),

                        #[watch]
                        set_active: model.power_mode == PowerMode::Balanced,
                        connect_toggled[sender] => move |btn| {
                            if btn.is_active() {
                                sender.input(GeneralPowerPageViewMsg::SetPowerMode(PowerMode::Balanced));
                            }
                        },
                    },
                },

                adw::ActionRow {
                    set_title: "Power Saver",
                    set_subtitle: "Reduced performance and power usage",
                    set_activatable: true,

                    set_activatable_widget: Some(&activatable_powersaver),

                    #[name = "activatable_powersaver"]
                    add_prefix = &gtk::CheckButton {
                        #[watch]
                        set_active: model.power_mode == PowerMode::PowerSaver,
                        connect_toggled[sender] => move |btn| {
                            if btn.is_active() {
                                sender.input(GeneralPowerPageViewMsg::SetPowerMode(PowerMode::PowerSaver));
                            }
                        },
                    },
                },

            },

            adw::PreferencesGroup {
                set_title: "General",

                #[local_ref]
                combo_row ->
                adw::ComboRow {
                    set_title: "Power Button Behavior",
                },

                adw::ActionRow {
                    set_title: "Show Battery Percentage",
                    set_subtitle: "Show exact charge level in the top bar",

                    set_visible: model.show_batteries,

                    add_suffix = &gtk::Switch {
                        set_valign: gtk::Align::Center,
                        #[watch]
                        set_active: model.show_battery_percentage,
                        connect_state_set[sender] => move |_, state| {
                            sender.input(GeneralPowerPageViewMsg::ToggleBatteryPercentage(state));
                            gtk::glib::Propagation::Proceed
                        },
                    },
                },
            },


            // Dim Screen
            adw::PreferencesGroup {
                set_title: "Power Saving",
                #[watch]
                set_visible: !model.show_batteries,
                add: model.dim_screen_controller.widget(),
            },

            // Automatic Screen Black
            adw::PreferencesGroup {
                #[watch]
                set_visible: !model.show_batteries,
                add: model.auto_screen_black_controller.widget(),
            },

            // Automatic Suspend
            adw::PreferencesGroup {
                #[watch]
                set_visible: !model.show_batteries,
                add: model.automatic_suspend_controller.widget(),
            },

            adw::PreferencesGroup {
                set_visible: !model.show_batteries,

                adw::ActionRow {
                    set_title: "Disabling automatic suspend will result in higher power consumption. It is recomended to keep automatic suspend enabled.",

                    #[watch]
                    set_visible: !model.sleep_inactive_ac_type,

                    add_prefix = &gtk::Image {
                        set_icon_name: Some("info-outline"),
                        set_pixel_size: 24,
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
        let connection = Connection::system().expect("Connection failed");
        let proxy = PpdProxyBlocking::new(&connection).expect("PPD Proxy failed");
        let settings = PowerSettings::new();

        let idle_dim = settings.power.boolean("idle-dim");
        let idle_delay = settings.session.uint("idle-delay").to_string();
        let show_battery_percentage = settings.interface.boolean("show-battery-percentage");
        let sleep_inactive_ac_type = matches!(
            (settings.power.string("sleep-inactive-ac-type").as_str(),),
            ("suspend",)
        );
        let sleep_inactive_ac_timeout = settings.power.int("sleep-inactive-ac-timeout") as u16;
        let power_button_action =
            get_power_button_action_enum(settings.power.string("power-button-action").to_string());
        let percentages_float = get_battery_percentages_float(read_file("capacity", "0".into()));
        let percentages_text = read_file("capacity", "0".into());
        let statuses = read_file("status", "Unknown".into());

        // If the percentages_float vector is empty is means there was not battery found in `/sys/class/power_supply/` folder.
        // self-explanatory: TRUE if battery exists FALSE if not
        let has_battery = !percentages_float.is_empty();

        // Battery level label
        let battery_level = gtk::ListBox::builder().build();

        let battery_label = match percentages_float.len() {
            1 => String::from("Battery Level"),
            _ => String::from("Battery Levels"),
        };

        let mut batteries = FactoryVecDeque::builder().launch(battery_level).detach();
        for i in 0..percentages_float.len() {
            batteries.guard().push_back(BatteryModel {
                index: i as u8,
                percentage: percentages_float[i],
                percentage_text: format!("{}%", percentages_text[i].trim()),
                status: statuses[i].trim().to_string(),
            });
        }

        let power_button_action_row = SimpleComboRow::builder()
            .launch(SimpleComboRow {
                variants: POWER_BUTTON_ACTIONS.to_vec(),
                active_index: Some(power_button_action as usize),
            })
            .forward(
                sender.input_sender(),
                GeneralPowerPageViewMsg::SelectPowerButtonAction,
            );

        let dim_screen_controller =
            DimScreen::builder()
                .launch(idle_dim)
                .forward(sender.input_sender(), |out| match out {
                    DimScreenOutput::Toggled(state) => GeneralPowerPageViewMsg::SetIdleDim(state),
                });

        let auto_screen_black_controller = AutoScreenBlank::builder()
            .launch(settings.to_owned())
            .forward(sender.input_sender(), |out| match out {
                // we do not need child and parent relationship in this case
                AutoScreenBlankOutput::Noop => GeneralPowerPageViewMsg::Noop,
            });

        let automatic_suspend_controller = AutomaticSuspend::builder()
            .launch((
                "Automatic Suspend".to_string(),
                "ac".to_string(),
                settings.to_owned(),
                SUSPEND_DELAY_LABELS
                    .iter()
                    .map(ToString::to_string)
                    .collect(),
                SUSPEND_DELAY_VALUES.to_vec(),
            ))
            .forward(sender.input_sender(), |out| match out {
                AutomaticSuspendOutput::Noop => GeneralPowerPageViewMsg::Noop,
            });

        let model = Self {
            settings,
            batteries,
            power_mode: get_current_profile(&proxy),
            charging_mode: decide_charging_mode(),

            show_battery_percentage,
            show_batteries: has_battery,
            battery_label_text: battery_label,

            power_button_action_row,
            power_button_action,

            ppd: Arc::new(proxy),
            tracker: 0,

            // In case there is no battery
            idle_dim,
            dim_screen_controller,

            // auto_screen_black: enabled,
            // auto_screen_black_delay: delay,
            auto_screen_black_controller,

            automatic_suspend_controller,
            sleep_inactive_ac_type,
            sleep_inactive_ac_timeout,
        };

        let combo_row = model.power_button_action_row.widget();
        let battery_list = model.batteries.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            GeneralPowerPageViewMsg::SetPowerMode(mode) => {
                self.power_mode = mode;

                self.ppd
                    .set_active_profile(format!("{}", mode).trim().to_lowercase())
                    .unwrap();
            }
            GeneralPowerPageViewMsg::ToggleBatteryPercentage(state) => {
                self.show_battery_percentage = state;

                let _ = self
                    .settings
                    .interface
                    .set_boolean("show-battery-percentage", state);
            }

            GeneralPowerPageViewMsg::SelectPowerButtonAction(index) => {
                self.power_button_action = index as u32;

                let action = &match POWER_BUTTON_ACTIONS.get(index).unwrap() {
                    &"Power Off" => "interactive",
                    s => s,
                };

                let _ = self
                    .settings
                    .power
                    .set_string("power-button-action", action.to_lowercase().as_str());
            }
            GeneralPowerPageViewMsg::SetChargingMode(mode) => {
                self.charging_mode = mode;

                match mode {
                    ChargingMode::Preserve => change_battery_threshold(40, 80),
                    ChargingMode::Maximize => change_battery_threshold(20, 100),
                    ChargingMode::Unsupported => change_battery_threshold(20, 100),
                }
            }

            GeneralPowerPageViewMsg::SetIdleDim(state) => {
                self.idle_dim = state;

                let _ = self.settings.power.set_boolean("idle-dim", state);
            }

            GeneralPowerPageViewMsg::SetSleepInactiveACType(state) => match state {
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
            GeneralPowerPageViewMsg::Noop => {}
        }
    }
}

fn get_current_profile(proxy: &PpdProxyBlocking) -> PowerMode {
    match proxy.active_profile().unwrap().trim() {
        "balanced" => PowerMode::Balanced,
        "power-saver" => PowerMode::PowerSaver,
        "performance" => PowerMode::Performance,
        _ => PowerMode::Balanced,
    }
}

pub(super) fn get_battery_path() -> Vec<fs::DirEntry> {
    // for debugging: /home/bahrom/workplace/xinux/settings/batteries /BAT0/capacity
    let global_path = Path::new("/sys/class/power_supply/");
    let re = Regex::new(r"BAT[0-9]+").expect("Wrong RegEx");

    let entries = match global_path.read_dir() {
        Ok(els) => els,
        Err(_) => return Vec::new(),
    };

    entries
        .filter_map(|el| el.ok())
        .filter(|el| re.is_match(el.path().to_str().unwrap()))
        .collect()
}

pub(super) fn read_file(file_name: &str, no_entry: String) -> Vec<String> {
    let batteries = get_battery_path();

    batteries
        .iter()
        .map(|el| {
            fs::read_to_string(format!("{}/{}", el.path().display(), file_name))
                .unwrap_or(no_entry.clone())
        })
        .collect()
}

pub(super) fn get_battery_percentages_float(els: Vec<String>) -> Vec<f64> {
    els.iter()
        .map(|el| el.trim().parse().unwrap_or(0.0))
        .collect()
}

fn change_battery_threshold(_start: u8, end: u8) {
    let batteries = get_battery_path();

    for bat in batteries {
        let path = bat.path();

        let start_path = path.join("charge_control_start_threshold");
        let end_path = path.join("charge_control_end_threshold");

        if !start_path.exists() || !end_path.exists() {
            continue;
        }

        relm4::spawn(async move {
            let echo_child = Command::new("echo")
                .arg(end.to_string())
                .stdout(Stdio::piped())
                .spawn();
            // ASSUMED that echo command never fails
            let echo_child_stdout = echo_child.unwrap().stdout.unwrap();
            // same as echo_child variable

            let output = tokio::process::Command::new("pkexec")
                .arg("tee")
                .arg(end_path)
                .stdin(Stdio::from(echo_child_stdout))
                .output()
                .await;

            println!("{:?}", output.unwrap());
        });
    }
}

fn get_power_button_action_enum(action: String) -> u32 {
    match action.as_str() {
        "Nothing" => 3,
        "Hibernate" => 1,
        "Suspend" => 2,
        // Expected Interactive or Power Off
        _ => 0,
    }
}

fn decide_charging_mode() -> ChargingMode {
    let batteries = get_battery_path();

    let charging_modes = batteries
        .iter()
        .filter_map(|bat| {
            let path = bat.path().join("charge_control_end_threshold");

            let content = match fs::read_to_string(&path) {
                Ok(c) => c,
                Err(error) => {
                    println!("Failed to read {:?}: {}", path, error);
                    return None;
                }
            };

            match content.trim().parse::<u32>() {
                Ok(v) => Some(v),
                Err(error) => {
                    eprintln!("Failed to parse content: {}", error);
                    None
                }
            }
        })
        .collect::<Vec<u32>>();

    if charging_modes.is_empty() {
        return ChargingMode::Unsupported;
    }

    println!("{:?}", charging_modes);

    if charging_modes.contains(&(100)) {
        ChargingMode::Maximize
    } else {
        ChargingMode::Preserve
    }
}
