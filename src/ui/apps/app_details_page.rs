use crate::ui::notifications::app_notification::app_settings_for_canonical;
use gio_unix;
use relm4::{adw, adw::prelude::*, gtk, gtk::gio, prelude::*};

#[derive(Debug, Clone)]
pub struct AppEntry {
    pub name: String,
    pub description: Option<String>,
    pub executable: Option<String>,
    pub icon: Option<gio::Icon>,
    pub app_info: gio::AppInfo,
    pub app_id: Option<String>,
    pub canonical_id: Option<String>,
}

pub struct AppStorageInfo {
    app: u64,
    data: u64,
    cache: u64,
    total: u64,
}

#[derive(Debug)]
pub struct AppDetailsPage {
    app: AppEntry,
}

#[derive(Debug, Clone)]
pub enum AppDetailsMsg {
    OpenApp,
    ShowDetails(gtk::Button),
    ShowFilesLinks(adw::ActionRow),
    ShowStorage(adw::ActionRow),
}

#[relm4::component(pub)]
impl SimpleComponent for AppDetailsPage {
    type Init = AppEntry;
    type Input = AppDetailsMsg;
    type Output = ();

    view! {
        adw::NavigationPage {
            set_title: &model.app.name,

            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                set_top_bar_style: adw::ToolbarStyle::Flat,

                add_top_bar = &adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                        set_title: &model.app.name
                    }
                },

                #[name = "sandbox_banner"]
                add_top_bar = &adw::Banner {
                    set_title: "App is not sandboxed",
                    set_revealed: false,
                },

                #[wrap(Some)]
                set_content = &gtk::ScrolledWindow {
                    set_vexpand: true,
                    set_hexpand: true,

                    #[wrap(Some)]
                    set_child = &adw::Clamp {
                        set_maximum_size: 700,
                        set_tightening_threshold: 500,

                        #[wrap(Some)]
                        set_child = &gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_spacing: 20,
                            set_margin_top: 24,
                            set_margin_bottom: 24,
                            set_margin_start: 24,
                            set_margin_end: 24,

                            gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_spacing: 16,
                                set_halign: gtk::Align::Center,
                                set_margin_top: 12,
                                set_margin_bottom: 12,

                                #[name = "app_icon"]
                                gtk::Image {
                                    set_pixel_size: 96,
                                    set_halign: gtk::Align::Center,
                                },

                                gtk::Label {
                                    set_label: &model.app.name,
                                    add_css_class: "title-1",
                                    set_halign: gtk::Align::Center,
                                },

                                gtk::Box {
                                    set_orientation: gtk::Orientation::Horizontal,
                                    set_spacing: 12,
                                    set_halign: gtk::Align::Center,

                                    gtk::Button {
                                        set_label: "Open",
                                        add_css_class: "suggested-action",

                                        connect_clicked => AppDetailsMsg::OpenApp,
                                    },

                                    gtk::Button {
                                        set_label: "App Details",

                                        connect_clicked[sender] => move |button| {
                                            sender.input(AppDetailsMsg::ShowDetails(
                                                button.clone(),
                                            ));
                                        }
                                    }
                                }
                            },

                            adw::PreferencesGroup {
                                set_title: "Permissions",

                                #[name = "notifications_row"]
                                adw::SwitchRow {
                                    set_title: "Notifications",
                                }
                            },

                            adw::PreferencesGroup {
                                set_title: "General",

                                #[name = "files_links_row"]
                                adw::ActionRow {
                                    set_title: "Files & Links",
                                    set_subtitle: "File and link types that are opened by the app",
                                    set_activatable: true,

                                    connect_activated[sender] => move |row| {
                                        sender.input(AppDetailsMsg::ShowFilesLinks(row.clone()));
                                    }
                                },

                                #[name = "storage_row"]
                                adw::ActionRow {
                                    set_title: "Storage",
                                    set_subtitle: "Disk space being used",
                                    set_activatable: true,


                                    connect_activated[sender] => move |row| {
                                        sender.input(AppDetailsMsg::ShowStorage(row.clone()));
                                    }
                                },
                            },
                        }
                    }
                }
            }
        }
    }

    fn init(
        app: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self { app };
        let widgets = view_output!();

        if let Some(icon) = &model.app.icon {
            widgets.app_icon.set_from_gicon(icon);
        } else {
            widgets
                .app_icon
                .set_icon_name(Some("application-x-executable-symbolic"));
        }

        widgets
            .sandbox_banner
            .set_revealed(!is_app_sandboxed(&model.app));

        setup_notifications_row(&model.app, &widgets.notifications_row);
        setup_files_links_row(&model.app, &widgets.files_links_row);
        setup_storage_row(&model.app, &widgets.storage_row);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppDetailsMsg::OpenApp => {
                let ctx = gio::AppLaunchContext::new();

                if let Err(err) = self.app.app_info.launch(&[], Some(&ctx)) {
                    eprintln!("Failed to launch app '{}': {err}", self.app.name);
                }
            }

            AppDetailsMsg::ShowDetails(button) => {
                self.app.show_app_details_dialog(&button);
            }

            AppDetailsMsg::ShowFilesLinks(row) => {
                self.app.show_files_links_dialog(&row);
            }

            AppDetailsMsg::ShowStorage(row) => {
                self.app.show_storage_dialog(&row);
            }
        }
    }
}

