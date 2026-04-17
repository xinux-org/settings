use std::process::Command;

use crate::ui::window::AppMsg;
use crate::utils::parse_dconf;
use relm4::adw::AccentColor;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug, Clone)]
struct AccentColorWrapped(AccentColor);

impl AccentColorWrapped {
    pub fn iterator() -> impl Iterator<Item = AccentColor> {
        use relm4::adw::AccentColor::*;
        [Blue, Teal, Green, Yellow, Orange, Red, Pink, Purple, Slate]
            .iter()
            .copied()
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

#[derive(Debug, Clone, Copy)]
pub enum AppearanceStyle {
    Default,
    Dark,
}

#[derive(Debug)]
pub struct AppearanceModel {
    style: AppearanceStyle,
    // color_buttons: FactoryVecDeque<MyColorButton>,
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

                                    #[wrap(Some)]
                                    set_child = &gtk::Picture{
                                        set_content_fit: gtk::ContentFit::Cover,
                                        set_filename:
                                            Some(parse_dconf("gsettings",&["get", "org.gnome.desktop.background", "picture-uri"]).unwrap_or_default())
                                    },

                                    connect_clicked => AppearanceMsg::SetStyle(AppearanceStyle::Default),
                                },

                                attach[0,1,1,1] = &gtk::Label {
                                    set_label: "Default",
                                    set_halign: gtk::Align::Center,
                                    set_hexpand: true,
                                },

                                #[name = "dark_style" ]
                                // #[wrap(Some)]
                                attach[1,0,1,1] = &gtk::ToggleButton{
                                    add_css_class: "style-toggle",
                                    set_overflow: gtk::Overflow::Hidden,

                                    #[wrap(Some)]
                                    set_child = &gtk::Picture{
                                        set_content_fit: gtk::ContentFit::Fill,
                                        set_filename:
                                            Some(parse_dconf("gsettings",&["get", "org.gnome.desktop.background", "picture-uri"]).unwrap_or_default())
                                    },

                                    connect_clicked => AppearanceMsg::SetStyle(AppearanceStyle::Dark),
                                },

                                attach[1,1,1,1] = &gtk::Label {
                                  set_label: "Dark",
                                  set_halign: gtk::Align::Center,
                                },
                            },
                        }
                    },
                },

                adw::PreferencesGroup {
                    set_title: "Accent Color",

                    adw::PreferencesRow {
                        set_halign: gtk::Align::Center,
                        // set_hexpand: true,

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

                            #[name = "accent_color" ]
                            gtk::ToggleButton {
                              add_css_class: "accent-button",
                              inline_css:"
                                  background-color: var(--accent-blue);
                              ",
                              connect_clicked[sender] => move |_| {
                                  sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Blue)));
                              },
                            },

                            gtk::ToggleButton {
                              set_group: Some(&accent_color),
                              add_css_class: "accent-button",
                              inline_css:"
                                  background-color: var(--accent-teal);
                              ",
                              connect_clicked[sender] => move |_| {
                                  sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Teal)));
                              },
                            },
                            gtk::ToggleButton {
                              set_group: Some(&accent_color),
                              add_css_class: "accent-button",
                              inline_css:"
                                  background-color: var(--accent-green);
                              ",
                              connect_clicked[sender] => move |_| {
                                  sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Green)));
                              },
                            },
                            gtk::ToggleButton {
                              set_group: Some(&accent_color),
                              add_css_class: "accent-button",
                              inline_css:"
                                  background-color: var(--accent-yellow);
                              ",
                              connect_clicked[sender] => move |_| {
                                  sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Yellow)));
                              },
                            },

                            gtk::ToggleButton {
                              set_group: Some(&accent_color),
                              add_css_class: "accent-button",
                              inline_css:"
                                  background-color: var(--accent-orange);
                              ",
                              connect_clicked[sender] => move |_| {
                                  sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Orange)));
                              },
                            },
                            gtk::ToggleButton {
                              set_group: Some(&accent_color),
                              add_css_class: "accent-button",
                              inline_css:"
                                  background-color: var(--accent-red);
                              ",
                              connect_clicked[sender] => move |_| {
                                  sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Red)));
                              },
                            },
                            gtk::ToggleButton {
                              set_group: Some(&accent_color),
                              add_css_class: "accent-button",
                              inline_css:"
                                  background-color: var(--accent-pink);
                              ",
                              connect_clicked[sender] => move |_| {
                                  sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Pink)));
                              },
                            },
                            gtk::ToggleButton {
                              set_group: Some(&accent_color),
                              add_css_class: "accent-button",
                              inline_css:"
                                  background-color: var(--accent-purple);
                              ",
                              connect_clicked[sender] => move |_| {
                                  sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Purple)));
                              },
                            },
                            gtk::ToggleButton {
                              set_group: Some(&accent_color),
                              add_css_class: "accent-button",
                              inline_css:"
                                  background-color: var(--accent-slate);
                              ",
                              connect_clicked[sender] => move |_| {
                                  sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Slate)));
                              },
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
        let style = AppearanceStyle::Default;
        // let color_buttons = FactoryVecDeque::builder()
        //     .launch(gtk::Box::default())
        //     .forward(sender.input_sender(), |output| match output {
        //         MyColorButtonOutput::SendPick(value) => AppearanceMsg::SendPick(value),
        //     });

        let mut model = AppearanceModel {
            style,
            // color_buttons,
        };

        // for color in AccentColorWrapped::iterator() {
        //     model
        //         // .color_buttons
        //         .guard()
        //         .push_back(AccentColorWrapped(color));
        // }

        // let color_button_box = model.color_buttons.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppearanceMsg::SetStyle(style) => {
                self.style = style;

                match style {
                    AppearanceStyle::Dark => {
                        let _ = Command::new("gsettings")
                            .args(&[
                                "set",
                                "org.gnome.desktop.interface",
                                "color-scheme",
                                "prefer-dark",
                            ])
                            .output()
                            .expect("Failed to set appearance style");
                    }

                    AppearanceStyle::Default => {
                        let _ = Command::new("gsettings")
                            .args(&[
                                "set",
                                "org.gnome.desktop.interface",
                                "color-scheme",
                                "prefer-light",
                            ])
                            .output()
                            .expect("Failed to set appearance style");
                    }
                }
            }
            AppearanceMsg::SendPick(color) => {
                let _ = Command::new("gsettings")
                    .args(&[
                        "set",
                        "org.gnome.desktop.interface",
                        "accent-color",
                        &format!("{:?}", color.0).to_lowercase(),
                    ])
                    // .args(&["set", "org.gnome.desktop.interface.accent-color", "blue"])
                    .output()
                    .expect("Failed to set appearance style");

                println!("ACTIVE ACCENT: {}", format!("{:?}", color.0).to_lowercase())
            }
        }
    }
}
