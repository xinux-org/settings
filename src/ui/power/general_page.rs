use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

use ppd::PpdProxyBlocking;
use zbus::blocking::Connection;

use relm4_components::simple_adw_combo_row::SimpleComboRow;

use std::fmt;
use std::fs;
use std::sync::Arc;

use regex::Regex;
use std::path::Path;

use crate::ui::power::power_page::PowerMsg;

use dconf_rs;

const POWER_BUTTON_ACTIONS: [&str; 4] = ["Power Off", "Hibernate", "Suspend", "Nothing"];

#[derive(Debug, Clone)]
struct BatteryModel {
    index: u8,
    percentage: f64,
    percentage_text: String,
    status: String,
}

#[derive(Debug)]
enum BatteryMsg {
    Update,
}

#[relm4::factory(pub)]
impl FactoryComponent for BatteryModel {
    type Init = BatteryModel;
    type Input = BatteryMsg;
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        #[root]
        adw::PreferencesGroup {
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_valign: gtk::Align::Center,
                set_spacing: 8,
                set_margin_all: 16,
                add_css_class: "action-row",

                gtk::LevelBar {
                    set_min_value: 1.0,
                    set_max_value: 100.0,
                    add_offset_value: ("low", 20.0),
                    add_offset_value: ("high", 60.0),
                    add_offset_value: ("full", 100.0),
                    #[watch]
                    set_value: self.percentage,
                    set_hexpand: true,
                    add_css_class: "battery-bar",
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 8,

                    gtk::Label {
                        #[watch]
                        set_label: &self.status,
                        set_halign: gtk::Align::Start,
                        set_hexpand: true,
                    },

                    gtk::Label {
                        #[watch]
                        set_label: &self.percentage_text,
                        set_halign: gtk::Align::End,
                    },
                },
            },
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, sender: FactorySender<Self>) -> Self {
        let asyncsender = sender.clone();

        relm4::spawn(async move {
            loop {
                asyncsender.input(BatteryMsg::Update);
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        });

        Self {
            index: init.index,
            percentage: init.percentage,
            percentage_text: init.percentage_text,
            status: init.status,
        }
    }

    fn update(&mut self, message: Self::Input, _sender: FactorySender<Self>) {
        match message {
            BatteryMsg::Update => {
                let percentages_float =
                    get_battery_percentages_float(read_file("capacity", "0".into()));
                let percentages_text = read_file("capacity", "0".into());
                let statuses = read_file("status", "Unknown".into());

                if let (Some(pf), Some(pt), Some(st)) = (
                    percentages_float.get(self.index as usize),
                    percentages_text.get(self.index as usize),
                    statuses.get(self.index as usize),
                ) {
                    self.percentage = *pf;
                    self.percentage_text = format!("{}%", pt.trim());
                    self.status = st.trim().to_string();
                }
            }
        }
    }
}

#[derive(Debug)]
#[tracker::track]
pub struct GeneralPowerPageView {
    pub power_mode: PowerMode,
    pub show_battery_percentage: bool,
    pub power_button_action: u32,

    #[tracker::do_not_track]
    pub power_button_action_row: Controller<SimpleComboRow<&'static str>>,

    #[tracker::do_not_track]
    batteries: FactoryVecDeque<BatteryModel>,

    #[tracker::do_not_track]
    pub ppd: Arc<PpdProxyBlocking<'static>>,
}

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
pub enum GeneralPowerPageViewMsg {
    SetPowerMode(PowerMode),
    ToggleBatteryPercentage(bool),
    SelectPowerButtonAction(usize),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerMode {
    Performance, // performance
    Balanced,    // balanced
    PowerSaver,  // power-saver
}

#[relm4::component(pub)]
impl Component for GeneralPowerPageView {
    type Init = ();
    type Input = GeneralPowerPageViewMsg;
    type Output = PowerMsg;
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 24,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 12,
                add_css_class: "heading",


                #[local_ref]
                battery_list -> gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 8,
                },
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 12,

                gtk::Label {
                    set_label: "Power Mode",
                    set_halign: gtk::Align::Start,
                    add_css_class: "heading",
                },

                adw::PreferencesGroup {
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
                            connect_activate[sender] => move |btn| {
                                if btn.is_active() {
                                    sender.input(GeneralPowerPageViewMsg::SetPowerMode(PowerMode::PowerSaver));
                                }
                            },
                        },
                    },
                },
            },


            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 12,

                gtk::Label {
                    set_label: "General",
                    set_halign: gtk::Align::Start,
                    add_css_class: "heading",
                },

                adw::PreferencesGroup {
                    #[local_ref]
                    combo_row ->
                    adw::ComboRow {
                        set_title: "Power Button Behavior",
                    },


                    #[local_ref]
                    show_battery_percentage_button -> adw::ActionRow {
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
            },
        },
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let connection = Connection::system().unwrap();
        let proxy = PpdProxyBlocking::new(&connection).unwrap();

        let percentages_float = get_battery_percentages_float(read_file("capacity", "0".into()));
        let percentages_text = read_file("capacity", "0".into());
        let statuses = read_file("status", "Unknown".into());

        // Make the button invisible in case there is no battery
        let show_battery_percentage_button = if percentages_float.is_empty() {
            adw::ActionRow::builder().visible(false).build()
        } else {
            adw::ActionRow::builder()
                .title("Show Battery Percentage")
                .subtitle("Show exact charge level in the top bar")
                .build()
        };

        // Battery level label
        let battery_level = gtk::Box::builder().build();

        if !percentages_float.is_empty() {
            battery_level.append(
                &gtk::Label::builder()
                    .label("Battery Level")
                    .halign(gtk::Align::Start)
                    .build(),
            );
        }

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
                active_index: Some(get_power_button_action_enum() as usize),
            })
            .forward(
                sender.input_sender(),
                GeneralPowerPageViewMsg::SelectPowerButtonAction,
            );

        let model = Self {
            batteries,
            power_mode: get_current_profile(&proxy),

            show_battery_percentage: false,

            power_button_action_row,
            power_button_action: get_power_button_action_enum(),

            ppd: Arc::new(proxy),
            tracker: 0,
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
            }

            GeneralPowerPageViewMsg::SelectPowerButtonAction(index) => {
                self.power_button_action = index as u32;

                let action = &match POWER_BUTTON_ACTIONS.get(index).unwrap() {
                    &"Power Off" => "interactive",
                    s => s,
                };

                let _ = dconf_rs::set_string(
                    "/org/gnome/settings-daemon/plugins/power/power-button-action",
                    action.to_lowercase().as_str(),
                );
            }
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

pub fn get_battery_path() -> Vec<fs::DirEntry> {
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

fn read_file(file_name: &str, no_entry: String) -> Vec<String> {
    let batteries = get_battery_path();

    batteries
        .iter()
        .map(|el| {
            fs::read_to_string(format!("{}/{}", el.path().display(), file_name))
                .unwrap_or(no_entry.clone())
        })
        .collect()
}

pub fn get_battery_percentages_float(els: Vec<String>) -> Vec<f64> {
    els.iter().map(|el| el.trim().parse().unwrap()).collect()
}

fn get_power_button_action_enum() -> u32 {
    match dconf_rs::get_string("/org/gnome/settings-daemon/plugins/power/power-button-action")
        .unwrap()
        .trim()
    {
        "Nothing" => 3,
        "Hibernate" => 1,
        "Suspend" => 2,
        // Expected Interactive or Power Off
        _ => 0,
    }
}
