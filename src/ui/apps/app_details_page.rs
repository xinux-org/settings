use crate::ui::notifications::app_notification::app_settings_for_canonical;
use gio_unix;
use relm4::{adw, adw::prelude::*, gtk, gtk::gio, prelude::*};

use crate::ui::apps::files_links::{FilesLinksDialog, FilesLinksDialogMsg};
use crate::ui::apps::required_permissions::{
    AppPermission, RequiredPermissionsDialog, RequiredPermissionsDialogMsg,
    load_required_permissions,
};
use crate::ui::apps::storage::{StorageDialog, StorageDialogMsg, calculate_storage, format_bytes};

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

#[derive(Debug)]
pub struct AppDetailsPage {
    app: AppEntry,
    detail_nav: adw::NavigationView,
    files_links_dialog: Controller<FilesLinksDialog>,
    storage_dialog: Controller<StorageDialog>,
    required_permissions_dialog: Controller<RequiredPermissionsDialog>,
    required_permissions: Vec<AppPermission>,
}

#[derive(Debug, Clone)]
pub enum AppDetailsMsg {
    OpenApp,
    ShowDetails(gtk::Button),
    ShowFilesLinks(adw::ActionRow),
    ShowStorage(adw::ActionRow),
    ShowRequiredPermissions(adw::ActionRow),
}

#[relm4::component(pub)]
impl SimpleComponent for AppDetailsPage {
    type Init = AppEntry;
    type Input = AppDetailsMsg;
    type Output = ();

    view! {
        adw::NavigationPage {
            set_title: &model.app.name,

            #[name = "detail_nav"]
            #[wrap(Some)]
            set_child = &adw::NavigationView {
                add = &adw::NavigationPage {
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
                                                    sender.input(AppDetailsMsg::ShowDetails(button.clone()));
                                                }
                                            }
                                        }
                                    },


                                    adw::PreferencesGroup {
                                        set_title: "Permissions",

                                        #[name = "notifications_row"]
                                        adw::SwitchRow {
                                            set_title: "Notifications",
                                            set_visible: false,
                                        },
                                    },


                                    #[name = "required_permissions_group"]
                                    adw::PreferencesGroup {
                                        #[name = "required_permissions_row"]
                                        adw::ActionRow {
                                            set_title: "Required Permissions",
                                            set_subtitle: "System permissions that the app requires",
                                            set_activatable: true,

                                            #[name = "required_permissions_count"]
                                            add_suffix = &gtk::Label {
                                                add_css_class: "dim-label",
                                            },
                                            add_suffix = &gtk::Image::from_icon_name("go-next-symbolic") {},

                                            connect_activated[sender] => move |row| {
                                                sender.input(AppDetailsMsg::ShowRequiredPermissions(row.clone()));
                                            }
                                        },
                                    },


                                    #[name = "general_group"]
                                    adw::PreferencesGroup {
                                        set_title: "General",

                                        #[name = "files_links_row"]
                                        adw::ActionRow {
                                            set_title: "Files and Links",
                                            set_subtitle: "File and link types that are opened by the app",
                                            set_activatable: true,

                                            #[name = "files_links_count"]
                                            add_suffix = &gtk::Label {
                                                add_css_class: "dim-label",
                                            },
                                            add_suffix = &gtk::Image::from_icon_name("go-next-symbolic") {},

                                            connect_activated[sender] => move |row| {
                                                sender.input(AppDetailsMsg::ShowFilesLinks(row.clone()));
                                            }
                                        },

                                        #[name = "storage_row"]
                                        adw::ActionRow {
                                            set_title: "Storage",
                                            set_subtitle: "Disk space being used",
                                            set_activatable: true,

                                            #[name = "storage_size"]
                                            add_suffix = &gtk::Label {
                                                add_css_class: "dim-label",
                                            },
                                            add_suffix = &gtk::Image::from_icon_name("go-next-symbolic") {},

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
        }
    }

    fn init(
        app: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let files_links_dialog = FilesLinksDialog::builder().launch(()).detach();
        let storage_dialog = StorageDialog::builder().launch(()).detach();
        let required_permissions_dialog = RequiredPermissionsDialog::builder().launch(()).detach();

        let required_permissions = load_required_permissions(app.app_id.as_deref());

        let mut model = Self {
            app,
            detail_nav: adw::NavigationView::new(),
            files_links_dialog,
            storage_dialog,
            required_permissions_dialog,
            required_permissions,
        };
        let widgets = view_output!();

        model.detail_nav = widgets.detail_nav.clone();

        if let Some(icon) = &model.app.icon {
            widgets.app_icon.set_from_gicon(icon);
        } else {
            widgets
                .app_icon
                .set_icon_name(Some("application-x-executable-symbolic"));
        }

        let sandboxed = is_app_sandboxed(&model.app);

        widgets.sandbox_banner.set_revealed(!sandboxed);

        widgets.general_group.set_visible(sandboxed);
        widgets.required_permissions_group.set_visible(sandboxed);

        if !sandboxed {
            setup_notifications_row(&model.app, &widgets.notifications_row);
        }

        let mime_types = model.app.app_info.supported_types();
        let count = mime_types.len();
        widgets.files_links_count.set_label(&format!(
            "{} {}",
            count,
            if count == 1 { "type" } else { "types" }
        ));
        if count == 0 {
            widgets.files_links_row.set_sensitive(false);
        }

        let storage_info = calculate_storage(model.app.app_id.as_ref());
        let storage_text = if storage_info.total == 0 {
            "—".to_string()
        } else {
            format_bytes(storage_info.total)
        };
        widgets.storage_size.set_label(&storage_text);

        let perm_count = model.required_permissions.len();
        widgets.required_permissions_count.set_label(&format!(
            "{} {}",
            perm_count,
            if perm_count == 1 {
                "permission"
            } else {
                "permissions"
            }
        ));

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

            AppDetailsMsg::ShowFilesLinks(_row) => {
                let mime_types = self.app.app_info.supported_types();

                self.files_links_dialog
                    .sender()
                    .send(FilesLinksDialogMsg::Show(self.app.name.clone(), mime_types))
                    .expect("Failed to update files links dialog");

                self.detail_nav.push(self.files_links_dialog.widget());
            }

            AppDetailsMsg::ShowStorage(_row) => {
                let info = calculate_storage(self.app.app_id.as_ref());

                self.storage_dialog
                    .sender()
                    .send(StorageDialogMsg::Show(self.app.name.clone(), info))
                    .expect("Failed to update storage dialog");

                self.detail_nav.push(self.storage_dialog.widget());
            }

            AppDetailsMsg::ShowRequiredPermissions(_row) => {
                self.required_permissions_dialog
                    .sender()
                    .send(RequiredPermissionsDialogMsg::Show(
                        self.app.name.clone(),
                        self.required_permissions.clone(),
                    ))
                    .expect("Failed to update required permissions dialog");

                self.detail_nav
                    .push(self.required_permissions_dialog.widget());
            }
        }
    }
}

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
        row.set_visible(true);

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
        row.set_visible(true);
    }
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
}