// Detect Flatpak apps via X-Flatpak desktop key
// https://gitlab.gnome.org/GNOME/gnome-control-center/-/blob/main/panels/applications/cc-applications-panel.c?ref_type=heads#L279
fn is_app_sandboxed(app: &AppEntry) -> bool {
    let Some(app_id) = &app.app_id else {
        return false;
    };

    let Some(desktop) = gio_unix::DesktopAppInfo::new(app_id) else {
        return false;
    };

    desktop.string("X-Flatpak").is_some()
}

fn setup_notifications_row(app: &AppEntry, row: &adw::SwitchRow) {
    if let Some(settings) = app.notification_settings() {
        row.set_subtitle("Show system notifications");

        row.set_active(settings.boolean("enable"));

        let settings_for_toggle = settings.clone();

        row.connect_active_notify(move |row| {
            let value = row.is_active();

            if settings_for_toggle.boolean("enable") != value {
                if let Err(e) = settings_for_toggle.set_boolean("enable", value) {
                    eprintln!("GSettings 'enable' yozishda xato: {e}");
                }
            }
        });

        let row_weak = row.downgrade();

        settings.connect_changed(Some("enable"), move |settings, _key| {
            if let Some(row) = row_weak.upgrade() {
                let value = settings.boolean("enable");

                if row.is_active() != value {
                    row.set_active(value);
                }
            }
        });
    } else {
        row.set_subtitle("Notification settings are unavailable for this app");
        row.set_active(false);
        row.set_sensitive(false);
    }
}

fn setup_files_links_row(app: &AppEntry, row: &adw::ActionRow) {
    let mime_types = app.app_info.supported_types();
    let count = mime_types.len();

    let label = gtk::Label::new(Some(&format!(
        "{} {}",
        count,
        if count == 1 { "type" } else { "types" }
    )));
    label.add_css_class("dim-label");

    let arrow = gtk::Image::from_icon_name("go-next-symbolic");

    row.add_suffix(&label);
    row.add_suffix(&arrow);

    if count == 0 {
        row.set_sensitive(false);
    }
}

fn setup_storage_row(app: &AppEntry, row: &adw::ActionRow) {
    let info = calculate_storage(app);

    let text = if info.total == 0 {
        "—".to_string()
    } else {
        format_bytes(info.total)
    };

    let label = gtk::Label::new(Some(&text));
    label.add_css_class("dim-label");

    let arrow = gtk::Image::from_icon_name("go-next-symbolic");

    row.add_suffix(&label);
    row.add_suffix(&arrow);
}

