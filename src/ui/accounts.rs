use crate::ui::window::AppMsg;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct AccountsModel {}

#[relm4::component(pub)]
impl SimpleComponent for AccountsModel {
    type Init = ();
    type Input = ();
    type Output = AppMsg;

    view! {
        adw::ToolbarView {
            set_top_bar_style: adw::ToolbarStyle::Flat,

            add_top_bar = &adw::HeaderBar {
                #[wrap(Some)]
                set_title_widget = &adw::WindowTitle{
                    set_title: "Online Accounts"
                }
            },

            gtk::Box {
                set_halign: gtk::Align::Center,
                set_orientation: gtk::Orientation::Vertical,

                gtk::Box {
                    set_margin_top: 10,
                    set_hexpand: true,

                    gtk::Label {
                        set_halign: gtk::Align::Center,
                        set_label: "Allow apps to access online services by connecting your cloud accounts",
                        add_css_class: "dim-label",
                    },
                },

                adw::PreferencesGroup {
                    set_margin_top: 10,
                    set_title: "Connect an Account",

                    adw::ActionRow {
                        set_title: "Nextcloud",
                        set_subtitle: "Calendar, contacts, files",
                        set_activatable: true,

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("software-update-urgent-symbolic"),
                            set_pixel_size: 16,
                        },

                        add_suffix = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 6,

                            gtk::Image {
                                set_icon_name: Some("go-next-symbolic"),
                                set_pixel_size: 16,
                            }
                        }
                    },

                    adw::ActionRow {
                        set_title: "Google",
                        set_subtitle: "Email, calendar, contacts, files",
                        set_activatable: true,

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("software-update-urgent-symbolic"),
                            set_pixel_size: 16,
                        },

                        add_suffix = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 6,

                            gtk::Image {
                                set_icon_name: Some("go-next-symbolic"),
                                set_pixel_size: 16,
                            }
                        }
                    },

                    adw::ActionRow {
                        set_title: "Google",
                        set_subtitle: "Email, calendar, contacts, files",
                        set_activatable: true,

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("software-update-urgent-symbolic"),
                            set_pixel_size: 16,
                        },

                        add_suffix = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 6,

                            gtk::Image {
                                set_icon_name: Some("go-next-symbolic"),
                                set_pixel_size: 16,
                            }
                        }
                    },

                    adw::ActionRow {
                        set_title: "Microsoft 365",
                        set_subtitle: "Email, calendar, contacts, files",
                        set_activatable: true,

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("software-update-urgent-symbolic"),
                            set_pixel_size: 16,
                        },

                        add_suffix = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 6,

                            gtk::Image {
                                set_icon_name: Some("go-next-symbolic"),
                                set_pixel_size: 16,
                            }
                        }
                    },

                    adw::ActionRow {
                        set_title: "Microsoft Exchange",
                        set_subtitle: "Email, calendar, contacts",
                        set_activatable: true,

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("software-update-urgent-symbolic"),
                            set_pixel_size: 16,
                        },

                        add_suffix = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 6,

                            gtk::Image {
                                set_icon_name: Some("go-next-symbolic"),
                                set_pixel_size: 16,
                            }
                        }
                    },

                    adw::ActionRow {
                        set_title: "Email Server",
                        set_subtitle: "IMAP/SMTP",
                        set_activatable: true,

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("software-update-urgent-symbolic"),
                            set_pixel_size: 16,
                        },

                        add_suffix = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 6,

                            gtk::Image {
                                set_icon_name: Some("go-next-symbolic"),
                                set_pixel_size: 16,
                            }
                        }
                    },

                    adw::ActionRow {
                        set_title: "Calendar, Contacts and Files",
                        set_subtitle: "WebDAV",
                        set_activatable: true,

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("software-update-urgent-symbolic"),
                            set_pixel_size: 16,
                        },

                        add_suffix = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 6,

                            gtk::Image {
                                set_icon_name: Some("go-next-symbolic"),
                                set_pixel_size: 16,
                            }
                        }
                    },

                    adw::ActionRow {
                        set_title: "Enterprise Authentication",
                        set_subtitle: "Kerberos",
                        set_activatable: true,

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("software-update-urgent-symbolic"),
                            set_pixel_size: 16,
                        },

                        add_suffix = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 6,

                            gtk::Image {
                                set_icon_name: Some("go-next-symbolic"),
                                set_pixel_size: 16,
                            }
                        }
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
