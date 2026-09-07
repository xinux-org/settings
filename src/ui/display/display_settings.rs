use std::sync::{Arc, RwLock, RwLockReadGuard};

use gettextrs::{dgettext, gettext};
use relm4::{adw::prelude::*, prelude::*};
use relm4_components::simple_adw_combo_row::{SimpleComboRow, SimpleComboRowMsg};
use struct_patch::Patch;

use super::config::{CcDisplayMonitor, GetList, Orientation};
use super::{RefreshRate, Resolution, Scale};

#[derive(Debug, Clone, PartialEq, Patch)]
#[patch(attribute(derive(Debug, Default, Clone)))]
pub struct DisplaySettings {
    pub scale: Scale,
    pub resolution: Resolution,
    pub orientation: Orientation,
    pub refresh_rate: RefreshRate,
    #[patch(skip_wrap)]
    pub hdr: Option<bool>,
    #[patch(skip_wrap)]
    pub underscanning: Option<bool>,
}

impl From<&RwLockReadGuard<'_, CcDisplayMonitor>> for DisplaySettings {
    fn from(value: &RwLockReadGuard<'_, CcDisplayMonitor>) -> Self {
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
            refresh_rate: current_mode.get_refresh_rate(),
        }
    }
}

#[derive(Debug)]
pub struct DisplaySettingsModel {
    monitor: Arc<RwLock<CcDisplayMonitor>>,

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

#[relm4::component(pub)]
impl SimpleComponent for DisplaySettingsModel {
    type Init = Arc<RwLock<CcDisplayMonitor>>;
    type Input = DisplaySettingsMsg;
    type Output = ();

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
                    set_active: model.settings.underscanning.is_some_and(|x| x),

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
        let monitor = init.read().unwrap();

        let lists = Self::build_lists(&monitor);
        let settings = DisplaySettings::from(&monitor);
        let controllers = Self::build_controllers(&monitor, &lists, sender.clone());

        drop(monitor);

        let model = DisplaySettingsModel {
            lists,
            settings,
            controllers,
            monitor: init,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
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

                if let Ok(mut monitor) = self.monitor.write() {
                    monitor.set_current_mode(resolution);

                    let current_mode = monitor.get_current_mode();

                    patch.resolution = Some(resolution);
                    patch.scale = Some(current_mode.get_preferred_scale());
                    patch.refresh_rate = Some(current_mode.get_refresh_rate());
                }

                self.settings.apply(patch);

                self.update_lists();
                self.update_controllers();
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
    }
}

impl DisplaySettingsModel {
    fn build_lists(monitor: &CcDisplayMonitor) -> DisplaySettingsLists {
        let current_mode = monitor.get_current_mode();

        DisplaySettingsLists {
            scale_list: current_mode.get_list(),
            resolution_list: monitor.get_list(),
            orientation_list: monitor.get_list(),
            refresh_rate_list: monitor.get_list(),
        }
    }

    fn build_controllers(
        monitor: &CcDisplayMonitor,
        lists: &DisplaySettingsLists,
        sender: ComponentSender<Self>,
    ) -> DisplaySettingsControllers {
        let lm = monitor.get_logical_monitor();
        let current_mode = monitor.get_current_mode();
        let monitor_orientation = monitor.get_orientation();

        DisplaySettingsControllers {
            orientation: SimpleComboRow::builder()
                .launch(SimpleComboRow {
                    active_index: lists
                        .orientation_list
                        .iter()
                        .position(|&orientation| orientation == monitor_orientation),
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
                    lm.map(|lm| lm.get_scale()).unwrap_or_default(),
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
                .position(|&refresh_rate_var| refresh_rate_var == refresh_rate),
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
                .position(|&resolution_var| resolution_var == resolution),
            variants,
        }
    }

    fn build_scale_combo_row(scale: Scale, variants: Vec<Scale>) -> SimpleComboRow<Scale> {
        SimpleComboRow {
            active_index: variants.iter().position(|&scale_var| scale_var == scale),
            variants,
        }
    }

    fn update_lists(&mut self) {
        let Ok(monitor) = self.monitor.read() else {
            return;
        };

        self.lists.scale_list = monitor.get_list();
        self.lists.refresh_rate_list = monitor.get_list();
    }

    fn update_controllers(&mut self) {
        let scale_row =
            Self::build_scale_combo_row(self.settings.scale, self.lists.scale_list.clone());
        let refresh_rate_row = Self::build_refresh_rate_row(
            self.settings.refresh_rate,
            self.lists.refresh_rate_list.clone(),
        );

        self.controllers
            .scale_combo
            .emit(SimpleComboRowMsg::UpdateData(scale_row));

        self.controllers
            .refresh_rate
            .emit(SimpleComboRowMsg::UpdateData(refresh_rate_row));
    }
}
