use crate::ui::appearance::appearance_background::{Background, BackgroundOutput};
use crate::ui::appearance::components::accent_box::{
    AccentColorModel, AccentColorOutput, AccentColorWrapped,
};
use crate::ui::appearance::util::{add_wallpaper, thumb};
use crate::utils::parse_dconf;

use anyhow::Context;
use relm4::loading_widgets::LoadingWidgets;
use std::path::Path;
use users::{get_current_uid, get_user_by_uid};

use crate::ui::window::AppMsg;
use rand;
use rand::prelude::*;
use relm4::{adw::prelude::*, gtk, gtk::gio::Settings, prelude::*, view};
use relm4_components::open_dialog::*;
use std::path::PathBuf;

// default base path for system wallpapers
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

// #[tracker::track]
#[derive(Debug)]
pub struct AppearanceModel {
    pub style: AppearanceStyle,
    accent_color: AccentColorWrapped,
    pub wallpaper_default: String,
    pub wallpaper_dark: String,
    pub wallpapers: AsyncFactoryVecDeque<Background>,
    pub recent_wallpapers: AsyncFactoryVecDeque<Background>,
    open_dialog: Controller<OpenDialog>,
    pub background_group: gtk::ToggleButton,
    accent_box_group: gtk::ToggleButton,
    accent_colors: FactoryVecDeque<AccentColorModel>,
}

#[derive(Debug)]
pub enum AppearanceMsg {
    SetStyle(AppearanceStyle),
    SetBackground(String),
    RemoveBackground(DynamicIndex, String),
    SendPick(AccentColorWrapped),
    OpenRequest,
    OpenResponse(PathBuf),
    Ignore,
    // Default(String, Option<String>),
    // Local(String, Option<String>),
    // WallpapersLoaded(Background),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppearanceStyle {
    Default,
    Dark,
}

#[derive(Debug)]
pub enum AddBackroundMsg {
    Default(String, Option<String>),
    Local(String, Option<String>),
}

#[relm4::component(pub, async)]
impl AsyncComponent for AppearanceModel {
    type Init = ();
    type Input = AppearanceMsg;
    type Output = AppMsg;
    type CommandOutput = AddBackroundMsg;

