use gettextrs::gettext;
use relm4::{adw::prelude::*, prelude::*};

use super::{DisplayConfigManager, DisplayModel};

#[derive(Debug, Default)]
pub struct DisplayPage {
    panel: Option<AsyncController<DisplayModel>>,
}

#[relm4::component(pub async)]
impl AsyncComponent for DisplayPage {
    type Init = ();
    type Input = ();
    type Output = ();
    type CommandOutput = ();

    view! {
        #[root]
        adw::Bin {
            #[name(status_page)]
            adw::StatusPage {
                set_vexpand: true,
                set_icon_name: Some("computer-symbolic"),
                set_title: &gettext("Display Settings Disabled"),
            },
        }
    }

    async fn init(
        _: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let mut model = Self::default();
        let widgets = view_output!();

        match DisplayConfigManager::new().await {
            Ok(manager) => {
                let panel = DisplayModel::builder().launch(manager).detach();
                root.set_child(Some(panel.widget()));
                model.panel = Some(panel);
            }
            Err(error) => {
                tracing::warn!("display config service unavailable: {error}");
            }
        }

        AsyncComponentParts { model, widgets }
    }
}
