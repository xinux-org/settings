use std::fmt::Display;

use gettextrs::{dgettext, gettext};
use relm4::{SimpleComponent, adw::prelude::*, prelude::*};
use relm4_components::simple_adw_combo_row::SimpleComboRow;

use super::{logical_monitor::LogicalMonitor, monitor::Geometry};
use super::{monitor::Monitor, transform::Transform};

const ROTATIONS: [Transform; 4] = [
    Transform::Normal,
    Transform::Rotate90,
    Transform::Rotate180,
    Transform::Rotate270,
];

#[derive(Debug)]
pub enum AspectRatio {
    Square,
    Portrait,
    Landscape,
}

#[derive(Debug, Clone)]
struct Orientation {
    label: String,
    rotation: Transform,
}

impl Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.label))
    }
}

#[derive(Debug, PartialEq)]
struct Resolution(i32, i32);

impl Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} × {}", self.0, self.1))
    }
}

#[derive(Debug)]
pub struct DisplaySettings {
    pub hdr: bool,
    pub scale: u64,
    pub resolution: u64,
    pub monitor: Monitor,
    pub refresh_rate: u64,
    pub underscanning: bool,
    pub enabled: Option<bool>,
    pub orientation: Option<Transform>,
    pub auto_orientation: bool,
    pub variable_refresh_rate: u64,
    pub logical_monitor: Option<LogicalMonitor>,
}

impl From<DisplaySettingsInit> for DisplaySettings {
    fn from(init: DisplaySettingsInit) -> Self {
        let current_mode = init.monitor.get_current_mode();
        let logical_monitor = init.logical_monitor;

        Self {
            scale: 0,
            hdr: false,
            resolution: 0,
            refresh_rate: 0,
            underscanning: false,
            monitor: init.monitor,
            enabled: logical_monitor.as_ref().map(|_| true),
            orientation: logical_monitor.as_ref().map(|lm| lm.transform),
            auto_orientation: false,
            variable_refresh_rate: 0,
            logical_monitor,
            // TODO: agar monitor faqat bitta bo'lsa uni disable qila olmasligi kk
        }
    }
}

pub struct DisplaySettingsInit {
    pub monitor: Monitor,
    pub logical_monitor: Option<LogicalMonitor>,
}

#[derive(Debug)]
pub enum DisplaySettingsMsg {
    ToggleHdr(bool),
    ToggleEnabled(bool),
    ToggleUnderscanning(bool),
    ToggleAutoOrientation(bool),
    ToggleVariableRefreshRate(bool),

    SelectScale(u32),
    SelectRefreshRate(u32),
    SelectOrientation(u32),
    SelectResoulution(u32),
}

#[derive(Debug)]
pub enum DisplaySettingsOutput {
    Changed(),
}

#[relm4::component(pub)]
impl SimpleComponent for DisplaySettings {
    type Init = DisplaySettingsInit;
    type Input = DisplaySettingsMsg;
    type Output = DisplaySettingsOutput;

