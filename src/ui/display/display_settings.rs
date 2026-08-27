use std::{cmp::Ordering, fmt::Display};

use gettextrs::{dgettext, gettext};
use relm4::{adw::prelude::*, prelude::*};
use relm4_components::simple_adw_combo_row::{SimpleComboRow, SimpleComboRowMsg};
use struct_patch::Patch;

use super::color_mode::ColorMode;
use super::display_mode::DisplayMode;
use super::display_mode::RefreshRateMode;
use super::monitor_spec::MonitorSpec;
use super::monitor::Geometry;
use super::monitor::Monitor;
use super::transform::Transform;
use super::config::CcDisplayMonitor;

const MAX_SCALE_BUTTONS: usize = 5;
const ROTATIONS: [Transform; 4] = [
    Transform::Normal,
    Transform::Rotate90,
    Transform::Rotate180,
    Transform::Rotate270,
];

#[derive(Debug)]
pub enum Aspect {
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

#[derive(Debug, Clone, PartialEq)]
struct Resolution(i32, i32);

impl From<DisplayMode> for Resolution {
    fn from(value: DisplayMode) -> Self {
        Self(value.width, value.height)
    }
}

impl PartialEq<DisplayMode> for Resolution {
    fn eq(&self, other: &DisplayMode) -> bool {
        self.0 == other.width && self.1 == other.height
    }
}

impl Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(aspect) = self.get_aspect_string() {
            return f.write_fmt(format_args!("{} × {} ({})", self.0, self.1, aspect));
        }

        f.write_fmt(format_args!("{} × {}", self.0, self.1))
    }
}

impl Resolution {
    fn get_aspect_string(&self) -> Option<String> {
        let ratio = if self.0 > self.1 {
            self.0 * 10 / self.1
        } else {
            self.1 * 10 / self.0
        };

        match ratio {
            10 => Some("1:1".to_string()),
            12 => Some("5:4".to_string()),
            13 => Some("4:3".to_string()),
            15 => Some("3:2".to_string()),
            16 => Some("16:10".to_string()),
            17 => Some("16:9".to_string()),
            18 => Some("9:5".to_string()),
            23 => Some("21:9".to_string()),
            35 => Some("32:9".to_string()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VariableRefreshRate(bool);

#[derive(Debug, Clone, Copy)]
pub struct RefreshRate(f64);

impl PartialEq<RefreshRate> for RefreshRate {
    fn eq(&self, other: &RefreshRate) -> bool {
        approx::relative_eq!(self.0, other.0, max_relative = 0.01)
    }
}

impl PartialEq<f64> for RefreshRate {
    fn eq(&self, other: &f64) -> bool {
        approx::relative_eq!(self.0, other, max_relative = 0.01)
    }
}

impl Display for RefreshRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:.2} Hz", self.0))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Scale(pub f64);

impl Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:.0} %", self.0 * 100.0))
    }
}

impl PartialEq<Scale> for Scale {
    fn eq(&self, other: &Scale) -> bool {
        approx::relative_eq!(self.0, other.0, max_relative = 0.01)
    }
}

impl TryFrom<&str> for Scale {
    type Error = std::num::ParseFloatError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.parse()?;

        Ok(Self(value))
    }
}

#[derive(Debug, Clone, PartialEq, Patch)]
#[patch(attribute(derive(Debug, Default, Clone)))]
pub struct DisplaySettings {
    pub scale: Scale,
    pub refresh_rate: RefreshRate,
    pub current_mode: DisplayMode,
    #[patch(skip_wrap)]
    pub hdr: Option<bool>,
    #[patch(skip_wrap)]
    pub enabled: Option<bool>,
    #[patch(skip_wrap)]
    pub underscanning: Option<bool>,
    #[patch(skip_wrap)]
    pub orientation: Option<Transform>,
    #[patch(skip_wrap)]
    pub auto_orientation: Option<bool>,
    #[patch(skip_wrap)]
    pub variable_refresh_rate: Option<VariableRefreshRate>,
}

