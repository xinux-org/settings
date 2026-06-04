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

#[derive(Debug)]
pub struct SystemDateTimePage {
    clock_settings: gio::Settings,
    calendar_settings: gio::Settings,
    active_clock_format: String,
    active_week_day: bool,
    active_date: bool,
    active_seconds: bool,
    active_week_numbers: bool,
}

#[derive(Debug)]
pub enum SystemDateTimeMsg {
    Switcher(ToggleSwitcher),
    ReloadFromGSettingsAll,
}

#[derive(Debug)]
pub enum ToggleSwitcher {
    ClockFormat(Option<String>),
    WeekDay(bool),
    Date(bool),
    Seconds(bool),
    WeekNumbers(bool),
}

#[relm4::component(pub)]
impl Component for SystemDateTimePage {
    type Init = ();
    type Input = SystemDateTimeMsg;
    type Output = SystemPageMsg;
    type CommandOutput = ();

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
                                #[block_signal(toggle_handler)]
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
                                connect_active_name_notify[sender] => move |toggle| {
                                    sender.input(SystemDateTimeMsg::Switcher(ToggleSwitcher::ClockFormat(toggle.active_name().map(|toggle| toggle.to_string()))))
                                } @toggle_handler,
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
                                sender.input(SystemDateTimeMsg::Switcher(ToggleSwitcher::WeekDay(row.is_active())));
                            }
                        },
                        #[name(date_row)]
                        adw::SwitchRow {
                            set_title: &gettext("Date"),
                            set_use_underline: true,
                            #[watch]
                            set_active: model.active_date,
                            connect_active_notify[sender] => move |row| {
                                sender.input(SystemDateTimeMsg::Switcher(ToggleSwitcher::Date(row.is_active())));
                            }
                        },
                        #[name(seconds_row)]
                        adw::SwitchRow {
                            set_title: &gettext("Seconds"),
                            set_use_underline: true,
                            #[watch]
                            set_active: model.active_seconds,
                            connect_active_notify[sender] => move |row| {
                                sender.input(SystemDateTimeMsg::Switcher(ToggleSwitcher::Seconds(row.is_active())));
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
                                sender.input(SystemDateTimeMsg::Switcher(ToggleSwitcher::WeekNumbers(row.is_active())));
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
        let calendar_settings = gio::Settings::new(CALENDAR_SCHEMA);

        for clock_settings_key in [
            CLOCK_FORMAT_KEY,
            CLOCK_SHOW_WEEKDAY_KEY,
            CLOCK_SHOW_DATE_KEY,
            CLOCK_SHOW_SECONDS_KEY,
        ] {
            let sender = sender.clone();

            clock_settings.connect_changed(Some(clock_settings_key), move |_settings, _key| {
                sender.input(SystemDateTimeMsg::ReloadFromGSettingsAll);
            });
        }
        for calendar_settings_key in [CALENDAR_SCHEMA, CALENDAR_SHOW_WEEK_NUMBERS_KEY] {
            let sender = sender.clone();

            calendar_settings.connect_changed(
                Some(calendar_settings_key),
                move |_settings, _key| {
                    sender.input(SystemDateTimeMsg::ReloadFromGSettingsAll);
                },
            );
        }

        let model = Self {
            clock_settings,
            calendar_settings,
            active_clock_format: String::default(),
            active_week_day: false,
            active_date: false,
            active_seconds: false,
            active_week_numbers: false,
        };
        sender.input(SystemDateTimeMsg::ReloadFromGSettingsAll);

        let widgets = view_output!();
        // set after widgets exist to avoid timing issue on
        // setting before toggles drawed
        widgets
            .time_format_toggle_group
            .set_active_name(Some(&model.active_clock_format));

        ComponentParts { model, widgets }
    }
    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            SystemDateTimeMsg::Switcher(ToggleSwitcher::ClockFormat(time_format)) => {
                let _success = self
                    .clock_settings
                    .set_string(CLOCK_FORMAT_KEY, time_format.as_deref().unwrap());
                self.active_clock_format = time_format.unwrap();
            }
            SystemDateTimeMsg::Switcher(ToggleSwitcher::WeekDay(week_day)) => {
                self.clock_settings
                    .set_boolean(CLOCK_SHOW_WEEKDAY_KEY, week_day)
                    .ok();
                self.active_week_day = week_day;
            }
            SystemDateTimeMsg::Switcher(ToggleSwitcher::Date(date)) => {
                self.clock_settings
                    .set_boolean(CLOCK_SHOW_DATE_KEY, date)
                    .ok();
                self.active_date = date;
            }
            SystemDateTimeMsg::Switcher(ToggleSwitcher::Seconds(seconds)) => {
                self.clock_settings
                    .set_boolean(CLOCK_SHOW_SECONDS_KEY, seconds)
                    .ok();
                self.active_seconds = seconds;
            }
            SystemDateTimeMsg::Switcher(ToggleSwitcher::WeekNumbers(week_numbers)) => {
                self.calendar_settings
                    .set_boolean(CALENDAR_SHOW_WEEK_NUMBERS_KEY, week_numbers)
                    .ok();
                self.active_week_numbers = week_numbers;
            }
            SystemDateTimeMsg::ReloadFromGSettingsAll => {
                self.active_clock_format = self.clock_settings.string(CLOCK_FORMAT_KEY).into();
                self.active_week_day = self.clock_settings.boolean(CLOCK_SHOW_WEEKDAY_KEY);
                self.active_date = self.clock_settings.boolean(CLOCK_SHOW_DATE_KEY);
                self.active_seconds = self.clock_settings.boolean(CLOCK_SHOW_SECONDS_KEY);
                self.active_week_numbers = self
                    .calendar_settings
                    .boolean(CALENDAR_SHOW_WEEK_NUMBERS_KEY);
            }
        }
    }
}
