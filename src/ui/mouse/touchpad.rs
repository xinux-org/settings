use relm4::adw::prelude::*;
use relm4::prelude::*;

use crate::ui::mouse::components::choice::{Alternate, ChoiceOutput, Default, Choice, ChoiceInit};

use crate::ui::mouse::components::pointer_speed::{PointerSpeed, PointerSpeedInit};
use crate::ui::mouse::mouse_page::{MouseMsg, MouseSettings};

#[derive(Debug)]
pub struct Touchpad {
    settings: MouseSettings,
    send_events: bool,
    disable_while_typing: bool,
    speed_controller: Controller<PointerSpeed>,

    // click-method: "areas"; "fingers"
    secondary_click: bool,
    secondary_click_controller: Controller<Choice>,

    tap_to_click: bool,

    scroll_method: bool,
    scroll_method_controller: Controller<Choice>,

    natural_scroll: bool,
    natural_scroll_controller: Controller<Choice>,
}

#[derive(Debug)]
pub enum TouchpadMsg {
    SendEvents(bool),
    DisableWhileTyping(bool),
    SecondaryClick(bool),
    TapToClick(bool),
    ScrollMethod(bool),
    ScrollDirection(bool),

    Noop,
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

                    add = model.secondary_click_controller.widget(),
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

                                append = &gtk::Picture {
                                    set_hexpand: true,
                                    set_halign: gtk::Align::Center,
                                    set_margin_top: 6,
                                    set_margin_bottom: 6,
                                    set_margin_start: 6,
                                    set_margin_end: 6,
                                    set_height_request: 128,

                                    set_paintable: Some(&tap_to_click_media),
                                },
                            },
                        }
                    },
                },

                add = &adw::PreferencesGroup {
                    #[watch]
                    set_sensitive: model.send_events,
                    set_title: "Scroll Method",

                    add = model.scroll_method_controller.widget(),
                },

                add = &adw::PreferencesGroup {
                    #[watch]
                    set_sensitive: model.send_events,
                    set_title: "Scroll Direction",

                    add = model.natural_scroll_controller.widget(),
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

        let tap_to_click_media = gtk::MediaFile::for_filename(format!(
                        "{}/src/ui/mouse/assets/tap-to-click.webm",
                        std::env::current_dir().unwrap().to_str().unwrap()
                    )
);
        tap_to_click_media.set_loop(true);
        tap_to_click_media.play();

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
        let secondary_click_controller = Choice::builder()
            .launch(ChoiceInit {
                title: "Secondary Click".to_string(),
                key: "click-method".to_string(),
                settings: settings.touchpad.clone(),

                default: Default {
                    value: "fingers".to_variant(),
                    media: format!(
                        "{}/src/ui/mouse/assets/push-to-click-anywhere.webm",
                        std::env::current_dir().unwrap().to_str().unwrap()
                    ),
                    title: "Two Finger Push".to_string(),
                    subtitle: 
                    "Push anywhere with 2 fingers".to_string(),

                    enabled: "fingers".to_variant() == click_method.to_variant(),
                },

                alternate: Alternate {
                    value: "areas".to_variant(),
                    media: format!(
                        "{}/src/ui/mouse/assets/push-areas.webm",
                        std::env::current_dir().unwrap().to_str().unwrap()
                    ),
                    title: "Corner Push".to_string(),
                    subtitle: 
                    "Push with a single finger in the corner".to_string(),

                    enabled: "areas".to_variant() == click_method.to_variant(),
                },
            })
            .forward(sender.input_sender(), |out| match out {
                ChoiceOutput::Noop => TouchpadMsg::Noop,
            });

        let tap_to_click = settings.touchpad.boolean("tap-to-click");

        let scroll_method = settings.touchpad.boolean("two-finger-scrolling-enabled");
        let scroll_method_controller = Choice::builder()
            .launch(ChoiceInit {
                title: "Scroll Method".to_string(),
                key: "natural-scroll".to_string(),
                settings: settings.touchpad.clone(),

                default: Default {
                    value: false.to_variant(),
                    media: format!(
                        "{}/src/ui/mouse/assets/scroll-2finger.webm",
                        std::env::current_dir().unwrap().to_str().unwrap()
                    ),
                    title: "Two Finger".to_string(),
                    subtitle: 
                    "Drag two fingers on the touchpad".to_string(),

                    enabled: false.to_variant() == scroll_method.to_variant(),
                },

                alternate: Alternate {
                    value: true.to_variant(),
                    media: format!(
                        "{}/src/ui/mouse/assets/edge-scroll.webm",
                        std::env::current_dir().unwrap().to_str().unwrap()
                    ),
                    title: "Edge".to_string(),
                    subtitle: 
                    "Drag one finger on the edge".to_string(),

                    enabled: true.to_variant() == scroll_method.to_variant()
                },
            })
            .forward(sender.input_sender(), |out| match out {
                ChoiceOutput::Noop => TouchpadMsg::Noop,
            });

        let natural_scroll = settings.touchpad.boolean("natural-scroll");
        let natural_scroll_controller = Choice::builder()
            .launch(ChoiceInit {
                title: "Scroll Direction".to_string(),
                key: "natural-scroll".to_string(),
                settings: settings.touchpad.clone(),

                default: Default {
                    value: false.to_variant(),
                    media: format!(
                        "{}/src/ui/mouse/assets/touch-scroll-traditional.webm",
                        std::env::current_dir().unwrap().to_str().unwrap()
                    ),
                    title: "Traditional".to_string(),
                    subtitle: 
                    "Scrolling moves the view".to_string(),

                    enabled: false.to_variant() == natural_scroll.to_variant(),
                },

                alternate: Alternate {
                    value: true.to_variant(),
                    media: format!(
                        "{}/src/ui/mouse/assets/touch-scroll-natural.webm",
                        std::env::current_dir().unwrap().to_str().unwrap()
                    ),
                    title: "Natural".to_string(),
                    subtitle: 
                    "Scrolling moves the view".to_string(),

                    enabled: true.to_variant() == natural_scroll.to_variant(),
                },
            })
            .forward(sender.input_sender(), |out| match out {
                ChoiceOutput::Noop => TouchpadMsg::Noop,
            });


        let model = Self {
            settings,

            send_events,
            disable_while_typing,
            speed_controller,

            secondary_click,
            secondary_click_controller,

            tap_to_click,

            scroll_method,
            scroll_method_controller,

            natural_scroll,
            natural_scroll_controller,
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
            TouchpadMsg::Noop => {},
        }
    }
}
