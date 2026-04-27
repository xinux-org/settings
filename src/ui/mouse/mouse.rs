use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

use crate::ui::mouse::mouse_page::MouseMsg;
use gtk::gio::Settings;

#[derive(Debug, Clone)]
pub struct MouseSettings {
    pub mouse: Settings,
}

impl MouseSettings {
    pub fn new() -> Self {
        Self {
            mouse: Settings::new("org.gnome.desktop.peripherals.mouse"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Mouse {
    settings: MouseSettings,

    /// key is left-handed
    primary_button: bool,

    /// pointer speed
    speed: f64,

    /// mouse acceleration
    /// default for true, flat for false
    accel_profile: bool,

    /// scroll direction
    natural_scroll: bool,
}

#[derive(Debug, Clone)]
pub enum MousePageMsg {
    PrimaryButton (bool),
    PointerSpeed(f64),
    MouseAcceleration(bool),
    ScrollDirection(bool)
}

#[relm4::component(pub)]
impl SimpleComponent for Mouse {
    type Init = ();
    type Input = MousePageMsg;
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
                                set_active: model.primary_button,
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
                                set_range: (-1.0, 1.0),
                                set_value: model.speed,
                            },

                            append = &gtk::Label {
                                set_label: "Fast",
                                add_css_class: "dim-label",
                            },
                        }
                    },

                    add = &adw::ActionRow {
                        set_title: "Mouse Acceleration",
                        set_subtitle: "Recommended for most users and applications",

                        add_suffix = &gtk::Box {
                            gtk::MenuButton {
                                set_icon_name: "help-about",

                                set_direction: gtk::ArrowType::Down,

                                #[wrap(Some)]
                                set_popover = &gtk::Popover {
                                    set_valign: gtk::Align::Center,

                                    gtk::Label {
                                        set_label: "Turning mouse acceleration off can allow faster and more\nprecise movements, but can also make the mouse more difficult\nto use.",
                                    },
                                },
                            },
                        },

                        add_suffix = &gtk::Switch {
                            set_valign: gtk::Align::Center,
                            #[watch]
                            set_active: model.accel_profile,
                            connect_state_set[sender] => move |_, state| {
                                // sender.input(PowerSavingMsg::SetAutoPowerSaver(state));
                                gtk::glib::Propagation::Proceed
                            },
                        },
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
                                        #[watch]
                                        set_active: !model.natural_scroll,
                                        connect_toggled[sender] => move |state| {
                                            sender.input(MousePageMsg::ScrollDirection(state.is_active()));
                                        },
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

                                        #[watch]
                                        set_active: model.natural_scroll,
                                        connect_toggled[sender] => move |state| {
                                            sender.input(MousePageMsg::ScrollDirection(state.is_active()));
                                        },
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
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let settings = MouseSettings::new();

        let acceleration = settings.mouse.string("accel-profile").to_string();
        let accel_profile = acceleration == String::from("default");
        let primary_button = settings.mouse.boolean("left-handed");
        let speed = settings.mouse.value("speed").get::<f64>().unwrap();
        let natural_scroll = settings.mouse.boolean("natural-scroll");

        let model = Self {
            settings,

            primary_button,
            speed,
            accel_profile,
            natural_scroll,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
