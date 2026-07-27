use relm4::{adw, adw::prelude::*, gtk::gio, prelude::*};
use gettextrs::gettext;

const GLOBAL_SCHEMA: &str = "org.gnome.desktop.notifications";
const APP_SCHEMA: &str = "org.gnome.desktop.notifications.application";
const APP_PREFIX: &str = "/org/gnome/desktop/notifications/application/";

#[derive(Debug, Clone)]
pub struct AppNotificationItem {
    pub app_id: String,
    pub canonical_id: String,
    pub title: String,
    pub icon: Option<gio::Icon>,
    pub enable: bool,
    pub enable_sound_alerts: bool,
    pub show_banners: bool,
    pub force_expanded: bool,
    pub show_in_lock_screen: bool,
    pub details_in_lock_screen: bool,
}

#[derive(Debug, Clone)]
pub struct AppNotificationsInit {
    pub app: AppNotificationItem,
    pub do_not_disturb: bool,
    pub lock_screen_notifications: bool,
}

#[derive(Debug)]
pub struct AppNotificationsPageModel {
    pub app: AppNotificationItem,
    pub do_not_disturb: bool,
    pub lock_screen_notifications: bool,
    settings: gio::Settings,
}

#[derive(Debug)]
pub enum AppNotificationsPageInput {
    SetNotifications(bool),
    SetSoundAlerts(bool),
    SetShowBanners(bool),
    SetForceExpanded(bool),
    SetShowInLockScreen(bool),
    SetDetailsInLockScreen(bool),
    ReloadFromGSettings,
}

#[derive(Debug)]
pub enum AppNotificationsPageOutput {
    Changed(AppNotificationItem),
}

#[relm4::component(pub)]
impl SimpleComponent for AppNotificationsPageModel {
    type Init = AppNotificationsInit;
    type Input = AppNotificationsPageInput;
    type Output = AppNotificationsPageOutput;

