use crate::ui::{
    apps::{
        app_details_page::{AppDetailsPage, AppEntry},
        default_apps::DefaultAppsPage,
    },
    window::AppMsg,
};

use relm4::{adw, adw::prelude::*, gtk, gtk::gio, prelude::*};

#[derive(Debug)]
pub struct AppModal {
    navigation: adw::NavigationView,
    apps_list: gtk::ListBox,
    default_apps: Controller<DefaultAppsPage>,
    details_pages: Vec<Controller<AppDetailsPage>>,
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
                                    sender.input(AppsMsg::SearchChanged(
                                        entry.text().to_string()
                                    ));
                                }
                            }
                        },

                        add = &adw::PreferencesGroup {
                            set_title: "General",

                            adw::ActionRow {
                                set_use_markup: false,
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
        let default_apps = DefaultAppsPage::builder().launch(()).detach();

        let apps = collect_apps();
        let filtered_apps = apps.clone();

        let mut model = Self {
            navigation: adw::NavigationView::new(),
            apps_list: gtk::ListBox::new(),
            default_apps,
            details_pages: Vec::new(),
            apps,
            filtered_apps,
        };

        let widgets = view_output!();

        model.navigation = widgets.navigation.clone();
        model.apps_list = widgets.apps_list.clone();

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

                self.filtered_apps = if query.is_empty() {
                    self.apps.clone()
                } else {
                    self.apps
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
                        .collect()
                };

                rebuild_apps_list(&self.apps_list, &self.filtered_apps, sender.clone());
            }

            AppsMsg::OpenAppDetails(app) => {
                let details_page = AppDetailsPage::builder().launch(app).detach();

                self.navigation.push(details_page.widget());

                self.details_pages.push(details_page);
            }
        }
    }
}

fn collect_apps() -> Vec<AppEntry> {
    let mut apps: Vec<AppEntry> = gio::AppInfo::all()
        .into_iter()
        .filter(|app| app.should_show())
        .map(|app| {
            let app_id = app.id().map(|s| s.to_string());
            let canonical_id = app_id.as_deref().map(notification_canonical_id);

            AppEntry {
                name: app.display_name().to_string(),
                description: app.description().map(|s| s.to_string()),
                executable: Some(app.executable().to_string_lossy().into_owned()),
                icon: app.icon(),
                app_info: app,
                app_id,
                canonical_id,
            }
        })
        .collect();

    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    apps.dedup_by(|a, b| {
        a.name == b.name
            && a.executable.as_deref().unwrap_or("") == b.executable.as_deref().unwrap_or("")
    });

    apps
}

fn notification_canonical_id(app_id: &str) -> String {
    let app_id = app_id.strip_suffix(".desktop").unwrap_or(app_id);

    app_id
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect()
}

fn rebuild_apps_list(list: &gtk::ListBox, apps: &[AppEntry], sender: ComponentSender<AppModal>) {
    while let Some(child) = list.first_child() {
        list.remove(&child);
    }

    for app in apps {
        let row = adw::ActionRow::new();

        row.set_use_markup(false);
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
