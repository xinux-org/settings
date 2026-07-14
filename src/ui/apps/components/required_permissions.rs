use relm4::{ComponentParts, ComponentSender, SimpleComponent, adw, adw::prelude::*, gtk};
use dirs::home_dir;
use serde::{Deserialize, Deserializer};
use serini::from_str;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppPermission {
    pub title: &'static str,
    pub subtitle: &'static str,
}

impl TryFrom<&str> for AppPermission {
    type Error = ();

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        let perm = match val {
            "network" => AppPermission {
                title: "Network",
                subtitle: "Can communicate over the network",
            },
            "system-bus" => AppPermission {
                title: "System Services",
                subtitle: "Full access to system D-Bus services",
            },
            "session-bus" => AppPermission {
                title: "Session Services",
                subtitle: "Full access to session D-Bus services",
            },
            "all" => AppPermission {
                title: "Devices",
                subtitle: "Can access system device files",
            },
            "home" | "home:rw" => AppPermission {
                title: "Home Folder",
                subtitle: "Can view, edit and create files",
            },
            "home:ro" => AppPermission {
                title: "Home Folder",
                subtitle: "Can view files",
            },
            "host" | "host:rw" => AppPermission {
                title: "File System",
                subtitle: "Can view, edit and create files",
            },
            "host:ro" => AppPermission {
                title: "File System",
                subtitle: "Can view files",
            },
            s if s.starts_with("xdg-download") && s.ends_with(":ro") => AppPermission {
                title: "Downloads Folder",
                subtitle: "Can view files",
            },
            s if s.starts_with("xdg-download") => AppPermission {
                title: "Downloads Folder",
                subtitle: "Can view, edit and create files",
            },
            _ => return Err(()),
        };
        Ok(perm)
    }
}

#[derive(Debug, Default, Deserialize)]
struct Metadata {
    #[serde(rename = "Context")]
    context: Option<Context>,
}

#[derive(Debug, Default, Deserialize)]
struct Context {
    #[serde(default, deserialize_with = "split")]
    shared: Vec<String>,
    #[serde(default, deserialize_with = "split")]
    sockets: Vec<String>,
    #[serde(default, deserialize_with = "split")]
    devices: Vec<String>,
    #[serde(default, deserialize_with = "split")]
    filesystems: Vec<String>,
}

fn split<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;
    Ok(raw
        .split(';')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect())
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
                    "<b>{}</b> requires access to the following system resources. To stop this access, the app must be romoved",
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

    let Ok(raw) = read_to_string(&metadata_path) else {
        return Vec::new();
    };

    let Ok(meta) = from_str::<Metadata>(&raw) else {
        return Vec::new();
    };

    let Some(ctx) = meta.context else {
        return Vec::new();
    };

    parse_permissions(&ctx)
}

fn find_metadata_path(flatpak_id: &str) -> Option<PathBuf> {
    let home = home_dir().unwrap_or_default();

    let user_path = home
        .join(".local/share/flatpak/app")
        .join(flatpak_id)
        .join("current/active/metadata");

    let system_path = PathBuf::from("/var/lib/flatpak/app")
        .join(flatpak_id)
        .join("current/active/metadata");

    [user_path, system_path].into_iter().find(|p| p.exists())
}

// https://gitlab.gnome.org/GNOME/gnome-control-center/-/blob/main/panels/applications/cc-applications-panel.c?ref_type=heads#L808
fn parse_permissions(ctx: &Context) -> Vec<AppPermission> {
    // let shared = split_list(&ctx.shared);
    // let sockets = split_list(&ctx.sockets);
    // let devices = split_list(&ctx.devices);
    // let filesystems = split_list(&ctx.filesystems);

    let result: Vec<AppPermission> = ctx
        .shared
        .iter()
        .chain(&ctx.sockets)
        .chain(&ctx.devices)
        .chain(&ctx.filesystems)
        .filter_map(|v| AppPermission::try_from(v.as_str()).ok())
        .collect();

    // let has_x11 = sockets.iter().any(|s| s == "x11" || s == "fallback-x11");
    // let has_wayland = sockets.iter().any(|s| s == "wayland");
    // if has_x11 && !has_wayland {
    //     result.push(AppPermission {
    //         title: "Legacy Display System",
    //         subtitle: "Uses an old, insecure display system",
    //     });
    // }

    result
}

// fn split_list(value: &Option<String>) -> Vec<String> {
//     value
//         .as_deref()
//         .unwrap_or("")
//         .split(';')
//         .map(str::trim)
//         .filter(|s| !s.is_empty())
//         .map(String::from)
//         .collect()
// }
