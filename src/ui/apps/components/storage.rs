use relm4::{
    ComponentParts, ComponentSender, SimpleComponent, adw, adw::prelude::*, gtk, gtk::gio,
};
use dirs::home_dir;
use std::{
    fs::{read_dir, remove_dir_all, remove_file},
    path::{PathBuf, Path},
    io::Result,
};
use gettextrs::gettext;

#[derive(Debug, Clone, Default)]
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
            set_title: &gettext("Storage"),
            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {},
                #[wrap(Some)]
                set_content = &adw::PreferencesPage {
                    #[watch]
                    set_description: &gettext(&format!(
                        "How much disk space <b>{}</b> is occupying with app data and caches",
                        model.app_name
                    )),
                    adw::PreferencesGroup {
                        adw::ActionRow {
                            set_title: &gettext("App"),
                            add_suffix = &gtk::Label {
                                #[watch]
                                set_label: &gettext(&format_bytes(model.info.app)),
                            },
                        },
                        adw::ActionRow {
                            set_title: &gettext("Data"),
                            add_suffix = &gtk::Label {
                                #[watch]
                                set_label: &gettext(&format_bytes(model.info.data)),
                            },
                        },
                        adw::ActionRow {
                            set_title: &gettext("Cache"),
                            add_suffix = &gtk::Label {
                                #[watch]
                                set_label: &gettext(&format_bytes(model.info.cache)),
                            },
                        },
                        adw::ActionRow {
                            set_title: &gettext("Total"),
                            add_suffix = &gtk::Label {
                                #[watch]
                                set_label: &gettext(&format_bytes(model.info.total)),
                            },
                        },
                    },

                    adw::PreferencesGroup {
                        gtk::Button {
                            set_label: &gettext("Clear Cache"),
                            add_css_class: "pill",
                            set_halign: gtk::Align::Center,
                            #[watch]
                            set_sensitive: model.info.cache > 0,
                            connect_clicked[sender] => move |button| {
                                sender.input(StorageDialogMsg::ClearCacheClicked(button.clone()));
                            }
                        }
                    },
                },
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            app_name: String::new(),
            app_id: None,
            info: AppStorageInfo::default(),
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            StorageDialogMsg::Show(name, app_id, info) => {
                self.app_name = name;
                self.app_id = app_id;
                self.info = info;
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

                dialog.set_buttons(&[&gettext("Cancel"), &gettext("Clear Cache")]);
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
                if let Some(id) = self.app_id.as_deref() {
                    if let Err(e) = clear_cache(id) {
                        eprintln!("Cache tozalashda xato: {e}");
                    }
                }
                self.info = calculate_storage(self.app_id.as_ref());
            }
        }
    }
}

pub fn calculate_storage(app_id: Option<&String>) -> AppStorageInfo {
    let Some(app_id) = app_id else {
        return AppStorageInfo::default();
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
        return "0 bytes".to_string();
    }
    gtk::glib::format_size(bytes).to_string()
}
