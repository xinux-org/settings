use crate::ui::system::system_page::SystemPageMsg;
use relm4::{
    adw::{self, prelude::*},
    gtk::{self},
    prelude::*,
};

#[derive(Debug, Default)]
pub struct SystemDateTimePage {}

#[derive(Debug)]
pub enum SystemDateTimeMsg {
    ChangeClockSettingsFormat(Option<String>),
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

                                connect_notify: (Some("twenty-four"),  move |_toogle, name| {
                                    // sender.input(ChangeClockSettingsFormat(name))
                                    println!("status coming: twenty-four");
                                }),

                                connect_notify: (Some("am-pm"),  move |_toogle, name| {
                                    // sender.input(ChangeClockSettingsFormat(name))
                                    println!("am-pm: {:?}", name);
                                }),
                                // notify::active => $change_clock_settings_cb(template);

                                add = adw::Toggle {
                                    set_label: Some("24-hour"),
                                    set_name: Some("twenty-four"),
                                    set_use_underline: true,
                                },

                                add = adw::Toggle {
                                    set_label: Some("AM / PM"),
                                    set_name: Some("am-pm"),
                                    set_use_underline: true,
                                },
                                // set_active_name: Some("all"),
                                // connect_active_name_notify[sender] => move |group| {
                                //     // let filter = group.active_name().map_or(ChatListFilter::default(), |tag| tag.as_str().into());
                                //     // sender.input(ChatListInput::ApplyFilter(filter));
                                // }
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
        let mut model = Self {};

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
