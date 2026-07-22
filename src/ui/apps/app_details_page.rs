use crate::ui::notifications::app_notification::app_settings_for_canonical;
use relm4::adw;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::gtk::gio;
use relm4::prelude::*;

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
}

#[derive(Debug, Clone)]
pub enum AppDetailsMsg {
    OpenApp,
    ShowDetails(gtk::Button),
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
                    set_revealed: true,
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
                                                button.clone()
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
        let model = Self { app };
        let widgets = view_output!();

        if let Some(icon) = &model.app.icon {
            widgets.app_icon.set_from_gicon(icon);
        } else {
            widgets
                .app_icon
                .set_icon_name(Some("application-x-executable-symbolic"));
        }

        setup_notifications_row(&model.app, &widgets.notifications_row);

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
        }
    }
}

fn setup_notifications_row(app: &AppEntry, row: &adw::SwitchRow) {
    if let Some(settings) = app.notification_settings() {
        row.set_subtitle("Show system notifications");
        row.set_sensitive(true);
        row.set_active(settings.boolean("enable"));

        let settings_for_toggle = settings.clone();

        row.connect_active_notify(move |row| {
            let value = row.is_active();

            if settings_for_toggle.boolean("enable") != value {
                let _ = settings_for_toggle.set_boolean("enable", value);
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

impl AppEntry {
    fn notification_settings(&self) -> Option<gio::Settings> {
        let canonical_id = self.canonical_id.as_deref()?;

        Some(app_settings_for_canonical(canonical_id))
    }

    fn show_app_details_dialog(&self, button: &gtk::Button) {
        let details = format!(
            "Name: {}\nApp ID: {}\nCanonical ID: {}\nDescription: {}\nExecutable: {}\nSupports files: {}\nSupports URIs: {}",
            self.name,
            self.app_id.clone().unwrap_or_else(|| "—".to_string()),
            self.canonical_id.clone().unwrap_or_else(|| "—".to_string()),
            self.description.clone().unwrap_or_else(|| "—".to_string()),
            self.executable.clone().unwrap_or_else(|| "—".to_string()),
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

        if let Some(root) = button.root() {
            if let Ok(window) = root.downcast::<gtk::Window>() {
                dialog.show(Some(&window));
            }
        }
    }
}

// impl AppDetailsPage {
//     fn show_app_details_dialog(&self, button: &gtk::Button) {
//         self.app.show_app_details_dialog(button);
//     }
// }
