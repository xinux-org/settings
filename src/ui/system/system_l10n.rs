use crate::ui::system::{
    components::{system_language::LanguageModel, system_region::RegionModel},
    system_page::SystemPageMsg,
};
use relm4::{
    adw::{self, prelude::*},
    gtk,
    prelude::*,
};
use std::{convert::identity, process::Command};

#[derive(Debug)]
pub struct SystemRegionLanguagePage {
    default_display_lang: String,
    language_dialog: Controller<LanguageModel>,
    region_dialog: Controller<RegionModel>,
    is_rebuilded: bool,
}

#[derive(Debug)]
pub enum SystemRegionLanguageMsg {
    ShowLanguageDialog,
    ShowRegionDialog,
    // single line nix path, argument and value
    SetDefaultDisplayLang(String),
    Rebuild(String, String, String),
    Close,
    LogOut,
    DoneRebuild,
}

#[relm4::component(pub)]
impl SimpleComponent for SystemRegionLanguagePage {
    type Init = ();
    type Input = SystemRegionLanguageMsg;
    type Output = SystemPageMsg;

    view! {
        adw::NavigationPage {
            set_title: "Region and language",
            adw::ToolbarView {
                set_top_bar_style: adw::ToolbarStyle::Flat,
                add_top_bar = &adw::HeaderBar {},
                add_top_bar = &adw::Banner {
                    set_align: gtk::Align::Fill,
                    set_vexpand: true,
                    set_title: "Language and format will be changed after next login",
                    #[watch]
                    set_revealed: model.is_rebuilded,
                    set_button_label: Some("Log out..."),
                    connect_button_clicked => SystemRegionLanguageMsg::LogOut,
                },
                adw::PreferencesPage {
                    adw::PreferencesGroup {
                        gtk::Box {
                            set_margin_top: 10,
                            set_hexpand: true,
                            gtk::Label {
                                set_halign: gtk::Align::Center,
                                set_label: "Filesystem locations which are selected by system apps, such as Files",
                                add_css_class: "dim-label",
                            },
                        },
                    },
                    adw::PreferencesGroup {
                      set_title: "User",
                        adw::ActionRow {
                            set_title: "Language",
                            set_activatable: true,
                            connect_activated => SystemRegionLanguageMsg::ShowLanguageDialog,
                            add_suffix = &gtk::Label {
                                #[watch]
                                set_label: &model.default_display_lang,
                                add_css_class: "dim-label",

                            },
                        },
                        adw::ActionRow {
                            set_title: "Region",
                            set_activatable: true,
                            connect_activated => SystemRegionLanguageMsg::ShowRegionDialog,
                            add_suffix = &gtk::Label {
                              set_label: "test region",
                              add_css_class: "dim-label",

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
        let language_dialog = LanguageModel::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let region_dialog = RegionModel::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let model = SystemRegionLanguagePage {
            default_display_lang: String::new(),
            language_dialog,
            region_dialog,
            is_rebuilded: false,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            SystemRegionLanguageMsg::ShowLanguageDialog => {
                self.language_dialog
                    .widget()
                    .present(relm4::main_application().active_window().as_ref());
            }
            SystemRegionLanguageMsg::ShowRegionDialog => {
                self.region_dialog
                    .widget()
                    .present(relm4::main_application().active_window().as_ref());
            }
            SystemRegionLanguageMsg::SetDefaultDisplayLang(lang) => {
                self.default_display_lang = lang;
            }
            SystemRegionLanguageMsg::Rebuild(relative_config_path, argument, value) => {
                let _a = sender.output(SystemPageMsg::Rebuild(
                    relative_config_path,
                    argument,
                    value,
                ));
                sender.input(SystemRegionLanguageMsg::Close);
                sender.input(SystemRegionLanguageMsg::DoneRebuild);
            }
            SystemRegionLanguageMsg::Close => {
                self.language_dialog.widget().close();
            }
            SystemRegionLanguageMsg::LogOut => {
                let _a = Command::new("gnome-session-quit")
                    .arg("--logout")
                    .output()
                    .expect("failed to execute process");
            }
            SystemRegionLanguageMsg::DoneRebuild => {
                self.is_rebuilded = true; // show logout banner
            }
        }
    }
}
