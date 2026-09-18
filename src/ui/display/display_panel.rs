use gettextrs::gettext;
use relm4::{adw::prelude::*, prelude::*};

use crate::ui::display::{DisplayConfigManager, DisplayConfigType};

use super::DisplaySettingsModel;

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
                    #[name(displays_titlebar)]
                    add_top_bar = &adw::HeaderBar { set_show_title: true },
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
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let DisplayConfigType::Single(monitor) = init.get_current_config().get_monitor();

        let model = DisplayModel {
            display_settings: DisplaySettingsModel::builder().launch(monitor).detach(),
        };
        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }
}
