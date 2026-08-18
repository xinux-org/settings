use gettextrs::gettext;
use relm4::{adw::prelude::*, prelude::*};

#[derive(Debug)]
pub struct ApplySettings {
    is_visible: bool,
    last_output_error: Option<ApplySettingsOut>,
}

#[derive(Debug)]
pub enum ApplySettingsMsg {
    Apply,
    Cancel,
    SetActive(bool),
}

#[derive(Debug)]
pub enum ApplySettingsOut {
    Apply,
    Cancel,
}

#[relm4::component(pub)]
impl SimpleComponent for ApplySettings {
    type Init = ();
    type Input = ApplySettingsMsg;
    type Output = ApplySettingsOut;

    view! {
        adw::HeaderBar {
            #[watch]
            set_visible: model.is_visible,

            set_show_end_title_buttons: false,
            set_show_start_title_buttons: false,

            #[wrap(Some)]
            set_title_widget = &adw::WindowTitle {
                set_title: &gettext("Apply Changes?"),
            },

            pack_start = &gtk::Button {
                set_can_shrink: true,
                set_use_underline: true,
                set_label: &gettext("Cancel"),
                connect_clicked => ApplySettingsMsg::Cancel,
            },
            pack_end = &gtk::Button {
                set_can_shrink: true,
                set_use_underline: true,
                set_label: &gettext("Apply"),
                add_css_class: "suggested-action",
                connect_clicked => ApplySettingsMsg::Apply,
            },
        },
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ApplySettings {
            is_visible: false,
            last_output_error: None,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            ApplySettingsMsg::Apply => {
                self.last_output_error = sender.output(ApplySettingsOut::Apply).err();
            }
            ApplySettingsMsg::Cancel => {
                self.last_output_error = sender.output(ApplySettingsOut::Cancel).err();
            }
            ApplySettingsMsg::SetActive(is_active) => {
                self.is_visible = is_active;
            }
        }
    }
}