#[derive(Debug)]
pub struct DisplaySettingsModel {
    pub monitor: CcDisplayMonitor,
    pub settings: DisplaySettings,

    scale_list: Vec<Scale>,
    resolution_list: Vec<Resolution>,
    orientation_list: Vec<Orientation>,
    refresh_rate_list: Vec<RefreshRate>,

    scale_combo_row: Controller<SimpleComboRow<Scale>>,
    resolution_row: Controller<SimpleComboRow<Resolution>>,
    orientation_row: Controller<SimpleComboRow<Orientation>>,
    refresh_rate_row: Controller<SimpleComboRow<RefreshRate>>,
}

struct DerivedDisplaySettings {
    monitor: CcDisplayMonitor,
    settings: DisplaySettings,
    scale_list: Vec<Scale>,
    resolution_list: Vec<Resolution>,
    orientation_list: Vec<Orientation>,
    refresh_rate_list: Vec<RefreshRate>,
}

impl DisplaySettingsModel {
    fn derive(init: &DisplaySettingsInit) -> DerivedDisplaySettings {
        let monitor = &init.monitor;
        let is_mirrored = init.is_mirrored;

        let current_mode = monitor.get_optimal_mode();

        let enabled = if is_mirrored {
            None
        } else {
            Some(monitor.is_active())
        };

        let orientation = monitor.get_rotation();
        let orientation = if orientation == Transform::Normal {
            None
        } else {
            Some(orientation)
        };
        let scale = Scale(monitor.get_scale());
        let refresh_rate = RefreshRate(current_mode.refresh_rate);

        let resolution_list = init.get_resolutions();
        let orientation_list = init.get_orientations();
        let scale_list = Self::get_scales(current_mode);
        let refresh_rate_list = Self::get_refresh_rates(&monitor.inner, current_mode);

        let hdr = if monitor.supports_color_mode(ColorMode::BT2100) {
            Some(monitor.get_color_mode() == ColorMode::BT2100)
        } else {
            None
        };

        let settings = DisplaySettings {
            hdr,
            scale,
            enabled,
            orientation,
            refresh_rate,
            // TODO
            auto_orientation: None,
            // TODO
            variable_refresh_rate: None,
            current_mode: current_mode.clone(),
            underscanning: monitor.inner.properties.is_underscanning,
        };

        DerivedDisplaySettings {
            monitor: monitor.clone(),
            settings,
            scale_list,
            resolution_list,
            orientation_list,
            refresh_rate_list,
        }
    }
}

