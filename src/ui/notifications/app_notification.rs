use relm4::adw;
use relm4::adw::prelude::*;
use relm4::gtk::gio;
use relm4::prelude::*;

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
}

#[derive(Debug)]
pub enum AppNotificationsPageInput {
    SetNotifications(bool),
    SetSoundAlerts(bool),
    SetShowBanners(bool),
    SetForceExpanded(bool),
    SetShowInLockScreen(bool),
    SetDetailsInLockScreen(bool),
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
                    set_title: "Notifications",
                    set_subtitle: "Show in notifications list",
                    #[watch]
                    set_active: model.app.enable,
                    set_sensitive: true,
                    connect_active_notify[sender] => move |row| {
                        sender.input(AppNotificationsPageInput::SetNotifications(row.is_active()));
                    }
                },

                #[name(sound_alerts_row)]
                adw::SwitchRow {
                    set_title: "Sound",
                    set_subtitle: "Allow notification sounds from app",
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
                set_title: "Banners",

                #[name(banners_row)]
                adw::SwitchRow {
                    set_title: "Show Banners",
                    set_subtitle: "Show notifications above apps",
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
                    set_title: "Show Content",
                    set_subtitle: "Include message details in notification banners",
                    #[watch]
                    set_active: model.app.force_expanded,
                    #[watch]
                    set_sensitive: model.app.enable && model.app.show_banners && !model.do_not_disturb,
                    connect_active_notify[sender] => move |row| {
                        sender.input(AppNotificationsPageInput::SetForceExpanded(row.is_active()));
                    }
                },
            },

            add = &adw::PreferencesGroup {
                set_title: "Lock Screen",

                #[name(lock_screen_row)]
                adw::SwitchRow {
                    set_title: "Show Banners",
                    set_subtitle: "Show notifications on lock screen",
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
                    set_title: "Show Content",
                    set_subtitle: "Include message details on lock screen",
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
        let model = AppNotificationsPageModel {
            app: init.app,
            do_not_disturb: init.do_not_disturb,
            lock_screen_notifications: init.lock_screen_notifications,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: AppNotificationsPageInput, sender: ComponentSender<Self>) {
        match msg {
            AppNotificationsPageInput::SetNotifications(value) => {
                self.app.enable = value;
                save_app_bool(&self.app.canonical_id, "enable", value);

                if !value {
                    self.app.enable_sound_alerts = false;
                    self.app.show_banners = false;
                    self.app.force_expanded = false;
                    self.app.show_in_lock_screen = false;
                    self.app.details_in_lock_screen = false;

                    save_app_bool(&self.app.canonical_id, "enable-sound-alerts", false);
                    save_app_bool(&self.app.canonical_id, "show-banners", false);
                    save_app_bool(&self.app.canonical_id, "force-expanded", false);
                    save_app_bool(&self.app.canonical_id, "show-in-lock-screen", false);
                    save_app_bool(&self.app.canonical_id, "details-in-lock-screen", false);
                }
            }

            AppNotificationsPageInput::SetSoundAlerts(value) => {
                if self.app.enable {
                    self.app.enable_sound_alerts = value;
                    save_app_bool(&self.app.canonical_id, "enable-sound-alerts", value);
                }
            }

            AppNotificationsPageInput::SetShowBanners(value) => {
                if self.app.enable && !self.do_not_disturb {
                    self.app.show_banners = value;
                    save_app_bool(&self.app.canonical_id, "show-banners", value);

                    if !value {
                        self.app.force_expanded = false;
                        save_app_bool(&self.app.canonical_id, "force-expanded", false);
                    }
                }
            }

            AppNotificationsPageInput::SetForceExpanded(value) => {
                if self.app.enable && self.app.show_banners && !self.do_not_disturb {
                    self.app.force_expanded = value;
                    save_app_bool(&self.app.canonical_id, "force-expanded", value);
                }
            }

            AppNotificationsPageInput::SetShowInLockScreen(value) => {
                if self.app.enable && self.lock_screen_notifications {
                    self.app.show_in_lock_screen = value;
                    save_app_bool(&self.app.canonical_id, "show-in-lock-screen", value);

                    if !value {
                        self.app.details_in_lock_screen = false;
                        save_app_bool(&self.app.canonical_id, "details-in-lock-screen", false);
                    }
                }
            }

            AppNotificationsPageInput::SetDetailsInLockScreen(value) => {
                if self.app.enable && self.app.show_in_lock_screen && self.lock_screen_notifications
                {
                    self.app.details_in_lock_screen = value;
                    save_app_bool(&self.app.canonical_id, "details-in-lock-screen", value);
                }
            }
        }

        let _ = sender.output(AppNotificationsPageOutput::Changed(self.app.clone()));
    }
}

fn save_app_bool(canonical_id: &str, key: &str, value: bool) {
    let path = format!("{APP_PREFIX}{canonical_id}/");
    let settings = gio::Settings::with_path(APP_SCHEMA, &path);
    let _ = settings.set_boolean(key, value);
}

pub fn app_bool_from_canonical(canonical_id: &str, key: &str) -> bool {
    let path = format!("{APP_PREFIX}{canonical_id}/");
    let settings = gio::Settings::with_path(APP_SCHEMA, &path);
    settings.boolean(key)
}
