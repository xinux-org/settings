use gettextrs::gettext;
use relm4::{adw::prelude::*, prelude::*};
use zbus::Connection;

use super::DisplayConfigProxy;
use super::apply_settings::{ApplySettings, ApplySettingsMsg, ApplySettingsOut};
use super::display_settings::{
    DisplaySettings, DisplaySettingsInit, DisplaySettingsModel, DisplaySettingsOutput,
};
use super::display_settings_group::{DisplaySettingsGroup, DisplaySettingsGroupOutput};
use super::display_state::DisplayState;
use super::logical_monitor::LogicalMonitor;
use super::monitor::Monitor;
use super::monitor_spec::MonitorSpec;

#[derive(Debug, Clone, Copy)]
pub enum ConfigType {
    Join,
    Mirror,
}

impl From<gtk::glib::GString> for ConfigType {
    fn from(value: gtk::glib::GString) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&str> for ConfigType {
    fn from(value: &str) -> Self {
        match value {
            "mirror" => ConfigType::Mirror,
            _ => ConfigType::Join,
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<&str> for ConfigType {
    fn into(self) -> &'static str {
        match self {
            ConfigType::Join => "join",
            ConfigType::Mirror => "mirror",
        }
    }
}

#[derive(Debug)]
pub enum NavigationPage {
    Main,
    NightLight,
    Monitor(Box<Monitor>),
}

#[derive(Debug)]
pub enum DisplayMsg {
    Apply,
    Cancel,
    PagePopped,
    PushNightLightPage,
    PrimaryMonitorChanged(usize),
    ConfigTypeChanged(ConfigType),
    PushDisplaySettings(Box<Monitor>),
    DisplayConfigChanged(DisplaySettings),
}

#[derive(Debug, Clone)]
pub struct MirroredDisplay(pub Monitor, pub LogicalMonitor);

#[derive(Debug, Clone)]
pub struct SingleDisplay(pub Monitor, pub LogicalMonitor);

impl From<SingleDisplay> for LogicalDisplay {
    fn from(value: SingleDisplay) -> LogicalDisplay {
        LogicalDisplay(value.0, Some(value.1))
    }
}

#[derive(Debug, Clone)]
pub struct LogicalDisplay(pub Monitor, pub Option<LogicalMonitor>);

#[derive(Debug)]
pub enum DisplayMonitor {
    Single(SingleDisplay),
    Mirrored(MirroredDisplay),
    Logical(Vec<LogicalDisplay>),
}

impl From<&DisplayState> for DisplayMonitor {
    fn from(value: &DisplayState) -> Self {
        if value.monitors.len() == 1
            && let Some(monitor) = value.monitors.first()
        {
            return DisplayMonitor::Single(SingleDisplay(
                monitor.clone(),
                value
                    .logical_monitors
                    .iter()
                    .find(|lm| lm.monitors.contains(&monitor.spec))
                    .unwrap()
                    .clone(),
            ));
        }

        if value.logical_monitors.len() == 1
            && let Some(lm) = value.logical_monitors.first()
            && lm.monitors.len() == value.monitors.len()
        {
            let builtin = value
                .monitors
                .iter()
                .find(|m| m.is_builtin.is_some_and(|x| x))
                .unwrap()
                .clone();

            return DisplayMonitor::Mirrored(MirroredDisplay(builtin, lm.clone()));
        }

        let logical_display = value
            .monitors
            .iter()
            .map(|m| {
                LogicalDisplay(
                    m.clone(),
                    value
                        .logical_monitors
                        .iter()
                        .find(|lm| lm.monitors.contains(&m.spec))
                        .cloned(),
                )
            })
            .collect();

        DisplayMonitor::Logical(logical_display)
    }
}

#[derive(Debug)]
pub struct DisplayModel {
    display: Option<DisplayMonitor>,

    config_type: ConfigType,
    state: Option<DisplayState>,
    showing_apply_titlebar: bool,
    current_navigation: NavigationPage,

    display_settings: Option<Controller<DisplaySettingsModel>>,
    display_settings_group: Option<Controller<DisplaySettingsGroup>>,
    display_settings_in_page: Option<Controller<DisplaySettingsModel>>,

    apply_settings0: Controller<ApplySettings>,
    apply_settings1: Controller<ApplySettings>,
    apply_settings2: Controller<ApplySettings>,
}

#[relm4::component(pub async)]
impl AsyncComponent for DisplayModel {
    type Init = ();
    type Input = DisplayMsg;
    type Output = ();
    type CommandOutput = ();

    view! {
        #[root]
        #[name(navigation_view)]
        adw::NavigationView {
            connect_popped[sender] => move |_, _| {
                sender.input(DisplayMsg::PagePopped);
            },

            #[name(main_page)]
            adw::NavigationPage {
                set_title: &gettext("Displays"),

                #[wrap(Some)]
                set_child = &adw::ToolbarView {
                    add_top_bar = model.apply_settings0.widget(),

                    #[name(displays_titlebar)]
                    add_top_bar = &adw::HeaderBar {
                        set_show_title: true,
                        #[watch]
                        set_visible: !model.showing_apply_titlebar,
                    },

                    #[wrap(Some)]
                    set_content = &adw::PreferencesPage {
                        #[name(display_settings_disabled_group)]
                        adw::PreferencesGroup {
                            #[watch]
                            set_visible: model.state.as_ref().is_none(),

                            adw::StatusPage {
                                set_vexpand: true,
                                set_icon_name: Some("computer-symbolic"),
                                set_title: &gettext("Display Settings Disabled"),
                            }
                        },

                        #[name(display_multiple_displays)]
                        adw::PreferencesGroup {
                            #[watch]
                            set_visible: model.display.as_ref().is_some_and(|d| !matches!(d, DisplayMonitor::Single(_))),

                            #[name(config_type_switcher_row)]
                            adw::ActionRow {
                                set_title: &gettext("Multiple Displays"),

                                #[name(display_config_type)]
                                add_suffix = &adw::ToggleGroup {
                                    add = adw::Toggle {
                                        set_use_underline: true,
                                        // Translators: 'Join' as in 'Join displays'
                                        set_label: Some(&gettext("_Join")),
                                        set_name: Some(ConfigType::Join.into()),
                                    },

                                    add = adw::Toggle {
                                        set_use_underline: true,
                                        set_label: Some(&gettext("_Mirror")),
                                        set_name: Some(ConfigType::Mirror.into()),
                                    },

                                    set_homogeneous: true,
                                    set_valign: gtk::Align::Center,

                                    #[watch]
                                    #[block_signal(display_config_type_handler)]
                                    set_active_name: Some(model.config_type.into()),

                                    connect_active_name_notify[sender] => move |toggle| {
                                        if let Some(active_name) = toggle.active_name() {
                                            sender.input(DisplayMsg::ConfigTypeChanged(active_name.into()));
                                        }
                                    } @display_config_type_handler
                                },
                            }
                        },

                        #[name(single_display_settings_group)]
                        adw::PreferencesGroup {
                            #[watch]
                            set_visible: model.display.as_ref().is_some_and(|d| !matches!(d, DisplayMonitor::Logical(_))),

                            adw::Bin {
                                #[watch]
                                set_child: model.display_settings.as_ref().map(|ds| ds.widget()),
                            }
                        },

                        #[local_ref]
                        display_settings_group -> adw::PreferencesGroup {
                            #[watch]
                            set_visible: model.display.as_ref().is_some_and(|d| matches!(d, DisplayMonitor::Logical(_))),
                        },

                        adw::PreferencesGroup {
                            adw::ActionRow {
                                set_activatable: true,
                                // Translators: This is the redshift functionality where we suppress blue light when the sun has gone down
                                set_title: &gettext("_Night Light"),
                                connect_activated => DisplayMsg::PushNightLightPage,
                                add_prefix = &gtk::Image::from_icon_name("night-light-symbolic"),
                            }
                        },
                    },
                },
            },

            #[name(night_light_page)]
            adw::NavigationPage {
                set_title: &gettext("Night Light"),

                #[wrap(Some)]
                set_child = &adw::ToolbarView {
                    add_top_bar = model.apply_settings1.widget(),

                    add_top_bar = &adw::HeaderBar {
                        set_show_title: true,
                        #[watch]
                        set_visible: !model.showing_apply_titlebar,
                    },

                    // TODO: impl night light page
                }
            },

            #[name(display_settings_page)]
            adw::NavigationPage {
                set_title: &gettext("Displays"),

                #[wrap(Some)]
                set_child = &adw::ToolbarView {
                    add_top_bar = model.apply_settings2.widget(),

                    add_top_bar = &adw::HeaderBar {
                        set_show_title: true,
                        #[watch]
                        set_visible: !model.showing_apply_titlebar,
                    },

                    #[wrap(Some)]
                    set_content = &adw::PreferencesPage {
                        adw::PreferencesGroup {
                            adw::Bin {
                                #[watch]
                                set_child: model.display_settings_in_page.as_ref().map(|ds| ds.widget()),
                            },
                        }
                    },
                }
            },
        }
    }

    async fn init(
        _: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let state = Self::get_display_state().await.ok();
        let display = state.as_ref().map(DisplayMonitor::from);

        let transform = |output| match output {
            ApplySettingsOut::Apply => DisplayMsg::Apply,
            ApplySettingsOut::Cancel => DisplayMsg::Cancel,
        };

        let mut model = DisplayModel {
            apply_settings0: ApplySettings::builder()
                .launch(())
                .forward(sender.input_sender(), transform),
            apply_settings1: ApplySettings::builder()
                .launch(())
                .forward(sender.input_sender(), transform),
            apply_settings2: ApplySettings::builder()
                .launch(())
                .forward(sender.input_sender(), transform),
            display_settings: None,
            display_settings_group: None,
            showing_apply_titlebar: false,
            display_settings_in_page: None,
            current_navigation: NavigationPage::Main,
            config_type: display
                .as_ref()
                .map(|d| {
                    if matches!(d, DisplayMonitor::Mirrored(_)) {
                        ConfigType::Mirror
                    } else {
                        ConfigType::Join
                    }
                })
                .unwrap_or(ConfigType::Join),
            display,
            state,
        };

        let mut display_settings_group = adw::PreferencesGroup::builder().visible(false).build();

        if let Some(display) = model.display.as_ref() {
            match display {
                DisplayMonitor::Logical(monitors) => {
                    let controller = DisplaySettingsGroup::builder()
                        .launch(monitors.clone())
                        .forward(sender.input_sender(), |output| match output {
                            DisplaySettingsGroupOutput::PushDisplaySettings(monitor) => {
                                DisplayMsg::PushDisplaySettings(monitor)
                            }
                            DisplaySettingsGroupOutput::ChangedPrimaryMonitor(index) => {
                                DisplayMsg::PrimaryMonitorChanged(index)
                            }
                        });

                    display_settings_group = controller.widget().to_owned();
                    model.display_settings_group = Some(controller);
                }
                DisplayMonitor::Mirrored(mirrored_display) => {
                    let display_settings = Self::build_display_settings(
                        &mirrored_display.0.spec,
                        display,
                        sender.clone(),
                    );
                    model.display_settings = Some(display_settings);
                }
                DisplayMonitor::Single(single_display) => {
                    let display_settings = Self::build_display_settings(
                        &single_display.0.spec,
                        display,
                        sender.clone(),
                    );
                    model.display_settings = Some(display_settings);
                }
            }
        }

        let mut widgets = view_output!();

        model.render_ui(&mut widgets);

        AsyncComponentParts { model, widgets }
    }

    async fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
        root: &Self::Root,
    ) {
        self.update(message, sender.clone(), root).await;
        self.update_view(widgets, sender);
        self.render_ui(widgets);
    }

    async fn update(
        &mut self,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            DisplayMsg::Apply => {}
            DisplayMsg::Cancel => {
                self.showing_apply_titlebar = false;
            }
            DisplayMsg::PagePopped => {
                self.current_navigation = NavigationPage::Main;
            }
            DisplayMsg::PushNightLightPage => {
                self.current_navigation = NavigationPage::NightLight;
            }
            DisplayMsg::PrimaryMonitorChanged(index) => {
                self.showing_apply_titlebar = true;

                if let Some(display) = self.display.as_mut() {
                    match display {
                        DisplayMonitor::Logical(items) => {
                            for (item_index, item) in items.iter_mut().enumerate() {
                                if let Some(lm) = item.1.as_mut() {
                                    lm.is_primary = item_index == index;
                                }
                            }
                        }
                        _ => {
                            tracing::warn!(
                                "Called DisplayMsg::PrimaryMonitorChanged on invalid state"
                            )
                        }
                    }
                }
            }
            DisplayMsg::ConfigTypeChanged(config_type) => {
                self.config_type = config_type;
                self.showing_apply_titlebar = true;

                if let Some(state) = self.state.as_ref() {
                    let display = match self.config_type {
                        ConfigType::Join => DisplayMonitor::from(state),
                        ConfigType::Mirror => {
                            let builtin = state
                                .monitors
                                .iter()
                                .find(|m| m.is_builtin.is_some_and(|x| x))
                                .unwrap();

                            DisplayMonitor::Mirrored(MirroredDisplay(
                                builtin.clone(),
                                state.logical_monitors.first().unwrap().clone(),
                            ))
                        }
                    };

                    self.display_settings = match &display {
                        DisplayMonitor::Single(single_display) => {
                            let settings = Self::build_display_settings(
                                &single_display.0.spec,
                                &display,
                                sender.clone(),
                            );
                            Some(settings)
                        }
                        DisplayMonitor::Mirrored(mirror_display) => {
                            let settings = Self::build_display_settings(
                                &mirror_display.0.spec,
                                &display,
                                sender.clone(),
                            );
                            Some(settings)
                        }
                        DisplayMonitor::Logical(_) => None,
                    };

                    self.display = Some(display);
                }
            }
            DisplayMsg::PushDisplaySettings(monitor) => {
                if let Some(display) = self.display.as_ref() {
                    self.display_settings_in_page =
                        Some(Self::build_display_settings(&monitor.spec, display, sender));
                }

                self.current_navigation = NavigationPage::Monitor(monitor);
            }
            DisplayMsg::DisplayConfigChanged(_) => {
                self.showing_apply_titlebar = true;
            }
        }
    }
}

impl DisplayModel {
    fn render_ui(&self, widgets: &mut <Self as AsyncComponent>::Widgets) {
        let visible_page = widgets.navigation_view.visible_page();

        match &self.current_navigation {
            NavigationPage::Main => {
                if visible_page.is_some_and(|page| page != widgets.main_page) {
                    widgets.navigation_view.pop_to_page(&widgets.main_page);
                }
            }
            NavigationPage::NightLight => {
                if visible_page.is_some_and(|page| page != widgets.night_light_page) {
                    widgets.navigation_view.push(&widgets.night_light_page);
                }
            }
            NavigationPage::Monitor(monitor) => {
                if visible_page.is_some_and(|page| page != widgets.display_settings_page) {
                    widgets
                        .display_settings_page
                        .set_title(&monitor.get_output_ui_name());
                    widgets.navigation_view.push(&widgets.display_settings_page);
                }
            }
        }

        if let Some(display_settings_group) = &self.display_settings_group {
            widgets.display_settings_group = display_settings_group.widget().to_owned();
        }

        self.apply_settings0
            .emit(ApplySettingsMsg::SetActive(self.showing_apply_titlebar));
        self.apply_settings1
            .emit(ApplySettingsMsg::SetActive(self.showing_apply_titlebar));
        self.apply_settings2
            .emit(ApplySettingsMsg::SetActive(self.showing_apply_titlebar));
    }

    async fn get_display_state() -> anyhow::Result<DisplayState> {
        let conn = Connection::session().await?;
        let proxy = DisplayConfigProxy::new(&conn).await?;
        let state = proxy.get_current_state().await?;

        Ok(DisplayState::from(state))
    }

    fn build_display_settings(
        spec: &MonitorSpec,
        monitor: &DisplayMonitor,
        sender: AsyncComponentSender<Self>,
    ) -> Controller<DisplaySettingsModel> {
        DisplaySettingsModel::builder()
            .launch(match monitor {
                DisplayMonitor::Single(display) => {
                    DisplaySettingsInit::LogicalDisplay(display.clone().into())
                }
                DisplayMonitor::Mirrored(display) => {
                    DisplaySettingsInit::MirroredDisplay(display.clone())
                }
                DisplayMonitor::Logical(items) => DisplaySettingsInit::LogicalDisplay(
                    items
                        .iter()
                        .find(|item| item.0.spec == *spec)
                        .unwrap()
                        .clone(),
                ),
            })
            .forward(sender.input_sender(), |output| match output {
                DisplaySettingsOutput::Changed(settings) => {
                    DisplayMsg::DisplayConfigChanged(settings)
                }
            })
    }
}