    view! {
        #[root]
        adw::BreakpointBin {
            set_width_request: 346,
            set_height_request: 200,

            add_breakpoint = adw::Breakpoint::new(
                adw::BreakpointCondition::new_length(
                    adw::BreakpointConditionLengthType::MaxWidth,
                    500.0,
                    adw::LengthUnit::Px,
                )
            ) {
               add_setters: &[
                    (recent_wallpaper_box, "min_children_per_line", &2),
                    (recent_wallpaper_box, "max_children_per_line", &2)
                ],

                add_setters: &[
                    (wallpaper_box, "min_children_per_line", &2),
                    (wallpaper_box, "max_children_per_line", &2)
                ],

                add_setters: &[
                    (&style_box, "margin_bottom", &6),
                    (&style_box, "margin_start", &6),
                    (&style_box, "margin_end", &6),
                ],

                add_setters: &[
                    (accent_color_box, "margin_top", &6),
                    (accent_color_box, "margin_bottom", &6),
                    (accent_color_box, "margin_start", &6),
                    (accent_color_box, "margin_end", &6),
                    (accent_color_box, "spacing", &6),
                ],

                add_setters: &[
                    (recent_wallpaper_box, "margin_all", &6),
                    (wallpaper_box, "margin_all", &6),
                ],

                add_setters: &[
                    (&default_style, "height_request", &100),
                    (&dark_style, "heigh_request", &100),
                ],

                add_setters:  &[
                    (&style_box, "maximum_size", &270)
                ],
            },

            add_breakpoint = adw::Breakpoint::new(
                adw::BreakpointCondition::new_length(
                    adw::BreakpointConditionLengthType::MinWidth,
                    500.0,
                    adw::LengthUnit::Px,
                )
            ) {
               add_setters: &[
                    (recent_wallpaper_box, "min_children_per_line", &3),
                    (recent_wallpaper_box, "max_children_per_line", &3)
                ],

                add_setters: &[
                    (wallpaper_box, "min_children_per_line", &3),
                    (wallpaper_box, "max_children_per_line", &3)
                ],

                add_setters: &[
                    (&style_box, "margin_bottom", &12),
                    (&style_box, "margin_start", &12),
                    (&style_box, "margin_end", &12),
                ],

                add_setters: &[
                    (accent_color_box, "margin_top", &12),
                    (accent_color_box, "margin_bottom", &12),
                    (accent_color_box, "margin_start", &12),
                    (accent_color_box, "margin_end", &12),
                    (accent_color_box, "spacing", &12),
                ],

                add_setters: &[
                    (recent_wallpaper_box, "margin_all", &12),
                    (wallpaper_box, "margin_all", &12),
                ],

                add_setters: &[
                    (&default_style, "height_request", &140),
                    (&dark_style, "heigh_request", &140),],

                add_setters:  &[
                    (&style_box, "maximum_size", &380)
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
                                set_accessible_role: gtk::AccessibleRole::Group,
                                set_activatable: false,
                                set_focusable: false,
                                #[wrap(Some)]
                                #[name = "style_box"]
                                set_child = &adw::Clamp {
                                    set_maximum_size: 380,

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
                                            // set_height_request: 140,

                                            #[wrap(Some)]
                                            set_child = &gtk::Picture{
                                                set_content_fit: gtk::ContentFit::Cover,
                                                set_isolate_contents: true,
                                                #[watch]
                                                set_filename: Some(&model.wallpaper_default)
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
                                            // set_height_request: 140,

                                            #[wrap(Some)]
                                            set_child = &gtk::Picture{
                                                set_content_fit: gtk::ContentFit::Cover,
                                                set_isolate_contents: true,
                                                #[watch]
                                                set_filename: Some(&model.wallpaper_dark)
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

                                // #[name = "accent_box"]
                                #[local_ref]
                                #[wrap(Some)]
                                set_child = accent_color_box -> gtk::Box {
                                    set_orientation: gtk::Orientation::Horizontal,
                                    set_spacing: 6,
                                    set_margin_all: 6,

                                    #[name="accent_color"]
                                    gtk::ToggleButton{
                                        set_visible: false
                                    }
                                },
                            },
                        },
                        adw::PreferencesGroup {
                            set_title: "Background",
                            set_accessible_role: gtk::AccessibleRole::Group,
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

                                    #[name="recent_box"]
                                    gtk::Box {
                                        set_orientation: gtk::Orientation::Vertical,
                                        set_halign: gtk::Align::Center,

                                        #[local_ref]
                                        recent_wallpaper_box -> gtk::FlowBox {
                                            add_css_class: "background-flowbox",
                                            set_orientation: gtk::Orientation::Horizontal,
                                            set_margin_all: 6,
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
                                            #[watch]
                                            set_visible: !model.recent_wallpapers.is_empty()
                                        }
                                    },
                                    #[local_ref]
                                    wallpaper_box -> gtk::FlowBox {
                                        add_css_class: "background-flowbox",
                                        set_margin_all: 6,
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
                                            set_visible: false,
                                        },
                                    },
                                },
                            },
                        },
                    },
                },
            },
        },
    }

    fn init_loading_widgets(root: Self::Root) -> Option<LoadingWidgets> {
        view! {
            #[local]
            root {
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
        let open_dialog = OpenDialog::builder()
            .transient_for_native(&root)
            .launch(OpenDialogSettings::default())
            .forward(sender.input_sender(), |response| match response {
                OpenDialogResponse::Accept(path) => AppearanceMsg::OpenResponse(path),
                OpenDialogResponse::Cancel => AppearanceMsg::Ignore,
            });
        let accent_colors = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .forward(sender.input_sender(), |output| match output {
                AccentColorOutput::SendPick(accent_color_wrapped) => {
                    AppearanceMsg::SendPick(accent_color_wrapped)
                }
            });

        let create_wallpaper_box = || {
            AsyncFactoryVecDeque::<Background>::builder()
                .launch(gtk::FlowBox::default())
                .forward(sender.input_sender(), |output| match output {
                    BackgroundOutput::SetBackground(path) => {
                        AppearanceMsg::SetBackground(path.clone())
                    }
                    BackgroundOutput::RemoveBackground(index, path) => {
                        AppearanceMsg::RemoveBackground(index, path)
                    }
                })
        };

        let settings = AppearanceSettings::new();

        let style = match settings.interface.string("color-scheme").as_str() {
            "prefer-dark" => AppearanceStyle::Dark,
            _ => AppearanceStyle::Default,
        };

        let wallpaper_default = parse_dconf(settings.background.string("picture-uri").to_string());
        let wallpaper_dark =
            parse_dconf(settings.background.string("picture-uri-dark").to_string());

        let mut model = Self {
            accent_color: AccentColorWrapped::from(
                settings.interface.get::<String>("accent-color"),
            ),
            wallpaper_default,
            wallpaper_dark,
            wallpapers: create_wallpaper_box(),
            recent_wallpapers: create_wallpaper_box(),
            background_group: gtk::ToggleButton::new(),
            accent_box_group: gtk::ToggleButton::new(),
            style,
            accent_colors,
            open_dialog,
        };

        // push colors to accent color component
        let _ = AccentColorWrapped::iterator()
            .map(|x| {
                let x_string: String = x.clone().into();
                model.accent_colors.guard().push_back(AccentColorModel {
                    is_active: String::from(model.accent_color.clone()) == x_string,
                    group: model.accent_box_group.clone(),
                    accent_color: x.clone(),
                    color: x_string,
                })
            })
            .collect::<Vec<_>>();

        // Load Local Wallpapers
        if let Some(user) = get_user_by_uid(get_current_uid()) {
            let local_path: String = format!(
                "/home/{}/.local/share/backgrounds",
                user.name().to_string_lossy()
            );
            add_wallpaper(Path::new(&local_path).to_path_buf(), &mut model, true);
        }

        // default paths for system wallpapers
        for folder in vec!["gnome", "nixos"] {
            let path: PathBuf = Path::new(BG_BASE_DIR).join(folder);
            add_wallpaper(path, &mut model, false);
        }

        let wallpaper_box = model.wallpapers.widget();
        let recent_wallpaper_box = model.recent_wallpapers.widget();
        let accent_color_box = model.accent_colors.widget();

        let widgets = view_output!();
        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        msg: Self::Input,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        let settings = AppearanceSettings::new();
        let user = get_user_by_uid(get_current_uid()).unwrap();

        match msg {
            AppearanceMsg::OpenRequest => self.open_dialog.emit(OpenDialogMsg::Open),
            AppearanceMsg::OpenResponse(path) => {
                // let f_name = path.file_name().context("extract filename failed");

                let haha = rand::rng()
                    .random_iter::<char>()
                    .take(16)
                    .collect::<String>();

                // add local wallpaper
                let dest = PathBuf::from("/home")
                    .join(user.name())
                    .join(".local/share/backgrounds")
                    .join(haha);

                let file_path = dest.to_string_lossy().to_string();
                match std::fs::copy(&path, &dest) {
                    Ok(_) => {
                        self.recent_wallpapers.guard().push_back(Background {
                            path: file_path.clone(),
                            group: self.background_group.clone(),
                            active: false,
                            thumb: thumb(&dest).unwrap_or(file_path),
                        });
                    }
                    Err(e) => eprintln!("{e:?}"),
                };
            }
            AppearanceMsg::SetStyle(style) => {
                self.style = style;

                match style {
                    AppearanceStyle::Dark => {
                        settings
                            .interface
                            .set("color-scheme", "prefer-dark")
                            .unwrap_or_else(|e| eprintln!("Couldn't set color-scheme: {e:?}"));
                    }

                    AppearanceStyle::Default => {
                        settings
                            .interface
                            .set("color-scheme", "default")
                            .unwrap_or_else(|e| eprintln!("Couldn't set color-scheme: {e:?}"));
                    }
                }
            }

            AppearanceMsg::SetBackground(path) => {
                let _ = settings.background.set(
                    match settings.interface.string("color-scheme").as_str() {
                        "prefer-dark" => {
                            self.wallpaper_dark = path.clone();
                            "picture-uri-dark"
                        }
                        _ => {
                            self.wallpaper_default = path.clone();
                            "picture-uri"
                        }
                    },
                    format!("file://{}", path),
                );
            }

            AppearanceMsg::RemoveBackground(index, path) => {
                self.recent_wallpapers.guard().remove(index.current_index());

                let set_wallpaper = |x: &Background| {
                    sender
                        .input_sender()
                        .emit(AppearanceMsg::SetBackground(x.path.clone()))
                };

                if self.recent_wallpapers.is_empty()
                    && let Some(x) = self.wallpapers.get(0)
                {
                    set_wallpaper(x)
                } else if (path == self.wallpaper_default || path == self.wallpaper_dark)
                    && let Some(x) = self.recent_wallpapers.get(self.recent_wallpapers.len() - 1)
                {
                    set_wallpaper(x)
                }

                relm4::spawn(async move { std::fs::remove_file(path) });
            }
            AppearanceMsg::SendPick(color) => {
                settings
                    .interface
                    .set("accent-color", format!("{:?}", color.0).to_lowercase())
                    .unwrap_or_else(|e| eprintln!("Couldn't set accent-color: {e:?}"));
            }
            AppearanceMsg::Ignore => {}
        }
    }
}
