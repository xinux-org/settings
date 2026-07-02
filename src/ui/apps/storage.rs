use relm4::{ComponentParts, ComponentSender, SimpleComponent, adw, adw::prelude::*, gtk};

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
    info: AppStorageInfo,
    desc_label: gtk::Label,
    pref_group: adw::PreferencesGroup,
    dynamic_rows: Vec<gtk::Widget>,
}

#[derive(Debug, Clone)]
pub enum StorageDialogMsg {
    Show(String, AppStorageInfo),
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
                            adw::PreferencesGroup {}
                        }
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();

        let model = Self {
            app_name: String::new(),
            info: AppStorageInfo {
                app: 0,
                data: 0,
                cache: 0,
                total: 0,
            },
            desc_label: widgets.desc_label.clone(),
            pref_group: widgets.pref_group.clone(),
            dynamic_rows: Vec::new(),
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            StorageDialogMsg::Show(name, info) => {
                self.app_name = name;
                self.info = info;

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
                    let text = if bytes == 0 {
                        "0 bytes".to_string()
                    } else {
                        format_bytes(bytes)
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
            }
        }
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
    let home = std::env::var("HOME").unwrap_or_default();

    let system_app = std::path::PathBuf::from("/var/lib/flatpak/app").join(flatpak_id);
    let user_app = std::path::PathBuf::from(&home)
        .join(".local/share/flatpak/app")
        .join(flatpak_id);
    let app_size = dir_size(&system_app).unwrap_or(0) + dir_size(&user_app).unwrap_or(0);

    let data_dir = std::path::PathBuf::from(&home)
        .join(".var/app")
        .join(flatpak_id)
        .join("data");
    let config_dir = std::path::PathBuf::from(&home)
        .join(".var/app")
        .join(flatpak_id)
        .join("config");
    let data_size = dir_size(&data_dir).unwrap_or(0) + dir_size(&config_dir).unwrap_or(0);

    let cache_dir = std::path::PathBuf::from(&home)
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

fn dir_size(path: &std::path::Path) -> std::io::Result<u64> {
    let mut total = 0u64;
    if !path.exists() {
        return Ok(0);
    }
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let meta = entry.metadata()?;

        if meta.is_dir() {
            total += dir_size(&entry.path()).unwrap_or(0);
        } else {
            total += meta.len();
        }
    }
    Ok(total)
}

pub fn format_bytes(bytes: u64) -> String {
    if bytes == 0 {
        return "—".to_string();
    }
    gtk::glib::format_size(bytes).to_string()
}