use std::process::Command;

use crate::ui::window::AppMsg;
use crate::utils::parse_dconf;
use relm4::adw::{AccentColor, prelude::*};
use relm4::gtk;
use relm4::prelude::*;

use gtk::gio::Settings;

#[derive(Debug, Clone)]
pub struct AppearanceSettings {
    pub background: Settings,
    pub interface: Settings,
}

impl AppearanceSettings {
    pub fn new() -> Self {
        Self {
            background: Settings::new("org.gnome.desktop.background"),
            interface: Settings::new("org.gnome.desktop.interface"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccentColorWrapped(AccentColor);

impl AccentColorWrapped {
    pub fn iterator() -> impl Iterator<Item = AccentColor> {
        use relm4::adw::AccentColor::*;
        [Blue, Teal, Green, Yellow, Orange, Red, Pink, Purple, Slate]
            .iter()
            .copied()
    }
}

impl From<String> for AccentColorWrapped {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "blue" => AccentColorWrapped(AccentColor::Blue),
            "teal" => AccentColorWrapped(AccentColor::Teal),
            "green" => AccentColorWrapped(AccentColor::Green),
            "yellow" => AccentColorWrapped(AccentColor::Yellow),
            "orange" => AccentColorWrapped(AccentColor::Orange),
            "red" => AccentColorWrapped(AccentColor::Red),
            "pink" => AccentColorWrapped(AccentColor::Pink),
            "purple" => AccentColorWrapped(AccentColor::Purple),
            "slate" => AccentColorWrapped(AccentColor::Slate),
            _ => AccentColorWrapped(AccentColor::Blue),
        }
    }
}

#[derive(Debug)]
struct MyColorButton {
    value: AccentColorWrapped,
}

#[derive(Debug)]
enum MyColorButtonOutput {
    SendPick(AccentColorWrapped),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppearanceStyle {
    Default,
    Dark,
}

#[derive(Debug)]
pub struct AppearanceModel {
    style: AppearanceStyle,
    wallpaper: String,
    accent_color: AccentColorWrapped,
}

#[derive(Debug)]
pub enum AppearanceMsg {
    SetStyle(AppearanceStyle),
    SendPick(AccentColorWrapped),
}

#[relm4::component(pub)]
impl SimpleComponent for AppearanceModel {
    type Init = ();
    type Input = AppearanceMsg;
    type Output = AppMsg;

    view! {
        #[root]
        adw::BreakpointBin {
            set_width_request: 346,
            set_height_request: 200,

            add_breakpoint = adw::Breakpoint::new(
            adw::BreakpointCondition::new_length(
                adw::BreakpointConditionLengthType::MaxWidth,
                420.0,
                adw::LengthUnit::Px,
            )
            ) {
            add_setters: &[
                (
                    &accent_box,
                    "spacing",
                    &6
                ),
                (
                    &accent_box,
                    "margin-top",
                    &6
                ),
                (
                    &accent_box,
                    "margin-bottom",
                    &6
                ),
            ]
            },

            add_breakpoint = adw::Breakpoint::new(
            adw::BreakpointCondition::new_length(
                adw::BreakpointConditionLengthType::MinWidth,
                421.0,
                adw::LengthUnit::Px,
            )
            ) {
            add_setters: &[
                (
                    &accent_box,
                    "spacing",
                    &12
                ),
                (
                    &accent_box,
                    "margin-top",
                    &12
                ),
                (
                    &accent_box,
                    "margin-bottom",
                    &12
                ),
            ]
            },
            #[wrap(Some)]
            set_child = &adw::ToolbarView {
            set_top_bar_style: adw::ToolbarStyle::Flat,

            add_top_bar = &adw::HeaderBar {
                #[wrap(Some)]
                set_title_widget = &adw::WindowTitle {
                    set_title: "Appearance",
                }
            },
            adw::ToastOverlay {
                adw::PreferencesPage {
                    adw::PreferencesGroup {
                        set_title: "Style",

                        adw::PreferencesRow {
                            #[wrap(Some)]
                            set_child = &adw::Clamp {
                                set_maximum_size: 400,
                                set_tightening_threshold: 300,

                                gtk::Grid {
                                    set_focusable: false,
                                    set_orientation: gtk::Orientation::Horizontal,
                                    set_column_spacing: 24,
                                    set_row_spacing: 12,
                                    set_column_homogeneous: true,
                                    set_hexpand: true,
                                    set_margin_top: 18,
                                    set_margin_bottom: 12,
                                    set_margin_start: 12,
                                    set_margin_end: 12,

                                    #[name = "default_style" ]
                                    attach[0,0,1,1] = &gtk::ToggleButton{
                                        set_group: Some(&dark_style),
                                        set_overflow: gtk::Overflow::Hidden,
                                        add_css_class: "style-toggle",
                                        set_active: model.style == AppearanceStyle::Default,

                                        #[wrap(Some)]
                                        set_child = &gtk::Picture{
                                            set_content_fit: gtk::ContentFit::Cover,
                                            set_filename: Some(&model.wallpaper)
                                        },

                                        connect_clicked => AppearanceMsg::SetStyle(AppearanceStyle::Default),
                                    },

                                    attach[0,1,1,1] = &gtk::Label {
                                        set_label: "Default",
                                        set_halign: gtk::Align::Center,
                                        set_hexpand: true,
                                    },

                                    #[name = "dark_style" ]
                                    attach[1,0,1,1] = &gtk::ToggleButton{
                                        add_css_class: "style-toggle",
                                        set_overflow: gtk::Overflow::Hidden,
                                        set_active: model.style == AppearanceStyle::Dark,

                                        #[wrap(Some)]
                                        set_child = &gtk::Picture{
                                            set_content_fit: gtk::ContentFit::Fill,
                                            set_filename: Some(&model.wallpaper)
                                        },

                                        connect_clicked => AppearanceMsg::SetStyle(AppearanceStyle::Dark),
                                    },

                                    attach[1,1,1,1] = &gtk::Label {
                                        set_label: "Dark",
                                        set_halign: gtk::Align::Center,
                                        set_hexpand: true,
                                    },
                                },
                            }
                        },
                    },
                    adw::PreferencesGroup {
                        set_title: "Accent Color",

                        adw::PreferencesRow {
                            set_halign: gtk::Align::Center,
                            set_accessible_role: gtk::AccessibleRole::Group,
                            set_activatable: false,
                            set_focusable: false,

                            #[name = "accent_box"]
                            #[wrap(Some)]
                            set_child = &gtk::Box {
                                set_orientation: gtk::Orientation::Horizontal,
                                set_spacing: 12,
                                set_margin_top: 12,
                                set_margin_bottom: 12,

                                #[name = "accent_color"]
                                gtk::ToggleButton {
                                    add_css_class: "accent-button",
                                    add_css_class: "blue",

                                    connect_clicked[sender] => move |_| {
                                        sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Blue)));
                                    },
                                    set_active: model.accent_color == AccentColorWrapped(AccentColor::Blue)
                                },
                                gtk::ToggleButton {
                                    set_group: Some(&accent_color),
                                    add_css_class: "accent-button",
                                    add_css_class: "teal",

                                    connect_clicked[sender] => move |_| {
                                        sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Teal)));
                                    },
                                    set_active: model.accent_color == AccentColorWrapped(AccentColor::Teal)
                                },
                                gtk::ToggleButton {
                                    set_group: Some(&accent_color),
                                    add_css_class: "accent-button",
                                    add_css_class: "green",

                                    connect_clicked[sender] => move |_| {
                                        sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Green)));
                                    },
                                    set_active: model.accent_color == AccentColorWrapped(AccentColor::Green)
                                },
                                gtk::ToggleButton {
                                    set_group: Some(&accent_color),
                                    add_css_class: "accent-button",
                                    add_css_class: "yellow",

                                    connect_clicked[sender] => move |_| {
                                        sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Yellow)));
                                    },
                                    set_active: model.accent_color == AccentColorWrapped(AccentColor::Yellow)
                                },
                                gtk::ToggleButton {
                                    set_group: Some(&accent_color),
                                    add_css_class: "accent-button",
                                    add_css_class: "orange",

                                    connect_clicked[sender] => move |_| {
                                        sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Orange)));
                                    },
                                    set_active: model.accent_color == AccentColorWrapped(AccentColor::Orange)
                                },
                                gtk::ToggleButton {
                                    set_group: Some(&accent_color),
                                    add_css_class: "accent-button",
                                    add_css_class: "red",

                                    connect_clicked[sender] => move |_| {
                                        sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Red)));
                                    },
                                    set_active: model.accent_color == AccentColorWrapped(AccentColor::Red)
                                },
                                gtk::ToggleButton {
                                    set_group: Some(&accent_color),
                                    add_css_class: "accent-button",
                                    add_css_class: "pink",

                                    connect_clicked[sender] => move |_| {
                                        sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Pink)));
                                    },
                                    set_active: model.accent_color == AccentColorWrapped(AccentColor::Pink)
                                },
                                gtk::ToggleButton {
                                    set_group: Some(&accent_color),
                                    add_css_class: "accent-button",
                                    add_css_class: "purple",

                                    connect_clicked[sender] => move |_| {
                                        sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Purple)));
                                    },
                                    set_active: model.accent_color == AccentColorWrapped(AccentColor::Purple)
                                },
                                gtk::ToggleButton {
                                    set_group: Some(&accent_color),
                                    add_css_class: "accent-button",
                                    add_css_class: "slate",

                                    connect_clicked[sender] => move |_| {
                                        sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Slate)));
                                    },
                                    set_active: model.accent_color == AccentColorWrapped(AccentColor::Slate)
                                },
                            },
                        },
                    },
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
        let settings = AppearanceSettings::new();

        let style = match settings.interface.get::<String>("color-scheme").as_str() {
            "prefer-dark" => AppearanceStyle::Dark,
            _ => AppearanceStyle::Default,
        };

        let wallpaper = parse_dconf(settings.background.get::<String>("picture-uri"));
        let accent_color =
            AccentColorWrapped::from(settings.interface.get::<String>("accent-color"));

        let model = AppearanceModel {
            style,
            wallpaper,
            accent_color,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        let settings = AppearanceSettings::new();

        match msg {
            AppearanceMsg::SetStyle(style) => {
                self.style = style;

                match style {
                    AppearanceStyle::Dark => {
                        settings
                            .interface
                            .set("color-scheme", "prefer-dark")
                            .unwrap();
                    }

                    AppearanceStyle::Default => {
                        settings.interface.set("color-scheme", "default").unwrap();
                    }
                }
            }
            AppearanceMsg::SendPick(color) => {
                settings
                    .interface
                    .set("accent-color", &format!("{:?}", color.0).to_lowercase())
                    .unwrap();
            }
        }
    }
}
