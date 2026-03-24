use crate::ui::apps::default_apps::DefaultAppsPage;
use crate::ui::window::AppMsg;
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
}

#[derive(Debug)]
pub struct AppModal {
    navigation: adw::NavigationView,
    apps_list: gtk::ListBox,
    default_apps: Controller<DefaultAppsPage>,
    apps: Vec<AppEntry>,
    filtered_apps: Vec<AppEntry>,
}

#[derive(Debug, Clone)]
pub enum AppsMsg {
    OpenDefaultApps,
    SearchChanged(String),
    OpenAppDetails(AppEntry),
}

#[relm4::component(pub)]
impl SimpleComponent for AppModal {
    type Init = ();
    type Input = AppsMsg;
    type Output = AppMsg;

    view! {
        #[name = "navigation"]
        adw::NavigationView {
            add = &adw::NavigationPage {
                set_title: "Apps",

                #[wrap(Some)]
                set_child = &adw::ToolbarView {
                    set_top_bar_style: adw::ToolbarStyle::Flat,

                    add_top_bar = &adw::HeaderBar {
                        #[wrap(Some)]
                        set_title_widget = &adw::WindowTitle {
                            set_title: "Apps"
                        }
                    },

                    #[wrap(Some)]
                    set_content = &adw::PreferencesPage {
                        set_title: "Apps",
                        set_icon_name: Some("application-x-executable-symbolic"),

                        add = &adw::PreferencesGroup {
                            set_title: "Search",

                            gtk::SearchEntry {
                                set_placeholder_text: Some("Search apps"),
                                connect_search_changed[sender] => move |entry| {
                                    sender.input(AppsMsg::SearchChanged(entry.text().to_string()));
                                }
                            }
                        },

                        add = &adw::PreferencesGroup {
                            set_title: "General",

                            adw::ActionRow {
                                set_title: "Default Apps",
                                set_subtitle: "Set which apps open links, files, and media",
                                set_activatable: true,
                                connect_activated => AppsMsg::OpenDefaultApps,

                                add_suffix = &gtk::Image {
                                    set_icon_name: Some("go-next-symbolic"),
                                    set_valign: gtk::Align::Center,
                                }
                            }
                        },

                        add = &adw::PreferencesGroup {
                            set_title: "Installed Apps",

                            #[name = "apps_list"]
                            gtk::ListBox {
                                add_css_class: "boxed-list",
                                set_selection_mode: gtk::SelectionMode::None,
                            }
                        }
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();

        let default_apps = DefaultAppsPage::builder().launch(()).detach();

        let apps = collect_apps();
        let filtered_apps = apps.clone();

        let model = Self {
            navigation: widgets.navigation.clone(),
            apps_list: widgets.apps_list.clone(),
            default_apps,
            apps,
            filtered_apps,
        };

        rebuild_apps_list(&model.apps_list, &model.filtered_apps, sender.clone());

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            AppsMsg::OpenDefaultApps => {
                let page = self.default_apps.widget();
                self.navigation.push(page);
            }
            AppsMsg::SearchChanged(query) => {
                let query = query.trim().to_lowercase();

                if query.is_empty() {
                    self.filtered_apps = self.apps.clone();
                } else {
                    self.filtered_apps = self
                        .apps
                        .iter()
                        .filter(|app| {
                            app.name.to_lowercase().contains(&query)
                                || app
                                    .description
                                    .as_deref()
                                    .unwrap_or("")
                                    .to_lowercase()
                                    .contains(&query)
                                || app
                                    .executable
                                    .as_deref()
                                    .unwrap_or("")
                                    .to_lowercase()
                                    .contains(&query)
                        })
                        .cloned()
                        .collect();
                }

                rebuild_apps_list(&self.apps_list, &self.filtered_apps, sender.clone());
            }
            AppsMsg::OpenAppDetails(app) => {
                let page = build_app_details_page(&app);
                self.navigation.push(&page);
            }
        }
    }
}

fn collect_apps() -> Vec<AppEntry> {
    let mut apps: Vec<AppEntry> = gio::AppInfo::all()
        .into_iter()
        .filter(|app| app.should_show())
        .map(|app| AppEntry {
            name: app.display_name().to_string(),
            description: app.description().map(|s| s.to_string()),
            executable: Some(app.executable().to_string_lossy().into_owned()),
            icon: app.icon(),
            app_info: app,
        })
        .collect();

    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    apps.dedup_by(|a, b| {
        a.name == b.name
            && a.executable.as_deref().unwrap_or("") == b.executable.as_deref().unwrap_or("")
    });

    apps
}

fn rebuild_apps_list(list: &gtk::ListBox, apps: &[AppEntry], sender: ComponentSender<AppModal>) {
    while let Some(child) = list.first_child() {
        list.remove(&child);
    }

    for app in apps {
        let row = adw::ActionRow::new();
        row.set_title(&app.name);
        row.set_activatable(true);

        if let Some(subtitle) = app
            .description
            .as_deref()
            .filter(|s| !s.is_empty())
            .or(app.executable.as_deref())
        {
            row.set_subtitle(subtitle);
        }

        let image = if let Some(icon) = &app.icon {
            gtk::Image::from_gicon(icon)
        } else {
            gtk::Image::from_icon_name("application-x-executable-symbolic")
        };

        image.set_pixel_size(24);
        image.set_valign(gtk::Align::Center);
        row.add_prefix(&image);

        let arrow = gtk::Image::from_icon_name("go-next-symbolic");
        arrow.set_valign(gtk::Align::Center);
        row.add_suffix(&arrow);

        let app_clone = app.clone();
        let sender_clone = sender.clone();
        row.connect_activated(move |_| {
            sender_clone.input(AppsMsg::OpenAppDetails(app_clone.clone()));
        });

        list.append(&row);
    }
}

fn build_app_details_page(app: &AppEntry) -> adw::NavigationPage {
    let toolbar = adw::ToolbarView::new();
    toolbar.set_top_bar_style(adw::ToolbarStyle::Flat);

    let header = adw::HeaderBar::new();
    let title = adw::WindowTitle::new(&app.name, "");
    header.set_title_widget(Some(&title));
    toolbar.add_top_bar(&header);

    let scrolled = gtk::ScrolledWindow::new();
    scrolled.set_vexpand(true);
    scrolled.set_hexpand(true);

    let clamp = adw::Clamp::new();
    clamp.set_maximum_size(700);
    clamp.set_tightening_threshold(500);

    let content = gtk::Box::new(gtk::Orientation::Vertical, 20);
    content.set_margin_top(24);
    content.set_margin_bottom(24);
    content.set_margin_start(24);
    content.set_margin_end(24);

    let status_group = adw::PreferencesGroup::new();
    let banner_row = adw::ActionRow::new();
    banner_row.set_title("App is not sandboxed");
    banner_row.add_suffix(&gtk::Image::from_icon_name("dialog-information-symbolic"));
    status_group.add(&banner_row);

    let hero_box = gtk::Box::new(gtk::Orientation::Vertical, 16);
    hero_box.set_halign(gtk::Align::Center);
    hero_box.set_margin_top(12);
    hero_box.set_margin_bottom(12);

    let icon = if let Some(icon) = &app.icon {
        gtk::Image::from_gicon(icon)
    } else {
        gtk::Image::from_icon_name("application-x-executable-symbolic")
    };
    icon.set_pixel_size(96);
    icon.set_halign(gtk::Align::Center);

    let name_label = gtk::Label::new(Some(&app.name));
    name_label.add_css_class("title-1");
    name_label.set_halign(gtk::Align::Center);

    let buttons_box = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    buttons_box.set_halign(gtk::Align::Center);

    let open_button = gtk::Button::with_label("Open");
    open_button.add_css_class("suggested-action");

    let details_button = gtk::Button::with_label("App Details");

    buttons_box.append(&open_button);
    buttons_box.append(&details_button);

    hero_box.append(&icon);
    hero_box.append(&name_label);
    hero_box.append(&buttons_box);

    let permissions_group = adw::PreferencesGroup::new();
    permissions_group.set_title("Permissions");

    let notifications_row = adw::SwitchRow::new();
    notifications_row.set_title("Notifications");
    notifications_row.set_subtitle("Show system notifications");
    notifications_row.set_active(true);

    permissions_group.add(&notifications_row);

    content.append(&status_group);
    content.append(&hero_box);
    content.append(&permissions_group);

    clamp.set_child(Some(&content));
    scrolled.set_child(Some(&clamp));
    toolbar.set_content(Some(&scrolled));

    let app_for_open = app.clone();
    open_button.connect_clicked(move |_| {
        let ctx = gio::AppLaunchContext::new();
        if let Err(err) = app_for_open.app_info.launch(&[], Some(&ctx)) {
            eprintln!("Failed to launch app '{}': {err}", app_for_open.name);
        }
    });

    let app_for_dialog = app.clone();
    details_button.connect_clicked(move |button| {
        let details = format!(
            "Name: {}\nDescription: {}\nExecutable: {}\nSupports files: {}\nSupports URIs: {}",
            app_for_dialog.name,
            app_for_dialog
                .description
                .clone()
                .unwrap_or_else(|| "—".to_string()),
            app_for_dialog
                .executable
                .clone()
                .unwrap_or_else(|| "—".to_string()),
            if app_for_dialog.app_info.supports_files() {
                "Yes"
            } else {
                "No"
            },
            if app_for_dialog.app_info.supports_uris() {
                "Yes"
            } else {
                "No"
            },
        );

        let dialog = gtk::AlertDialog::builder()
            .modal(true)
            .message(&app_for_dialog.name)
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
    });

    adw::NavigationPage::builder()
        .title(&app.name)
        .child(&toolbar)
        .build()
}