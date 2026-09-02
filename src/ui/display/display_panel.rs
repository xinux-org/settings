use gettextrs::gettext;
use relm4::{adw::prelude::*, prelude::*};

use super::{DisplayConfigManager, DisplayConfigType, display_settings::DisplaySettingsModel};

#[derive(Debug)]
pub enum DisplayMsg {}

#[derive(Debug)]
pub struct DisplayModel {
    #[allow(dead_code)]
    manager: DisplayConfigManager,
    display_settings: Controller<DisplaySettingsModel>,
}

#[relm4::component(pub async)]
impl SimpleAsyncComponent for DisplayModel {
    type Init = DisplayConfigManager;
    type Input = DisplayMsg;
    type Output = ();

    view! {
        #[root]
        #[name(navigation_view)]
        adw::NavigationView {
            #[name(main_page)]
            adw::NavigationPage {
                set_title: &gettext("Displays"),

                #[wrap(Some)]
                set_child = &adw::ToolbarView {
                    #[name(displays_titlebar)]
                    add_top_bar = &adw::HeaderBar { set_show_title: true },

                    #[wrap(Some)]
                    set_content = &adw::PreferencesPage {
                        #[name(single_display_settings_group)]
                        adw::PreferencesGroup {
                            add = model.display_settings.widget(),
                        },
                    },
                },
            },
        }
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let DisplayConfigType::Single(monitor) = init.get_current_config().get_monitor();

        let model = DisplayModel {
            manager: init,
            display_settings: DisplaySettingsModel::builder().launch(monitor).detach(),
        };

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, message: Self::Input, _sender: AsyncComponentSender<Self>) {
        match message {}
    }
}

impl DisplayModel {}
