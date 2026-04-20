use crate::ui::system::system_page::SystemPageMsg;
use gettextrs::gettext;
use relm4::{
    adw::{self, prelude::*},
    gtk::{self, gio},
    prelude::*,
};

#[derive(Debug)]
pub struct UserPageModel {
    user_model: Controller<UserModel>,
}

#[derive(Debug)]
pub enum UserPageMsg {
    Load(String),
    Noop,
}

#[relm4::component(pub)]
impl SimpleComponent for UserPageModel {
    type Init = UserModelInit;
    type Input = UserPageMsg;
    type Output = SystemPageMsg;

    view! {
        adw::NavigationPage {
            set_title: &gettext("Date & Time"),

            adw::ToolbarView {
                set_top_bar_style: adw::ToolbarStyle::Flat,

                add_top_bar = &adw::HeaderBar {},

                adw::PreferencesPage {
                    #[local_ref]
                    user_model -> adw::PreferencesGroup {},
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let user_model = UserModel::builder()
            .launch(init)
            .forward(sender.input_sender(), |out| match out {
                UserModelOutput::Noop => UserPageMsg::Noop,
            });
        let model = Self { user_model };
        let user_model = model.user_model.widget();

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            UserPageMsg::Noop => {}
            UserPageMsg::Load(username) => {
                self.user_model.emit(UserModelMsg::Load(username));
            }
        }
    }
}

#[derive(Debug)]
pub struct UserModelInit {
    pub name: String,
    pub username: String,
}

#[derive(Debug)]
pub struct UserModel {
    name: String,
}

#[derive(Debug)]
pub enum UserModelMsg {
    Load(String),
}

#[derive(Debug)]
pub enum UserModelOutput {
    // Open(String),
    Noop,
}

#[relm4::component(pub)]
impl SimpleComponent for UserModel {
    type Init = UserModelInit;
    type Input = UserModelMsg;
    type Output = UserModelOutput;

    view! {
        adw::PreferencesGroup {
            adw::PreferencesGroup {
                gtk::Overlay {
                    set_halign: gtk::Align::Center,
                    // set_sensitive: bind template.avatar-editable;
                    // set_has_tooltip: bind template.avatar-editable no-sync-create inverted;
                    set_tooltip_text: Some(&gettext("Unlock to Change This Setting")),

                    #[name(avatar)]
                    adw::Avatar {
                        set_show_initials: true,
                        set_size: 120,
                        set_halign: gtk::Align::Center,
                    },

                    // [overlay]
                    adw::Bin {
                        add_css_class: "cutout-button",

                        set_halign: gtk::Align::End,
                        set_valign: gtk::Align::End,

                        #[name(avatar_edit_button)]
                        gtk::MenuButton {
                            set_tooltip_text: Some(&gettext("Change Avatar")),
                            set_icon_name: "document-edit-symbolic",
                            add_css_class: "circular",
                        }
                    }
                }
            },

            adw::PreferencesGroup {
                #[name(fullname_row)]
                adw::EntryRow {
                    // sensitive: bind template.editable;
                    // has-tooltip: bind template.editable no-sync-create inverted;
                    set_tooltip_text: Some(&gettext("Unlock to Change This Setting")),
                    set_input_purpose: gtk::InputPurpose::Name,
                    set_show_apply_button: true,
                    set_title: &gettext("Name"),
                    set_use_underline: true,
                    #[watch]
                    set_text: &model.name,
                    // connect_activated => UserModel::ChangeUsername,
                },

                #[name(password_row)]
                adw::ActionRow {
                    // sensitive: bind template.editable;
                    // set_has_tooltip: bind template.editable no-sync-create inverted;
                    set_tooltip_text: Some(&gettext("Unlock to Change This Setting")),
                    set_activatable: true,
                    set_title: &gettext("Password"),
                    set_use_underline: true,
                    // set_show_arrow: true,
                    // set_activated => $change_password(template);
                },
            },
        },
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self { name: init.name };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            UserModelMsg::Load(username) => self.name = username,
        }
    }
}
