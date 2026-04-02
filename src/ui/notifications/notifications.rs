use super::app_notification::{
    AppNotificationItem, AppNotificationsInit, AppNotificationsPageModel,
    AppNotificationsPageOutput, app_bool_from_canonical,
};
use relm4::adw;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::gtk::gio;
use relm4::prelude::*;
use std::collections::HashSet;

const MASTER_SCHEMA: &str = "org.gnome.desktop.notifications";
const APP_SCHEMA: &str = "org.gnome.desktop.notifications.application";
const APP_PREFIX: &str = "/org/gnome/desktop/notifications/application/";

#[derive(Debug)]
pub struct NotificationsModel {
    pub do_not_disturb: bool,
    pub lock_screen_notifications: bool,
    pub apps: Vec<AppNotificationItem>,
    pub selected_app: Option<usize>,
    pub app_page: Option<Controller<AppNotificationsPageModel>>,
}

#[derive(Debug)]
pub enum NotificationsInput {
    ToggleDoNotDisturb(bool),
    ToggleLockScreen(bool),
    OpenApp(String),
    BackToList,
    AppPageChanged(AppNotificationItem),
}

#[relm4::component(pub)]
impl Component for NotificationsModel {
    type Init = ();
    type Input = NotificationsInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        #[root]
        adw::NavigationPage {
            set_title: "Notifications",

            #[name(nav_view)]
            adw::NavigationView {

                // ── Page 1: List ──
                adw::NavigationPage {
                    set_title: "Notifications",
                    set_tag: Some("list"),

                    adw::ToolbarView {
                        set_top_bar_style: adw::ToolbarStyle::Flat,

                        add_top_bar = &adw::HeaderBar {
                            #[wrap(Some)]
                            set_title_widget = &adw::WindowTitle {
                                set_title: "Notifications",
                            }
                        },

                        adw::PreferencesPage {
                            add = &adw::PreferencesGroup {
                                set_title: "General",

                                adw::SwitchRow {
                                    set_title: "Do Not Disturb",
                                    set_subtitle: "Temporarily suppress notification banners",
                                    #[watch]
                                    set_active: model.do_not_disturb,
                                    connect_active_notify[sender] => move |row| {
                                        sender.input(NotificationsInput::ToggleDoNotDisturb(row.is_active()));
                                    }
                                },

                                adw::SwitchRow {
                                    set_title: "Lock Screen Notifications",
                                    set_subtitle: "Show notifications on the lock screen",
                                    #[watch]
                                    set_active: model.lock_screen_notifications,
                                    connect_active_notify[sender] => move |row| {
                                        sender.input(NotificationsInput::ToggleLockScreen(row.is_active()));
                                    }
                                },
                            },

                            add = &adw::PreferencesGroup {
                                set_title: "App Notifications",
                                set_description: Some("Choose which applications can show notifications"),

                                #[name(app_listbox)]
                                gtk::ListBox {
                                    add_css_class: "boxed-list",
                                    set_selection_mode: gtk::SelectionMode::None,
                                }
                            }
                        }
                    }
                },


                #[name(detail_nav_page)]
                adw::NavigationPage {
                    set_title: "App",
                    set_tag: Some("detail"),

                    adw::ToolbarView {
                        set_top_bar_style: adw::ToolbarStyle::Flat,

                        add_top_bar = &adw::HeaderBar {
                            #[wrap(Some)]
                            set_title_widget = &adw::WindowTitle {
                                #[watch]
                                set_title: if let Some(i) = model.selected_app {
                                    &model.apps[i].title
                                } else {
                                    "App"
                                },
                            }
                        },

                        #[name(detail_box)]
                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_vexpand: true,
                        }
                    }
                },
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let master_settings = gio::Settings::new(MASTER_SCHEMA);
        let apps = load_notification_apps(&master_settings);

        let model = NotificationsModel {
            do_not_disturb: !master_settings.boolean("show-banners"),
            lock_screen_notifications: master_settings.boolean("show-in-lock-screen"),
            apps,
            selected_app: None,
            app_page: None,
        };

        let widgets = view_output!();
        populate_app_list(&widgets.app_listbox, &model.apps, sender.input_sender());
        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        msg: NotificationsInput,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            NotificationsInput::ToggleDoNotDisturb(value) => {
                self.do_not_disturb = value;
                let settings = gio::Settings::new(MASTER_SCHEMA);
                let _ = settings.set_boolean("show-banners", !value);
            }
            NotificationsInput::ToggleLockScreen(value) => {
                self.lock_screen_notifications = value;
                let settings = gio::Settings::new(MASTER_SCHEMA);
                let _ = settings.set_boolean("show-in-lock-screen", value);
            }
            NotificationsInput::OpenApp(app_id) => {
                if let Some(index) = self.apps.iter().position(|a| a.app_id == app_id) {
                    self.selected_app = Some(index);
                    self.mount_app_page(index, widgets, &sender);
                    widgets.nav_view.push_by_tag("detail");
                }
            }
            NotificationsInput::BackToList => {
                self.selected_app = None;
                widgets.nav_view.pop();
            }
            NotificationsInput::AppPageChanged(updated) => {
                if let Some(i) = self
                    .apps
                    .iter()
                    .position(|a| a.canonical_id == updated.canonical_id)
                {
                    self.apps[i] = updated;
                }
                populate_app_list(&widgets.app_listbox, &self.apps, sender.input_sender());
            }
        }
    }
}

