use crate::ui::system::system_page::SystemPageMsg;
use gettextrs::gettext;
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
const _CALENDAR_WEEK_START_DAY_KEY: &str = "week-start-day";
const _FILECHOOSER_SCHEMA: &str = "org.gtk.Settings.FileChooser";
const _DATETIME_SCHEMA: &str = "org.gnome.desktop.datetime";
const _AUTO_TIMEZONE_KEY: &str = "automatic-timezone";

#[derive(Debug, Default)]
pub struct SystemDateTimePage {
    clock_settings: Option<gio::Settings>,
    calendar_settings: Option<gio::Settings>,
    active_clock_format: String,
    active_week_day: bool,
    active_date: bool,
    active_seconds: bool,
    active_week_numbers: bool,
}

#[derive(Debug)]
pub enum SystemDateTimeMsg {
    ToggleClockFormat(Option<String>),
    ToggleWeekDay(bool),
    ToggleDate(bool),
    ToggleSeconds(bool),
    ToggleWeekNumbers(bool),
}

#[relm4::component(pub)]
impl SimpleComponent for SystemDateTimePage {
    type Init = ();
    type Input = SystemDateTimeMsg;
    type Output = SystemPageMsg;

    view! {
        adw::NavigationPage {
            set_title: &gettext("Date & Time"),

            adw::ToolbarView {
                set_top_bar_style: adw::ToolbarStyle::Flat,

                add_top_bar = &adw::HeaderBar {},

                adw::PreferencesPage {
                    // We do not need automatic clock time.
                    // Itʻs done by nix config?
                    adw::PreferencesGroup {
                        adw::ActionRow {
                            set_title: &gettext("Time format"),
                            set_use_underline: true,

                            #[name(time_format_toggle_group)]
                            add_suffix = &adw::ToggleGroup {
                                set_valign: gtk::Align::Center,
                                set_homogeneous: true,
                                #[watch]
                                set_active_name: Some(&model.active_clock_format),

                                add = adw::Toggle {
                                    set_label: Some(&gettext("24-hour")),
                                    set_name: Some("24h"), // donʻt trans
                                    set_use_underline: true,
                                },

                                add = adw::Toggle {
                                    set_label: Some(&gettext("AM / PM")),
                                    set_name: Some("12h"), // donʻt trans
                                    set_use_underline: true,
                                },

                                connect_active_name_notify[sender] => move |toogle| {
                                    sender.input(SystemDateTimeMsg::ToggleClockFormat(toogle.active_name().map(|toogle| toogle.to_string())))
                                }

                            },
                        },
                    },

                    adw::PreferencesGroup {
                        set_title: &gettext("Clock and Calendar"),
                        set_description: Some(&gettext("Control how the time and date is shown in the top bar")),

                        #[name(weekday_row)]
                        adw::SwitchRow {
                            set_title: &gettext("Week day"),
                            set_use_underline: true,
                            #[watch]
                            set_active: model.active_week_day,

                            connect_active_notify[sender] => move |row| {
                                sender.input(SystemDateTimeMsg::ToggleWeekDay(row.is_active()));
                            }
                        },

                        #[name(date_row)]
                        adw::SwitchRow {
                            set_title: &gettext("Date"),
                            set_use_underline: true,
                            #[watch]
                            set_active: model.active_date,

                            connect_active_notify[sender] => move |row| {
                                sender.input(SystemDateTimeMsg::ToggleDate(row.is_active()));
                            }
                        },

                        #[name(seconds_row)]
                        adw::SwitchRow {
                            set_title: &gettext("Seconds"),
                            set_use_underline: true,
                            #[watch]
                            set_active: model.active_seconds,

                            connect_active_notify[sender] => move |row| {
                                sender.input(SystemDateTimeMsg::ToggleSeconds(row.is_active()));
                            }
                        },

                        #[name(week_numbers_row)]
                        adw::SwitchRow {
                            set_title: &gettext("Week numbers"),
                            set_subtitle: &gettext("Shown in the dropdown calendar"),
                            set_use_underline: true,
                            #[watch]
                            set_active: model.active_week_numbers,

                            connect_active_notify[sender] => move |row| {
                                sender.input(SystemDateTimeMsg::ToggleWeekNumbers(row.is_active()));
                            }
                        },
                    },
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let clock_settings = gio::Settings::new(CLOCK_SCHEMA);
        let active_clock_format: String = clock_settings.string(CLOCK_FORMAT_KEY).to_string();
        let active_week_day: bool = clock_settings.boolean(CLOCK_SHOW_WEEKDAY_KEY);
        let active_date: bool = clock_settings.boolean(CLOCK_SHOW_DATE_KEY);
        let active_seconds: bool = clock_settings.boolean(CLOCK_SHOW_SECONDS_KEY);

        let calendar_settings = gio::Settings::new(CALENDAR_SCHEMA);
        let active_week_numbers: bool = calendar_settings.boolean(CALENDAR_SHOW_WEEK_NUMBERS_KEY);

        let model = Self {
            clock_settings: Some(clock_settings),
            calendar_settings: Some(calendar_settings),
            active_clock_format,
            active_week_day,
            active_date,
            active_seconds,
            active_week_numbers,
        };

        let widgets = view_output!();

        // set after widgets exist to avoid timing issue on
        // setting before toggles drawed
        widgets
            .time_format_toggle_group
            .set_active_name(Some(&model.active_clock_format));

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            SystemDateTimeMsg::ToggleClockFormat(time_format) => {
                let _success = self
                    .clock_settings
                    .clone()
                    .map(|s| s.set_string(CLOCK_FORMAT_KEY, time_format.as_deref().unwrap()));
                self.active_clock_format = time_format.unwrap();
            }
            SystemDateTimeMsg::ToggleWeekDay(week_day) => {
                let _success = self
                    .clock_settings
                    .clone()
                    .map(|s| s.set_boolean(CLOCK_SHOW_WEEKDAY_KEY, week_day));
                self.active_week_day = week_day;
            }
            SystemDateTimeMsg::ToggleDate(date) => {
                let _success = self
                    .clock_settings
                    .clone()
                    .map(|s| s.set_boolean(CLOCK_SHOW_DATE_KEY, date));
                self.active_date = date;
            }
            SystemDateTimeMsg::ToggleSeconds(seconds) => {
                let _success = self
                    .clock_settings
                    .clone()
                    .map(|s| s.set_boolean(CLOCK_SHOW_SECONDS_KEY, seconds));
                self.active_seconds = seconds;
            }
            SystemDateTimeMsg::ToggleWeekNumbers(week_numbers) => {
                let _success = self
                    .calendar_settings
                    .clone()
                    .map(|s| s.set_boolean(CALENDAR_SHOW_WEEK_NUMBERS_KEY, week_numbers));
                self.active_week_numbers = week_numbers;
            }
        }
    }
}
