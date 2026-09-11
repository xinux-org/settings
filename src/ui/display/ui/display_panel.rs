use std::sync::Arc;

use gettextrs::gettext;
use relm4::{adw::prelude::*, prelude::*};

use crate::ui::display::{DisplayConfigManager, DisplayConfigType};

use super::{Apply, ApplyMsg, DisplaySettingsModel, DisplaySettingsOutput};

#[derive(Debug)]
pub struct DisplayModel {
    apply: AsyncController<Apply>,
    display_settings: Controller<DisplaySettingsModel>,
}

#[derive(Debug)]
pub enum DisplayMsg {
    DisplaySettingsChanged(),
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
            apply: Apply::builder().launch(manager).detach(),
            display_settings: DisplaySettingsModel::builder().launch(monitor).forward(
                sender.input_sender(),
                |output| match output {
                    DisplaySettingsOutput::SettingsChanged() => {
                        DisplayMsg::DisplaySettingsChanged()
                    }
                },
            ),
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