impl NotificationsModel {
    fn mount_app_page(
        &mut self,
        index: usize,
        widgets: &mut NotificationsModelWidgets,
        sender: &ComponentSender<Self>,
    ) {
        if let Some(ctrl) = self.app_page.take() {
            widgets.detail_box.remove(ctrl.widget());
        }

        let init = AppNotificationsInit {
            app: self.apps[index].clone(),
            do_not_disturb: self.do_not_disturb,
            lock_screen_notifications: self.lock_screen_notifications,
        };

        let controller = AppNotificationsPageModel::builder().launch(init).forward(
            sender.input_sender(),
            |msg| match msg {
                AppNotificationsPageOutput::Changed(app) => NotificationsInput::AppPageChanged(app),
            },
        );

        widgets.detail_box.append(controller.widget());
        self.app_page = Some(controller);
    }
}

fn populate_app_list(
    listbox: &gtk::ListBox,
    apps: &[AppNotificationItem],
    sender: &relm4::Sender<NotificationsInput>,
) {
    while let Some(child) = listbox.first_child() {
        listbox.remove(&child);
    }
    for app in apps {
        let row = build_app_row(app.clone(), sender.clone());
        listbox.append(&row);
    }
}

fn build_app_row(
    app: AppNotificationItem,
    sender: relm4::Sender<NotificationsInput>,
) -> adw::ActionRow {
    let row = adw::ActionRow::builder()
        .title(&app.title)
        .activatable(true)
        .build();

    let image = if let Some(icon) = &app.icon {
        gtk::Image::from_gicon(icon)
    } else {
        gtk::Image::from_icon_name("application-x-executable-symbolic")
    };
    image.set_pixel_size(20);
    image.add_css_class("lowres-icon");
    row.add_prefix(&image);

    let suffix_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);

    let status_label = gtk::Label::new(Some(if app.enable { "On" } else { "Off" }));
    status_label.set_valign(gtk::Align::Center);
    status_label.add_css_class("dim-label");
    suffix_box.append(&status_label);

    let arrow = gtk::Image::from_icon_name("go-next-symbolic");
    arrow.set_pixel_size(16);
    suffix_box.append(&arrow);
    row.add_suffix(&suffix_box);

    row.connect_activated(move |_| {
        let _ = sender.send(NotificationsInput::OpenApp(app.app_id.clone()));
    });

    row
}

