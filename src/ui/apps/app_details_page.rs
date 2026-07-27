use crate::ui::notifications::app_notification::app_settings_for_canonical;
use gio_unix::DesktopAppInfo;
use relm4::{adw, adw::prelude::*, gtk, gtk::gio, prelude::*};
use crate::ui::apps::{
    components::{
        files_links::{FilesLinksDialog, FilesLinksDialogMsg},
        required_permissions::{
            AppPermission, RequiredPermissionsDialog, RequiredPermissionsDialogMsg,
            load_required_permissions,
        },
        storage::{
            StorageDialog, StorageDialogMsg, calculate_storage, format_bytes
        }
    },
    background::BackgroundPermission
};
use gettextrs::gettext;

#[derive(Debug, Clone)]
pub struct AppEntry {
    pub name: String,
    pub description: Option<String>,
    pub executable: Option<String>,
    pub icon: Option<gio::Icon>,
    pub app_info: gio::AppInfo,
    pub app_id: Option<String>,
    pub canonical_id: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug)]
pub struct AppDetailsPage {
    app: AppEntry,
    detail_nav: adw::NavigationView,
    files_links_dialog: Controller<FilesLinksDialog>,
    storage_dialog: Controller<StorageDialog>,
    required_permissions_dialog: Controller<RequiredPermissionsDialog>,
    required_permissions: Vec<AppPermission>,
    sandboxed: bool,
    mime_count: usize,
    storage_total: u64,
}

#[derive(Debug, Clone)]
pub enum AppDetailsMsg {
    OpenApp,
    ShowDetails(gtk::Button),
    ShowFilesLinks,
    ShowStorage,
    ShowRequiredPermissions,
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
                                set_title: &model.app.name,
                            }
                        },
                        add_top_bar = &adw::Banner {
                            set_title: &gettext("App is not sandboxed"),
                            set_revealed: !model.sandboxed,
                        },
                        #[wrap(Some)]
                        set_content = &adw::PreferencesPage {
                            adw::PreferencesGroup {
                                gtk::Box {
                                    set_orientation: gtk::Orientation::Vertical,
                                    set_spacing: 16,
                                    set_halign: gtk::Align::Center,
                                    set_margin_top: 12,
                                    set_margin_bottom: 12,
                                    gtk::Image {
                                        set_pixel_size: 96,
                                        set_halign: gtk::Align::Center,
                                        set_icon_name: Some("application-x-executable-symbolic"),
                                        set_from_gicon?: model.app.icon.as_ref(),
                                    },
                                    gtk::Label {
                                        set_label: &gettext(&model.app.name),
                                        add_css_class: "title-1",
                                        set_halign: gtk::Align::Center,
                                    },
                                    gtk::Box {
                                        set_orientation: gtk::Orientation::Horizontal,
                                        set_spacing: 12,
                                        set_halign: gtk::Align::Center,
                                        gtk::Button {
                                            set_label: &gettext("Open"),
                                            add_css_class: "suggested-action",
                                            connect_clicked => AppDetailsMsg::OpenApp,
                                        },
                                        gtk::Button {
                                            set_label: &gettext("App Details"),
                                            connect_clicked[sender] => move |button| {
                                                sender.input(AppDetailsMsg::ShowDetails(button.clone()));
                                            }
                                        }
                                    }
                                }
                            },
                            adw::PreferencesGroup {
                                set_title: &gettext("Permissions"),
                                #[name = "notifications_row"]
                                adw::SwitchRow {
                                    set_title: &gettext("Notifications"),
                                    set_visible: false,
                                },
                                #[name = "background_row"]
                                adw::SwitchRow {
                                    set_title: &gettext("Run in Background"),
                                    set_visible: model.sandboxed,
                                }
                            },
                            adw::PreferencesGroup {
                                set_visible: model.sandboxed,
                                    adw::ActionRow {
                                        set_title: &gettext("Required Permissions"),
                                        set_subtitle: &("System permissions that the app requires"),
                                        set_activatable: true,
                                        add_suffix = &gtk::Label {
                                            add_css_class: "dim-label",
                                            #[watch]
                                            set_label: &gettext(&model.permissions_label()),
                                        },
                                        add_suffix = &gtk::Image::from_icon_name("go-next-symbolic") {},
                                        connect_activated => AppDetailsMsg::ShowRequiredPermissions,
                                    },
                            },
                            adw::PreferencesGroup {
                                set_title: &gettext("General"),
                                set_visible: model.sandboxed,
                                    adw::ActionRow {
                                        set_title: &gettext("Files and Links"),
                                        set_subtitle: &gettext("File and link types that are opened by the app"),
                                        set_activatable: true,
                                        set_sensitive: model.mime_count > 0,
                                        add_suffix = &gtk::Label {
                                            add_css_class: "dim-label",
                                            #[watch]
                                            set_label: &gettext(&model.files_links_label()),
                                        },
                                        add_suffix = &gtk::Image::from_icon_name("go-next-symbolic") {},
                                        connect_activated => AppDetailsMsg::ShowFilesLinks,
                                    },
                                    adw::ActionRow {
                                        set_title: &gettext("Storage"),
                                        set_subtitle: "Disk space being used",
                                        set_activatable: true,
                                        add_suffix = &gtk::Label {
                                            add_css_class: "dim-label",
                                            #[watch]
                                            set_label: &gettext(&model.storage_label()),
                                        },
                                        add_suffix = &gtk::Image::from_icon_name("go-next-symbolic") {},
                                        connect_activated => AppDetailsMsg::ShowStorage,
                                    }
                            }
                        }
                    }
                }
            }
        }
    }

    fn init(app: Self::Init, _root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let sandboxed = is_app_sandboxed(&app);
        let mime_count = app.app_info.supported_types().len();
        let storage_total = calculate_storage(app.app_id.as_ref()).total;
        let required_permissions = load_required_permissions(app.app_id.as_deref());

        let mut model = Self {
            app,
            sandboxed,
            mime_count,
            storage_total,
            detail_nav: adw::NavigationView::new(),
            files_links_dialog: FilesLinksDialog::builder().launch(()).detach(),
            storage_dialog: StorageDialog::builder().launch(()).detach(),
            required_permissions_dialog: RequiredPermissionsDialog::builder().launch(()).detach(),
            required_permissions,
        };

        let widgets = view_output!();
        model.detail_nav = widgets.detail_nav.clone();

        setup_notifications_row(&model.app, &widgets.notifications_row);
        if model.sandboxed {
            setup_background_row(&model.app, &widgets.background_row);
        }

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
                self.app.open_in_software_center(&button);
            }
            AppDetailsMsg::ShowFilesLinks => {
                let mime_types = self.app.app_info.supported_types();
                let msg = FilesLinksDialogMsg::Show(self.app.name.clone(), mime_types);
                if self.files_links_dialog.sender().send(msg).is_ok() {
                    self.detail_nav.push(self.files_links_dialog.widget());
                }
            }
            AppDetailsMsg::ShowStorage => {
                let info = calculate_storage(self.app.app_id.as_ref());
                self.storage_total = info.total;
                let msg =
                    StorageDialogMsg::Show(self.app.name.clone(), self.app.app_id.clone(), info);
                if self.storage_dialog.sender().send(msg).is_ok() {
                    self.detail_nav.push(self.storage_dialog.widget());
                }
            }
            AppDetailsMsg::ShowRequiredPermissions => {
                let msg = RequiredPermissionsDialogMsg::Show(
                    self.app.name.clone(),
                    self.required_permissions.clone(),
                );
                if self.required_permissions_dialog.sender().send(msg).is_ok() {
                    self.detail_nav
                        .push(self.required_permissions_dialog.widget());
                }
            }
        }
    }
}

