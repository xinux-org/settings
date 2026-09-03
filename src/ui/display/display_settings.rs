use std::sync::Arc;

use gettextrs::{dgettext, gettext};
use relm4::{adw::prelude::*, prelude::*};
use relm4_components::simple_adw_combo_row::{SimpleComboRow, SimpleComboRowMsg};
use struct_patch::Patch;

use super::config::{CcDisplayMonitor, Orientation};
use super::{RefreshRate, Resolution, Scale};

#[derive(Debug, Clone, PartialEq, Patch)]
#[patch(attribute(derive(Debug, Default, Clone)))]
pub struct DisplaySettings {
    pub scale: Scale,
    pub resolution: Resolution,
    #[patch(skip_wrap)]
    pub hdr: Option<bool>,
    #[patch(skip_wrap)]
    pub underscanning: Option<bool>,
    #[patch(skip_wrap)]
    pub orientation: Option<Orientation>,
    #[patch(skip_wrap)]
    pub refresh_rate: Option<RefreshRate>,
}

impl From<&Arc<CcDisplayMonitor>> for DisplaySettings {
    fn from(value: &Arc<CcDisplayMonitor>) -> Self {
        let current_mode = value.get_current_mode();
        let scale = value
            .get_logical_monitor()
            .map(|lm| lm.get_scale())
            .unwrap_or_default();

        DisplaySettings {
            scale,
            hdr: value.is_hdr(),
            orientation: value.get_orientation(),
            underscanning: value.is_underscanning(),
            resolution: current_mode.get_resolution(),
            // TODO: in cloning mode this will None
            refresh_rate: Some(current_mode.get_refresh_rate()),
        }
    }
}

#[derive(Debug)]
pub struct DisplaySettingsModel {
    monitor: Arc<CcDisplayMonitor>,

    settings: DisplaySettings,

    lists: DisplaySettingsLists,
    controllers: DisplaySettingsControllers,
}

#[derive(Debug)]
struct DisplaySettingsLists {
    scale_list: Vec<Scale>,
    resolution_list: Vec<Resolution>,
    orientation_list: Vec<Orientation>,
    refresh_rate_list: Vec<RefreshRate>,
}

#[derive(Debug)]
struct DisplaySettingsControllers {
    scale_combo: Controller<SimpleComboRow<Scale>>,
    resolution: Controller<SimpleComboRow<Resolution>>,
    orientation: Controller<SimpleComboRow<Orientation>>,
    refresh_rate: Controller<SimpleComboRow<RefreshRate>>,
}

#[derive(Debug)]
pub enum DisplaySettingsMsg {
    UpdateSettings(DisplaySettingsPatch),

    SelectScale(usize),
    SelectResolution(usize),
    SelectRefreshRate(usize),
    SelectOrientation(usize),
}

#[derive(Debug)]
pub enum DisplaySettingsOutput {
    SettingsChanged(),
}

#[relm4::component(pub)]
impl SimpleComponent for DisplaySettingsModel {
    type Init = Arc<CcDisplayMonitor>;
    type Input = DisplaySettingsMsg;
    type Output = DisplaySettingsOutput;

