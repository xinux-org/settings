use gettextrs::gettext;
use relm4::{adw::prelude::*, prelude::*};

use super::config::CcDisplayMonitor;
use super::display_settings::{
    DisplaySettings, DisplaySettingsInit, DisplaySettingsModel, DisplaySettingsMsg,
    DisplaySettingsOutput,
};
use super::display_settings_group::{
    DisplaySettingsGroup, DisplaySettingsGroupMsg, DisplaySettingsGroupOutput,
};
use super::monitor_spec::MonitorSpec;
use super::{ConfigType, DisplayConfigManager};

#[derive(Debug)]
pub enum NavigationPage {
    Main,
    NightLight,
    Monitor(Box<CcDisplayMonitor>),
}

#[derive(Debug)]
pub enum DisplayMsg {
    PagePopped,
    PushNightLightPage,
    PrimaryMonitorChanged(usize),
    ConfigTypeChanged(ConfigType),
    PushDisplaySettings(Box<CcDisplayMonitor>),
    DisplayConfigChanged(MonitorSpec, DisplaySettings),
}

#[derive(Debug)]
pub enum DisplayCmd {
    RefetchChanges,
}

#[derive(Debug)]
pub struct DisplayModel {
    manager: DisplayConfigManager,

    current_navigation: NavigationPage,

    display_settings_group: Controller<DisplaySettingsGroup>,
    display_settings: Option<Controller<DisplaySettingsModel>>,
    display_settings_in_page: Option<Controller<DisplaySettingsModel>>,
}

#[relm4::component(pub async)]
impl AsyncComponent for DisplayModel {
    type Init = DisplayConfigManager;
    type Input = DisplayMsg;
    type Output = ();
    type CommandOutput = DisplayCmd;

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
                        #[name(displays_titlebar)]
                        add_top_bar = &adw::HeaderBar {
                            set_show_title: true,
                        },

                    #[wrap(Some)]
                    set_content = &adw::PreferencesPage {
                        #[name(display_multiple_displays)]
                        adw::PreferencesGroup {
                            #[watch]
                            set_visible: model.manager.monitor_count() > 1,

                            #[name(config_type_switcher_row)]
                            adw::ActionRow {
                                set_title: &gettext("Multiple Displays"),

                                #[name(display_config_type)]
                                add_suffix = &adw::ToggleGroup {
                                    add = adw::Toggle {
                                        set_use_underline: true,
                                        // Translators: 'Join' as in 'Join displays'
                                        set_label: Some(&gettext("_Join")),
                                        set_name: Some(ConfigType::Join.as_ref()),
                                    },

                                    add = adw::Toggle {
                                        set_use_underline: true,
                                        set_label: Some(&gettext("_Mirror")),
                                        set_name: Some(ConfigType::Mirror.as_ref()),
                                    },

                                    set_homogeneous: true,
                                    set_valign: gtk::Align::Center,

                                    #[watch]
                                    #[block_signal(display_config_type_handler)]
                                    set_active_name: Some(model.manager.config_type.as_ref()),

                                    connect_active_name_notify[sender] => move |toggle| {
                                        if let Some(active_name) = toggle.active_name()
                                        && let Ok(active_name) = ConfigType::try_from(active_name.as_str()) {
                                            sender.input(DisplayMsg::ConfigTypeChanged(active_name));
                                        }
                                    } @display_config_type_handler
                                },
                            }
                        },

                        #[name(single_display_settings_group)]
                        adw::PreferencesGroup {
                            #[watch]
                            set_visible: model.manager.logical_monitor_count() <= 1,

                            adw::Bin {
                                #[watch]
                                set_child: model.display_settings.as_ref().map(|ds| ds.widget()),
                            }
                        },

                        model.display_settings_group.widget() -> &adw::PreferencesGroup,

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
                        add_top_bar = &adw::HeaderBar {
                            set_show_title: true,
                        },

                    // TODO: impl night light page
                }
            },

            #[name(display_settings_page)]
            adw::NavigationPage {
                set_title: &gettext("Displays"),

                    #[wrap(Some)]
                    set_child = &adw::ToolbarView {
                        add_top_bar = &adw::HeaderBar {
                            set_show_title: true,
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
        manager: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let mut model = DisplayModel {
            manager,
            display_settings: None,
            display_settings_in_page: None,
            current_navigation: NavigationPage::Main,
            display_settings_group: Self::build_display_settings_group(vec![], sender.clone()),
        };

        model.rebuild_display_ui(sender.clone());

        let mut widgets = view_output!();

        model.render_ui(&mut widgets);

        sender.command(|output, shutdown| {
            shutdown
                .register(async {
                    if let Err(err) = DisplayConfigManager::listen_changes(move || {
                        output.emit(DisplayCmd::RefetchChanges);
                    })
                    .await
                    {
                        tracing::error!("{:?}", err)
                    }
                })
                .drop_on_shutdown()
        });

        AsyncComponentParts { model, widgets }
    }

    async fn update_cmd_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::CommandOutput,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            DisplayCmd::RefetchChanges => match self.manager.refetch().await {
                Ok(_) => {
                    self.rebuild_display_ui(sender.clone());
                    self.update_view(widgets, sender);
                    self.render_ui(widgets);
                }
                Err(err) => {
                    tracing::error!("{}", err)
                }
            },
        }
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
            DisplayMsg::PagePopped => {
                self.current_navigation = NavigationPage::Main;
            }
            DisplayMsg::PushNightLightPage => {
                self.current_navigation = NavigationPage::NightLight;
            }
            DisplayMsg::PrimaryMonitorChanged(index) => {
                if !self.primary_is_at_index(index) {
                    self.manager.set_primary_monitor(index);
                }
            }
            DisplayMsg::ConfigTypeChanged(config_type) => {
                self.manager.ensure_config_type(config_type);
                self.rebuild_display_ui(sender.clone());
            }
            DisplayMsg::PushDisplaySettings(monitor) => {
                let is_mirrored = self.manager.config_type == ConfigType::Mirror;

                self.display_settings_in_page =
                    Some(Self::build_display_settings(&monitor, is_mirrored, sender));
                self.current_navigation = NavigationPage::Monitor(monitor);
            }
            DisplayMsg::DisplayConfigChanged(spec, settings) => {
                self.manager.set_monitor_config(spec, settings);
            }
        }
    }
}