impl AppDetailsPage {
    fn files_links_label(&self) -> String {
        let n = self.mime_count;
        format!("{n} {}", if n == 1 { "type" } else { "types" })
    }

    fn storage_label(&self) -> String {
        if self.storage_total == 0 {
            "—".to_string()
        } else {
            format_bytes(self.storage_total)
        }
    }

    fn permissions_label(&self) -> String {
        let n = self.required_permissions.len();
        format!("{n} {}", if n == 1 { "permission" } else { "permissions" })
    }
}

fn is_app_sandboxed(app: &AppEntry) -> bool {
    app.app_id
        .as_deref()
        .and_then(DesktopAppInfo::new)
        .is_some_and(|desktop| desktop.string("X-Flatpak").is_some())
}

pub fn detect_app_source(app_id: Option<&str>, _executable: Option<&str>) -> Option<String> {
    let desktop = app_id.and_then(DesktopAppInfo::new)?;
    desktop.string("X-Flatpak").map(|_| "Flatpak".to_string())
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

fn setup_background_row(app: &AppEntry, row: &adw::SwitchRow) {
    let Some(app_id) = app.flatpak_id() else {
        return;
    };
    let perm = match BackgroundPermission::new(&app_id) {
        Ok(perm) => perm,
        Err(e) => {
            eprintln!("PermissionStore unavailable: {e}");
            return;
        }
    };
    row.set_subtitle("Allow the app to run in the background");
    row.set_active(perm.is_allowed());
    row.connect_active_notify(move |row| {
        if let Err(e) = perm.set_allowed(row.is_active()) {
            eprintln!("Error writing background permission: {e}");
        }
    });
}

impl AppEntry {
    fn notification_settings(&self) -> Option<gio::Settings> {
        Some(app_settings_for_canonical(self.canonical_id.as_deref()?))
    }
    fn flatpak_id(&self) -> Option<String> {
        let raw = self.app_id.as_deref().or(self.canonical_id.as_deref())?;
        Some(raw.strip_suffix(".desktop").unwrap_or(raw).to_string())
    }
    fn open_in_software_center(&self, button: &gtk::Button) {
        let raw_id = self.app_id.as_deref().or(self.canonical_id.as_deref());
        let Some(raw_id) = raw_id else {
            eprintln!("No app_id available: {}", self.name);
            return;
        };
        let component_id = raw_id.strip_suffix(".desktop").unwrap_or(raw_id);
        let uri = format!("appstream://{component_id}");

        let launcher = gtk::UriLauncher::new(&uri);
        let window = button.root().and_then(|r| r.downcast::<gtk::Window>().ok());

        launcher.launch(window.as_ref(), gio::Cancellable::NONE, move |res| {
            if let Err(e) = res {
                eprintln!("Failed to open software center: {e}");
            }
        });
    }
}