    view! {
        #[root]
        adw::PreferencesPage {
            set_vexpand: true,
            add = &adw::PreferencesGroup {
                #[name(notifications_row)]
                adw::SwitchRow {
                    set_title: &gettext("Notifications"),
                    set_subtitle: &gettext("Show in notifications list"),
                    #[watch]
                    set_active: model.app.enable,
                    set_sensitive: true,
                    connect_active_notify[sender] => move |row| {
                        sender.input(AppNotificationsPageInput::SetNotifications(row.is_active()));
                    }
                },
                #[name(sound_alerts_row)]
                adw::SwitchRow {
                    set_title: &gettext("Sound"),
                    set_subtitle: &gettext("Allow notification sounds from app"),
                    #[watch]
                    set_active: model.app.enable_sound_alerts,
                    #[watch]
                    set_sensitive: model.app.enable,
                    connect_active_notify[sender] => move |row| {
                        sender.input(AppNotificationsPageInput::SetSoundAlerts(row.is_active()));
                    }
                },
            },
            add = &adw::PreferencesGroup {
                set_title: &gettext("Banners"),
                #[name(banners_row)]
                adw::SwitchRow {
                    set_title: &gettext("Show Banners"),
                    set_subtitle: &gettext("Show notifications above apps"),
                    #[watch]
                    set_active: model.app.show_banners,
                    #[watch]
                    set_sensitive: model.app.enable && !model.do_not_disturb,
                    connect_active_notify[sender] => move |row| {
                        sender.input(AppNotificationsPageInput::SetShowBanners(row.is_active()));
                    }
                },
                #[name(banner_content_row)]
                adw::SwitchRow {
                    set_title: &gettext("Show Content"),
                    set_subtitle: &gettext("Include message details in notification banners"),
                    #[watch]
                    set_active: model.app.force_expanded,
                    #[watch]
                    set_sensitive: model.app.enable
                        && model.app.show_banners
                        && !model.do_not_disturb,
                    connect_active_notify[sender] => move |row| {
                        sender.input(AppNotificationsPageInput::SetForceExpanded(row.is_active()));
                    }
                },
            },
            add = &adw::PreferencesGroup {
                set_title: &gettext("Lock Screen"),
                #[name(lock_screen_row)]
                adw::SwitchRow {
                    set_title: &gettext("Show Banners"),
                    set_subtitle: &gettext("Show notifications on lock screen"),
                    #[watch]
                    set_active: model.app.show_in_lock_screen,
                    #[watch]
                    set_sensitive: model.app.enable && model.lock_screen_notifications,
                    connect_active_notify[sender] => move |row| {
                        sender.input(AppNotificationsPageInput::SetShowInLockScreen(row.is_active()));
                    }
                },

                #[name(lock_screen_content_row)]
                adw::SwitchRow {
                    set_title: &gettext("Show Content"),
                    set_subtitle: &gettext("Include message details on lock screen"),
                    #[watch]
                    set_active: model.app.details_in_lock_screen,
                    #[watch]
                    set_sensitive: model.app.enable
                        && model.app.show_in_lock_screen
                        && model.lock_screen_notifications,
                    connect_active_notify[sender] => move |row| {
                        sender.input(AppNotificationsPageInput::SetDetailsInLockScreen(row.is_active()));
                    }
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        ensure_app_notification_child(&init.app.canonical_id);

        let settings = app_settings_for_canonical(&init.app.canonical_id);

        let mut app = init.app;
        reload_app_from_settings(&mut app, &settings);

        for key in [
            "enable",
            "enable-sound-alerts",
            "show-banners",
            "force-expanded",
            "show-in-lock-screen",
            "details-in-lock-screen",
        ] {
            let sender = sender.clone();

            settings.connect_changed(Some(key), move |_settings, _key| {
                sender.input(AppNotificationsPageInput::ReloadFromGSettings);
            });
        }

        let model = AppNotificationsPageModel {
            app,
            do_not_disturb: init.do_not_disturb,
            lock_screen_notifications: init.lock_screen_notifications,
            settings,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: AppNotificationsPageInput, sender: ComponentSender<Self>) {
        let mut changed = false;
        match msg {
            AppNotificationsPageInput::SetNotifications(value) => {
                if self.app.enable != value {
                    self.app.enable = value;
                    Self::save_app_bool(&self.settings, "enable", value);
                    changed = true;
                }
            }
            AppNotificationsPageInput::SetSoundAlerts(value) => {
                if self.app.enable && self.app.enable_sound_alerts != value {
                    self.app.enable_sound_alerts = value;
                    Self::save_app_bool(&self.settings, "enable-sound-alerts", value);
                    changed = true;
                }
            }
            AppNotificationsPageInput::SetShowBanners(value) => {
                if self.app.enable && !self.do_not_disturb && self.app.show_banners != value {
                    self.app.show_banners = value;
                    Self::save_app_bool(&self.settings, "show-banners", value);
                    changed = true;
                    if !value && self.app.force_expanded {
                        self.app.force_expanded = false;
                        Self::save_app_bool(&self.settings, "force-expanded", false);
                    }
                }
            }
            AppNotificationsPageInput::SetForceExpanded(value) => {
                if self.app.enable
                    && self.app.show_banners
                    && !self.do_not_disturb
                    && self.app.force_expanded != value
                {
                    self.app.force_expanded = value;
                    Self::save_app_bool(&self.settings, "force-expanded", value);
                    changed = true;
                }
            }
            AppNotificationsPageInput::SetShowInLockScreen(value) => {
                if self.app.enable
                    && self.lock_screen_notifications
                    && self.app.show_in_lock_screen != value
                {
                    self.app.show_in_lock_screen = value;
                    Self::save_app_bool(&self.settings, "show-in-lock-screen", value);
                    changed = true;

                    if !value && self.app.details_in_lock_screen {
                        self.app.details_in_lock_screen = false;
                        Self::save_app_bool(&self.settings, "details-in-lock-screen", false);
                    }
                }
            }
            AppNotificationsPageInput::SetDetailsInLockScreen(value) => {
                if self.app.enable
                    && self.app.show_in_lock_screen
                    && self.lock_screen_notifications
                    && self.app.details_in_lock_screen != value
                {
                    self.app.details_in_lock_screen = value;
                    Self::save_app_bool(&self.settings, "details-in-lock-screen", value);
                    changed = true;
                }
            }
            AppNotificationsPageInput::ReloadFromGSettings => {
                let old_app = self.app.clone();
                reload_app_from_settings(&mut self.app, &self.settings);
                if old_app.enable != self.app.enable
                    || old_app.enable_sound_alerts != self.app.enable_sound_alerts
                    || old_app.show_banners != self.app.show_banners
                    || old_app.force_expanded != self.app.force_expanded
                    || old_app.show_in_lock_screen != self.app.show_in_lock_screen
                    || old_app.details_in_lock_screen != self.app.details_in_lock_screen
                {
                    changed = true;
                }
            }
        }
        if changed {
            let _ = sender.output(AppNotificationsPageOutput::Changed(self.app.clone()));
        }
    }
}

impl AppNotificationsPageModel {
    fn save_app_bool(settings: &gio::Settings, key: &str, value: bool) {
        if settings.boolean(key) != value {
            let _ = settings.set_boolean(key, value);
        }
    }
}

pub fn ensure_app_notification_child(canonical_id: &str) {
    let global_settings = gio::Settings::new(GLOBAL_SCHEMA);
    let mut children: Vec<String> = global_settings
        .strv("application-children")
        .iter()
        .map(|child| child.to_string())
        .collect();
    if children.iter().any(|child| child == canonical_id) {
        return;
    }
    children.push(canonical_id.to_string());
    let children_refs: Vec<&str> = children.iter().map(String::as_str).collect();
    let _ = global_settings.set_strv("application-children", children_refs.as_slice());
}

pub fn app_settings_for_canonical(canonical_id: &str) -> gio::Settings {
    ensure_app_notification_child(canonical_id);

    let path = format!("{APP_PREFIX}{canonical_id}/");

    gio::Settings::with_path(APP_SCHEMA, &path)
}

pub fn app_bool_from_canonical(canonical_id: &str, key: &str) -> bool {
    let settings = app_settings_for_canonical(canonical_id);
    settings.boolean(key)
}

fn reload_app_from_settings(app: &mut AppNotificationItem, settings: &gio::Settings) {
    app.enable = settings.boolean("enable");
    app.enable_sound_alerts = settings.boolean("enable-sound-alerts");
    app.show_banners = settings.boolean("show-banners");
    app.force_expanded = settings.boolean("force-expanded");
    app.show_in_lock_screen = settings.boolean("show-in-lock-screen");
    app.details_in_lock_screen = settings.boolean("details-in-lock-screen");
}