fn load_notification_apps(master_settings: &gio::Settings) -> Vec<AppNotificationItem> {
    let mut items = Vec::new();
    let mut seen = HashSet::<String>::new();

    for canonical_id in master_settings.strv("application-children") {
        maybe_add_app_from_canonical(&canonical_id, &mut items, &mut seen);
    }

    for app_info in gio::AppInfo::all() {
        let Some(app_id) = app_info.id() else {
            continue;
        };
        let Some(desktop) = gio::DesktopAppInfo::new(&app_id) else {
            continue;
        };
        if !desktop.boolean("X-GNOME-UsesNotifications") {
            continue;
        }
        if app_is_system_service(&desktop) {
            continue;
        }
        let canonical_id = canonicalize_app_id(&app_id);
        if seen.contains(&canonical_id) {
            continue;
        }
        let title = app_info.name().to_string();
        let icon = app_info.icon();
        seen.insert(canonical_id.clone());
        items.push(AppNotificationItem {
            app_id: strip_desktop_suffix(&app_id),
            canonical_id: canonical_id.clone(),
            title,
            icon,
            enable: app_bool_from_canonical(&canonical_id, "enable"),
            enable_sound_alerts: app_bool_from_canonical(&canonical_id, "enable-sound-alerts"),
            show_banners: app_bool_from_canonical(&canonical_id, "show-banners"),
            force_expanded: app_bool_from_canonical(&canonical_id, "force-expanded"),
            show_in_lock_screen: app_bool_from_canonical(&canonical_id, "show-in-lock-screen"),
            details_in_lock_screen: app_bool_from_canonical(
                &canonical_id,
                "details-in-lock-screen",
            ),
        });
    }

    items.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
    items
}

fn maybe_add_app_from_canonical(
    canonical_id: &str,
    items: &mut Vec<AppNotificationItem>,
    seen: &mut HashSet<String>,
) {
    if canonical_id.is_empty() || seen.contains(canonical_id) {
        return;
    }

    let path = format!("{APP_PREFIX}{canonical_id}/");
    let settings = gio::Settings::with_path(APP_SCHEMA, &path);
    let full_app_id = settings.string("application-id");

    if full_app_id.is_empty() {
        return;
    }

    let Some(desktop) = gio::DesktopAppInfo::new(full_app_id.as_str()) else {
        return;
    };
    if app_is_system_service(&desktop) {
        return;
    }

    let app_info: gio::AppInfo = desktop.clone().upcast();
    let title = app_info.name().to_string();
    if title.is_empty() {
        return;
    }

    let icon = app_info.icon();
    seen.insert(canonical_id.to_string());
    items.push(AppNotificationItem {
        app_id: strip_desktop_suffix(full_app_id.as_str()),
        canonical_id: canonical_id.to_string(),
        title,
        icon,
        enable: settings.boolean("enable"),
        enable_sound_alerts: settings.boolean("enable-sound-alerts"),
        show_banners: settings.boolean("show-banners"),
        force_expanded: settings.boolean("force-expanded"),
        show_in_lock_screen: settings.boolean("show-in-lock-screen"),
        details_in_lock_screen: settings.boolean("details-in-lock-screen"),
    });
}

fn canonicalize_app_id(app_id: &str) -> String {
    let raw = strip_desktop_suffix(app_id);
    raw.chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect()
}

fn strip_desktop_suffix(app_id: &str) -> String {
    app_id
        .strip_suffix(".desktop")
        .unwrap_or(app_id)
        .to_string()
}

fn app_is_system_service(app: &gio::DesktopAppInfo) -> bool {
    let categories = app.categories().unwrap_or_default();
    if categories.is_empty() {
        return false;
    }
    categories
        .split(';')
        .any(|cat| matches!(cat, "X-GNOME-Settings-Panel" | "Settings" | "System"))
}
