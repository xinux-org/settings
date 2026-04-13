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

#[relm4::factory(pub)]
impl FactoryComponent for MyColorButton {
    type Init = AccentColorWrapped;
    type Input = ();
    type Output = MyColorButtonOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        #[root]
            #[name = "accent_color" ]
            gtk::ToggleButton{
              set_group: Some(&accent_color),
              // set_overflow: gtk::Overflow::Hidden,
              add_css_class: "style-toggle",

              #[wrap(Some)]
              set_child = &gtk::ColorDialogButton{
                  add_css_class: "accent-color-button",
                  set_rgba: &self.value.0.to_rgba(),
                  set_width_request: 20,
                  set_height_request: 20,
              },

              connect_clicked[sender, index, value = self.value.0.to_owned()] => move |_| {
                  sender.output(MyColorButtonOutput::SendPick(AccentColorWrapped(value.to_owned()))).unwrap();
              },
            },
    }

    fn init_model(value: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self { value }
    }
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

                                #[name = "left" ]
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

                                #[name = "right" ]
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

                            // #[local_ref]
                            // color_button_box -> gtk::Box {
                            //   set_orientation: gtk::Orientation::Horizontal,
                            //   set_spacing: 5,
                            // },
                            #[name = "accent_color" ]
                            gtk::ToggleButton {
                            add_css_class: "style-toggle",

                            #[wrap(Some)]
                            set_child = &gtk::ColorDialogButton{
                                add_css_class: "accent-color-button",
                                set_rgba: &AccentColor::Blue.to_rgba(),
                                set_width_request: 20,
                                set_height_request: 20,
                            },

                            connect_clicked[sender] => move |_| {
                                sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Blue)));
                            },
                          },

                          gtk::ToggleButton {
                            set_group: Some(&accent_color),
                            add_css_class: "style-toggle",

                            #[wrap(Some)]
                            set_child = &gtk::ColorDialogButton{
                                add_css_class: "accent-color-button",
                                set_rgba: &AccentColor::Teal.to_rgba(),
                                set_width_request: 20,
                                set_height_request: 20,
                            },

                            connect_clicked[sender] => move |_| {
                                sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Teal)));
                            },
                          },
                          gtk::ToggleButton {
                            set_group: Some(&accent_color),
                            add_css_class: "style-toggle",

                            #[wrap(Some)]
                            set_child = &gtk::ColorDialogButton{
                                add_css_class: "accent-color-button",
                                set_rgba: &AccentColor::Green.to_rgba(),
                                set_width_request: 20,
                                set_height_request: 20,
                            },

                            connect_clicked[sender] => move |_| {
                                sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Green)));
                            },
                          },
                          gtk::ToggleButton {
                            set_group: Some(&accent_color),
                            add_css_class: "style-toggle",

                            #[wrap(Some)]
                            set_child = &gtk::ColorDialogButton{
                                add_css_class: "accent-color-button",
                                set_rgba: &AccentColor::Yellow.to_rgba(),
                                set_width_request: 20,
                                set_height_request: 20,
                            },

                            connect_clicked[sender] => move |_| {
                                sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Yellow)));
                            },
                          },
                          gtk::ToggleButton {
                            set_group: Some(&accent_color),
                            add_css_class: "style-toggle",

                            #[wrap(Some)]
                            set_child = &gtk::ColorDialogButton{
                                add_css_class: "accent-color-button",
                                set_rgba: &AccentColor::Orange.to_rgba(),
                                set_width_request: 20,
                                set_height_request: 20,
                            },

                            connect_clicked[sender] => move |_| {
                                sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Orange)));
                            },
                          },

                          gtk::ToggleButton {
                            set_group: Some(&accent_color),
                            add_css_class: "style-toggle",

                            #[wrap(Some)]
                            set_child = &gtk::ColorDialogButton{
                                add_css_class: "accent-color-button",
                                set_rgba: &AccentColor::Red.to_rgba(),
                                set_width_request: 20,
                                set_height_request: 20,
                            },

                            connect_clicked[sender] => move |_| {
                                sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Red)));
                            },
                          },

                          gtk::ToggleButton {
                            set_group: Some(&accent_color),
                            add_css_class: "style-toggle",

                            #[wrap(Some)]
                            set_child = &gtk::ColorDialogButton{
                                add_css_class: "accent-color-button",
                                set_rgba: &AccentColor::Pink.to_rgba(),
                                set_width_request: 20,
                                set_height_request: 20,
                            },

                            connect_clicked[sender] => move |_| {
                                sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Pink)));
                            },
                          },  

                          gtk::ToggleButton {
                            set_group: Some(&accent_color),
                            add_css_class: "style-toggle",

                            #[wrap(Some)]
                            set_child = &gtk::ColorDialogButton{
                                add_css_class: "accent-color-button",
                                set_rgba: &AccentColor::Purple.to_rgba(),
                                set_width_request: 20,
                                set_height_request: 20,
                            },

                            connect_clicked[sender] => move |_| {
                                sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Purple)));
                            },
                          },

                          gtk::ToggleButton {
                            set_group: Some(&accent_color),
                            add_css_class: "style-toggle",

                            #[wrap(Some)]
                            set_child = &gtk::ColorDialogButton{
                                add_css_class: "accent-color-button",
                                set_rgba: &AccentColor::Slate.to_rgba(),
                                set_width_request: 20,
                                set_height_request: 20,
                            },

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
