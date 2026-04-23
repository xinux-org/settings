use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

use gtk::gio::Settings;
use crate::ui::mouse::mouse_page::MouseMsg;

#[derive(Debug, Clone)]
pub struct Mouse {}

#[relm4::component(pub)]
impl SimpleComponent for Mouse {
    type Init = ();
    type Input = ();
    type Output = MouseMsg;

    view! {
        #[root]
        adw::ToolbarView {
            set_top_bar_style: adw::ToolbarStyle::Flat,

            add_top_bar = &adw::HeaderBar {
                #[wrap(Some)]
                set_title_widget = &adw::WindowTitle {
                    set_title: "Mouse & Touchpad",
                }
            },

            #[wrap(Some)]
            set_content = &adw::PreferencesPage {
                add = &adw::PreferencesGroup {
                    set_title: "General",

                    add = &adw::ActionRow {
                        set_title: "Primary Button",
                        set_subtitle: "Order of physical buttons on mice and touchpads",

                        add_suffix = &gtk::Box {
                            set_spacing: 0,
                            set_halign: gtk::Align::End,
                            set_valign: gtk::Align::Center,
                            add_css_class: "linked",

                            #[name= "left" ]
                            append = &gtk::ToggleButton {
                                set_group: Some(&right),
                                set_label: "Left",
                                set_active: true,
                            },

                            #[name= "right" ]
                            append = &gtk::ToggleButton {
                                set_label: "Right",
                            },
                        }
                    },
                },


                add = &adw::PreferencesGroup {
                    set_title: "Mouse",

                    add = &adw::ActionRow {
                        set_title: "Pointer Speed",

                        add_suffix = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 6,
                            set_halign: gtk::Align::Fill,
                            set_valign: gtk::Align::Center,
                            set_hexpand: true,

                            append = &gtk::Label {
                                set_label: "Slow",
                                add_css_class: "dim-label",
                            },

                            append = &gtk::Scale {
                                set_orientation: gtk::Orientation::Horizontal,
                                set_hexpand: true,
                                set_draw_value: false,
                                set_range: (0.0, 1.0),
                                set_value: 0.6,
                            },

                            append = &gtk::Label {
                                set_label: "Fast",
                                add_css_class: "dim-label",
                            },
                        }
                    },

                    add = &adw::SwitchRow {
                        set_title: "Mouse Acceleration",
                        set_subtitle: "Recommended for most users and applications",
                        set_active: true,
                    },
                },

                add = &adw::PreferencesGroup {
                    set_title: "Scroll Direction",

                    add = &adw::ActionRow {

                        add_suffix = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 12,
                            set_homogeneous: true,
                            set_hexpand: true,

                            append = &gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_spacing: 6,

                                append = &gtk::Frame {
                                    set_hexpand: true,

                                    #[wrap(Some)]
                                    set_child = &gtk::Image {
                                        set_icon_name: Some("input-mouse-symbolic"),
                                        set_pixel_size: 64,
                                    },
                                },

                                append = &gtk::Box {
                                    set_orientation: gtk::Orientation::Horizontal,
                                    set_spacing: 6,
                                    set_halign: gtk::Align::Start,

                                    #[name = "traditional"]
                                    append = &gtk::CheckButton {
                                        set_active: true,
                                    },

                                    append = &gtk::Box {
                                        set_orientation: gtk::Orientation::Vertical,

                                        append = &gtk::Label {
                                            set_label: "Traditional",
                                            set_halign: gtk::Align::Start,
                                        },

                                        append = &gtk::Label {
                                            set_label: "Scrolling moves the view",
                                            set_halign: gtk::Align::Start,
                                            add_css_class: "dim-label",
                                            add_css_class: "caption",
                                        },
                                    },
                                },
                            },

                            append = &gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_spacing: 6,

                                append = &gtk::Frame {
                                    set_hexpand: true,
                                    #[wrap(Some)]
                                    set_child = &gtk::Image {
                                        set_icon_name: Some("input-mouse-symbolic"),
                                        set_pixel_size: 64,
                                    },
                                },

                                append = &gtk::Box {
                                    set_orientation: gtk::Orientation::Horizontal,
                                    set_spacing: 6,
                                    set_halign: gtk::Align::Start,

                                    append = &gtk::CheckButton {
                                        set_group: Some(&traditional),
                                        set_active: false,
                                    },

                                    append = &gtk::Box {
                                        set_orientation: gtk::Orientation::Vertical,

                                        append = &gtk::Label {
                                            set_label: "Natural",
                                            set_halign: gtk::Align::Start,
                                        },

                                        append = &gtk::Label {
                                            set_label: "Scrolling moves the content",
                                            set_halign: gtk::Align::Start,
                                            add_css_class: "dim-label",
                                            add_css_class: "caption",
                                        },
                                    },
                                },
                            },
                        }
                    },
                },

                add = &adw::PreferencesGroup {
                    add = &adw::ButtonRow {
                            set_title: "Test Settings",
                            set_end_icon_name: Some("go-next-symbolic"),
                    },
                },
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let settings = Settings::new("org.gnome.desktop.peripherals.touchpad");
        let touchpad_exists = settings.boolean("send-events");
        println!("Touchpad: {:?}\n\n\n\n\n\n\n\n\n\n", touchpad_exists);
        let model = Self {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
