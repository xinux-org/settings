use std::sync::Arc;

use gettextrs::gettext;
use relm4::{adw::prelude::*, prelude::*};

use crate::ui::display::{DisplayConfigManager, DisplayConfigType, DisplayMonitor};

use super::{DisplayGroup, DisplayGroupOut, DisplaySettingsModel};

#[derive(Debug)]
enum DisplayPreference {
    Group(Controller<DisplayGroup>),
    Settings(Controller<DisplaySettingsModel>),
}

impl DisplayPreference {
    fn widget(&self) -> &adw::PreferencesGroup {
        match self {
            DisplayPreference::Group(controller) => controller.widget(),
            DisplayPreference::Settings(controller) => controller.widget(),
        }
    }
}

#[derive(Debug, Default)]
enum NavigationPage {
    #[default]
    Main,
    Settings(Controller<DisplaySettingsModel>),
}

#[derive(Debug)]
pub struct DisplayModel {
    navigation: NavigationPage,

    controller: DisplayPreference,
}

#[derive(Debug)]
pub enum DisplayPanelMsg {
    PagePopped,
    PushDisplaySettings(Arc<DisplayMonitor>),
}

#[relm4::component(pub async)]
impl AsyncComponent for DisplayModel {
    type Init = DisplayConfigManager;
    type Input = DisplayPanelMsg;
    type Output = ();
    type CommandOutput = ();

    view! {
        #[root]
        #[name(navigation_view)]
        adw::NavigationView {
            connect_popped[sender] => move |_, _| {
                sender.input(DisplayPanelMsg::PagePopped);
            },

            #[name(main_page)]
            adw::NavigationPage {
                set_tag: Some("main"),
                set_title: &gettext("Displays"),

                #[wrap(Some)]
                set_child = &adw::ToolbarView {
                    #[name(displays_titlebar)]
                    add_top_bar = &adw::HeaderBar { set_show_title: true },

                    #[wrap(Some)]
                    set_content = &adw::PreferencesPage {
                        adw::PreferencesGroup {
                            #[watch]
                            set_visible: matches!(model.controller, DisplayPreference::Group(_)),

                            #[name(config_type_switcher_row)]
                            adw::ActionRow {
                                set_title: &gettext("Multiple Displays"),

                                #[name(display_config_type)]
                                add_suffix = &adw::ToggleGroup {
                                    set_homogeneous: true,
                                    set_valign: gtk::Align::Center,

                                    add = adw::Toggle {
                                        set_use_underline: true,
                                        // Translators: 'Join' as in 'Join displays'
                                        set_label: Some(&gettext("_Join")),
                                    },

                                    add = adw::Toggle {
                                        set_use_underline: true,
                                        set_label: Some(&gettext("_Mirror")),
                                    },
                                },
                            }
                        },

                        model.controller.widget(),
                    },
                },
            },

            #[name(display_settings_page)]
            adw::NavigationPage {
                set_tag: Some("display"),
                set_title: &gettext("Displays"),

                #[wrap(Some)]
                set_child = &adw::ToolbarView {
                    add_top_bar = &adw::HeaderBar { set_show_title: true },

                    #[wrap(Some)]
                    #[name(display_settings_preference_page)]
                    set_content = &adw::PreferencesPage {

                    }
                }
            }
        }
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let monitor = init.get_current_config().get_monitor();

        let model = DisplayModel {
            navigation: NavigationPage::default(),
            controller: match monitor {
                DisplayConfigType::Single(monitor) => {
                    let it = DisplaySettingsModel::builder().launch(monitor).detach();
                    DisplayPreference::Settings(it)
                }
                DisplayConfigType::Multi(monitors) => {
                    let it = DisplayGroup::builder().launch(monitors).forward(
                        sender.input_sender(),
                        |out| match out {
                            DisplayGroupOut::PushDisplaySettings(monitor) => {
                                DisplayPanelMsg::PushDisplaySettings(monitor)
                            }
                        },
                    );

                    DisplayPreference::Group(it)
                }
            },
        };

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            DisplayPanelMsg::PagePopped => {
                if let NavigationPage::Settings(controller) = &self.navigation {
                    widgets
                        .display_settings_preference_page
                        .remove(controller.widget());
                }

                self.navigation = NavigationPage::Main;
            }
            DisplayPanelMsg::PushDisplaySettings(monitor) => {
                let controller = DisplaySettingsModel::builder().launch(monitor).detach();

                self.navigation = NavigationPage::Settings(controller);
            }
        }

        self.update_view(widgets, sender);
    }

    fn post_view() {
        match &self.navigation {
            NavigationPage::Main => {}
            NavigationPage::Settings(controller) => {
                widgets
                    .display_settings_preference_page
                    .add(controller.widget());
                widgets.navigation_view.push(&widgets.display_settings_page);
            }
        }
    }
}
