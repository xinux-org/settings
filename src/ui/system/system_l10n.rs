use std::convert::identity;
use tracing::{info, trace};

use crate::ui::system::system_page::SystemPageMsg;
use crate::utils::language::get_languages;
use gettextrs::gettext;
use relm4::{
    adw::{self, prelude::*},
    gtk::{self},
    prelude::*,
    view,
};

#[derive(Debug)]
pub struct SystemRegionLanguagePage {
    language_dialog: Controller<LanguageModel>,
}

#[derive(Debug)]
pub enum SystemRegionLanguageMsg {
    ShowLanguageDialog,
    // single line nix path, argument and value
    Rebuild(String, String, String),
    Close,
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


                adw::PreferencesPage {
                    adw::PreferencesGroup {
                        gtk::Box {
                            set_margin_top: 10,
                            set_hexpand: true,

                            gtk::Label {
                                set_halign: gtk::Align::Center,
                                set_label: "Filesystem locations which are selected by system apps, such as Files",
                                add_css_class: "grey_color",
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
                              set_label: "test language",
                              add_css_class: "grey_color",

                            },
                        },
                        adw::ActionRow {
                            set_title: "Region",
                            set_activatable: true,
                            // connect_activated => SearchAppMsg::OpenSearchLocations,

                            add_suffix = &gtk::Label {
                              set_label: "test region",
                              add_css_class: "grey_color",

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
        let dialog = LanguageModel::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let model = SystemRegionLanguagePage {
            language_dialog: dialog,
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
            SystemRegionLanguageMsg::Rebuild(relative_config_path, argument, value) => {
                let _a = sender.output(SystemPageMsg::Rebuild(
                    relative_config_path,
                    argument,
                    value,
                ));
                sender.input(SystemRegionLanguageMsg::Close);
            }
            SystemRegionLanguageMsg::Close => {
                self.language_dialog.widget().close();
            }
        }
    }
}

// ------------------------------------------ Language dialog
#[tracker::track]
#[derive(Debug, Clone)]
pub struct LanguageModel {
    showall: bool,
    selected: Option<String>,
    rebuild_sensitive: bool,
    selectiongroup: gtk::CheckButton,
    expanders: Vec<adw::ExpanderRow>,
}

#[derive(Debug, Clone)]
pub enum LanguageModelMsg {
    ToggleShowall,
    SetSelected(Option<String>),
    CheckSelected,
    Rebuild(String, String, String), // single line nix path, argument and value
}

#[relm4::component(pub)]
impl SimpleComponent for LanguageModel {
    type Init = ();
    type Input = LanguageModelMsg;
    type Output = SystemRegionLanguageMsg;

    view! {
        // todo: replase this with adw::Window to remove x button
        dialog = adw::Dialog {
            set_title: &gettext("Select language"),
            set_content_width: 450,
            set_content_height: 450,
            // set_hexpand: true,
            set_vexpand: true,

            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {
                    pack_start = &gtk::Button {
                        set_label: &gettext("Cancel"),
                        #[watch]
                        set_visible: true,

                        connect_clicked[dialog] => move |_| {
                            dialog.close();
                        }
                    },

                    pack_end = &gtk::Button {
                        set_label: &gettext("Apply"),
                        add_css_class: "suggested-action",
                        #[watch]
                        set_visible: true,
                        #[watch]
                        set_sensitive: model.rebuild_sensitive,

                        connect_clicked[sender] => move |_| {
                            sender.input(LanguageModelMsg::CheckSelected);
                        }
                    },
                },

                #[wrap(Some)]
                set_content = &gtk::ScrolledWindow {
                    set_hexpand: true,
                    set_vexpand: true,
                    adw::Clamp {
                        gtk::Box {
                            set_hexpand: true,
                            set_vexpand: true,
                            set_valign: gtk::Align::Center,
                            set_orientation: gtk::Orientation::Vertical,
                            set_spacing: 20,
                            set_margin_all: 20,

                            #[name(langstack)]
                            if model.showall {
                                #[local_ref]
                                langbox -> gtk::ListBox {
                                    add_css_class: "boxed-list",
                                    set_selection_mode: gtk::SelectionMode::None,
                                    connect_row_activated => move |_, row| {
                                        let checkbutton = row.child().unwrap().downcast::<gtk::Box>().unwrap().last_child().unwrap().downcast::<gtk::CheckButton>().unwrap();
                                        checkbutton.set_active(true);
                                    },
                                }
                            } else {
                                #[local_ref]
                                shortlangbox -> gtk::ListBox {
                                    add_css_class: "boxed-list",
                                    set_selection_mode: gtk::SelectionMode::None,
                                    connect_row_activated => move |_, row| {
                                        let checkbutton = row.child().unwrap().downcast::<gtk::Box>().unwrap().last_child().unwrap().downcast::<gtk::CheckButton>().unwrap();
                                        checkbutton.set_active(true);
                                    },
                                }
                            },
                            gtk::Button {
                                add_css_class: "pill",
                                set_halign: gtk::Align::Center,
                                #[watch]
                                set_label: &if model.showall { gettext("Show less") } else { gettext("Show all") },
                                connect_clicked[sender] => move |_| {
                                    sender.input(LanguageModelMsg::ToggleShowall);
                                    sender.input(LanguageModelMsg::SetSelected(None));
                                }
                            }

                        }
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
        let mut model = LanguageModel {
            showall: false,
            selected: None,
            rebuild_sensitive: false,
            selectiongroup: gtk::CheckButton::new(),
            expanders: vec![],
            tracker: 0,
        };

        // List of 6 popular languages
        let shortlangs = vec!["uz_UZ.UTF-8", "en_US.UTF-8", "ru_RU.UTF-8"];

        let defaultlang = "uz_UZ.UTF-8";
        model.selected = Some(defaultlang.to_string());

        let langbox = gtk::ListBox::new();
        let shortlangbox = gtk::ListBox::new();

        let mut languages = get_languages().into_iter().collect::<Vec<_>>();
        languages.sort_by(|a, b| a.0.cmp(&b.0));
        for (title, languages) in languages {
            for locale in &shortlangs {
                if let Some(title) = languages.get(locale.to_owned()) {
                    view! {
                        row = adw::PreferencesRow {
                            set_title: locale,
                            set_activatable: true,
                            #[wrap(Some)]
                            set_child = &gtk::Box {
                                set_orientation: gtk::Orientation::Horizontal,
                                set_spacing: 6,
                                set_margin_start: 15,
                                set_margin_end: 7,
                                set_margin_top: 15,
                                set_margin_bottom: 15,
                                gtk::Label {
                                    set_label: title,
                                },
                                gtk::Separator {
                                    set_hexpand: true,
                                    set_opacity: 0.0,
                                },
                                #[name(rowbtn)]
                                gtk::CheckButton {
                                    set_halign: gtk::Align::End,
                                    set_group: Some(&model.selectiongroup),
                                    connect_toggled[sender, locale = locale.to_string()] => move |x| {
                                        if x.is_active() {
                                            sender.input(LanguageModelMsg::SetSelected(Some(locale.to_string())))
                                        }
                                    }
                                }
                            }
                        }
                    };
                    shortlangbox.append(&row);
                    rowbtn.set_active(locale == &defaultlang);
                }
            }

            if languages.len() > 1 {
                view! {
                    expander = adw::ExpanderRow {
                        set_title: &title,
                    }
                };
                langbox.append(&expander);

                let mut langvec = languages.into_iter().collect::<Vec<_>>();
                langvec.sort_by(|a, b| a.1.cmp(&b.1));
                for (locale, title) in &langvec {
                    view! {
                        row = adw::PreferencesRow {
                            set_title: locale,
                            set_activatable: true,
                            #[wrap(Some)]
                            set_child = &gtk::Box {
                                set_orientation: gtk::Orientation::Horizontal,
                                set_spacing: 6,
                                set_margin_start: 15,
                                set_margin_end: 7,
                                set_margin_top: 15,
                                set_margin_bottom: 15,
                                gtk::Label {
                                    set_label: title,
                                },
                                gtk::Separator {
                                    set_hexpand: true,
                                    set_opacity: 0.0,
                                },
                                gtk::CheckButton {
                                    set_halign: gtk::Align::End,
                                    set_group: Some(&model.selectiongroup),
                                    connect_toggled[sender, locale] => move |x| {
                                        if x.is_active() {
                                            sender.input(LanguageModelMsg::SetSelected(Some(locale.to_string())))
                                        }
                                    }
                                }
                            }
                        }
                    };
                    expander
                        .first_child()
                        .unwrap()
                        .last_child()
                        .unwrap()
                        .first_child()
                        .unwrap()
                        .downcast::<gtk::ListBox>()
                        .unwrap()
                        .connect_row_activated(move |_, x| {
                            let checkbutton = x
                                .child()
                                .unwrap()
                                .downcast::<gtk::Box>()
                                .unwrap()
                                .last_child()
                                .unwrap()
                                .downcast::<gtk::CheckButton>()
                                .unwrap();
                            checkbutton.set_active(true);
                        });
                    expander.add_row(&row);
                }
                model.expanders.push(expander);
            } else {
                let (locale, title) = languages.into_iter().next().unwrap();
                view! {
                    row = adw::PreferencesRow {
                        set_title: &locale,
                        set_activatable: true,
                        #[wrap(Some)]
                        set_child = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 6,
                            set_margin_start: 15,
                            set_margin_end: 7,
                            set_margin_top: 15,
                            set_margin_bottom: 15,
                            gtk::Label {
                                set_label: &title,
                            },
                            gtk::Separator {
                                set_hexpand: true,
                                set_opacity: 0.0,
                            },
                            gtk::CheckButton {
                                set_halign: gtk::Align::End,
                                set_group: Some(&model.selectiongroup),
                                connect_toggled[sender, locale] => move |x| {
                                    if x.is_active() {
                                        sender.input(LanguageModelMsg::SetSelected(Some(locale.to_string())))
                                    }
                                }
                            }
                        }
                    }
                };
                langbox.append(&row);
            }
        }

        let widgets = view_output!();
        widgets.langstack.set_vhomogeneous(false);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        self.reset();
        match message {
            LanguageModelMsg::ToggleShowall => {
                if !self.showall {
                    for expander in &self.expanders {
                        expander.set_expanded(false);
                    }
                }
                self.set_showall(!self.showall);
            }
            LanguageModelMsg::SetSelected(x) => {
                info!("Selected language: {:?}", x);
                self.selectiongroup.set_active(!x.is_some());
                self.set_rebuild_sensitive(x.is_some());
                self.set_selected(x);
            }
            LanguageModelMsg::CheckSelected => {
                trace!(
                    "LanguageModelMsg::CheckSelected {}",
                    self.selected.is_some()
                );
                if let Some(val) = &self.selected {
                    sender.input(LanguageModelMsg::Rebuild(
                        "modules/nixos/l10n/default.nix".to_string(),
                        "i18n.defaultLocale".to_string(),
                        val.to_string(),
                    ));
                }
            }
            LanguageModelMsg::Rebuild(relative_config_path, argument, value) => {
                let _a = sender.output(SystemRegionLanguageMsg::Rebuild(
                    relative_config_path,
                    argument,
                    value,
                ));
            }
        }
    }
}