impl From<(DisplaySettingsInit, ComponentSender<Self>)> for DisplaySettingsModel {
    fn from((init, sender): (DisplaySettingsInit, ComponentSender<Self>)) -> Self {
        let DerivedDisplaySettings {
            monitor,
            settings,
            scale_list,
            resolution_list,
            orientation_list,
            refresh_rate_list,
        } = Self::derive(&init);

        let current_mode = &settings.current_mode;
        let scale = &settings.scale;
        let refresh_rate = &settings.refresh_rate;
        let orientation = settings.orientation.as_ref();

        Self {
            scale_combo_row: Self::build_scale_combo_row(scale, &scale_list, sender.clone()),
            orientation_row: Self::build_orientation_row(
                orientation,
                &orientation_list,
                sender.clone(),
            ),
            resolution_row: Self::build_resolution_row(
                current_mode,
                &resolution_list,
                sender.clone(),
            ),
            refresh_rate_row: Self::build_refresh_rate_row(
                refresh_rate,
                &refresh_rate_list,
                sender.clone(),
            ),

            monitor,
            scale_list,
            resolution_list,
            orientation_list,
            refresh_rate_list,
            settings,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DisplaySettingsInit {
    pub monitor: CcDisplayMonitor,
    pub is_mirrored: bool,
}

#[derive(Debug)]
pub enum DisplaySettingsMsg {
    UpdateSettings(DisplaySettingsPatch),

    Update(DisplaySettingsInit),

    SelectScale(usize),
    SelectResolution(usize),
    SelectRefreshRate(usize),
    SelectOrientation(usize),
}

#[derive(Debug)]
pub enum DisplaySettingsOutput {
    Changed(MonitorSpec, DisplaySettings),
}

#[relm4::component(pub)]
impl Component for DisplaySettingsModel {
    type Init = DisplaySettingsInit;
    type Input = DisplaySettingsMsg;
    type Output = DisplaySettingsOutput;
    type CommandOutput = ();

    view! {
        #[root]
        gtk::Box {
            set_spacing: 18,
            set_orientation: gtk::Orientation::Vertical,

            #[name(enabled_listbox)]
            gtk::ListBox {
                #[watch]
                set_visible: model.settings.enabled.is_some(),

                set_hexpand: true,
                add_css_class: "boxed-list",
                set_selection_mode: gtk::SelectionMode::None,

                #[name(enabled_row)]
                adw::SwitchRow {
                    #[watch]
                    #[block_signal(enabled_handler)]
                    set_active: model.settings.enabled.as_ref().is_some_and(|x| *x),

                    set_title: &model.monitor.get_output_ui_name(),

                    connect_active_notify[sender] => move |switch_row| {
                        sender.input(DisplaySettingsMsg::UpdateSettings(DisplaySettingsPatch { enabled: Some(switch_row.is_active()), ..Default::default() }));
                    } @enabled_handler
                }
            },

            #[name(listbox)]
            gtk::ListBox {
                set_hexpand: true,
                add_css_class: "boxed-list",
                set_selection_mode: gtk::SelectionMode::None,

                #[watch]
                set_sensitive: model.settings.enabled.as_ref().is_none_or(|x| *x),

                #[name(auto_orientation_row)]
                adw::SwitchRow {
                    #[watch]
                    set_visible: model.settings.auto_orientation.as_ref().is_some(),
                    #[watch]
                    #[block_signal(auto_orientation_handler)]
                    set_active: model.settings.auto_orientation.as_ref().is_some_and(|x| *x),

                    set_width_request: 100,
                    set_use_underline: true,
                    set_title: &dgettext("display setting", "_Auto Rotate"),

                    connect_active_notify[sender] => move |switch_row| {
                        sender.input(DisplaySettingsMsg::UpdateSettings(DisplaySettingsPatch {  auto_orientation: Some(switch_row.is_active()), ..Default::default() }));
                    } @auto_orientation_handler
                },

                model.orientation_row.widget().to_owned() -> adw::ComboRow {
                   #[watch]
                   set_sensitive: model.settings.auto_orientation.is_none(),

                   set_width_request: 100,
                   set_use_underline: true,
                   set_title: &dgettext("display setting", "_Orientation"),
                },

                model.resolution_row.widget().to_owned() -> adw::ComboRow {
                    set_width_request: 100,
                    set_use_underline: true,
                    set_title: &dgettext("display setting", "_Resolution")
                },

                model.refresh_rate_row.widget().to_owned() -> adw::ComboRow {
                    set_width_request: 100,
                    set_use_underline: true,
                    set_title: &dgettext("display setting", "R_efresh Rate")
                },

                #[name(refresh_rate_expander_row)]
                adw::ExpanderRow {
                    #[watch]
                    set_visible: model.settings.variable_refresh_rate.is_some(),

                    set_width_request: 100,
                    set_use_underline: true,
                    set_title: &dgettext("display setting", "R_efresh Rate"),

                    #[name(refresh_rate_expander_suffix_label)]
                    add_suffix = &gtk::Label {},

                    #[name(variable_refresh_rate_row)]
                    add_row = &adw::SwitchRow {
                        #[watch]
                        #[block_signal(variable_refresh_rate_handler)]
                        set_active: model.settings.variable_refresh_rate.as_ref().is_some_and(|x| x.0),

                        set_width_request: 100,
                        set_use_underline: true,
                        set_title: &gettext("_Variable Refresh Rate"),

                        connect_activated[sender] => move |switch_row| {
                            sender.input(DisplaySettingsMsg::UpdateSettings(DisplaySettingsPatch { variable_refresh_rate: Some(VariableRefreshRate(switch_row.is_active())), ..Default::default() }));
                        } @variable_refresh_rate_handler
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
                    #[watch]
                    set_visible: model.settings.hdr.is_some(),

                    #[watch]
                    #[block_signal(hdr_handler)]
                    set_active: model.settings.hdr.is_some_and(|x| x),

                    set_width_request: 100,
                    set_use_underline: true,
                    set_title: &gettext("_HDR (High Dynamic Range)"),

                    connect_active_notify[sender] => move |hdr| {
                        sender.input(DisplaySettingsMsg::UpdateSettings(DisplaySettingsPatch { hdr: Some(hdr.is_active()), ..Default::default() }));
                    } @hdr_handler
                },

                #[name(underscanning_row)]
                adw::SwitchRow {
                    #[watch]
                    set_visible: model.settings.underscanning.is_some(),

                    #[watch]
                    #[block_signal(underscanning_handler)]
                    set_active: model.settings.underscanning.as_ref().is_some_and(|x| *x),

                    set_width_request: 100,
                    set_use_underline: true,
                    set_title: &gettext("Adjust for _TV"),

                    connect_activated[sender] => move |underscanning| {
                        sender.input(DisplaySettingsMsg::UpdateSettings(DisplaySettingsPatch { underscanning: Some(underscanning.is_active()), ..Default::default() }));
                    } @underscanning_handler
                },

                #[name(scale_buttons_row)]
                adw::ActionRow {
                    #[watch]
                    set_visible: model.scale_list.len() <= MAX_SCALE_BUTTONS,

                    set_width_request: 100,
                    set_use_underline: true,
                    set_title: &dgettext("display settings", "_Scale"),

                    #[name(scale_buttons)]
                    add_suffix = &adw::ToggleGroup {
                        set_homogeneous: true,
                        set_valign: gtk::Align::Center,

                        connect_active_name_notify[sender] => move |scale_toggle| {
                            if let Some(active_name) = scale_toggle.active_name()
                                && let Ok(scale) = Scale::try_from(active_name.as_str()) {
                                    sender.input(DisplaySettingsMsg::UpdateSettings(DisplaySettingsPatch { scale: Some(scale), ..Default::default() }));
                                }
                        }
                    },
                },

                model.scale_combo_row.widget().to_owned() -> adw::ComboRow {
                    #[watch]
                    set_visible: model.scale_list.len() > MAX_SCALE_BUTTONS,

                    set_width_request: 100,
                    set_use_underline: true,
                    set_title: &dgettext("display settings", "_Scale"),
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = DisplaySettingsModel::from((init, sender.clone()));
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            DisplaySettingsMsg::UpdateSettings(settings) => {
                self.settings.apply(settings);
            }
            DisplaySettingsMsg::Update(init) => {
                let d = Self::derive(&init);

                self.scale_combo_row
                    .sender()
                    .emit(SimpleComboRowMsg::UpdateData(SimpleComboRow {
                        variants: d.scale_list.clone(),
                        active_index: d.scale_list.iter().position(|v| v == &d.settings.scale),
                    }));
                self.resolution_row
                    .sender()
                    .emit(SimpleComboRowMsg::UpdateData(SimpleComboRow {
                        variants: d.resolution_list.clone(),
                        active_index: d
                            .resolution_list
                            .iter()
                            .position(|v| v == &d.settings.current_mode),
                    }));
                self.refresh_rate_row
                    .sender()
                    .emit(SimpleComboRowMsg::UpdateData(SimpleComboRow {
                        variants: d.refresh_rate_list.clone(),
                        active_index: d
                            .refresh_rate_list
                            .iter()
                            .position(|v| v == &d.settings.refresh_rate),
                    }));
                self.orientation_row
                    .sender()
                    .emit(SimpleComboRowMsg::UpdateData(SimpleComboRow {
                        variants: d.orientation_list.clone(),
                        active_index: d
                            .orientation_list
                            .iter()
                            .position(|v| d.settings.orientation.is_some_and(|c| c == v.rotation)),
                    }));

                self.monitor = d.monitor;
                self.scale_list = d.scale_list;
                self.resolution_list = d.resolution_list;
                self.orientation_list = d.orientation_list;
                self.refresh_rate_list = d.refresh_rate_list;
                self.settings = d.settings;
            }
            DisplaySettingsMsg::SelectScale(index) => {
                self.settings.apply(DisplaySettingsPatch {
                    scale: Some(self.scale_list[index]),
                    ..Default::default()
                });
            }
            DisplaySettingsMsg::SelectResolution(index) => {
                let current_mode = self
                    .monitor
                    .inner
                    .modes
                    .iter()
                    .find(|m| self.resolution_list[index] == **m)
                    .unwrap();

                let scale = Scale(current_mode.preferred_scale);
                let refresh_rate = RefreshRate(current_mode.refresh_rate);

                self.scale_list = Self::get_scales(current_mode);
                self.scale_combo_row =
                    Self::build_scale_combo_row(&scale, &self.scale_list, sender.clone());

                self.refresh_rate_list = Self::get_refresh_rates(&self.monitor.inner, current_mode);
                self.refresh_rate_row = Self::build_refresh_rate_row(
                    &refresh_rate,
                    &self.refresh_rate_list,
                    sender.clone(),
                );

                self.settings.apply(DisplaySettingsPatch {
                    scale: Some(scale),
                    refresh_rate: Some(refresh_rate),
                    current_mode: Some(current_mode.clone()),
                    ..Default::default()
                });
            }
            DisplaySettingsMsg::SelectRefreshRate(index) => {
                self.settings.apply(DisplaySettingsPatch {
                    refresh_rate: Some(self.refresh_rate_list[index]),
                    ..Default::default()
                })
            }
            DisplaySettingsMsg::SelectOrientation(index) => {
                self.settings.apply(DisplaySettingsPatch {
                    orientation: Some(self.orientation_list[index].rotation),
                    ..Default::default()
                });
            }
        };

        let cmd = DisplaySettingsOutput::Changed(self.monitor.inner.spec.clone(), self.settings.clone());

        if let Err(command) = sender.output(cmd) {
            tracing::error!("Error sending output command: {:?}", command);
        }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        self.update(message, sender.clone(), root);
        self.update_view(widgets, sender);
        self.render_ui(widgets);
    }
}

impl DisplaySettingsModel {
    fn render_ui(&self, widgets: &mut <Self as Component>::Widgets) {
        widgets.scale_buttons.remove_all();
        for scale in &self.scale_list {
            let toggle = adw::Toggle::builder()
                .name(scale.to_string())
                .label(format!("{}", scale))
                .build();
            widgets.scale_buttons.add(toggle);
        }
        widgets
            .scale_buttons
            .set_active_name(Some(&self.settings.scale.to_string()));
    }

    fn get_scales(current_mode: &DisplayMode) -> Vec<Scale> {
        current_mode
            .supported_scales
            .iter()
            .map(|s| Scale(*s))
            .collect()
    }

    fn get_refresh_rates(monitor: &Monitor, current_mode: &DisplayMode) -> Vec<RefreshRate> {
        let mut seen: Vec<&DisplayMode> = vec![];

        for mode in &monitor.modes {
            if current_mode.width != mode.width || current_mode.height != mode.height {
                continue;
            }

            if current_mode.get_refresh_rate_mode() != mode.get_refresh_rate_mode() {
                continue;
            }

            seen.push(mode);
        }

        seen.sort_by(|a, b| {
            if a.get_refresh_rate_mode() != b.get_refresh_rate_mode() {
                if a.get_refresh_rate_mode() == RefreshRateMode::Variable {
                    return Ordering::Less;
                } else {
                    return Ordering::Greater;
                }
            }

            let delta = (b.refresh_rate - a.refresh_rate) * 1000.0;

            if delta > 0.0 {
                Ordering::Greater
            } else if delta < 0.0 {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
        seen.dedup_by(|a, b| RefreshRate(a.refresh_rate) == b.refresh_rate);
        seen.iter().map(|s| RefreshRate(s.refresh_rate)).collect()
    }

    fn build_orientation_row(
        orientation: Option<&Transform>,
        orientation_list: &[Orientation],
        sender: ComponentSender<Self>,
    ) -> Controller<SimpleComboRow<Orientation>> {
        let variants = orientation_list.to_owned();
        let active_index = variants
            .iter()
            .position(|v| orientation.is_some_and(|c| *c == v.rotation));

        SimpleComboRow::builder()
            .launch(SimpleComboRow {
                variants,
                active_index,
            })
            .forward(sender.input_sender(), DisplaySettingsMsg::SelectOrientation)
    }

    fn build_resolution_row(
        current_mode: &DisplayMode,
        resolution_list: &[Resolution],
        sender: ComponentSender<Self>,
    ) -> Controller<SimpleComboRow<Resolution>> {
        let variants = resolution_list.to_owned();
        let active_index = variants.iter().position(|v| v == current_mode);

        SimpleComboRow::builder()
            .launch(SimpleComboRow {
                variants,
                active_index,
            })
            .forward(sender.input_sender(), DisplaySettingsMsg::SelectResolution)
    }

    fn build_scale_combo_row(
        scale: &Scale,
        scale_list: &[Scale],
        sender: ComponentSender<Self>,
    ) -> Controller<SimpleComboRow<Scale>> {
        let variants = scale_list.to_owned();
        let active_index = variants.iter().position(|v| v == scale);

        SimpleComboRow::builder()
            .launch(SimpleComboRow {
                variants,
                active_index,
            })
            .forward(sender.input_sender(), DisplaySettingsMsg::SelectScale)
    }

    fn build_refresh_rate_row(
        refresh_rate: &RefreshRate,
        refresh_rate_list: &[RefreshRate],
        sender: ComponentSender<Self>,
    ) -> Controller<SimpleComboRow<RefreshRate>> {
        let variants = refresh_rate_list.to_owned();
        let active_index = variants.iter().position(|v| v == refresh_rate);

        SimpleComboRow::builder()
            .launch(SimpleComboRow {
                variants,
                active_index,
            })
            .forward(sender.input_sender(), DisplaySettingsMsg::SelectRefreshRate)
    }
}

impl DisplaySettingsInit {
    fn get_orientations(&self) -> Vec<Orientation> {
        let Geometry { width, height, .. } = self.monitor.get_geometry();

        let aspect = if width > height {
            Aspect::Landscape
        } else if width < height {
            Aspect::Portrait
        } else {
            Aspect::Square
        };

        ROTATIONS
            .map(|rotation| -> Orientation {
                let label = match aspect {
                    Aspect::Landscape => match rotation {
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
                    Aspect::Portrait => match rotation {
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
                    Aspect::Square => match rotation {
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

        for m in &self.monitor.inner.modes {
            let res = Resolution(m.width, m.height);

            if !seen.contains(&res) {
                seen.push(res);
            }
        }

        seen.sort_by_key(|Resolution(w, h)| std::cmp::Reverse(w * h));
        seen
    }
}
