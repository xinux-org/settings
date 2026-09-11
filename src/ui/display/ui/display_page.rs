use gettextrs::gettext;
use relm4::{adw::prelude::*, prelude::*};

use crate::ui::display::DisplayConfigManager;

use super::DisplayModel;

#[derive(Debug, Default)]
pub struct DisplayPage {
    panel: Option<AsyncController<DisplayModel>>,
}

#[relm4::component(pub async)]
impl SimpleAsyncComponent for DisplayPage {
    type Init = ();
    type Input = ();
    type Output = ();

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
        let panel = DisplayConfigManager::new()
            .await
            .inspect_err(|err| tracing::warn!("display config service unavailable: {err}"))
            .ok()
            .map(|manager| DisplayModel::builder().launch(manager).detach());

        let model = DisplayPage { panel };

        let widgets = view_output!();

        if let Some(panel) = model.panel.as_ref() {
            root.set_child(Some(panel.widget()));
        }

        AsyncComponentParts { model, widgets }
    }
}
