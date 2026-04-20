use crate::ui::system::{
    system_page::SystemPageMsg,
    system_user::{UserModel, UserModelInit, UserModelOutput},
};
use gettextrs::gettext;
use relm4::{
    adw::{self, prelude::*},
    gtk::{self},
    prelude::*,
};
#[derive(Debug)]
pub struct SystemUsersPage {
    currnet_user_model: Controller<UserModel>,
}

#[derive(Debug)]
pub enum SystemUsersMsg {
    OpenUser(String),
    Noop,
}

#[relm4::component(pub)]
impl SimpleComponent for SystemUsersPage {
    type Init = ();
    type Input = SystemUsersMsg;
    type Output = SystemPageMsg;

    view! {
        adw::NavigationPage {
            set_tag: Some("users"),
            set_title: &gettext("Users"),

            adw::ToolbarView {
                set_top_bar_style: adw::ToolbarStyle::Flat,

                add_top_bar = &adw::HeaderBar {},
                add_top_bar = &adw::Banner {
                    set_align: gtk::Align::Fill,
                    set_vexpand: true,
                    set_title: "Unlock to edit",
                    #[watch]
                    set_revealed: true,
                    set_button_label: Some("Unlock..."),

                    // connect_button_clicked => SystemRegionLanguageMsg::LogOut,
                },

                #[name(preferences_page)]
                adw::PreferencesPage {
                    #[local_ref]
                    currnet_user_model -> adw::PreferencesGroup {},

                    adw::PreferencesGroup {
                        #[watch]
                        // set_visible: model.show_other_users,
                        set_title: "Other Users",

                        #[name(user_list)]
                        gtk::ListBox {
                            set_selection_mode: gtk::SelectionMode::None,
                            add_css_class: "boxed-list",
                            // connect_row_activated => MoveMsg::UserRowActivated,
                        },

                        adw::ButtonRow {
                            set_title: "Add User",
                            set_end_icon_name: Some("go-next-symbolic"),
                            set_use_underline: true,
                            // connect_activated => MoveMsg::AddUser,
                        },

                        adw::ButtonRow {
                            set_title: "second user test",
                            set_end_icon_name: Some("go-next-symbolic"),
                            set_use_underline: true,
                            connect_activated => SystemUsersMsg::OpenUser(String::from("test user")),
                        },
                    },
                },
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let currnet_user_model = UserModel::builder()
            .launch(UserModelInit {
                name: String::from("value"),
                username: String::from("value"),
            })
            .forward(sender.input_sender(), |out| match out {
                UserModelOutput::Noop => SystemUsersMsg::Noop,
            });

        let model = Self { currnet_user_model };
        let currnet_user_model = model.currnet_user_model.widget();

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            SystemUsersMsg::OpenUser(username) => {
                let _ = sender.output(SystemPageMsg::OpenSystemUserPage(username));
            }
            SystemUsersMsg::Noop => {}
        }
    }
}