    view! {
        #[root]
        gtk::Box {
            set_spacing: 18,
            set_orientation: gtk::Orientation::Vertical,

            #[name(listbox)]
            gtk::ListBox {
                set_hexpand: true,
                add_css_class: "boxed-list",
                set_selection_mode: gtk::SelectionMode::None,

                model.controllers.orientation.widget() -> &adw::ComboRow {
                   set_width_request: 100,
                   set_use_underline: true,
                   set_title: &dgettext("display setting", "_Orientation"),
                },

                model.controllers.resolution.widget() -> &adw::ComboRow {
                    set_width_request: 100,
                    set_use_underline: true,
                    set_title: &dgettext("display setting", "_Resolution")
                },

                model.controllers.refresh_rate.widget() -> &adw::ComboRow {
                    set_width_request: 100,
                    set_use_underline: true,
                    set_title: &dgettext("display setting", "R_efresh Rate")
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

                model.controllers.scale_combo.widget() -> &adw::ComboRow {
                    set_width_request: 100,
                    set_use_underline: true,
                    set_title: &dgettext("display settings", "_Scale"),
                }
            },
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let lists = Self::build_lists(&init);
        let controllers = Self::build_controllers(&init, &lists, sender.clone());

        let model = DisplaySettingsModel {
            lists,
            controllers,
            settings: DisplaySettings::from(&init),
            monitor: init,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            DisplaySettingsMsg::UpdateSettings(settings) => {
                self.settings.apply(settings);
            }
            DisplaySettingsMsg::SelectScale(index) => {
                self.settings.apply(DisplaySettingsPatch {
                    scale: self.lists.scale_list.get(index).cloned(),
                    ..Default::default()
                });
            }
            DisplaySettingsMsg::SelectResolution(index) => {
                let mut patch = DisplaySettingsPatch::default();

                let resolution = self.lists.resolution_list[index];

                if let Some(current_mode) = self
                    .monitor
                    .get_modes()
                    .iter()
                    .find(|&mode| mode.get_resolution() == resolution)
                {
                    let refresh_rate = current_mode.get_refresh_rate();

                    patch.refresh_rate = Some(refresh_rate);

                    self.lists.refresh_rate_list =
                        self.monitor.get_supported_refresh_rates(current_mode);
                    self.controllers
                        .refresh_rate
                        .emit(SimpleComboRowMsg::UpdateData(Self::build_refresh_rate_row(
                            refresh_rate,
                            self.lists.refresh_rate_list.clone(),
                        )));

                    let scale = current_mode.get_preferred_scale();

                    patch.scale = Some(scale);

                    self.lists.scale_list = current_mode.get_supported_scales();
                    self.controllers
                        .scale_combo
                        .emit(SimpleComboRowMsg::UpdateData(Self::build_scale_combo_row(
                            Some(scale),
                            self.lists.scale_list.clone(),
                        )));
                }

                self.settings.apply(patch);
            }
            DisplaySettingsMsg::SelectRefreshRate(index) => {
                self.settings.apply(DisplaySettingsPatch {
                    refresh_rate: Some(self.lists.refresh_rate_list[index]),
                    ..Default::default()
                })
            }
            DisplaySettingsMsg::SelectOrientation(index) => {
                self.settings.apply(DisplaySettingsPatch {
                    orientation: Some(self.lists.orientation_list[index]),
                    ..Default::default()
                });
            }
        };

        sender
            .output_sender()
            .emit(DisplaySettingsOutput::SettingsChanged());
    }
}

impl DisplaySettingsModel {
    fn build_lists(monitor: &Arc<CcDisplayMonitor>) -> DisplaySettingsLists {
        let current_mode = monitor.get_current_mode();

        DisplaySettingsLists {
            orientation_list: monitor.get_orientations(),
            scale_list: current_mode.get_supported_scales(),
            resolution_list: monitor.get_supported_resolutions(),
            refresh_rate_list: monitor.get_supported_refresh_rates(monitor.get_current_mode()),
        }
    }

    fn build_controllers(
        monitor: &Arc<CcDisplayMonitor>,
        lists: &DisplaySettingsLists,
        sender: ComponentSender<Self>,
    ) -> DisplaySettingsControllers {
        let lm = monitor.get_logical_monitor();
        let current_mode = monitor.get_current_mode();
        let monitor_orientation = monitor.get_orientation();

        DisplaySettingsControllers {
            orientation: SimpleComboRow::builder()
                .launch(SimpleComboRow {
                    active_index: lists.orientation_list.iter().position(|orientation| {
                        monitor_orientation
                            .as_ref()
                            .is_some_and(|monitor_orientation| monitor_orientation == orientation)
                    }),
                    variants: lists.orientation_list.clone(),
                })
                .forward(sender.input_sender(), DisplaySettingsMsg::SelectOrientation),
            resolution: SimpleComboRow::builder()
                .launch(Self::build_resolution_row(
                    current_mode.get_resolution(),
                    lists.resolution_list.clone(),
                ))
                .forward(sender.input_sender(), DisplaySettingsMsg::SelectResolution),
            refresh_rate: SimpleComboRow::builder()
                .launch(Self::build_refresh_rate_row(
                    current_mode.get_refresh_rate(),
                    lists.refresh_rate_list.clone(),
                ))
                .forward(sender.input_sender(), DisplaySettingsMsg::SelectRefreshRate),
            scale_combo: SimpleComboRow::builder()
                .launch(Self::build_scale_combo_row(
                    lm.map(|lm| lm.get_scale()),
                    lists.scale_list.clone(),
                ))
                .forward(sender.input_sender(), DisplaySettingsMsg::SelectScale),
        }
    }

    fn build_refresh_rate_row(
        refresh_rate: RefreshRate,
        variants: Vec<RefreshRate>,
    ) -> SimpleComboRow<RefreshRate> {
        SimpleComboRow {
            active_index: variants
                .iter()
                .position(|refresh_rate_var| *refresh_rate_var == refresh_rate),
            variants,
        }
    }

    fn build_resolution_row(
        resolution: Resolution,
        variants: Vec<Resolution>,
    ) -> SimpleComboRow<Resolution> {
        SimpleComboRow {
            active_index: variants
                .iter()
                .position(|resolution_var| *resolution_var == resolution),
            variants,
        }
    }

    fn build_scale_combo_row(scale: Option<Scale>, variants: Vec<Scale>) -> SimpleComboRow<Scale> {
        SimpleComboRow {
            active_index: variants
                .iter()
                .position(|scale_var| scale.is_some_and(|scale| scale == *scale_var)),
            variants,
        }
    }
}