fn calculate_storage(app: &AppEntry) -> AppStorageInfo {
    let Some(app_id) = &app.app_id else {
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

fn format_bytes(bytes: u64) -> String {
    if bytes == 0 {
        return "—".to_string();
    }

    gtk::glib::format_size(bytes).to_string()
}

impl AppEntry {
    fn notification_settings(&self) -> Option<gio::Settings> {
        let canonical_id = self.canonical_id.as_deref()?;

        Some(app_settings_for_canonical(canonical_id))
    }

    fn show_app_details_dialog(&self, button: &gtk::Button) {
        let details = format!(
            "Name: {}\nApp ID: {}\nCanonical ID: {}\nDescription: {}\nExecutable: {}\nSupports files: {}\nSupports URIs: {}",
            self.name,
            self.app_id.as_deref().unwrap_or("—"),
            self.canonical_id.as_deref().unwrap_or("—"),
            self.description.as_deref().unwrap_or("—"),
            self.executable.as_deref().unwrap_or("—"),
            if self.app_info.supports_files() {
                "Yes"
            } else {
                "No"
            },
            if self.app_info.supports_uris() {
                "Yes"
            } else {
                "No"
            },
        );

        let dialog = gtk::AlertDialog::builder()
            .modal(true)
            .message(&self.name)
            .detail(&details)
            .build();

        dialog.set_buttons(&["Close"]);
        dialog.set_cancel_button(0);
        dialog.set_default_button(0);

        let window = button.root().and_then(|r| r.downcast::<gtk::Window>().ok());

        dialog.show(window.as_ref());
    }

    fn show_files_links_dialog(&self, row: &adw::ActionRow) {
        let mime_types = self.app_info.supported_types();

        let window = adw::Window::builder()
            .title("Files & Links")
            .modal(true)
            .hide_on_close(true)
            .default_width(450)
            .default_height(450)
            .build();

        if let Some(parent) = row.root().and_then(|r| r.downcast::<gtk::Window>().ok()) {
            window.set_transient_for(Some(&parent));
        }

        let toolbar_view = adw::ToolbarView::new();
        window.set_content(Some(&toolbar_view));

        let header_bar = adw::HeaderBar::new();
        toolbar_view.add_top_bar(&header_bar);

        let clamp = adw::Clamp::builder()
            .maximum_size(450)
            .tightening_threshold(350)
            .build();

        let box_layout = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(24)
            .margin_top(24)
            .margin_bottom(24)
            .margin_start(16)
            .margin_end(16)
            .build();

        let description_label = gtk::Label::builder()
            .label(format!(
                "File and link types that are opened by <b>{}</b>",
                self.name
            ))
            .use_markup(true)
            .wrap(true)
            .justify(gtk::Justification::Center)
            .build();
        description_label.add_css_class("dim-label");
        box_layout.append(&description_label);

        if mime_types.is_empty() {
            let status = adw::StatusPage::builder()
                .icon_name("text-x-generic-symbolic")
                .title("No File Types")
                .description("This app has not registered any file or link types")
                .build();
            status.add_css_class("compact");
            box_layout.append(&status);
        } else {
            let pref_group = adw::PreferencesGroup::new();
            pref_group.set_title(&format!(
                "{} {}",
                mime_types.len(),
                if mime_types.len() == 1 {
                    "type"
                } else {
                    "types"
                }
            ));

            for mime in mime_types.iter() {
                let mime_str = mime.to_string();
                let description = gio::content_type_get_description(&mime_str);

                let action_row = adw::ActionRow::builder()
                    .title(gtk::glib::markup_escape_text(&description).as_str())
                    .subtitle(gtk::glib::markup_escape_text(&mime_str).as_str())
                    .build();

                let icon = gio::content_type_get_icon(&mime_str);
                let image = gtk::Image::from_gicon(&icon);
                action_row.add_prefix(&image);

                pref_group.add(&action_row);
            }

            box_layout.append(&pref_group);
        }

        clamp.set_child(Some(&box_layout));

        let scroll = gtk::ScrolledWindow::new();
        scroll.set_child(Some(&clamp));
        toolbar_view.set_content(Some(&scroll));

        window.present();
    }

    fn show_storage_dialog(&self, row: &adw::ActionRow) {
        let info = calculate_storage(self);

        let window = adw::Window::builder()
            .title("Storage")
            .modal(true)
            .hide_on_close(true)
            .default_width(450)
            .default_height(450)
            .build();

        if let Some(parent) = row.root().and_then(|r| r.downcast::<gtk::Window>().ok()) {
            window.set_transient_for(Some(&parent));
        }

        let toolbar_view = adw::ToolbarView::new();
        window.set_content(Some(&toolbar_view));

        let header_bar = adw::HeaderBar::new();
        toolbar_view.add_top_bar(&header_bar);

        let clamp = adw::Clamp::builder()
            .maximum_size(450)
            .tightening_threshold(350)
            .build();

        let box_layout = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(24)
            .margin_top(24)
            .margin_bottom(24)
            .margin_start(16)
            .margin_end(16)
            .build();

        let description_label = gtk::Label::builder()
            .label(format!(
                "How much disk space <b>{}</b> is occupying with app data and caches",
                self.name
            ))
            .use_markup(true)
            .wrap(true)
            .justify(gtk::Justification::Center)
            .build();
        description_label.add_css_class("dim-label");

        let pref_group = adw::PreferencesGroup::new();

        let create_row = |title: &str, bytes: u64| -> adw::ActionRow {
            let row = adw::ActionRow::builder().title(title).build();

            let text = if bytes == 0 {
                "0 bytes".to_string()
            } else {
                format_bytes(bytes)
            };

            let label = gtk::Label::new(Some(&text));
            row.add_suffix(&label);
            row
        };

        pref_group.add(&create_row("App", info.app));
        pref_group.add(&create_row("Data", info.data));
        pref_group.add(&create_row("Cache", info.cache));
        pref_group.add(&create_row("Total", info.total));

        box_layout.append(&description_label);
        box_layout.append(&pref_group);
        clamp.set_child(Some(&box_layout));

        let scroll = gtk::ScrolledWindow::new();
        scroll.set_child(Some(&clamp));
        toolbar_view.set_content(Some(&scroll));

        window.present();
    }
}
