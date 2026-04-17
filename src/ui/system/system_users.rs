use crate::ui::system::system_page::SystemPageMsg;
use adw::{ActionRow, gtk::ListBoxRow};
use gettextrs::gettext;
use relm4::{
    adw::{self, prelude::*},
    gtk::{self},
    prelude::*,
};

#[derive(Debug, Default)]
pub struct SystemUsersPage {}

#[derive(Debug)]
pub enum SystemUsersMsg {}

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

                #[name(navigation)]
                adw::NavigationView {
                    add = &adw::NavigationPage {
                        #[name(preferences_page)]
                        adw::PreferencesPage {
                            // #[local_ref]
                            // current_user -> adw::PreferencesGroup {},
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
                                    // apply => $fullname_entry_apply_cb(template);
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
                            },

                        },
                    }
                },


                // adw::ButtonRow {
                //     #[watch]
                //     // set_visible: model.is_enterprise_enabled,
                //     set_title: "Add Enterprise Login",
                //     set_use_underline: true,
                //     set_end_icon_name: Some("go-next-symbolic"),
                //     // connect_activated => MoveMsg::AddEnterpriseUser,
                // }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {}
}
