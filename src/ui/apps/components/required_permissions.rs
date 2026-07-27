use dirs::home_dir;
use relm4::{
    ComponentParts, ComponentSender, SimpleComponent, adw,
    adw::prelude::*,
    factory::{DynamicIndex, FactoryComponent, FactorySender, FactoryVecDeque},
};
use serde::{Deserialize, Deserializer};
use serini::from_str;
use std::fs::read_to_string;
use std::path::PathBuf;
use gettextrs::gettext;

#[derive(Debug, Clone)]
pub struct AppPermission {
    pub title: String,
    pub subtitle: String,
}

impl TryFrom<&str> for AppPermission {
    type Error = ();
    fn try_from(val: &str) -> Result<Self, Self::Error> {
        let perm = match val {
            "network" => AppPermission {
                title: gettext("Network"),
                subtitle: gettext("Can communicate over the network"),
            },
            "system-bus" => AppPermission {
                title: gettext("System Services"),
                subtitle: gettext("Full access to system D-Bus services"),
            },
            "session-bus" => AppPermission {
                title: gettext("Session Services"),
                subtitle: gettext("Full access to session D-Bus services"),
            },
            "all" => AppPermission {
                title: gettext("Devices"),
                subtitle: gettext("Can access system device files"),
            },
            "home" | "home:rw" => AppPermission {
                title: gettext("Home Folder"),
                subtitle: gettext("Can view, edit and create files"),
            },
            "home:ro" => AppPermission {
                title: gettext("Home Folder"),
                subtitle: gettext("Can view files"),
            },
            "host" | "host:rw" => AppPermission {
                title: gettext("File System"),
                subtitle: gettext("Can view, edit and create files"),
            },
            "host:ro" => AppPermission {
                title: gettext("File System"),
                subtitle: gettext("Can view files"),
            },
            s if s.starts_with("xdg-download") && s.ends_with(":ro") => AppPermission {
                title: gettext("Downloads Folder"),
                subtitle: gettext("Can view files"),
            },
            s if s.starts_with("xdg-download") => AppPermission {
                title: gettext("Downloads Folder"),
                subtitle: gettext("Can view, edit and create files"),
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

// https://www.carolinemorton.co.uk/blog/rust-serde-data-pipelines/
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
struct PermissionRow {
    permission: AppPermission,
}

#[relm4::factory]
impl FactoryComponent for PermissionRow {
    type Init = AppPermission;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = adw::PreferencesGroup;

    view! {
        adw::ActionRow {
            set_title: &self.permission.title,
            set_subtitle: &self.permission.subtitle,
        }
    }

    fn init_model(
        permission: Self::Init,
        _index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self { permission }
    }
}

#[derive(Debug)]
pub struct RequiredPermissionsDialog {
    app_name: String,
    rows: FactoryVecDeque<PermissionRow>,
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
            set_title: &gettext("Permissions"),
            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {},
                #[wrap(Some)]
                set_content = &adw::PreferencesPage {
                    #[watch]
                    set_description: &gettext(&format!(
                        "<b>{}</b> requires access to the following system resources. To stop this access, the app must be removed",
                        model.app_name
                    )),
                    #[local_ref]
                    perm_group -> adw::PreferencesGroup {
                        #[watch]
                        set_visible: !model.rows.is_empty(),
                    },
                    add = &adw::PreferencesGroup {
                        #[watch]
                        set_visible: model.rows.is_empty(),
                        adw::StatusPage {
                            set_icon_name: Some("security-high-symbolic"),
                            set_title: &gettext("Sandboxed"),
                            set_description: Some(&gettext("This app does not request any extra permissions")),
                            add_css_class: "compact",
                        }
                    },
                },
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            app_name: String::new(),
            rows: FactoryVecDeque::builder()
                .launch(adw::PreferencesGroup::new())
                .detach(),
        };
        let perm_group = model.rows.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            RequiredPermissionsDialogMsg::Show(name, permissions) => {
                self.app_name = name;
                let mut guard = self.rows.guard();
                guard.clear();
                for permission in permissions {
                    guard.push_back(permission);
                }
            }
        }
    }
}

pub fn load_required_permissions(app_id: Option<&str>) -> Vec<AppPermission> {
    app_id
        .and_then(|app_id| {
            let flatpak_id = app_id.strip_suffix(".desktop").unwrap_or(app_id);
            find_metadata_path(flatpak_id)
        })
        .and_then(|metadata_path| read_to_string(&metadata_path).ok())
        .and_then(|raw| from_str::<Metadata>(&raw).ok())
        .and_then(|meta| meta.context)
        .map(|ctx| parse_permissions(&ctx))
        .unwrap_or_default()
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
    ctx.shared
        .iter()
        .chain(&ctx.sockets)
        .chain(&ctx.devices)
        .chain(&ctx.filesystems)
        .filter_map(|v| AppPermission::try_from(v.as_str()).ok())
        .collect()
}
