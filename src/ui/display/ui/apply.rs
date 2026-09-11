use std::{fmt::Display, sync::Arc};

use gettextrs::gettext;
use relm4::{adw::prelude::*, prelude::*};

use crate::ui::display::manager::DisplayConfigManager;

#[derive(Debug)]
pub struct Apply {
    state: ApplyState,
    manager: Arc<DisplayConfigManager>,
}

#[derive(Debug)]
pub enum ApplyState {
    NoChanges,
    Applicable,
    NoApplicalble,
}

impl Display for ApplyState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = match self {
            ApplyState::NoChanges => None,
            ApplyState::Applicable => Some("Apply Changes?"),
            ApplyState::NoApplicalble => Some("Cannot Apply Changes!"),
        };

        f.write_fmt(format_args!("{}", title.map(gettext).unwrap_or_default()))
    }
}

#[derive(Debug)]
pub enum ApplyMsg {
    Apply,
    Cancel,
    SettingsChanged(),
}

#[relm4::component(pub async)]
impl SimpleAsyncComponent for Apply {
    type Init = Arc<DisplayConfigManager>;
    type Input = ApplyMsg;
    type Output = ();

    view! {
        #[name(root)]
        adw::HeaderBar {
            set_show_title: true,

            #[watch]
            set_show_end_title_buttons: matches!(model.state, ApplyState::NoChanges),
            #[watch]
            set_show_start_title_buttons: matches!(model.state, ApplyState::NoChanges),

            #[wrap(Some)]
            set_title_widget = &adw::WindowTitle {
                #[watch]
                set_title: &model.state.to_string(),
            },

            pack_start = &gtk::Button {
                set_can_shrink: true,
                set_use_underline: true,
                set_label: &gettext("Cancel"),
                connect_clicked => ApplyMsg::Cancel,
                #[watch]
                set_visible: !matches!(model.state, ApplyState::NoChanges),
            },

            pack_end = &gtk::Button {
                set_can_shrink: true,
                set_use_underline: true,
                set_label: &gettext("Apply"),
                add_css_class: "suggested-action",
                connect_clicked => ApplyMsg::Apply,
                #[watch]
                set_visible: !matches!(model.state, ApplyState::NoChanges),
                #[watch]
                set_sensitive: matches!(model.state, ApplyState::Applicable),
            },
        }
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = Apply {
            manager: init,
            state: ApplyState::NoChanges,
        };

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, message: Self::Input, _sender: AsyncComponentSender<Self>) {
        match message {
            ApplyMsg::Apply => {}
            ApplyMsg::Cancel => {
                self.state = ApplyState::NoChanges;
            }
            ApplyMsg::SettingsChanged() => match self.manager.config_is_applicable().await {
                Ok(()) => {
                    self.state = ApplyState::Applicable;
                }
                Err(err) => {
                    tracing::error!("{err}");

                    self.state = ApplyState::NoApplicalble;
                }
            },
        }
    }
}