    view! {
        #[root]
        gtk::Box {
            set_spacing: 18,
            set_orientation: gtk::Orientation::Vertical,

            #[name(enabled_listbox)]
            gtk::ListBox {
                set_hexpand: true,
                add_css_class: "boxed-list",
                set_selection_mode: gtk::SelectionMode::None,

                #[name(enabled_row)]
                adw::SwitchRow {
                    #[watch]
                    #[block_signal(enabled_handler)]
                    set_active: model.enabled.as_ref().is_some_and(|x| *x),

                    set_visible: model.enabled.is_some(),
                    set_title: &model.monitor.get_output_ui_name(),
                    connect_active_notify[sender] => move |row| {
                        sender.input(DisplaySettingsMsg::ToggleEnabled(row.is_active()));
                    } @enabled_handler
                }
            },

            #[name(listbox)]
            gtk::ListBox {
                set_hexpand: true,
                add_css_class: "boxed-list",
                set_selection_mode: gtk::SelectionMode::None,

                #[watch]
                set_sensitive: model.enabled.as_ref().is_none_or(|x| *x),

                // TODO: hide unless the monitor has an accelerometer
                #[name(auto_orientation_row)]
                adw::SwitchRow {
                    #[watch]
                    #[block_signal(auto_orientation_handler)]
                    set_active: model.auto_orientation,

                    set_width_request: 100,
                    set_use_underline: true,
                    set_title: &dgettext("display setting", "_Auto Rotate"),
                    connect_active_notify[sender] => move |row| {
                        sender.input(DisplaySettingsMsg::ToggleAutoOrientation(row.is_active()))
                    } @auto_orientation_handler
                },

                orientation_row.widget().to_owned() -> adw::ComboRow {
                   set_width_request: 100,
                   set_use_underline: true,
                   #[watch]
                   set_sensitive: !model.auto_orientation,
                   set_title: &dgettext("display setting", "_Orientation"),
                },

                resolution_row.widget().to_owned() -> adw::ComboRow {
                    set_width_request: 100,
                    set_use_underline: true,
                    set_title: &dgettext("display setting", "_Resolution")
                },

                #[name(refresh_rate_row)]
                adw::ComboRow {
                    set_width_request: 100,
                    set_use_underline: true,
                    set_title: &dgettext("display setting", "R_efresh Rate")

                    // TODO: `notify::active => $on_refresh_rate_selection_changed_cb(template)`
                },

                #[name(refresh_rate_expander_row)]
                adw::ExpanderRow {
                    set_width_request: 100,
                    set_use_underline: true,
                    set_title: &dgettext("display setting", "R_efresh Rate"),

                    #[name(refresh_rate_expander_suffix_label)]
                    add_suffix = &gtk::Label {},

                    #[name(variable_refresh_rate_row)]
                    add_row = &adw::SwitchRow {
                        set_width_request: 100,
                        set_use_underline: true,
                        set_title: &gettext("_Variable Refresh Rate"),

                        // TODO: notify::active => $on_variable_refresh_rate_active_changed_cb(template);
                    },

                    #[name(preferred_refresh_rate_row)]
                    add_row = &adw::ComboRow {
                        set_width_request: 100,
                        set_use_underline: true,
                        set_title: &dgettext("display setting", "R_efresh Rate"),

                        // TODO: selected: bind refresh_rate_row.selected no-sync-create bidirectional
                    }
                },

                #[name(hdr_row)]
                adw::SwitchRow {
                    set_width_request: 100,
                    set_use_underline: true,
                    set_title: &gettext("_HDR (High Dynamic Range)")

                    // TODO: `notify::active => $on_hdr_row_active_changed_cb(template)`
                },

                #[name(underscanning_row)]
                adw::SwitchRow {
                    set_width_request: 100,
                    set_use_underline: true,
                    set_title: &gettext("Adjust for _TV")

                    // TODO: `notify::active => $on_underscanning_row_active_changed_cb(template)`
                },

                #[name(scale_buttons_row)]
                adw::SwitchRow {
                    set_width_request: 100,
                    set_use_underline: true,
                    set_title: &dgettext("display settings", "_Scale"),

                    #[wrap(Some)]
                    set_child = &adw::ToggleGroup {
                        set_homogeneous: true,
                        set_valign: gtk::Align::Center,

                        // TODO: `notify::active => $on_scale_btn_active_changed_cb(template)`
                    },
                },

                #[name(scale_combo_row)]
                adw::ComboRow {
                    set_width_request: 100,
                    set_use_underline: true,
                    set_title: &dgettext("display settings", "_Scale"),

                    // TODO: `notify::selected-item => $on_scale_selection_changed_cb(template)`
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = DisplaySettings::from(init);

        let orientation_row = {
            let variants = model.get_orientations();
            let active_index = variants
                .iter()
                .position(|v| model.orientation.as_ref().is_some_and(|c| *c == v.rotation));

            SimpleComboRow::builder()
                .launch(SimpleComboRow {
                    variants,
                    active_index,
                })
                .forward(sender.input_sender(), |orientation| {
                    DisplaySettingsMsg::SelectOrientation(orientation as u32)
                })
        };

        let resolution_row = {
            let variants = model.get_resolutions();

            SimpleComboRow::builder()
                .launch(SimpleComboRow {
                    variants,
                    active_index: None,
                })
                .forward(sender.input_sender(), |resolution| {
                    DisplaySettingsMsg::SelectResoulution(resolution as u32)
                })
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            DisplaySettingsMsg::ToggleHdr(_) => todo!(),
            DisplaySettingsMsg::ToggleEnabled(enabled) => self.enabled = Some(enabled),
            DisplaySettingsMsg::ToggleUnderscanning(_) => todo!(),
            DisplaySettingsMsg::ToggleAutoOrientation(_) => todo!(),
            DisplaySettingsMsg::ToggleVariableRefreshRate(_) => todo!(),
            DisplaySettingsMsg::SelectScale(_) => todo!(),
            DisplaySettingsMsg::SelectRefreshRate(_) => todo!(),
            DisplaySettingsMsg::SelectOrientation(_) => todo!(),
            DisplaySettingsMsg::SelectResoulution(_) => todo!(),
        };
    }
}

impl DisplaySettings {
    fn get_orientations(&self) -> Vec<Orientation> {
        eprintln!("Has logical monitor: {:?}", self.logical_monitor.is_some());

        let Geometry { width, height, .. } =
            self.monitor.get_geometry(self.logical_monitor.as_ref());

        let ratio = if width > height {
            AspectRatio::Landscape
        } else if width < height {
            AspectRatio::Portrait
        } else {
            AspectRatio::Square
        };

        ROTATIONS
            .map(|rotation| -> Orientation {
                let label = match ratio {
                    AspectRatio::Square => match rotation {
                        Transform::Normal | Transform::Flipped180 => {
                            dgettext("Display rotation", "Landscape")
                        }
                        Transform::Rotate90 | Transform::Flipped270 => {
                            dgettext("Display rotation", "Portrait Right")
                        }
                        Transform::Rotate270 | Transform::Flipped90 => {
                            dgettext("Display rotation", "Portrait Left")
                        }
                        Transform::Rotate180 | Transform::Flipped => {
                            dgettext("Display rotation", "Landscape (flipped)")
                        }
                    },
                    AspectRatio::Portrait => match rotation {
                        Transform::Normal | Transform::Flipped180 => {
                            dgettext("Display rotation", "Portrait")
                        }
                        Transform::Rotate90 | Transform::Flipped270 => {
                            dgettext("Display rotation", "Landscape Right")
                        }
                        Transform::Rotate270 | Transform::Flipped90 => {
                            dgettext("Display rotation", "Landscape Left")
                        }
                        Transform::Rotate180 | Transform::Flipped => {
                            dgettext("Display rotation", "Portrait (flipped)")
                        }
                    },
                    AspectRatio::Landscape => match rotation {
                        Transform::Normal | Transform::Flipped180 => {
                            dgettext("Display rotation", "Upright")
                        }
                        Transform::Rotate90 | Transform::Flipped270 => {
                            dgettext("Display rotation", "Right")
                        }
                        Transform::Rotate270 | Transform::Flipped90 => {
                            dgettext("Display rotation", "Left")
                        }
                        Transform::Rotate180 | Transform::Flipped => {
                            dgettext("Display rotation", "Flipped")
                        }
                    },
                };

                Orientation { label, rotation }
            })
            .to_vec()
    }

    fn get_resolutions(&self) -> Vec<Resolution> {
        let mut seen = vec![];

        for m in &self.monitor.modes {
            let res = Resolution(m.width, m.height);

            if seen.contains(&res) {
                seen.push(res);
            }
        }

        seen.sort_by_key(|Resolution(w, h)| std::cmp::Reverse(w * h));
        seen
    }
}
