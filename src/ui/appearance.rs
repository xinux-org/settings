use std::process::Command;

use crate::ui::window::AppMsg;
use crate::utils::parse_dconf;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

pub enum Colors {
    Blue = 0x3d7ccf,
    Magenta = 0x2e889c,
    Green = 0x438c50,
    Yellow = 0xb47805,
    Brown = 0xd05306,
    Red = 0xcb2d3f,
    Pink = 0xc05887,
    Purple = 0x973d96,
    Gray = 0x7b7382
}

#[derive(Debug, Clone, Copy)]
pub enum AppearanceStyle {
    Default,
    Dark,
}

#[derive(Debug, Clone, Copy)]
pub struct AppearanceModel {
    style: AppearanceStyle,
}

#[derive(Debug)]
pub enum AppearanceMsg {
    SetStyle(AppearanceStyle),
}

#[relm4::component(pub)]
impl SimpleComponent for AppearanceModel {
    type Init = ();
    type Input = AppearanceMsg;
    type Output = AppMsg;

    view! {
        #[root]
        adw::ToolbarView {
            set_top_bar_style: adw::ToolbarStyle::Flat,

            add_top_bar = &adw::HeaderBar {
                #[wrap(Some)]
                set_title_widget = &adw::WindowTitle {
                    set_title: "Appearance",
                }
            },
            adw::PreferencesPage {
                adw::PreferencesGroup {
                    set_title: "Style",

                    adw::ActionRow {
                        add_suffix = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 24,
                            set_homogeneous: true,
                            set_hexpand: true,
                            set_margin_top: 18,
                            set_margin_bottom: 18,
                            set_margin_start: 86,
                            set_margin_end: 86,

                            append = &gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_spacing: 18,

                                #[name= "left" ]
                                // #[wrap(Some)]
                                append = &gtk::ToggleButton{
                                    set_group: Some(&right),
                                    set_overflow: gtk::Overflow::Hidden,
                                    add_css_class: "style-toggle",

                                    #[wrap(Some)]
                                    set_child = &gtk::Picture{
                                        set_content_fit: gtk::ContentFit::Fill,
                                        set_filename:
                                            Some(parse_dconf("gsettings",&["get", "org.gnome.desktop.background", "picture-uri"]).unwrap_or_default())
                                    },

                                    connect_clicked => AppearanceMsg::SetStyle(AppearanceStyle::Default),
                                },

                                append = &gtk::Label {
                                   set_label: "Default",
                                   set_halign: gtk::Align::Center,
                                   set_hexpand: true,
                                },
                            },

                            append = &gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_spacing: 18,

                                #[name= "right" ]
                                // #[wrap(Some)]
                                append = &gtk::ToggleButton{
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

                                append = &gtk::Label {
                                  set_label: "Dark",
                                  set_halign: gtk::Align::Center,
                                },
                            },
                        }
                    },
                },

                adw::PreferencesGroup {
                    set_title: "Accent Color",

                    adw::ActionRow {
                        add_suffix = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 24,
                            set_homogeneous: true,
                            set_hexpand: true,
                            set_margin_top: 18,
                            set_margin_bottom: 18,
                            set_margin_start: 86,
                            set_margin_end: 86,

                            append = &gtk::Box {}
                        }
                    },
                },

            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let style = AppearanceStyle::Default;
        let model = AppearanceModel { style };
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

                // self.add
            }
        }
    }
}
