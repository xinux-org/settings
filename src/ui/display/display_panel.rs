use std::sync::Arc;

use gettextrs::gettext;
use relm4::{adw::prelude::*, prelude::*};

use super::{
    DisplayConfigManager, DisplayConfigType,
    apply::{Apply, ApplyMsg},
    display_settings::{DisplaySettingsModel, DisplaySettingsOutput},
};

#[derive(Debug)]
pub struct DisplayModel {
    display_settings: Controller<DisplaySettingsModel>,
}

#[relm4::component(pub async)]
impl SimpleAsyncComponent for DisplayModel {
    type Init = DisplayConfigManager;
    type Input = ();
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
                    add_top_bar = model.apply.widget(),

                    #[wrap(Some)]
                    set_content = &adw::PreferencesPage {
                        #[name(single_display_settings_group)]
                        adw::PreferencesGroup {
                            model.display_settings.widget(),
                        },
                    },
                },
            },
        }
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let manager = Arc::new(init);

        let DisplayConfigType::Single(monitor) = manager.get_current_config().get_monitor();

        let model = DisplayModel {
            display_settings: DisplaySettingsModel::builder().launch(monitor).detach(),
        };

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, message: Self::Input, _sender: AsyncComponentSender<Self>) {
        match message {
            DisplayMsg::DisplaySettingsChanged() => {
                self.apply.sender().emit(ApplyMsg::SettingsChanged());
            }
        }
    }
}
