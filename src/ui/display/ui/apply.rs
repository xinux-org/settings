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
    Applied,
    NoChanges,
    Applicable,
    ApplyFailed,
    NoApplicable,
}

impl Display for ApplyState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = match self {
            ApplyState::Applied => None,
            ApplyState::NoChanges => None,
            ApplyState::Applicable => Some("Apply Changes?"),
            ApplyState::ApplyFailed => Some("Apply failed!"),
            ApplyState::NoApplicable => Some("Cannot Apply Changes!"),
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

    additional_fields! {
        title_widget: Option<adw::WindowTitle>,
    }

    view! {
        #[name(root)]
        adw::HeaderBar {
            set_show_title: true,

            #[watch]
            set_show_end_title_buttons: matches!(model.state, ApplyState::NoChanges | ApplyState::Applied),
            #[watch]
            set_show_start_title_buttons: matches!(model.state, ApplyState::NoChanges | ApplyState::Applied),

            pack_start = &gtk::Button {
                set_can_shrink: true,
                set_use_underline: true,
                set_label: &gettext("Cancel"),
                connect_clicked => ApplyMsg::Cancel,
                #[watch]
                set_visible: !matches!(model.state, ApplyState::NoChanges | ApplyState::Applied),
            },

            pack_end = &gtk::Button {
                set_can_shrink: true,
                set_use_underline: true,
                set_label: &gettext("Apply"),
                add_css_class: "suggested-action",
                connect_clicked => ApplyMsg::Apply,
                #[watch]
                set_visible: !matches!(model.state, ApplyState::NoChanges | ApplyState::Applied),
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

        let title_widget = None;

        let mut widgets = view_output!();

        model.render_title(&mut widgets);

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, message: Self::Input, _sender: AsyncComponentSender<Self>) {
        match message {
            ApplyMsg::Apply => {
                self.state = self
                    .manager
                    .config_apply()
                    .await
                    .inspect_err(|err| tracing::error!("{err}"))
                    .map_or(ApplyState::ApplyFailed, |()| ApplyState::Applied);
            }
            ApplyMsg::Cancel => {
                self.state = ApplyState::NoChanges;
            }
            ApplyMsg::SettingsChanged() => {
                self.state = self
                    .manager
                    .config_is_applicable()
                    .await
                    .inspect_err(|err| tracing::error!("{err}"))
                    .map_or(ApplyState::NoApplicable, |()| ApplyState::Applicable);
            }
        }
    }

    fn post_view() {
        self.render_title(widgets);
    }
}

impl Apply {
    fn render_title(&self, widgets: &mut <Self as SimpleAsyncComponent>::Widgets) {
        let title = self.state.to_string();

        let title_widget: Option<adw::WindowTitle> = match title.is_empty() {
            true => None,
            false => {
                let title_widget = adw::WindowTitle::builder().title(&title).build();
                Some(title_widget)
            }
        };

        widgets.root.set_title_widget(title_widget.as_ref());
    }
}
