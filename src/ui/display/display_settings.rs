use std::sync::{Arc, RwLock, RwLockReadGuard};

use gettextrs::{dgettext, gettext};
use relm4::{adw::prelude::*, prelude::*};
use relm4_components::simple_adw_combo_row::{SimpleComboRow, SimpleComboRowMsg};
use struct_patch::Patch;

use super::config::{CcDisplayMonitor, GetList, Orientation};
use super::{RefreshRate, Resolution, Scale};

macro_rules! patch_settings {
    ($self:ident, $name:ident, $index:ident) => {
        let option = $self.lists.$name.get($index);
        let patch = DisplaySettingsPatch {
            $name: option.copied(),
            ..Default::default()
        };
        $self.settings.apply(patch);
    };
    ($self:ident, $current_mode:ident) => {
        let patch = DisplaySettingsPatch {
            scale: Some($current_mode.get_preferred_scale()),
            resolution: Some($current_mode.get_resolution()),
            refresh_rate: Some($current_mode.get_refresh_rate()),
            ..Default::default()
        };
        $self.settings.apply(patch);
    };
}

macro_rules! build_combo_row {
    ($lists:ident, $name:ident, $value:ident) => {
        SimpleComboRow {
            active_index: $lists.$name.iter().position(|&$name| $name == $value),
            variants: $lists.$name.clone(),
        }
    };
}

macro_rules! build_controller {
    ($lists: ident, $name:ident, $value:ident, $input:ident, $sender:ident) => {
        SimpleComboRow::builder()
            .launch(build_combo_row!($lists, $name, $value))
            .forward($sender.input_sender(), DisplaySettingsMsg::$input)
    };
}

macro_rules! update_controller {
    (&$model:ident, $name:ident) => {
        let lists = &$model.lists;
        let value = $model.settings.$name;
        let update = build_combo_row!(lists, $name, value);

        $model
            .controllers
            .$name
            .emit(SimpleComboRowMsg::UpdateData(update));
    };
}

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
    scale: Vec<Scale>,
    resolution: Vec<Resolution>,
    orientation: Vec<Orientation>,
    refresh_rate: Vec<RefreshRate>,
}

#[derive(Debug)]
struct DisplaySettingsControllers {
    scale: Controller<SimpleComboRow<Scale>>,
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

                model.controllers.scale.widget() -> &adw::ComboRow {
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
                patch_settings!(self, scale, index);
            }
            DisplaySettingsMsg::SelectRefreshRate(index) => {
                patch_settings!(self, refresh_rate, index);
            }
            DisplaySettingsMsg::SelectOrientation(index) => {
                patch_settings!(self, orientation, index);
            }
            DisplaySettingsMsg::SelectResolution(index) => {
                let resolution = self.lists.resolution[index];

                if let Ok(mut monitor) = self.monitor.write() {
                    monitor.set_current_mode(resolution);

                    let current_mode = monitor.get_current_mode();

                    patch_settings!(self, current_mode);
                }

                self.update_model();
            }
        };
    }
}

impl DisplaySettingsModel {
    fn build_lists(monitor: &CcDisplayMonitor) -> DisplaySettingsLists {
        let current_mode = monitor.get_current_mode();

        DisplaySettingsLists {
            scale: current_mode.get_list(),
            resolution: monitor.get_list(),
            orientation: monitor.get_list(),
            refresh_rate: monitor.get_list(),
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
        let monitor_resolution = current_mode.get_resolution();
        let monitor_refresh_rate = current_mode.get_refresh_rate();
        let monitor_scale = lm.map(|lm| lm.get_scale()).unwrap_or_default();

        DisplaySettingsControllers {
            scale: build_controller!(lists, scale, monitor_scale, SelectScale, sender),
            resolution: build_controller!(
                lists,
                resolution,
                monitor_resolution,
                SelectResolution,
                sender
            ),
            orientation: build_controller!(
                lists,
                orientation,
                monitor_orientation,
                SelectOrientation,
                sender
            ),
            refresh_rate: build_controller!(
                lists,
                refresh_rate,
                monitor_refresh_rate,
                SelectRefreshRate,
                sender
            ),
        }
    }

    fn update_model(&mut self) {
        let Ok(monitor) = self.monitor.read() else {
            return;
        };

        self.lists.scale = monitor.get_list();
        self.lists.refresh_rate = monitor.get_list();

        update_controller!(&self, scale);
        update_controller!(&self, refresh_rate);
    }
}
