use relm4::adw::prelude::*;
use relm4::prelude::*;

use crate::ui::mouse::components::pointer_speed::PointerSpeed;
use crate::ui::mouse::components::pointer_speed::PointerSpeedInit;
use crate::ui::mouse::mouse_page::{MouseMsg, MouseSettings};

#[derive(Debug)]
pub struct Touchpad {
    settings: MouseSettings,
    send_events: bool,
    disable_while_typing: bool,
    speed_controller: Controller<PointerSpeed>,

    // click-method: "areas"; "fingers"
    secondary_click: bool,

    tap_to_click: bool,

    scroll_method: bool,

    natural_scroll: bool,
}

#[derive(Debug)]
pub enum TouchpadMsg {
    SendEvents(bool),
    DisableWhileTyping(bool),
    SecondaryClick(bool),
    TapToClick(bool),
    ScrollMethod(bool),
    ScrollDirection(bool),
}

#[relm4::component(pub)]
impl SimpleComponent for Touchpad {
    type Init = ();
    type Input = TouchpadMsg;
    type Output = MouseMsg;

    view! {
        #[root]
        adw::ToolbarView {
            set_top_bar_style: adw::ToolbarStyle::Flat,

            #[wrap(Some)]
            set_content = &adw::PreferencesPage {
                add = &adw::PreferencesGroup {
                    add = &adw::SwitchRow {
                        set_title: "Touchpad",
                        set_active: model.send_events,

                        connect_active_notify[sender] => move |row| {
                            sender.input(TouchpadMsg::SendEvents(row.is_active()));
                        }
                    },
                },

                add = &adw::PreferencesGroup {
                    #[watch]
                    set_sensitive: model.send_events,
                    set_title: "General",

                    add = &adw::SwitchRow {
                        set_title: "Disable Touchpad While Typing",
                        set_active: model.disable_while_typing,

                        connect_active_notify[sender] => move |row| {
                            sender.input(TouchpadMsg::DisableWhileTyping(row.is_active()));
                        }
                    },

                    add = model.speed_controller.widget(),
                },

                add = &adw::PreferencesGroup {
                    #[watch]
                    set_sensitive: model.send_events,
                    set_title: "Secondary Click",

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

                                    #[name = "fingers"]
                                    append = &gtk::CheckButton {
                                        #[watch]
                                        set_active: model.secondary_click,
                                        connect_toggled[sender] => move |btn| {
                                            if  btn.is_active() {
                                                sender.input(TouchpadMsg::SecondaryClick(btn.is_active()));
                                            }
                                        },
                                    },

                                    append = &gtk::Box {
                                        set_orientation: gtk::Orientation::Vertical,

                                        append = &gtk::Label {
                                            set_label: "Two Finger Push",
                                            set_halign: gtk::Align::Start,
                                        },

                                        append = &gtk::Label {
                                            set_label: "Push anywhere with 2 fingers",
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
                                        set_group: Some(&fingers),

                                        #[watch]
                                        set_active: !model.secondary_click,
                                        connect_toggled[sender] => move |btn| {
                                            if  btn.is_active() {
                                                sender.input(TouchpadMsg::SecondaryClick(!btn.is_active()));
                                            }
                                        },
                                    },

                                    append = &gtk::Box {
                                        set_orientation: gtk::Orientation::Vertical,

                                        append = &gtk::Label {
                                            set_label: "Corner Push",
                                            set_halign: gtk::Align::Start,
                                        },

                                        append = &gtk::Label {
                                            set_label: "Push with a single finger in the corner",
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
                    #[watch]
                    set_sensitive: model.send_events,
                    set_title: "Tap to Click",

                    add = &adw::ActionRow {

                        add_suffix = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 12,
                            set_hexpand: true,

                            append = &gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_spacing: 6,

                                append = &adw::SwitchRow {
                                    #[watch]
                                    set_active: model.tap_to_click,
                                    connect_active_notify[sender] => move |btn| {
                                        sender.input(TouchpadMsg::TapToClick(btn.is_active()));
                                    },
                                },

                                append = &gtk::Frame {
                                    set_hexpand: true,
                                    #[wrap(Some)]
                                    set_child = &gtk::Image {
                                        set_icon_name: Some("input-mouse-symbolic"),
                                        set_pixel_size: 64,
                                    },
                                },
                            },
                        }
                    },
                },

                add = &adw::PreferencesGroup {
                    #[watch]
                    set_sensitive: model.send_events,
                    set_title: "Scroll Method",

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

                                    #[name = "two_finger"]
                                    append = &gtk::CheckButton {
                                        #[watch]
                                        set_active: model.scroll_method,
                                        connect_toggled[sender] => move |btn| {
                                            if  btn.is_active() {
                                                sender.input(TouchpadMsg::ScrollMethod(btn.is_active()));
                                            }
                                        },
                                    },

                                    append = &gtk::Box {
                                        set_orientation: gtk::Orientation::Vertical,

                                        append = &gtk::Label {
                                            set_label: "Two Finger",
                                            set_halign: gtk::Align::Start,
                                        },

                                        append = &gtk::Label {
                                            set_label: "Drag two fingers on the touchpad",
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
                                        set_group: Some(&two_finger),

                                        #[watch]
                                        set_active: !model.scroll_method,
                                        connect_toggled[sender] => move |btn| {
                                            if  btn.is_active() {
                                                sender.input(TouchpadMsg::ScrollMethod(!btn.is_active()));
                                            }
                                        },
                                    },

                                    append = &gtk::Box {
                                        set_orientation: gtk::Orientation::Vertical,

                                        append = &gtk::Label {
                                            set_label: "Edge",
                                            set_halign: gtk::Align::Start,
                                        },

                                        append = &gtk::Label {
                                            set_label: "Drag one finger on the edge",
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
                    #[watch]
                    set_sensitive: model.send_events,
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
                                        connect_toggled[sender] => move |btn| {
                                            if  btn.is_active() {
                                                sender.input(TouchpadMsg::ScrollDirection(!btn.is_active()));
                                            }
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
                                        connect_toggled[sender] => move |btn| {
                                            if  btn.is_active() {
                                                sender.input(TouchpadMsg::ScrollDirection(btn.is_active()));
                                            }
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

            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let settings = MouseSettings::new();

        let events = settings.touchpad.string("send-events");
        let send_events = events == String::from("enabled");
        let disable_while_typing = settings.touchpad.boolean("disable-while-typing");
        let speed = settings.touchpad.value("speed").get::<f64>().unwrap();

        let speed_controller = PointerSpeed::builder()
            .launch(PointerSpeedInit {
                speed,
                settings: settings.touchpad.clone(),
            })
            .detach();

        let click_method = settings.touchpad.string("click-method").to_string();
        let secondary_click = click_method == "fingers";

        let tap_to_click = settings.touchpad.boolean("tap-to-click");

        let scroll_method = settings.touchpad.boolean("two-finger-scrolling-enabled");

        let natural_scroll = settings.touchpad.boolean("natural-scroll");

        let model = Self {
            settings,

            send_events,
            disable_while_typing,
            speed_controller,

            secondary_click,

            tap_to_click,

            scroll_method,

            natural_scroll,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            TouchpadMsg::SendEvents(state) => {
                self.send_events = state;

                let variant = if state { "enabled" } else { "disabled" };

                let _ = self
                    .settings
                    .touchpad
                    .set_value("send-events", &variant.to_variant());
            }
            TouchpadMsg::DisableWhileTyping(state) => {
                self.disable_while_typing = state;

                let _ = self
                    .settings
                    .touchpad
                    .set_value("disable-while-typing", &state.to_variant());
            }
            TouchpadMsg::SecondaryClick(state) => {
                self.secondary_click = state;

                let variant = if state {
                    "fingers".to_variant()
                } else {
                    "areas".to_variant()
                };

                let _ = self.settings.touchpad.set_value("click-method", &variant);
            }
            TouchpadMsg::TapToClick(state) => {
                self.tap_to_click = state;

                let _ = self
                    .settings
                    .touchpad
                    .set_value("tap-to-click", &state.to_variant());
            }
            TouchpadMsg::ScrollMethod(state) => {
                self.scroll_method = state;

                println!("State: {:?}", state);

                if state {
                    let _ = self
                        .settings
                        .touchpad
                        .set_value("edge-scrolling-enabled", &false.to_variant());

                    let _ = self
                        .settings
                        .touchpad
                        .set_value("two-finger-scrolling-enabled", &true.to_variant());
                } else {
                    let _ = self
                        .settings
                        .touchpad
                        .set_value("edge-scrolling-enabled", &true.to_variant());

                    let _ = self
                        .settings
                        .touchpad
                        .set_value("two-finger-scrolling-enabled", &false.to_variant());
                }
            }
            TouchpadMsg::ScrollDirection(state) => {
                self.natural_scroll = state;

                let _ = self
                    .settings
                    .touchpad
                    .set_value("natural-scroll", &state.to_variant());
            }
        }
    }
}
