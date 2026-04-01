use crate::ui::system::system_page::SystemPageMsg;
use relm4::{
    adw::{self, prelude::*},
    gtk::{self, gio},
    prelude::*,
};

const CLOCK_SCHEMA: &str = "org.gnome.desktop.interface";
const CLOCK_FORMAT_KEY: &str = "clock-format";
const CLOCK_SHOW_WEEKDAY_KEY: &str = "clock-show-weekday";
const CLOCK_SHOW_DATE_KEY: &str = "clock-show-date";
const CLOCK_SHOW_SECONDS_KEY: &str = "clock-show-seconds";
const CALENDAR_SCHEMA: &str = "org.gnome.desktop.calendar";
const CALENDAR_SHOW_WEEK_NUMBERS_KEY: &str = "show-weekdate";
const CALENDAR_WEEK_START_DAY_KEY: &str = "week-start-day";
const FILECHOOSER_SCHEMA: &str = "org.gtk.Settings.FileChooser";
const DATETIME_SCHEMA: &str = "org.gnome.desktop.datetime";
const AUTO_TIMEZONE_KEY: &str = "automatic-timezone";

#[derive(Debug, Default)]
pub struct SystemDateTimePage {
    active_name: String,
}

#[derive(Debug)]
pub enum SystemDateTimeMsg {
    ToggleClockFormat(Option<String>),
}

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
                            set_use_underline: true,

                            #[name(time_format_toggle_group)]
                            add_suffix = &adw::ToggleGroup {
                                set_valign: gtk::Align::Center,
                                set_homogeneous: true,
                                #[watch]
                                set_active_name: Some(&model.active_name),

                                add = adw::Toggle {
                                    set_label: Some("24-hour"),
                                    set_name: Some("24h"),
                                    set_use_underline: true,
                                },

                                add = adw::Toggle {
                                    set_label: Some("AM / PM"),
                                    set_name: Some("12h"),
                                    set_use_underline: true,
                                },

                                connect_notify: (None, move |toogle, _param_sec| {
                                    sender.input(SystemDateTimeMsg::ToggleClockFormat(toogle.active_name().map(|toogle| toogle.to_string())))
                                }),

                            },
                        },
                    },

                    adw::PreferencesGroup {
                        set_title: "Clock and Calendar",
                        set_description: Some("Control how the time and date is shown in the top bar"),

                        #[name(weekday_row)]
                        adw::SwitchRow {
                            set_title: "Week day",
                            set_use_underline: true,
                        },

                        #[name(date_row)]
                        adw::SwitchRow {
                            set_title: "Date",
                            set_use_underline: true,
                        },

                        #[name(seconds_row)]
                        adw::SwitchRow {
                            set_title: "Seconds",
                            set_use_underline: true,
                        },

                        #[name(week_numbers_row)]
                        adw::SwitchRow {
                            set_title: "Week numbers",
                            set_subtitle: "Shown in the dropdown calendar",
                            set_use_underline: true,
                        },
                    },
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let settings = gio::Settings::new(CLOCK_SCHEMA);
        let active_format: String = settings.string(CLOCK_FORMAT_KEY).to_string();
        println!("DEBUG: System GSetting value is: {}", active_format);

        let model = Self {
            active_name: active_format,
        };

        let widgets = view_output!();
        // set after widgets exist to avoid timing issue on
        // setting before toggles drawed
        // widgets
        //     .time_format_toggle_group
        //     .set_active_name(Some(&model.active_name));

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            SystemDateTimeMsg::ToggleClockFormat(time_format) => {
                // gio::Settings::connect_changed
                let settings = gio::Settings::new(CLOCK_SCHEMA);
                // let current_format: String = settings.string("clock-format").to_string();
                let _success =
                    settings.set_string(CLOCK_FORMAT_KEY, time_format.as_deref().unwrap());
                self.active_name = time_format.unwrap();
            }
        }
    }
}

fn get_active_clock_format() -> String {
    let settings = gio::Settings::new(CLOCK_SCHEMA);
    let active_format: String = settings.string(CLOCK_FORMAT_KEY).to_string();
    println!("DEBUG: System GSetting value is: {}", active_format);
    // Some(active_format)
    active_format
}
