use relm4::adw;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::gtk::{gio, glib};
use relm4::prelude::*;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct DefaultAppsPage {
    rows: Vec<RowState>,
}

#[derive(Debug, Clone)]
pub enum DefaultAppsMsg {
    RowChanged(DefaultCategory, u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DefaultCategory {
    Web,
    Mail,
    Calendar,
    Music,
    Video,
    Photos,
}

#[derive(Debug, Clone)]
struct AppChoice {
    name: String,
    app_info: gio::AppInfo,
}

#[derive(Debug)]
struct RowState {
    kind: DefaultCategory,
    content_type: &'static str,
    filters: Option<&'static str>,
    choices: Vec<AppChoice>,
}

impl DefaultCategory {
    fn title(self) -> &'static str {
        match self {
            Self::Web => "Web",
            Self::Mail => "Mail",
            Self::Calendar => "Calendar",
            Self::Music => "Music",
            Self::Video => "Video",
            Self::Photos => "Photos",
        }
    }

    fn content_type(self) -> &'static str {
        match self {
            Self::Web => "text/html",
            Self::Mail => "x-scheme-handler/mailto",
            Self::Calendar => "text/calendar",
            Self::Music => "audio/x-vorbis+ogg",
            Self::Video => "video/x-ogm+ogg",
            Self::Photos => "image/jpeg",
        }
    }

    fn filters(self) -> Option<&'static str> {
        match self {
            Self::Web => Some(
                "x-scheme-handler/http;x-scheme-handler/https;text/html;text/xml;application/xhtml+xml",
            ),
            Self::Mail => Some("x-scheme-handler/mailto;message/rfc822"),
            Self::Calendar => Some("text/calendar"),
            Self::Music => Some("audio/*"),
            Self::Video => Some("video/*"),
            Self::Photos => Some("image/*"),
        }
    }
}

#[relm4::component(pub)]
impl SimpleComponent for DefaultAppsPage {
    type Init = ();
    type Input = DefaultAppsMsg;
    type Output = ();