impl DisplayModel {
    fn rebuild_display_ui(&mut self, sender: AsyncComponentSender<Self>) {
        let monitor_count = self.manager.monitor_count();
        let is_mirrored = self.manager.config_type == ConfigType::Mirror;

        if monitor_count > 1 && !is_mirrored {
            self.display_settings = None;
            self.display_settings_in_page = None;

            let monitors: Vec<CcDisplayMonitor> = self.manager.monitors().to_vec();

            self.display_settings_group
                .emit(DisplaySettingsGroupMsg::Update(Some(monitors)));
        } else {
            self.display_settings_group
                .emit(DisplaySettingsGroupMsg::Update(None));

            let monitor = self
                .manager
                .monitors()
                .first()
                .cloned()
                .expect("at least one monitor must exist");

            let init = DisplaySettingsInit {
                monitor: monitor.clone(),
                is_mirrored,
            };

            match self.display_settings.as_mut() {
                Some(controller) => {
                    controller.sender().emit(DisplaySettingsMsg::Update(init));
                }
                None => {
                    self.display_settings = Some(Self::build_display_settings(
                        &monitor,
                        is_mirrored,
                        sender.clone(),
                    ));
                }
            }
        }

        if matches!(self.current_navigation, NavigationPage::Monitor(_)) {
            self.current_navigation = NavigationPage::Main;
        }
    }

    fn primary_is_at_index(&self, index: usize) -> bool {
        self.manager
            .monitors()
            .get(index)
            .is_some_and(|m| m.is_primary())
    }

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
    }

    fn build_display_settings_group(
        monitors: Vec<CcDisplayMonitor>,
        sender: AsyncComponentSender<Self>,
    ) -> Controller<DisplaySettingsGroup> {
        DisplaySettingsGroup::builder()
            .launch(monitors)
            .forward(sender.input_sender(), |output| match output {
                DisplaySettingsGroupOutput::PushDisplaySettings(monitor) => {
                    DisplayMsg::PushDisplaySettings(monitor)
                }
                DisplaySettingsGroupOutput::ChangedPrimaryMonitor(index) => {
                    DisplayMsg::PrimaryMonitorChanged(index)
                }
            })
    }

    fn build_display_settings(
        monitor: &CcDisplayMonitor,
        is_mirrored: bool,
        sender: AsyncComponentSender<Self>,
    ) -> Controller<DisplaySettingsModel> {
        let init = DisplaySettingsInit {
            monitor: monitor.clone(),
            is_mirrored,
        };

        DisplaySettingsModel::builder()
            .launch(init)
            .forward(sender.input_sender(), |output| match output {
                DisplaySettingsOutput::Changed(spec, settings) => {
                    DisplayMsg::DisplayConfigChanged(spec, settings)
                }
            })
    }
}
