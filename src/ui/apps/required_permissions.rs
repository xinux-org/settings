use relm4::{ComponentParts, ComponentSender, SimpleComponent, adw, adw::prelude::*, gtk};

#[derive(Debug, Clone)]
pub struct AppPermission {
    pub icon: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
}

#[derive(Debug)]
pub struct RequiredPermissionsDialog {
    app_name: String,
    desc_label: gtk::Label,
    pref_group: adw::PreferencesGroup,
    dynamic_rows: Vec<gtk::Widget>,
}

#[derive(Debug, Clone)]
pub enum RequiredPermissionsDialogMsg {
    Show(String, Vec<AppPermission>),
}

#[relm4::component(pub)]
impl SimpleComponent for RequiredPermissionsDialog {
    type Init = ();
    type Input = RequiredPermissionsDialogMsg;
    type Output = ();

    view! {
        adw::NavigationPage {
            set_title: "Permissions",

            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {},

                #[wrap(Some)]
                set_content = &gtk::ScrolledWindow {
                    #[wrap(Some)]
                    set_child = &adw::Clamp {
                        set_maximum_size: 420,
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
            desc_label: widgets.desc_label.clone(),
            pref_group: widgets.pref_group.clone(),
            dynamic_rows: Vec::new(),
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            RequiredPermissionsDialogMsg::Show(name, permissions) => {
                self.app_name = name;

                let desc = format!(
                    "System resources that <b>{}</b> is permitted to access outside its sandbox",
                    self.app_name
                );
                self.desc_label.set_label(&desc);

                for row in self.dynamic_rows.drain(..) {
                    self.pref_group.remove(&row);
                }

                if permissions.is_empty() {
                    let status = adw::StatusPage::builder()
                        .icon_name("security-high-symbolic")
                        .title("Sandboxed")
                        .description("This app does not request any extra permissions")
                        .build();
                    status.add_css_class("compact");

                    self.pref_group.add(&status);
                    self.dynamic_rows.push(status.upcast());
                } else {
                    for permission in &permissions {
                        let row = adw::ActionRow::builder()
                            .title(permission.title)
                            .subtitle(permission.subtitle)
                            .build();
                        row.add_prefix(&gtk::Image::from_icon_name(permission.icon));

                        self.pref_group.add(&row);
                        self.dynamic_rows.push(row.upcast());
                    }
                }
            }
        }
    }
}

pub fn load_required_permissions(app_id: Option<&str>) -> Vec<AppPermission> {
    let Some(app_id) = app_id else {
        return Vec::new();
    };
    let flatpak_id = app_id.strip_suffix(".desktop").unwrap_or(app_id);

    let Some(metadata_path) = find_metadata_path(flatpak_id) else {
        return Vec::new();
    };

    // https://docs.gtk.org/glib/struct.KeyFile.html
    let keyfile = gtk::glib::KeyFile::new();
    // https://docs.gtk.org/glib/flags.KeyFileFlags.html
    if keyfile
        .load_from_file(&metadata_path, gtk::glib::KeyFileFlags::NONE)
        .is_err()
    {
        return Vec::new();
    }

    parse_permissions(&keyfile)
}

fn find_metadata_path(flatpak_id: &str) -> Option<std::path::PathBuf> {
    let home = std::env::var("HOME").unwrap_or_default();

    let user_path = std::path::PathBuf::from(&home)
        .join(".local/share/flatpak/app")
        .join(flatpak_id)
        .join("current/active/metadata");

    let system_path = std::path::PathBuf::from("/var/lib/flatpak/app")
        .join(flatpak_id)
        .join("current/active/metadata");

    [user_path, system_path].into_iter().find(|p| p.exists())
}

// https://gitlab.gnome.org/GNOME/gnome-control-center/-/blob/main/panels/applications/cc-applications-panel.c?ref_type=heads#L808
fn parse_permissions(keyfile: &gtk::glib::KeyFile) -> Vec<AppPermission> {
    let mut result = Vec::new();

    let shared = string_list(keyfile, "Context", "shared");
    let sockets = string_list(keyfile, "Context", "sockets");
    let devices = string_list(keyfile, "Context", "devices");
    let filesystems = string_list(keyfile, "Context", "filesystems");

    if shared.iter().any(|s| s == "network") {
        result.push(AppPermission {
            icon: "network-wireless-symbolic",
            title: "Network",
            subtitle: "Can communicate over the network",
        });
    }

    if sockets.iter().any(|s| s == "system-bus") {
        result.push(AppPermission {
            icon: "applications-system-symbolic",
            title: "System Services",
            subtitle: "Full access to system D-Bus services",
        });
    }

    if sockets.iter().any(|s| s == "session-bus") {
        result.push(AppPermission {
            icon: "preferences-desktop-symbolic",
            title: "Session Services",
            subtitle: "Full access to session D-Bus services",
        });
    }

    if devices.iter().any(|d| d == "all") {
        result.push(AppPermission {
            icon: "drive-harddisk-symbolic",
            title: "Devices",
            subtitle: "Can access system device files",
        });
    }

    if filesystems.iter().any(|f| f == "home" || f == "home:rw") {
        result.push(AppPermission {
            icon: "user-home-symbolic",
            title: "Home Folder",
            subtitle: "Can view, edit and create files",
        });
    } else if filesystems.iter().any(|f| f == "home:ro") {
        result.push(AppPermission {
            icon: "user-home-symbolic",
            title: "Home Folder",
            subtitle: "Can view files",
        });
    }

    if filesystems.iter().any(|f| f == "host" || f == "host:rw") {
        result.push(AppPermission {
            icon: "drive-harddisk-symbolic",
            title: "File System",
            subtitle: "Can view, edit and create files",
        });
    } else if filesystems.iter().any(|f| f == "host:ro") {
        result.push(AppPermission {
            icon: "drive-harddisk-symbolic",
            title: "File System",
            subtitle: "Can view files",
        });
    }

    if filesystems
        .iter()
        .any(|f| f.starts_with("xdg-download") && !f.ends_with(":ro"))
    {
        result.push(AppPermission {
            icon: "folder-download-symbolic",
            title: "Downloads Folder",
            subtitle: "Can view, edit and create files",
        });
    } else if filesystems.iter().any(|f| f.starts_with("xdg-download")) {
        result.push(AppPermission {
            icon: "folder-download-symbolic",
            title: "Downloads Folder",
            subtitle: "Can view files",
        });
    }

    let has_x11 = sockets.iter().any(|s| s == "x11" || s == "fallback-x11");
    let has_wayland = sockets.iter().any(|s| s == "wayland");
    if has_x11 && !has_wayland {
        result.push(AppPermission {
            icon: "dialog-warning-symbolic",
            title: "Legacy Display System",
            subtitle: "Uses an old, insecure display system",
        });
    }

    result
}

fn string_list(keyfile: &gtk::glib::KeyFile, group: &str, key: &str) -> Vec<String> {
    keyfile
        .string_list(group, key)
        .map(|list| list.iter().map(|s| s.to_string()).collect())
        .unwrap_or_default()
}