    view! {
        adw::NavigationPage {
            set_title: "Default Apps",
            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                set_top_bar_style: adw::ToolbarStyle::Flat,
                add_top_bar = &adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                        set_title: "Default Apps"
                    }
                },
                #[wrap(Some)]
                set_content = &adw::PreferencesPage {
                    adw::PreferencesGroup {
                        set_title: "Default Apps",

                        #[name = "web_row"]
                        adw::ComboRow {
                            set_title: "Web",
                        },

                        #[name = "mail_row"]
                        adw::ComboRow {
                            set_title: "Mail",
                        },

                        #[name = "calendar_row"]
                        adw::ComboRow {
                            set_title: "Calendar",
                        },

                        #[name = "music_row"]
                        adw::ComboRow {
                            set_title: "Music",
                        },

                        #[name = "video_row"]
                        adw::ComboRow {
                            set_title: "Video",
                        },

                        #[name = "photos_row"]
                        adw::ComboRow {
                            set_title: "Photos",
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

        let mut model = DefaultAppsPage { rows: Vec::new() };

        setup_row(
            &mut model,
            &widgets.web_row,
            DefaultCategory::Web,
            sender.clone(),
        );
        setup_row(
            &mut model,
            &widgets.mail_row,
            DefaultCategory::Mail,
            sender.clone(),
        );
        setup_row(
            &mut model,
            &widgets.calendar_row,
            DefaultCategory::Calendar,
            sender.clone(),
        );
        setup_row(
            &mut model,
            &widgets.music_row,
            DefaultCategory::Music,
            sender.clone(),
        );
        setup_row(
            &mut model,
            &widgets.video_row,
            DefaultCategory::Video,
            sender.clone(),
        );
        setup_row(
            &mut model,
            &widgets.photos_row,
            DefaultCategory::Photos,
            sender.clone(),
        );

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            DefaultAppsMsg::RowChanged(kind, selected) => {
                if let Some(row_state) = self.rows.iter().find(|row| row.kind == kind) {
                    if let Some(choice) = row_state.choices.get(selected as usize) {
                        if let Err(err) = set_default_for_row(row_state, &choice.app_info) {
                            eprintln!(
                                "Failed to set default app for {} to {}: {}",
                                kind.title(),
                                choice.name,
                                err
                            );
                        }
                    }
                }
            }
        }
    }
}

fn setup_row(
    model: &mut DefaultAppsPage,
    row: &adw::ComboRow,
    kind: DefaultCategory,
    sender: ComponentSender<DefaultAppsPage>,
) {
    let content_type = kind.content_type();
    let filters = kind.filters();
    let choices = collect_apps_for_row(content_type);

    let names: Vec<&str> = if choices.is_empty() {
        vec!["No Apps Available"]
    } else {
        choices.iter().map(|choice| choice.name.as_str()).collect()
    };

    let selected = current_default_index(content_type, &choices).unwrap_or(0) as u32;

    let string_list = gtk::StringList::new(&names);
    row.set_model(Some(&string_list));
    row.set_selected(selected.min((names.len().saturating_sub(1)) as u32));

    if choices.is_empty() {
        row.set_sensitive(false);
    } else {
        row.set_sensitive(true);

        row.connect_selected_notify(move |combo| {
            sender.input(DefaultAppsMsg::RowChanged(kind, combo.selected()));
        });
    }

    model.rows.push(RowState {
        kind,
        content_type,
        filters,
        choices,
    });
}

fn collect_apps_for_row(content_type: &str) -> Vec<AppChoice> {
    let mut map: BTreeMap<String, AppChoice> = BTreeMap::new();
    let default_app = gio::AppInfo::default_for_type(content_type, false);

    if let Some(app) = default_app.clone() {
        insert_app_choice(&mut map, app);
    }

    for app in gio::AppInfo::recommended_for_type(content_type) {
        let is_same_as_default = default_app
            .as_ref()
            .map(|default| app.equal(default))
            .unwrap_or(false);

        if is_same_as_default {
            continue;
        }

        insert_app_choice(&mut map, app);
    }

    map.into_values().collect()
}

fn insert_app_choice(map: &mut BTreeMap<String, AppChoice>, app: gio::AppInfo) {
    if !app.should_show() {
        return;
    }

    let key = app
        .id()
        .map(|s| s.to_string())
        .unwrap_or_else(|| app.display_name().to_string().to_lowercase());

    map.entry(key).or_insert_with(|| AppChoice {
        name: app.display_name().to_string(),
        app_info: app,
    });
}

fn current_default_index(content_type: &str, choices: &[AppChoice]) -> Option<usize> {
    let current = gio::AppInfo::default_for_type(content_type, false)?;

    let current_id = current.id().map(|s| s.to_string());
    let current_name = current.display_name().to_string();

    choices.iter().position(|choice| {
        choice.app_info.id().map(|s| s.to_string()) == current_id
            || choice.app_info.display_name() == current_name
    })
}

fn mime_matches_filter(mime: &str, filter: &str) -> bool {
    if let Some(prefix) = filter.strip_suffix("/*") {
        mime.starts_with(&format!("{prefix}/"))
    } else {
        mime == filter
    }
}

fn set_default_for_row(row: &RowState, app: &gio::AppInfo) -> Result<(), glib::Error> {
    app.set_as_default_for_type(row.content_type)?;

    if let Some(filters) = row.filters {
        let patterns: Vec<&str> = filters
            .split(';')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect();

        let supported_types = app.supported_types();

        for mime in supported_types.iter() {
            let mime = mime.as_str();

            let matched = patterns
                .iter()
                .any(|pattern| mime_matches_filter(mime, pattern));

            if !matched {
                continue;
            }

            if let Err(err) = app.set_as_default_for_type(mime) {
                eprintln!(
                    "Failed to set '{}' as default for secondary content type '{}': {}",
                    app.display_name(),
                    mime,
                    err
                );
            }
        }
    }

    Ok(())
}
