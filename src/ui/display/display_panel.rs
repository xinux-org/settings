use gettextrs::gettext;
use relm4::{adw::prelude::*, prelude::*};
use zbus::Connection;

use super::DisplayConfigProxy;
use super::display_settings::DisplaySettingsOutput;
use super::display_settings::{DisplaySettings, DisplaySettingsInit};
use super::display_settings_group::{
    DisplaySettingsGroup, DisplaySettingsGroupInit, DisplaySettingsGroupOutput,
};
use super::display_state::DisplayState;
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
pub enum DisplayMsg {
    Apply,
    Cancel,
    DisplayConfigChanged(),
    PrimaryMonitorChanged(usize),
    ConfigTypeChanged(ConfigType),
    PushDisplaySettings(Box<Monitor>),
}

#[derive(Debug)]
pub struct DisplayModel {
    config_type: ConfigType,
    state: Option<DisplayState>,
    showing_apply_titlebar: bool,
    primary_monitor: Option<MonitorSpec>,

    display_settings: Option<Controller<DisplaySettings>>,
    display_settings_in_page: Option<Controller<DisplaySettings>>,
    display_settings_group: Option<Controller<DisplaySettingsGroup>>,
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
            #[name(main_page)]
            adw::NavigationPage {
                set_tag: Some("main"),
                set_title: &gettext("Displays"),

                #[wrap(Some)]
                set_child = &adw::ToolbarView {
                    #[name(apply_titlebar)]
                    add_top_bar = &adw::HeaderBar {
                        set_show_end_title_buttons: false,
                        set_show_start_title_buttons: false,

                        #[watch]
                        set_visible: model.showing_apply_titlebar,

                        pack_start = &gtk::Button {
                            set_can_shrink: true,
                            set_use_underline: true,
                            set_label: &gettext("Cancel"),
                            connect_clicked => DisplayMsg::Cancel,
                        },

                        #[wrap(Some)]
                        set_title_widget = &adw::WindowTitle {
                            set_title: &gettext("Apply Changes?"),
                        },

                        pack_end = &gtk::Button {
                            set_can_shrink: true,
                            set_use_underline: true,
                            set_label: &gettext("Apply"),
                            add_css_class: "suggested-action",
                            connect_clicked => DisplayMsg::Apply,
                        },
                    },

                    #[name(displays_titlebar)]
                    add_top_bar = &adw::HeaderBar {
                        #[watch]
                        set_visible: !model.showing_apply_titlebar,

                        #[wrap(Some)]
                        set_title_widget = &adw::WindowTitle {
                            set_title: &gettext("Displays"),
                        },
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
                            set_visible: model.state.as_ref().is_some_and(|s| s.monitors.len() > 1),

                            #[name(config_type_switcher_row)]
                            adw::ActionRow {
                                set_title: &gettext("Multiple Displays"),

                                #[name(display_config_type)]
                                add_suffix = &adw::ToggleGroup {
                                    set_homogeneous: true,
                                    set_valign: gtk::Align::Center,
                                    #[watch]
                                    #[block_signal(display_config_type_handler)]
                                    set_active_name: Some(model.config_type.into()),

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
                            set_visible: model.display_settings.is_some() && (matches!(model.config_type, ConfigType::Mirror) || model.state.as_ref().is_some_and(|s| s.monitors.len() == 1)),

                            adw::Bin {
                                #[watch]
                                set_child: model.display_settings.as_ref().map(|ds| ds.widget()),
                            }
                        },

                        #[local_ref]
                        display_settings_group -> adw::PreferencesGroup {
                            #[watch]
                            set_visible: matches!(model.config_type, ConfigType::Join) && model.state.as_ref().is_some_and(|s| s.monitors.len() > 1),
                        },

                        adw::PreferencesGroup {
                            adw::ActionRow {
                                set_activatable: true,
                                // Translators: This is the redshift functionality where we suppress blue light when the sun has gone down
                                set_title: &gettext("_Night Light"),

                                add_prefix = &gtk::Image::from_icon_name("night-light-symbolic"),

                                // TODO: impl action
                            }
                        },
                    },
                },
            },

            #[name(night_light_page)]
            adw::NavigationPage {
                set_tag: Some("night-light"),
                set_title: &gettext("Night Light"),

                #[wrap(Some)]
                set_child = &adw::ToolbarView {
                    add_top_bar = &adw::HeaderBar {
                        set_show_end_title_buttons: false,
                        set_show_start_title_buttons: false,

                        #[watch]
                        set_visible: model.showing_apply_titlebar,

                        pack_start = &gtk::Button {
                            set_can_shrink: true,
                            set_use_underline: true,
                            set_label: &gettext("Cancel"),
                            connect_clicked => DisplayMsg::Cancel,
                        },

                        #[wrap(Some)]
                        set_title_widget = &adw::WindowTitle {},

                        pack_end = &gtk::Button {
                            set_can_shrink: true,
                            set_use_underline: true,
                            set_label: &gettext("Apply"),
                            add_css_class: "suggested-action",
                            connect_clicked => DisplayMsg::Apply,
                        },
                    },

                    add_top_bar = &adw::HeaderBar {
                        #[watch]
                        set_visible: !model.showing_apply_titlebar,
                    },

                    // TODO: impl night light page
                }
            },

            #[name(display_settings_page)]
            adw::NavigationPage {
                set_title: &gettext("Displays"),
                set_tag: Some("display-settings"),

                #[wrap(Some)]
                set_child = &adw::ToolbarView {
                    add_top_bar = &adw::HeaderBar {
                        set_show_end_title_buttons: false,
                        set_show_start_title_buttons: false,

                        #[watch]
                        set_visible: model.showing_apply_titlebar,

                        pack_start = &gtk::Button {
                            set_can_shrink: true,
                            set_use_underline: true,
                            set_label: &gettext("Cancel"),
                            connect_clicked => DisplayMsg::Cancel,
                        },

                        #[wrap(Some)]
                        set_title_widget = &adw::WindowTitle {
                            set_title: &gettext("Apply Changes?"),
                        },

                        pack_end = &gtk::Button {
                            set_can_shrink: true,
                            set_use_underline: true,
                            set_label: &gettext("Apply"),
                            add_css_class: "suggested-action",
                            connect_clicked => DisplayMsg::Apply,
                        },
                    },

                    add_top_bar = &adw::HeaderBar {
                        #[watch]
                        set_visible: !model.showing_apply_titlebar,

                        #[wrap(Some)]
                        #[name(display_settings_page_title)]
                        set_title_widget = &adw::WindowTitle {},
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
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let state = Self::get_display_state().await.ok();

        let mut model = DisplayModel {
            state,
            primary_monitor: None,
            display_settings: None,
            display_settings_group: None,
            showing_apply_titlebar: false,
            config_type: ConfigType::Join,
            display_settings_in_page: None,
        };

        let mut display_settings_group = adw::PreferencesGroup::default();

        if let Some(state) = &model.state {
            if state.monitors.len() == 1
                && let Some(monitor) = state.monitors.first()
            {
                let controller = Self::build_display_settings(
                    monitor,
                    matches!(model.config_type, ConfigType::Mirror),
                    state,
                    sender.clone(),
                );
                model.display_settings = Some(controller);
            } else {
                let controller = DisplaySettingsGroup::builder()
                    .launch(DisplaySettingsGroupInit {
                        monitors: state.monitors.clone(),
                        logical_monitors: state.logical_monitors.clone(),
                    })
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
        }

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
        root: &Self::Root,
    ) {
        match &message {
            DisplayMsg::Apply => {}
            DisplayMsg::Cancel => {
                self.showing_apply_titlebar = false;
            }
            DisplayMsg::DisplayConfigChanged() => {
                self.showing_apply_titlebar = true;
            }
            DisplayMsg::ConfigTypeChanged(config_type) => {
                self.config_type = *config_type;

                if let Some(state) = &self.state
                    && let Some(monitor) = state
                        .monitors
                        .iter()
                        .find(|m| m.is_builtin.is_some_and(|x| x))
                {
                    let display_settings = Self::build_display_settings(
                        monitor,
                        matches!(self.config_type, ConfigType::Mirror),
                        state,
                        sender.clone(),
                    );

                    self.display_settings = Some(display_settings);
                }
            }
            DisplayMsg::PrimaryMonitorChanged(index) => {
                self.primary_monitor = self
                    .state
                    .as_ref()
                    .and_then(|s| s.monitors.get(*index))
                    .map(|m| m.spec.clone());
                self.showing_apply_titlebar = true;
            }
            DisplayMsg::PushDisplaySettings(monitor) => {
                if let Some(state) = &self.state {
                    let display_settings = Self::build_display_settings(
                        monitor,
                        matches!(self.config_type, ConfigType::Mirror),
                        state,
                        sender.clone(),
                    );

                    self.display_settings_in_page = Some(display_settings);

                    widgets.navigation_view.push(&widgets.display_settings_page);
                    widgets
                        .display_settings_page_title
                        .set_title(&monitor.get_output_ui_name());
                }
            }
        }

        self.update(message, sender.clone(), root).await;
        self.update_view(widgets, sender);
        self.update_ui(widgets);
    }
}

impl DisplayModel {
    fn update_ui(&self, widgets: &mut <Self as AsyncComponent>::Widgets) {
        if let Some(display_settings_group) = &self.display_settings_group {
            widgets.display_settings_group = display_settings_group.widget().to_owned();
        }
    }

    async fn get_display_state() -> anyhow::Result<DisplayState> {
        let conn = Connection::session().await?;
        let proxy = DisplayConfigProxy::new(&conn).await?;
        let state = proxy.get_current_state().await?;

        Ok(DisplayState::from(state))
    }

    fn build_display_settings(
        monitor: &Monitor,
        is_clone_mode: bool,
        state: &DisplayState,
        sender: AsyncComponentSender<Self>,
    ) -> Controller<DisplaySettings> {
        let monitor = monitor.clone();
        let logical_monitor = state
            .logical_monitors
            .iter()
            .find(|lm| lm.monitors.contains(&monitor.spec));

        DisplaySettings::builder()
            .launch(DisplaySettingsInit {
                monitor,
                is_cloning: is_clone_mode,
                can_disable: state.monitors.len() > 1,
                logical_monitor: logical_monitor.cloned(),
            })
            .forward(sender.input_sender(), |output| match output {
                DisplaySettingsOutput::Changed() => DisplayMsg::DisplayConfigChanged(),
            })
    }
}
