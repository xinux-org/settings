use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

use crate::ui::mouse::components::pointer_speed::PointerSpeed;
use crate::ui::mouse::components::pointer_speed::PointerSpeedInit;
use crate::ui::mouse::mouse_page::{MouseMsg, MouseSettings};

use input::Libinput;
use input::event::EventTrait;
use crate::utils::input::Interface;
use input;

#[derive(Debug)]
pub struct Mouse {
    settings: MouseSettings,

    /// key is left-handed
    left_handed: bool,

    /// pointer speed
    speed_controller: Controller<PointerSpeed>,

    /// mouse acceleration
    /// default for true, flat for false
    accel_profile: bool,

    /// scroll direction
    natural_scroll: bool,
}

#[derive(Debug, Clone)]
pub enum MousePageMsg {
    PrimaryButton(bool),
    MouseAcceleration(bool),
    ScrollDirection(bool),
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
                                #[watch]
                                set_active: !model.left_handed,

                                connect_toggled[sender] => move |btn| {
                                    if btn.is_active() {
                                        sender.input(MousePageMsg::PrimaryButton(!btn.is_active()));
                                    }
                                },
                            },

                            #[name= "right" ]
                            append = &gtk::ToggleButton {
                                set_label: "Right",
                                #[watch]
                                set_active: model.left_handed,

                                connect_toggled[sender] => move |btn| {
                                    if btn.is_active() {
                                        sender.input(MousePageMsg::PrimaryButton(btn.is_active()));
                                    }
                                },
                            },
                        }
                    },
                },


                add = &adw::PreferencesGroup {
                    set_title: "Mouse",

                    add = model.speed_controller.widget(),

                    add = &adw::ActionRow {
                        set_title: "Mouse Acceleration",
                        set_subtitle: "Recommended for most users and applications",
                        set_activatable_widget: Some(&mouse_acceleration),

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
                        #[name = "mouse_acceleration"]
                        add_suffix = &gtk::Switch {
                            set_valign: gtk::Align::Center,
                            #[watch]
                            set_active: model.accel_profile,
                            connect_state_set[sender] => move |_, state| {
                                sender.input(MousePageMsg::MouseAcceleration(state));
                                gtk::glib::Propagation::Proceed
                            },
                        },
                    },

                    ////////////////////////////// gif
                    add = &gtk::Box {
                        set_hexpand: true,
                        set_homogeneous: true,
                        set_spacing: 3,
                        #[name(default_option_box)]
                        gtk::Box {
                            set_accessible_role: gtk::AccessibleRole::Radio,
                            set_orientation: gtk::Orientation::Vertical,
                            set_can_focus: true,
                            set_focusable: true,
                            set_receives_default: true,
                            // accessibility {
                            //     labelled-by: default_option_title;
                            //     described-by: default_option_subtitle;
                            // }
                            #[iterate]
                            add_css_class: ["activatable","card"],
                            // gtk::EventControllerMotion {
                            //     // Read C code blya
                            //     // connect_enter => some sender
                            //     // connect_leave => some sender
                            // },
                            //
                            // gtk::GestureClick {
                            //     released => $on_option_released_cb(default_option_box);
                            // }

                            adw::Bin {
                                set_margin_top: 9,
                                set_margin_bottom: 9,
                                set_margin_start: 9,
                                set_margin_end: 9,

                                #[iterate]
                                add_css_class: ["background","frame"],

                                #[name(default_option_picture)]
                                gtk::Picture {
                                    set_hexpand: true,
                                    set_halign: gtk::Align::Center,
                                    set_margin_top: 6,
                                    set_margin_bottom: 6,
                                    set_margin_start: 6,
                                    set_margin_end: 6,
                                    set_height_request: 128,

                                    // #[name(default_option_mask)]
                                    set_filename: Some("/home/bahrom/workplace/xinux/settings/src/ui/mouse/assets/scroll-traditional.webm"),
                                },
                            },
                            gtk::Box {
                                set_margin_start: 15,
                                set_margin_bottom: 9,
                                #[name(default_checkbutton_image)]
                                adw::Bin {
                                    add_css_class: "radio",
                                    set_can_target: false,
                                    set_valign: gtk::Align::Center,
                                    set_halign: gtk::Align::Center
                                },
                                gtk::Box {
                                    set_valign: gtk::Align::Center,
                                    set_margin_start: 6,
                                    set_orientation: gtk::Orientation::Vertical,
                                    add_css_class: "title",

                                    #[name(default_option_title)]
                                    gtk::Label {
                                        set_use_underline: true,
                                        set_xalign: 0.0,
                                        set_wrap: true,
                                        set_wrap_mode: pango::WrapMode::WordChar,
                                        set_label: "aaaaaaaaaaaaah",
                                        add_css_class: "title",
                                    },
                                    #[name(default_option_subtitle)]
                                    gtk::Label {
                                        set_xalign: 0.0,
                                        set_wrap: true,
                                        set_wrap_mode: pango::WrapMode::WordChar,
                                        set_label: "aaaaaaaaaaaaah subtitle",
                                        add_css_class: "subtitle",
                                    },
                                }
                            },
                        },
                        #[name(alternative_option_box)]
                        gtk::Box {
                            set_accessible_role: gtk::AccessibleRole::Radio,
                            set_orientation: gtk::Orientation::Vertical,
                            set_can_focus: true,
                            set_focusable: true,
                            set_receives_default: true,
                            // accessibility {
                            //     labelled-by: default_option_title;
                            //     described-by: default_option_subtitle;
                            // }
                            #[iterate]
                            add_css_class: ["activatable","card"],
                            // gtk::EventControllerMotion {
                            //     // Read C code blya
                            //     // connect_enter => some sender
                            //     // connect_leave => some sender
                            // },
                            //
                            // gtk::GestureClick {
                            //     released => $on_option_released_cb(default_option_box);
                            // }

                            adw::Bin {
                                set_margin_top: 9,
                                set_margin_bottom: 9,
                                set_margin_start: 9,
                                set_margin_end: 9,

                                #[iterate]
                                add_css_class: ["background","frame"],

                                #[name(alternative_option_picture)]
                                gtk::Picture {
                                    set_hexpand: true,
                                    set_halign: gtk::Align::Center,
                                    set_margin_top: 6,
                                    set_margin_bottom: 6,
                                    set_margin_start: 6,
                                    set_margin_end: 6,
                                    set_height_request: 128,

                                    // #[name(default_option_mask)]
                                    // paintable: $CcMaskPaintable {
                                    //     follow-accent: true;
                                    // };
                                },
                            },
                            gtk::Box {
                                set_margin_start: 15,
                                set_margin_bottom: 9,
                                #[name(alternative_checkbutton_image)]
                                adw::Bin {
                                    add_css_class: "radio",
                                    set_can_target: false,
                                    set_valign: gtk::Align::Center,
                                    set_halign: gtk::Align::Center
                                },
                                gtk::Box {
                                    set_valign: gtk::Align::Center,
                                    set_margin_start: 6,
                                    set_orientation: gtk::Orientation::Vertical,
                                    add_css_class: "title",

                                    #[name(alternative_option_title)]
                                    gtk::Label {
                                        set_use_underline: true,
                                        set_xalign: 0.0,
                                        set_wrap: true,
                                        set_wrap_mode: pango::WrapMode::WordChar,
                                        set_label: "aaaaaaaaaaaaah",
                                        add_css_class: "title",
                                    },
                                    #[name(alternative_option_subtitle)]
                                    gtk::Label {
                                        set_xalign: 0.0,
                                        set_wrap: true,
                                        set_wrap_mode: pango::WrapMode::WordChar,
                                        set_label: "aaaaaaaaaaaaah subtitle",
                                        add_css_class: "subtitle",
                                    },
                                }
                            },
                        },
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

        let acceleration = settings.mouse.string("accel-profile");
        let accel_profile = acceleration.as_str() == "default";
        let left_handed = settings.mouse.boolean("left-handed");
        let speed = settings.mouse.value("speed").get::<f64>().unwrap();
        let natural_scroll = settings.mouse.boolean("natural-scroll");

        let mut input = Libinput::new_with_udev(Interface);
        input.udev_assign_seat("seat0").unwrap();
        input.dispatch().unwrap();

        let events: Vec<bool> = input
            .clone()
            .collect::<Vec<input::Event>>()
            .into_iter()
            .map(|event| event.device())
            .filter(|device| device.has_capability(input::DeviceCapability::Gesture))
            .map(|device| device.has_capability(input::DeviceCapability::Gesture))
            .collect();

        let show_header = events.is_empty();

        let speed_controller = PointerSpeed::builder()
            .launch(PointerSpeedInit {
                speed,
                settings: settings.mouse.clone(),
            })
            .detach();

        let model = Self {
            settings,

            left_handed,
            speed_controller,
            accel_profile,
            natural_scroll,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            MousePageMsg::PrimaryButton(state) => {
                self.left_handed = state;

                let _ = self.settings.mouse.set_boolean("left-handed", state);
            }
            MousePageMsg::MouseAcceleration(state) => {
                self.accel_profile = state;

                let profile = if state { "default" } else { "flat" };

                let _ = self.settings.mouse.set_string("accel-profile", profile);
            }
            MousePageMsg::ScrollDirection(state) => {
                self.natural_scroll = state;

                // let _ = self.settings.mouse.set_boolean("natural-scroll", state);
                let _ = self
                    .settings
                    .mouse
                    .set_value("natural-scroll", &state.to_variant());
            }
        }
    }
}

use relm4::gtk::{
    gdk::Texture,
    gdk_pixbuf::Pixbuf,
    gio::{Cancellable, MemoryInputStream},
    glib,
};

fn embedded_logo() -> Texture {
    let bytes = include_bytes!("assets/scroll-traditional.webm");
    let g_bytes = glib::Bytes::from(&bytes.to_vec());
    let stream = MemoryInputStream::from_bytes(&g_bytes);
    let pixbuf = Pixbuf::from_stream(&stream, Cancellable::NONE).unwrap();
    Texture::for_pixbuf(&pixbuf)
}
