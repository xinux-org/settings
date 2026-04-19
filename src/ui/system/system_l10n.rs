use crate::{ui::system::system_page::SystemPageMsg, utils::language::get_languages};
use gettextrs::gettext;
use gnome_desktop::WallClockExt;
use relm4::{
    adw::{self, prelude::*},
    gtk::{
        self,
        glib::{self, TimeZone},
    },
    prelude::*,
    view,
};
use std::{collections::HashMap, convert::identity, fs, process::Command};
use tracing::{debug, info, trace};

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
                                #[watch]
                                set_label: &model.default_display_lang,
                                add_css_class: "grey_color",

                            },
                        },
                        adw::ActionRow {
                            set_title: "Region",
                            set_activatable: true,
                            connect_activated => SystemRegionLanguageMsg::ShowRegionDialog,

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

// ------------------------------------------ Language dialog
#[tracker::track]
#[derive(Debug, Clone)]
pub struct LanguageModel {
    showall: bool,
    selected: Option<String>,
    default_display_lang: String,
    rebuild_sensitive: bool,
    selectiongroup: gtk::CheckButton,
    expanders: Vec<adw::ExpanderRow>,
}

#[derive(Debug, Clone)]
pub enum LanguageModelMsg {
    ToggleShowall,
    SetSelected(Option<String>, Option<String>),
    CheckSelected,
    Rebuild(String, String, String), // single line nix path, argument and value
}

#[relm4::component(pub)]
impl SimpleComponent for LanguageModel {
    type Init = ();
    type Input = LanguageModelMsg;
    type Output = SystemRegionLanguageMsg;

    view! {
        dialog = adw::Dialog {
            set_content_width: 450,
            set_content_height: 450,
            set_vexpand: true,

            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {
                    set_show_end_title_buttons: false,

                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                    set_title: &gettext("Select language"),
                    },

                    pack_start = &gtk::Button {
                        set_label: &gettext("Cancel"),
                        #[watch]
                        set_visible: true,

                        connect_clicked[dialog] => move |_| {
                            dialog.close();
                            dialog.set_can_close(true);
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
                                    sender.input(LanguageModelMsg::SetSelected(None, None));
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
            default_display_lang: String::new(),
            rebuild_sensitive: false,
            selectiongroup: gtk::CheckButton::new(),
            expanders: vec![],
            tracker: 0,
        };

        let binding = fs::read_to_string("/etc/locale.conf").unwrap_or_default();
        let currentlang = binding
            .lines()
            .find(|line| line.starts_with("LANG="))
            .and_then(|line| line.split_once("="))
            .map(|(_lang, val)| val.trim())
            .filter(|val| !val.is_empty());

        let defaultlang = match currentlang {
            Some(val) => val,
            None => "",
        };

        // List of 6 popular languages
        let mut shortlangs = vec!["uz_UZ.UTF-8", "en_US.UTF-8", "ru_RU.UTF-8"];
        if !shortlangs.contains(&defaultlang) {
            shortlangs.push(defaultlang)
        }

        model.selected = Some(defaultlang.to_string());
        // model.defaultlang = defaultlang.to_string();

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
                                    connect_toggled[sender, locale = locale.to_string(), title = title.to_string()] => move |x| {
                                        if x.is_active() {
                                            sender.input(LanguageModelMsg::SetSelected(Some(title.to_string()), Some(locale.to_string())))
                                        }
                                    }
                                }
                            }
                        }
                    };
                    shortlangbox.append(&row);
                    if locale == &defaultlang {
                        rowbtn.set_active(true);
                        let _ = sender.output(SystemRegionLanguageMsg::SetDefaultDisplayLang(
                            title.to_string(),
                        ));
                    }
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
                                    connect_toggled[sender, locale, title] => move |x| {
                                        if x.is_active() {
                                            sender.input(LanguageModelMsg::SetSelected(Some(title.to_string()), Some(locale.to_string())))
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
                                connect_toggled[sender, locale, title] => move |x| {
                                    if x.is_active() {
                                        sender.input(LanguageModelMsg::SetSelected(Some(title.to_string()), Some(locale.to_string())))
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
            LanguageModelMsg::SetSelected(title, locale) => {
                info!("Selected language: {:?}", locale);
                self.selectiongroup.set_active(locale.is_none());
                self.set_rebuild_sensitive(locale.is_some());
                self.set_selected(locale);
                if let Some(title) = title {
                    self.set_default_display_lang(title);
                }
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
                let _ = sender.output(SystemRegionLanguageMsg::Rebuild(
                    relative_config_path,
                    argument,
                    value,
                ));
                let _ = sender.output(SystemRegionLanguageMsg::SetDefaultDisplayLang(
                    self.default_display_lang.clone(),
                ));
            }
        }
    }
}

// ------------------------------------------ Region dialog
#[tracker::track]
#[derive(Debug, Clone)]
pub struct RegionModel {
    showall: bool,
    selected: Option<String>,
    rebuild_sensitive: bool,
    timezones: Vec<(String, Vec<(String, TimeZone)>)>,
    language: Option<String>,
    country: Option<String>,
    selectiongroup: gtk::CheckButton,
    expanders: Vec<adw::ExpanderRow>,
    time: String,
    timelist: HashMap<TimeZone, gtk::Label>,
}

#[derive(Debug, Clone)]
pub enum RegionModelMsg {
    ToggleShowall,
    SetSelected(Option<String>),
    CheckSelected,
    Rebuild(String, String, String), // single line nix path, argument and value
}

#[relm4::component(pub)]
impl SimpleComponent for RegionModel {
    type Init = ();
    type Input = RegionModelMsg;
    type Output = SystemRegionLanguageMsg;

    view! {
        dialog = adw::Dialog {
            set_content_width: 450,
            set_content_height: 450,
            set_vexpand: true,

            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {
                    set_show_end_title_buttons: false,

                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                    set_title: &gettext("Select region"),
                    },

                    pack_start = &gtk::Button {
                        set_label: &gettext("Cancel"),
                        #[watch]
                        set_visible: true,

                        connect_clicked[dialog] => move |_| {
                            dialog.close();
                            dialog.set_can_close(true);
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
                            sender.input(RegionModelMsg::CheckSelected);
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

                            #[name(tzstack)]
                            if model.showall {
                                #[local_ref]
                                tzbox -> gtk::ListBox {
                                    add_css_class: "boxed-list",
                                    set_selection_mode: gtk::SelectionMode::None,
                                }
                            } else {
                                #[local_ref]
                                shorttzbox -> gtk::ListBox {
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
                                    sender.input(RegionModelMsg::ToggleShowall);
                                    sender.input(RegionModelMsg::SetSelected(None));
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
        let mut locvec: Vec<libgweather::Location> = vec![];
        let x = libgweather::Location::world().unwrap();
        loop {
            let loc = x.next_child(locvec.last().cloned());
            if loc.is_none() {
                break;
            }
            let loc = loc.unwrap();
            locvec.push(loc);
        }

        let mut timezones: HashMap<String, Vec<(String, TimeZone)>> = HashMap::new();

        let countries = libgweather::Location::world().unwrap();

        let y = countries.timezones();

        let shorttzvec_entries = &[
            "Asia/Tashkent",
            "America/New_York",
            "Asia/Tokyo",
            "Europe/Moscow",
        ];

        let mut shorttzvec = vec![];
        let mut selected = gnome_desktop::WallClock::new()
            .timezone()
            .map(|x| x.identifier().to_string());

        if ["UTC", "CET", "Etc/GMT+12"].contains(&selected.as_ref().unwrap().as_str()) {
            selected = Some("America/New_York".to_string());
        }
        debug!("Selected timezone: {:?}", selected);

        for tz in y.get(2..).unwrap() {
            if let (Some(country), Some(region)) = (
                tz.identifier().split('/').next(),
                tz.identifier().split('/').nth(1),
            ) {
                if !timezones.contains_key(country) {
                    timezones.insert(country.to_string(), vec![]);
                }
                timezones
                    .get_mut(country)
                    .unwrap()
                    .push((region.to_string(), tz.clone()));
                if shorttzvec_entries.contains(&tz.identifier().as_str()) {
                    shorttzvec.push((format!("{}/{}", country, region), tz.clone()));
                } else if selected.as_ref() == Some(&tz.identifier().to_string()) {
                    shorttzvec.insert(0, (format!("{}/{}", country, region), tz.clone()));
                }
            }
        }

        shorttzvec.sort_by(|a, b| a.0.cmp(&b.0));

        timezones.iter_mut().for_each(|(_, vec)| {
            vec.sort();
        });

        let mut tzvec = timezones.into_iter().collect::<Vec<_>>();
        tzvec.sort_by(|a, b| a.0.cmp(&b.0));

        let mut model = Self {
            language: Some("en".to_string()),
            country: Some("us".to_string()),
            timezones: tzvec,
            showall: false,
            selected,
            selectiongroup: gtk::CheckButton::new(),
            expanders: vec![],
            time: String::default(),
            timelist: HashMap::new(),
            rebuild_sensitive: false,
            tracker: 0,
        };

        let tzbox = gtk::ListBox::new();
        let shorttzbox = gtk::ListBox::new();

        let widgets = view_output!();

        for (country, zones) in &model.timezones {
            view! {
                expander = adw::ExpanderRow {
                    set_title: country,
                }
            }
            for (zone, tz) in zones {
                let timestr = if let Ok(time) = glib::DateTime::now(tz) {
                    time.format("%H:%M")
                        .unwrap_or_else(|_| glib::GString::from("??"))
                        .to_string()
                } else {
                    "??".to_string()
                };
                view! {
                    row = adw::PreferencesRow {
                        set_title: &zone.replace('_', " "),
                        // set_subtitle: &layout,
                        set_activatable: true,
                        // set_subtitle: &locale
                        #[wrap(Some)]
                        set_child = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 6,
                            set_margin_start: 15,
                            set_margin_end: 7,
                            set_margin_top: 15,
                            set_margin_bottom: 15,
                            gtk::Box {
                                gtk::Label {
                                    set_label: &zone.replace('_', " "),
                                },
                                gtk::Separator {
                                    set_hexpand: true,
                                    set_opacity: 0.0,
                                },
                                #[name(timelabel)]
                                gtk::Label {
                                    set_label: &timestr,
                                },
                            },
                            gtk::CheckButton {
                                set_halign: gtk::Align::End,
                                set_group: Some(&model.selectiongroup),
                                connect_toggled[sender, country, zone] => move |x| {
                                    if x.is_active() {
                                        sender.input(RegionModelMsg::SetSelected(Some(format!("{}/{}", country, zone))))
                                    }
                                }
                            }
                        }
                    }
                }
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
                model.timelist.insert(tz.clone(), timelabel);
            }
            tzbox.append(&expander);
            model.expanders.push(expander);
        }

        for (zone, tz) in shorttzvec.iter().take(8) {
            let timestr = if let Ok(time) = glib::DateTime::now(tz) {
                time.format("%H:%M")
                    .unwrap_or_else(|_| glib::GString::from("??"))
                    .to_string()
            } else {
                "??".to_string()
            };
            view! {
                row = adw::PreferencesRow {
                    set_title: zone,
                    set_activatable: true,
                    #[wrap(Some)]
                    set_child = &gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 6,
                        set_margin_start: 15,
                        set_margin_end: 7,
                        set_margin_top: 15,
                        set_margin_bottom: 15,
                        gtk::Box {
                            gtk::Label {
                                set_label: zone,
                            },
                            gtk::Separator {
                                set_hexpand: true,
                                set_opacity: 0.0,
                            },
                            #[name(timelabel)]
                            gtk::Label {
                                set_label: &timestr,
                            },
                        },
                        #[name(rowbtn)]
                        gtk::CheckButton {
                            set_halign: gtk::Align::End,
                            set_group: Some(&model.selectiongroup),
                            connect_toggled[sender, zone = zone.to_string()] => move |x| {
                                if x.is_active() {
                                    sender.input(RegionModelMsg::SetSelected(Some(zone.to_string())))
                                }
                            }
                        }
                    }
                }
            }
            shorttzbox.append(&row);
            rowbtn.set_active(Some(&zone.to_string()) == model.selected.as_ref());
            model.timelist.insert(tz.clone(), timelabel);
        }

        widgets.tzstack.set_vhomogeneous(false);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        self.reset();
        match message {
            RegionModelMsg::ToggleShowall => {
                if !self.showall {
                    for expander in &self.expanders {
                        expander.set_expanded(false);
                    }
                }
                self.set_showall(!self.showall);
            }
            RegionModelMsg::SetSelected(region) => {
                info!("Selected language: {:?}", region);
                self.selectiongroup.set_active(region.is_none());
                self.set_rebuild_sensitive(region.is_some());
                self.set_selected(region);
            }
            RegionModelMsg::CheckSelected => {
                trace!("RegionModelMsg::CheckSelected {}", self.selected.is_some());
                if let Some(val) = &self.selected {
                    sender.input(RegionModelMsg::Rebuild(
                        "modules/nixos/l10n/default.nix".to_string(),
                        "time.timeZone".to_string(),
                        val.to_string(),
                    ));
                }
            }
            RegionModelMsg::Rebuild(relative_config_path, argument, value) => {
                let _a = sender.output(SystemRegionLanguageMsg::Rebuild(
                    relative_config_path,
                    argument,
                    value,
                ));
            }
        }
    }
}
