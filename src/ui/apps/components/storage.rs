use relm4::{
    ComponentParts, ComponentSender, SimpleComponent, adw, adw::prelude::*, gtk, gtk::gio,
};

use dirs::home_dir;
use std::{
    fs::{read_dir, remove_dir_all, remove_file},
    path::{PathBuf, Path},
    io::Result,
};

#[derive(Debug, Clone)]
pub struct AppStorageInfo {
    pub app: u64,
    pub data: u64,
    pub cache: u64,
    pub total: u64,
}

#[derive(Debug)]
pub struct StorageDialog {
    app_name: String,
    app_id: Option<String>,
    info: AppStorageInfo,
    desc_label: gtk::Label,
    pref_group: adw::PreferencesGroup,
    clear_cache_button: gtk::Button,
    dynamic_rows: Vec<gtk::Widget>,
}

#[derive(Debug, Clone)]
pub enum StorageDialogMsg {
    Show(String, Option<String>, AppStorageInfo),
    ClearCacheClicked(gtk::Button),
    ConfirmClearCache,
}

#[relm4::component(pub)]
impl SimpleComponent for StorageDialog {
    type Init = ();
    type Input = StorageDialogMsg;
    type Output = ();

    view! {
        adw::NavigationPage {
            set_title: "Storage",
            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {},
                #[wrap(Some)]
                set_content = &gtk::ScrolledWindow {
                    #[wrap(Some)]
                    set_child = &adw::Clamp {
                        set_maximum_size: 450,
                        set_tightening_threshold: 350,
                        #[wrap(Some)]
                        set_child = &gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_spacing: 24,
                            set_margin_top: 24,
                            set_margin_bottom: 24,
                            set_margin_start: 16,
                            set_margin_end: 16,
                            #[name = "desc_label"]
                            gtk::Label {
                                set_use_markup: true,
                                set_wrap: true,
                                set_justify: gtk::Justification::Center,
                                add_css_class: "dim-label",
                            },
                            #[name = "pref_group"]
                            adw::PreferencesGroup {},
                            #[name = "clear_cache_button"]
                            gtk::Button {
                                set_label: "Clear Cache",
                                add_css_class: "pill",
                                set_halign: gtk::Align::Center,
                                set_sensitive: false,
                                connect_clicked[sender] => move |button| {
                                    sender.input(StorageDialogMsg::ClearCacheClicked(button.clone()));
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
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();

        let model = Self {
            app_name: String::new(),
            app_id: None,
            info: AppStorageInfo {
                app: 0,
                data: 0,
                cache: 0,
                total: 0,
            },
            desc_label: widgets.desc_label.clone(),
            pref_group: widgets.pref_group.clone(),
            clear_cache_button: widgets.clear_cache_button.clone(),
            dynamic_rows: Vec::new(),
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            StorageDialogMsg::Show(name, app_id, info) => {
                self.app_name = name;
                self.app_id = app_id;
                self.info = info;
                self.refresh_rows();
            }
            StorageDialogMsg::ClearCacheClicked(button) => {
                let dialog = gtk::AlertDialog::builder()
                    .modal(true)
                    .message("Clear Cache?")
                    .detail(&format!(
                        "This will delete all cached data for {}.",
                        self.app_name
                    ))
                    .build();

                dialog.set_buttons(&["Cancel", "Clear Cache"]);
                dialog.set_cancel_button(0);
                dialog.set_default_button(0);

                let window = button.root().and_then(|r| r.downcast::<gtk::Window>().ok());
                let sender = sender.clone();

                dialog.choose(window.as_ref(), gio::Cancellable::NONE, move |response| {
                    if let Ok(1) = response {
                        sender.input(StorageDialogMsg::ConfirmClearCache);
                    }
                });
            }
            StorageDialogMsg::ConfirmClearCache => {
                if let Some(id) = self.app_id.as_deref(){
                    if let Err(e) = clear_cache(id) {
                        eprintln!("Cache tozalashda xato: {e}");
                    }
                }
                self.info = calculate_storage(self.app_id.as_ref());
                self.refresh_rows();
            }
        }
    }
}

impl StorageDialog {
    fn refresh_rows(&mut self) {
        let desc = format!(
            "How much disk space <b>{}</b> is occupying with app data and caches",
            self.app_name
        );
        self.desc_label.set_label(&desc);

        for row in self.dynamic_rows.drain(..) {
            self.pref_group.remove(&row);
        }

        let mut add_row = |title: &str, bytes: u64| {
            let row = adw::ActionRow::builder().title(title).build();
            let text = match bytes {
                0 => "0 bytes".to_string(),
                _ => format_bytes(bytes)
            };
            let label = gtk::Label::new(Some(&text));
            row.add_suffix(&label);

            self.pref_group.add(&row);
            self.dynamic_rows.push(row.upcast());
        };

        add_row("App", self.info.app);
        add_row("Data", self.info.data);
        add_row("Cache", self.info.cache);
        add_row("Total", self.info.total);

        self.clear_cache_button.set_sensitive(self.info.cache > 0);
    }
}

pub fn calculate_storage(app_id: Option<&String>) -> AppStorageInfo {
    let Some(app_id) = app_id else {
        return AppStorageInfo {
            app: 0,
            data: 0,
            cache: 0,
            total: 0,
        };
    };

    let flatpak_id = app_id.strip_suffix(".desktop").unwrap_or(app_id);
    let home = home_dir().unwrap_or_default();

    let system_app = PathBuf::from("/var/lib/flatpak/app").join(flatpak_id);
    let user_app = PathBuf::from(&home)
        .join(".local/share/flatpak/app")
        .join(flatpak_id);
    let app_size = dir_size(&system_app).unwrap_or(0) + dir_size(&user_app).unwrap_or(0);

    let data_dir = PathBuf::from(&home)
        .join(".var/app")
        .join(flatpak_id)
        .join("data");
    let config_dir = PathBuf::from(&home)
        .join(".var/app")
        .join(flatpak_id)
        .join("config");
    let data_size = dir_size(&data_dir).unwrap_or(0) + dir_size(&config_dir).unwrap_or(0);

    let cache_dir = PathBuf::from(&home)
        .join(".var/app")
        .join(flatpak_id)
        .join("cache");
    let cache_size = dir_size(&cache_dir).unwrap_or(0);

    let total = app_size + data_size + cache_size;

    AppStorageInfo {
        app: app_size,
        data: data_size,
        cache: cache_size,
        total,
    }
}

pub fn clear_cache(app_id: &str) -> Result<()> {
    // let Some(app_id) = app_id else {
    //     return Ok(());
    // };

    let flatpak_id = app_id.strip_suffix(".desktop").unwrap_or(app_id);
    let home = home_dir().unwrap_or_default();

    let cache_dir = PathBuf::from(&home)
        .join(".var/app")
        .join(flatpak_id)
        .join("cache");

    if !cache_dir.exists() {
        return Ok(());
    }

    for entry in read_dir(&cache_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            remove_dir_all(&path)?;
        } else {
            remove_file(&path)?;
        }
    }

    Ok(())
}

fn dir_size(path: &Path) -> Result<u64> {
    match read_dir(path) {
        Ok(mut entries) => entries.try_fold(0u64, |total, entry| {
            let entry = entry?;
            let meta = entry.metadata()?;
            Ok(total + if meta.is_dir() {
                dir_size(&entry.path())?
            } else {
                meta.len()
            })
        }),
        Err(e) => Err(e),
    }
}

pub fn format_bytes(bytes: u64) -> String {
    if bytes == 0 {
        return "—".to_string();
    }
    gtk::glib::format_size(bytes).to_string()
}
