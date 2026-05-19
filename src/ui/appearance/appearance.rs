use crate::ui::appearance::appearance_background::{Background, BackgroundOutput};

use anyhow::Context;
use relm4::loading_widgets::LoadingWidgets;
use std::thread;
use std::time::Duration;
use std::{fs, path::Path};
use users::{get_current_uid, get_user_by_uid};

use crate::ui::window::AppMsg;
use crate::utils::parse_dconf;
use relm4::adw::AccentColor;
use relm4::{adw::prelude::*, gtk, gtk::gio::Settings, prelude::*, view};
use relm4_components::open_dialog::*;
use std::path::PathBuf;

const BG_BASE_DIR: &str = "/run/current-system/sw/share/backgrounds";

#[derive(Debug, Clone)]
pub struct AppearanceSettings {
    pub background: Settings,
    pub interface: Settings,
}

impl AppearanceSettings {
    pub fn new() -> Self {
        Self {
            background: Settings::new("org.gnome.desktop.background"),
            interface: Settings::new("org.gnome.desktop.interface"),
        }
    }
}

impl Default for AppearanceSettings {
    fn default() -> Self {
        AppearanceSettings::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccentColorWrapped(pub AccentColor);

impl AccentColorWrapped {
    pub fn iterator() -> impl Iterator<Item = AccentColor> {
        use relm4::adw::AccentColor::*;
        [Blue, Teal, Green, Yellow, Orange, Red, Pink, Purple, Slate]
            .iter()
            .copied()
    }
}

impl From<String> for AccentColorWrapped {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "blue" => AccentColorWrapped(AccentColor::Blue),
            "teal" => AccentColorWrapped(AccentColor::Teal),
            "green" => AccentColorWrapped(AccentColor::Green),
            "yellow" => AccentColorWrapped(AccentColor::Yellow),
            "orange" => AccentColorWrapped(AccentColor::Orange),
            "red" => AccentColorWrapped(AccentColor::Red),
            "pink" => AccentColorWrapped(AccentColor::Pink),
            "purple" => AccentColorWrapped(AccentColor::Purple),
            "slate" => AccentColorWrapped(AccentColor::Slate),
            _ => AccentColorWrapped(AccentColor::Blue),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppearanceStyle {
    Default,
    Dark,
}

#[derive(Debug)]
pub enum AppearanceMsg {
    SetStyle(AppearanceStyle),
    SetBackground(String),
    SendPick(AccentColorWrapped),
    OpenRequest,
    OpenResponse(PathBuf),
    Ignore,
}

#[derive(Debug)]
pub struct AppearanceModel {
    style: AppearanceStyle,
    wallpaper: String,
    wallpapers: AsyncFactoryVecDeque<Background>,
    recent_wallpapers: AsyncFactoryVecDeque<Background>,
    accent_color: AccentColorWrapped,
    open_dialog: Controller<OpenDialog>,
    group: gtk::ToggleButton,
}

#[relm4::component(pub, async)]
impl AsyncComponent for AppearanceModel {
    type Init = ();
    type Input = AppearanceMsg;
    type Output = AppMsg;
    type CommandOutput = ();

    view! {
        #[root]
        adw::BreakpointBin {
            set_width_request: 346,
            set_height_request: 200,

            add_breakpoint = adw::Breakpoint::new(
                adw::BreakpointCondition::new_length(
                    adw::BreakpointConditionLengthType::MaxWidth,
                    420.0,
                    adw::LengthUnit::Px,
                )
            ) {
                add_setters: &[
                    (
                        &accent_box,
                        "spacing",
                        &6
                    ),
                    (
                        &accent_box,
                        "margin-top",
                        &6
                    ),
                    (
                        &accent_box,
                        "margin-bottom",
                        &6
                    ),
                ],

                add_setters: &[
                    (recent_wallpaper_box, "min_children_per_line", &1),
                    (recent_wallpaper_box, "max_children_per_line", &1)
                ],

                add_setters: &[
                    (wallpaper_box, "min_children_per_line", &1),
                    (wallpaper_box, "max_children_per_line", &1)
                ],
            },

            add_breakpoint = adw::Breakpoint::new(
                adw::BreakpointCondition::new_length(
                    adw::BreakpointConditionLengthType::MinWidth,
                    421.0,
                    adw::LengthUnit::Px,
                )
            ) {
                add_setters: &[
                    (
                        &accent_box,
                        "spacing",
                        &12
                    ),
                    (
                        &accent_box,
                        "margin-top",
                        &12
                    ),
                    (
                        &accent_box,
                        "margin-bottom",
                        &12
                    ),
                ],

                add_setters: &[
                    (recent_wallpaper_box, "min_children_per_line", &3),
                    (recent_wallpaper_box, "max_children_per_line", &3)
                ],


                add_setters: &[
                    (wallpaper_box, "min_children_per_line", &3),
                    (wallpaper_box, "max_children_per_line", &3)
                ],
            },

            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                set_top_bar_style: adw::ToolbarStyle::Flat,

                add_top_bar = &adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                        set_title: "Appearance",
                    }
                },
                #[name(toast_overlay)]
                adw::ToastOverlay {
                    adw::PreferencesPage {
                        adw::PreferencesGroup {
                            set_title: "Style",

                            adw::PreferencesRow {
                                #[wrap(Some)]
                                set_child = &adw::Clamp {
                                    set_maximum_size: 400,
                                    set_tightening_threshold: 300,

                                    gtk::Grid {
                                        set_focusable: false,
                                        set_orientation: gtk::Orientation::Horizontal,
                                        set_column_spacing: 24,
                                        set_row_spacing: 12,
                                        set_column_homogeneous: true,
                                        set_hexpand: true,
                                        set_margin_top: 18,
                                        set_margin_bottom: 12,
                                        set_margin_start: 12,
                                        set_margin_end: 12,

                                        #[name = "default_style" ]
                                        attach[0,0,1,1] = &gtk::ToggleButton{
                                            set_group: Some(&dark_style),
                                            set_overflow: gtk::Overflow::Hidden,
                                            add_css_class: "style-toggle",
                                            set_active: model.style == AppearanceStyle::Default,

                                            #[wrap(Some)]
                                            set_child = &gtk::Picture{
                                                set_content_fit: gtk::ContentFit::Cover,
                                                set_filename: Some(&model.wallpaper)
                                            },

                                            connect_clicked => AppearanceMsg::SetStyle(AppearanceStyle::Default),
                                        },

                                        attach[0,1,1,1] = &gtk::Label {
                                            set_label: "Default",
                                            set_halign: gtk::Align::Center,
                                            set_hexpand: true,
                                        },

                                        #[name = "dark_style" ]
                                        attach[1,0,1,1] = &gtk::ToggleButton{
                                            add_css_class: "style-toggle",
                                            set_overflow: gtk::Overflow::Hidden,
                                            set_active: model.style == AppearanceStyle::Dark,

                                            #[wrap(Some)]
                                            set_child = &gtk::Picture{
                                                set_content_fit: gtk::ContentFit::Fill,
                                                set_filename: Some(&model.wallpaper)
                                            },

                                            connect_clicked => AppearanceMsg::SetStyle(AppearanceStyle::Dark),
                                        },

                                        attach[1,1,1,1] = &gtk::Label {
                                            set_label: "Dark",
                                            set_halign: gtk::Align::Center,
                                            set_hexpand: true,
                                        },
                                    },
                                }
                            },
                        },
                        adw::PreferencesGroup {
                            set_title: "Accent Color",

                            adw::PreferencesRow {
                                set_halign: gtk::Align::Center,
                                set_accessible_role: gtk::AccessibleRole::Group,
                                set_activatable: false,
                                set_focusable: false,

                                #[name = "accent_box"]
                                #[wrap(Some)]
                                set_child = &gtk::Box {
                                    set_orientation: gtk::Orientation::Horizontal,
                                    set_spacing: 12,
                                    set_margin_top: 12,
                                    set_margin_bottom: 12,

                                    #[name = "accent_color"]
                                    gtk::ToggleButton {
                                        add_css_class: "accent-button",
                                        add_css_class: "blue",

                                        connect_clicked[sender] => move |_| {
                                            sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Blue)));
                                        },
                                        set_active: model.accent_color == AccentColorWrapped(AccentColor::Blue)
                                    },
                                    gtk::ToggleButton {
                                        set_group: Some(&accent_color),
                                        add_css_class: "accent-button",
                                        add_css_class: "teal",

                                        connect_clicked[sender] => move |_| {
                                            sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Teal)));
                                        },
                                        set_active: model.accent_color == AccentColorWrapped(AccentColor::Teal)
                                    },
                                    gtk::ToggleButton {
                                        set_group: Some(&accent_color),
                                        add_css_class: "accent-button",
                                        add_css_class: "green",

                                        connect_clicked[sender] => move |_| {
                                            sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Green)));
                                        },
                                        set_active: model.accent_color == AccentColorWrapped(AccentColor::Green)
                                    },
                                    gtk::ToggleButton {
                                        set_group: Some(&accent_color),
                                        add_css_class: "accent-button",
                                        add_css_class: "yellow",

                                        connect_clicked[sender] => move |_| {
                                            sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Yellow)));
                                        },
                                        set_active: model.accent_color == AccentColorWrapped(AccentColor::Yellow)
                                    },
                                    gtk::ToggleButton {
                                        set_group: Some(&accent_color),
                                        add_css_class: "accent-button",
                                        add_css_class: "orange",

                                        connect_clicked[sender] => move |_| {
                                            sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Orange)));
                                        },
                                        set_active: model.accent_color == AccentColorWrapped(AccentColor::Orange)
                                    },
                                    gtk::ToggleButton {
                                        set_group: Some(&accent_color),
                                        add_css_class: "accent-button",
                                        add_css_class: "red",

                                        connect_clicked[sender] => move |_| {
                                            sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Red)));
                                        },
                                        set_active: model.accent_color == AccentColorWrapped(AccentColor::Red)
                                    },
                                    gtk::ToggleButton {
                                        set_group: Some(&accent_color),
                                        add_css_class: "accent-button",
                                        add_css_class: "pink",

                                        connect_clicked[sender] => move |_| {
                                            sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Pink)));
                                        },
                                        set_active: model.accent_color == AccentColorWrapped(AccentColor::Pink)
                                    },
                                    gtk::ToggleButton {
                                        set_group: Some(&accent_color),
                                        add_css_class: "accent-button",
                                        add_css_class: "purple",

                                        connect_clicked[sender] => move |_| {
                                            sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Purple)));
                                        },
                                        set_active: model.accent_color == AccentColorWrapped(AccentColor::Purple)
                                    },
                                    gtk::ToggleButton {
                                        set_group: Some(&accent_color),
                                        add_css_class: "accent-button",
                                        add_css_class: "slate",

                                        connect_clicked[sender] => move |_| {
                                            sender.input(AppearanceMsg::SendPick(AccentColorWrapped(AccentColor::Slate)));
                                        },
                                        set_active: model.accent_color == AccentColorWrapped(AccentColor::Slate)
                                    },
                                },
                            },
                        },
                        adw::PreferencesGroup {
                            set_title: "Background",

                            #[wrap(Some)]
                            set_header_suffix = &gtk::Button {
                                add_css_class: "flat",

                                connect_clicked => AppearanceMsg::OpenRequest,

                                adw::ButtonContent {
                                    set_icon_name: "list-add-symbolic",
                                    set_label: "Add Picture",
                                    set_use_underline: true,
                                }
                            },

                            adw::Bin {
                                add_css_class: "card",
                                set_accessible_role: gtk::AccessibleRole::Group,

                                #[name = "background_chooser"]
                                gtk::Box {
                                    set_orientation: gtk::Orientation::Vertical,
                                    set_hexpand: true,
                                    // set_toast_overlay: toast_overlay,

                                    #[name="recent_box"]
                                    gtk::Box {
                                        set_orientation: gtk::Orientation::Vertical,
                                        set_halign: gtk::Align::Center,

                                        #[local_ref]
                                        recent_wallpaper_box -> gtk::FlowBox {
                                            add_css_class: "background-flowbox",
                                            set_orientation: gtk::Orientation::Horizontal,
                                            set_margin_all: 12,
                                            set_column_spacing: 12,
                                            set_row_spacing: 12,
                                            set_homogeneous: true,
                                            set_halign: gtk::Align::Center,
                                            set_hexpand: true,
                                            set_min_children_per_line: 3,
                                            set_max_children_per_line: 3,
                                            set_activate_on_single_click: true,
                                            set_selection_mode: gtk::SelectionMode::Single
                                        },

                                        gtk::Separator {
                                            set_margin_top: 12,
                                            set_margin_bottom: 12,
                                        }
                                    },
                                    ////////////////////////////////////////////////////////////

                                    // #[name = "flowbox"]
                                    #[local_ref]
                                    wallpaper_box -> gtk::FlowBox {
                                        add_css_class: "background-flowbox",
                                        set_margin_all: 12,
                                        set_column_spacing: 12,
                                        set_row_spacing: 12,
                                        set_homogeneous: true,
                                        set_halign: gtk::Align::Center,
                                        set_min_children_per_line: 3,
                                        set_max_children_per_line: 3,
                                        set_activate_on_single_click: true,
                                        set_selection_mode: gtk::SelectionMode::Single,


                                        #[name="wallpaper_group"]
                                        gtk::ToggleButton {
                                            // set_group: Some(&wallpaper_group),
                                        }
                                    },
                                },
                            },
                        }
                    }
                }
            }
        }
    }

    fn init_loading_widgets(root: Self::Root) -> Option<LoadingWidgets> {
        view! {
            #[local]
            root {
                // set_title: Some("Simple app"),
                // set_default_size: (300, 100),

                // This will be removed automatically by
                // LoadingWidgets when the full view has loaded
                #[name(spinner)]
                gtk::Spinner {
                    start: (),
                    set_halign: gtk::Align::Center,
                }
            }
        }
        Some(LoadingWidgets::new(root, spinner))
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        tokio::time::sleep(Duration::from_secs(1)).await;

        let open_dialog = OpenDialog::builder()
            .transient_for_native(&root)
            .launch(OpenDialogSettings::default())
            .forward(sender.input_sender(), |response| match response {
                OpenDialogResponse::Accept(path) => AppearanceMsg::OpenResponse(path),
                OpenDialogResponse::Cancel => AppearanceMsg::Ignore,
            });

        let background_factory = || {
            AsyncFactoryVecDeque::<Background>::builder()
                .launch(gtk::FlowBox::default())
                .forward(sender.input_sender(), |output| match output {
                    BackgroundOutput::SetBackgroundPath(path) => {
                        AppearanceMsg::SetBackground(path.clone())
                    }
                })
        };
        let (wallpapers, recent_wallpapers) = (background_factory(), background_factory());

        let settings = AppearanceSettings::new();
        let wallpaper = parse_dconf(settings.background.get::<String>("picture-uri"));
        let accent_color =
            AccentColorWrapped::from(settings.interface.get::<String>("accent-color"));
        let style = match settings.interface.get::<String>("color-scheme").as_str() {
            "prefer-dark" => AppearanceStyle::Dark,
            _ => AppearanceStyle::Default,
        };
        let group = gtk::ToggleButton::new();
        let mut model = AppearanceModel {
            style,
            wallpaper,
            wallpapers,
            recent_wallpapers,
            accent_color,
            open_dialog,
            group,
        };

        let folders: [&str; 2] = ["nixos", "gnome"];

        for folder in folders {
            let path: PathBuf = Path::new(BG_BASE_DIR).join(folder);
            match fs::read_dir(&path) {
                Ok(rd) => {
                    let _ = rd
                        .filter(|x| x.is_ok())
                        .map(|x| {
                            x.and_then(|y| {
                                let path = y.path().to_string_lossy().to_string();

                                model.wallpapers.guard().push_back(Background {
                                    path: path.clone(),
                                    group: model.group.clone(),
                                    active: path[47..]
                                        == settings.background.get::<String>("picture-uri")[51..],
                                });

                                Ok(y)
                            })
                        })
                        .collect::<Vec<_>>();
                }
                Err(err) => {
                    eprintln!(
                        "Failed to read '{1}': {0}",
                        err,
                        path.to_str().unwrap_or_default()
                    );
                    continue;
                }
            }
        }

        let user = get_user_by_uid(get_current_uid()).unwrap();

        let _: Vec<_> = fs::read_dir(format!(
            "/home/{}/.local/share/backgrounds",
            user.name().to_string_lossy()
        ))
        .unwrap()
        .map(|x| {
            model.recent_wallpapers.guard().push_back(Background {
                path: x.unwrap().path().to_str().unwrap().to_string(),
                group: model.group.clone(),
                active: false,
            });
        })
        .collect();

        let wallpaper_box = model.wallpapers.widget();
        let recent_wallpaper_box = model.recent_wallpapers.widget();

        let widgets = view_output!();
        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        msg: Self::Input,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        tokio::time::sleep(Duration::from_secs(1)).await;

        let settings = AppearanceSettings::new();
        let user = get_user_by_uid(get_current_uid()).unwrap();

        match msg {
            AppearanceMsg::OpenRequest => self.open_dialog.emit(OpenDialogMsg::Open),
            AppearanceMsg::OpenResponse(path) => {
                self.recent_wallpapers.guard().push_back(Background {
                    path: path.to_str().unwrap().to_string(),
                    group: self.group.clone(),
                    active: false,
                });

                let f_name = path.file_name().context("extract filename failed");

                let dest = PathBuf::from("/home")
                    .join(user.name())
                    .join(".local/share/backgrounds")
                    .join(f_name.unwrap_or_default());

                std::fs::copy(&path, &dest).unwrap_or_default();
            }
            AppearanceMsg::SetStyle(style) => {
                self.style = style;

                match style {
                    AppearanceStyle::Dark => {
                        settings
                            .interface
                            .set("color-scheme", "prefer-dark")
                            .unwrap();
                    }

                    AppearanceStyle::Default => {
                        settings.interface.set("color-scheme", "default").unwrap();
                    }
                }
            }

            AppearanceMsg::SetBackground(path) => {
                println!("setting wallapaper");
                let _ = settings.background.set(
                    match settings.interface.get::<String>("color-scheme").as_str() {
                        "prefer-dark" => "picture-uri-dark",
                        _ => "picture-uri",
                    },
                    format!("file://{}", path),
                );
                println!("BACKGROUND: {}", &path.clone());
            }
            AppearanceMsg::SendPick(color) => {
                settings
                    .interface
                    .set("accent-color", format!("{:?}", color.0).to_lowercase())
                    .unwrap();
            }
            AppearanceMsg::Ignore => {}
        }
    }
}
