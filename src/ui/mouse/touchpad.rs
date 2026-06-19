use gettextrs::gettext;
use relm4::{adw::prelude::*, prelude::*};

use crate::ui::mouse::{
    components::{
        choice::{Alternate, Choice, ChoiceInit, Default},
        pointer_speed::{PointerSpeed, PointerSpeedInit},
        single_choice::{RowOption, SingleChoice, SingleChoiceInit, SingleChoiceOutput},
        template::ChoiceWidget,
    },
    mouse_page::{MouseMsg, MouseSettings},
};

#[derive(Debug)]
pub struct ScrollMethod {
    pub title: String,

    pub default: Default,
    pub alternate: Alternate,
}

#[derive(Debug)]
pub struct Touchpad {
    settings: MouseSettings,
    send_events: bool,
    disable_while_typing: bool,
    speed_controller: Controller<PointerSpeed>,

    // click-method: "areas"; "fingers"
    secondary_click_controller: Controller<Choice>,

    tap_to_click: bool,
    tap_to_click_controller: Controller<SingleChoice>,

    scroll_method: bool,
    scroll_method_data: ScrollMethod,

    natural_scroll_controller: Controller<Choice>,
}

#[derive(Debug)]
pub enum TouchpadMsg {
    SendEvents(bool),
    DisableWhileTyping(bool),
    TapToClick(bool),

    // Scroll Method Messages
    ScrollMethod(bool),
    ScrollMethodDefaultMedia(bool),
    ScrollMethodAlternativeMedia(bool),
}

#[relm4::component(pub)]
impl SimpleComponent for Touchpad {
    type Init = MouseSettings;
    type Input = TouchpadMsg;
    type Output = MouseMsg;

    view! {
        #[root]
        adw::PreferencesPage {
            add = &adw::PreferencesGroup {
                add = &adw::SwitchRow {
                    set_title: &gettext("Touchpad"),
                    set_active: model.send_events,

                    connect_active_notify[sender] => move |row| {
                        sender.input(TouchpadMsg::SendEvents(row.is_active()));
                    }
                },
            },
            add = &adw::PreferencesGroup {
                #[watch]
                set_sensitive: model.send_events,
                set_title: &gettext("General"),

                add = &adw::SwitchRow {
                    set_title: &gettext("Disable Touchpad While Typing"),
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
                set_title: &gettext("Clicking"),

                add = model.secondary_click_controller.widget(),
            },
            add = &adw::PreferencesGroup {
                #[watch]
                set_sensitive: model.send_events,

                add = model.tap_to_click_controller.widget(),
            },
            add = &adw::PreferencesGroup {
                #[watch]
                set_sensitive: model.send_events,
                set_title: &gettext("Scrolling"),

                #[template]
                add = &ChoiceWidget {
                    #[template_child]
                    title {
                        #[watch]
                        set_label: model.scroll_method_data.title.as_str(),
                    },

                    #[template_child]
                    default_option_box {
                        add_controller = gtk::EventControllerMotion {
                            connect_enter[sender] => move |_,_,_| {
                                sender.input_sender().send(TouchpadMsg::ScrollMethodDefaultMedia(true));
                            },

                            connect_leave[sender] => move |_| {
                                sender.input_sender().send(TouchpadMsg::ScrollMethodDefaultMedia(false));
                            },
                        },

                        add_controller = gtk::GestureClick {
                            connect_pressed[sender] => move |_,_,_,_| {
                                sender.input_sender().send(TouchpadMsg::ScrollMethod(true));
                            },
                        },
                    },

                    #[template_child]
                    default_option_picture {
                        set_paintable: Some(&model.scroll_method_data.default.media),
                    },

                    #[template_child]
                    default_check_button {
                        #[watch]
                        set_active: model.scroll_method_data.default.enabled,
                        connect_toggled[sender] => move |btn| {
                            sender.input(TouchpadMsg::ScrollMethod(btn.is_active()));
                        },
                    },

                    #[template_child]
                    default_option_title {
                        set_label: model.scroll_method_data.default.title.as_str(),
                    },

                    #[template_child]
                    default_option_subtitle {
                        set_label: model.scroll_method_data.default.subtitle.as_str(),
                    },


                    #[template_child]
                    alternative_option_box {
                        add_controller = gtk::EventControllerMotion {
                            connect_enter[sender] => move |_,_,_| {
                                sender.input_sender().send(TouchpadMsg::ScrollMethodAlternativeMedia(true));
                            },

                            connect_leave[sender] => move |_| {
                                sender.input_sender().send(TouchpadMsg::ScrollMethodAlternativeMedia(false));
                            },
                        },

                        add_controller = gtk::GestureClick {
                            connect_pressed[sender] => move |_,_,_,_| {
                                sender.input_sender().send(TouchpadMsg::ScrollMethod(false));
                            },
                        },
                    },

                    #[template_child]
                    alternative_option_picture {
                        set_paintable: Some(&model.scroll_method_data.alternate.media),
                    },

                    #[template_child]
                    alternative_check_button {
                        #[watch]
                        set_active: model.scroll_method_data.alternate.enabled,
                        connect_toggled[sender] => move |btn| {
                            sender.input(TouchpadMsg::ScrollMethod(!btn.is_active()));
                        },
                    },

                    #[template_child]
                    alternative_option_title {
                        set_label: model.scroll_method_data.alternate.title.as_str(),
                    },

                    #[template_child]
                    alternative_option_subtitle {
                        set_label: model.scroll_method_data.alternate.subtitle.as_str(),
                    },

                },
            },

            add = &adw::PreferencesGroup {
                #[watch]
                set_sensitive: model.send_events,

                add = model.natural_scroll_controller.widget(),
            },

            add = &adw::PreferencesGroup {
                add = &adw::ButtonRow {
                    set_title: &gettext("Test Settings"),
                    set_end_icon_name: Some("go-next-symbolic"),
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let settings = init;

        let tap_to_click_media = gtk::MediaFile::for_filename(format!(
            "{}/src/ui/mouse/assets/tap-to-click.webm",
            std::env::current_dir().unwrap().to_str().unwrap()
        ));

        let events = settings.touchpad.string("send-events");
        let send_events = events == "enabled";
        let disable_while_typing = settings.touchpad.boolean("disable-while-typing");
        let speed = settings.touchpad.value("speed").get::<f64>().unwrap();

        let speed_controller = PointerSpeed::builder()
            .launch(PointerSpeedInit {
                speed,
                settings: settings.touchpad.clone(),
            })
            .detach();

        let click_method = settings.touchpad.string("click-method").to_string();
        let secondary_click_controller = Choice::builder()
            .launch(ChoiceInit {
                key: "click-method".to_string(),
                settings: settings.touchpad.clone(),
                title: "Secondary Click".to_string(),

                default: Default {
                    value: "fingers".to_variant(),
                    media: gtk::MediaFile::for_filename(format!(
                        "{}/src/ui/mouse/assets/push-to-click-anywhere.webm",
                        std::env::current_dir().unwrap().to_str().unwrap()
                    )),
                    title: "Two Finger Push".to_string(),
                    subtitle: "Push anywhere with 2 fingers".to_string(),

                    enabled: "fingers".to_variant() == click_method.to_variant(),
                },

                alternate: Alternate {
                    value: "areas".to_variant(),
                    media: gtk::MediaFile::for_filename(format!(
                        "{}/src/ui/mouse/assets/push-areas.webm",
                        std::env::current_dir().unwrap().to_str().unwrap()
                    )),
                    title: "Corner Push".to_string(),
                    subtitle: "Push with a single finger in the corner".to_string(),

                    enabled: "areas".to_variant() == click_method.to_variant(),
                },
            })
            .detach();

        let tap_to_click = settings.touchpad.boolean("tap-to-click");
        let tap_to_click_controller = SingleChoice::builder()
            .launch(SingleChoiceInit {
                row_option: RowOption {
                    media: tap_to_click_media.clone(),
                    title: "Tap to Click".to_string(),
                    subtitle: "Quickly touch the touchpad to click".to_string(),

                    enabled: tap_to_click,
                },
            })
            .forward(sender.input_sender(), |out| match out {
                SingleChoiceOutput::Switch(state) => TouchpadMsg::TapToClick(state),
            });

        let scroll_method = settings.touchpad.boolean("two-finger-scrolling-enabled");
        let scroll_method_data = ScrollMethod {
            title: "Scroll Method".to_string(),

            default: Default {
                value: false.to_variant(),
                media: gtk::MediaFile::for_filename(format!(
                    "{}/src/ui/mouse/assets/scroll-2finger.webm",
                    std::env::current_dir().unwrap().to_str().unwrap()
                )),
                title: "Two Finger".to_string(),
                subtitle: "Drag two fingers on the touchpad".to_string(),

                enabled: false.to_variant() == scroll_method.to_variant(),
            },

            alternate: Alternate {
                value: true.to_variant(),
                media: gtk::MediaFile::for_filename(format!(
                    "{}/src/ui/mouse/assets/edge-scroll.webm",
                    std::env::current_dir().unwrap().to_str().unwrap()
                )),
                title: "Edge".to_string(),
                subtitle: "Drag one finger on the edge".to_string(),

                enabled: true.to_variant() == scroll_method.to_variant(),
            },
        };

        let natural_scroll = settings.touchpad.boolean("natural-scroll");
        let natural_scroll_controller = Choice::builder()
            .launch(ChoiceInit {
                key: "natural-scroll".to_string(),
                settings: settings.touchpad.clone(),
                title: "Scroll Direction".to_string(),

                default: Default {
                    value: false.to_variant(),
                    media: gtk::MediaFile::for_filename(format!(
                        "{}/src/ui/mouse/assets/touch-scroll-traditional.webm",
                        std::env::current_dir().unwrap().to_str().unwrap()
                    )),
                    title: "Traditional".to_string(),
                    subtitle: "Scrolling moves the view".to_string(),

                    enabled: false.to_variant() == natural_scroll.to_variant(),
                },

                alternate: Alternate {
                    value: true.to_variant(),
                    media: gtk::MediaFile::for_filename(format!(
                        "{}/src/ui/mouse/assets/touch-scroll-natural.webm",
                        std::env::current_dir().unwrap().to_str().unwrap()
                    )),
                    title: "Natural".to_string(),
                    subtitle: "Scrolling moves the view".to_string(),

                    enabled: true.to_variant() == natural_scroll.to_variant(),
                },
            })
            .detach();

        let model = Self {
            settings,

            send_events,
            disable_while_typing,
            speed_controller,

            secondary_click_controller,

            tap_to_click,
            tap_to_click_controller,

            scroll_method,
            scroll_method_data,

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

                self.settings
                    .touchpad
                    .set_value("send-events", &variant.to_variant());
            }
            TouchpadMsg::DisableWhileTyping(state) => {
                self.disable_while_typing = state;

                self.settings
                    .touchpad
                    .set_value("disable-while-typing", &state.to_variant());
            }
            TouchpadMsg::TapToClick(state) => {
                self.tap_to_click = state;

                self.settings
                    .touchpad
                    .set_value("tap-to-click", &state.to_variant());
            }
            TouchpadMsg::ScrollMethod(state) => {
                self.scroll_method = state;

                if state {
                    self.scroll_method_data.default.enabled = true;
                    self.scroll_method_data.alternate.enabled = false;
                    self.settings
                        .touchpad
                        .set_value("edge-scrolling-enabled", &false.to_variant());

                    self.settings
                        .touchpad
                        .set_value("two-finger-scrolling-enabled", &true.to_variant());
                } else {
                    self.scroll_method_data.default.enabled = false;
                    self.scroll_method_data.alternate.enabled = true;
                    self.settings
                        .touchpad
                        .set_value("edge-scrolling-enabled", &true.to_variant());

                    self.settings
                        .touchpad
                        .set_value("two-finger-scrolling-enabled", &false.to_variant());
                }
            }
            TouchpadMsg::ScrollMethodDefaultMedia(state) => {
                if state {
                    self.scroll_method_data.default.media.play();
                } else {
                    self.scroll_method_data.default.media.pause();
                }
            }
            TouchpadMsg::ScrollMethodAlternativeMedia(state) => {
                if state {
                    self.scroll_method_data.alternate.media.play();
                } else {
                    self.scroll_method_data.alternate.media.pause();
                }
            }
        }
    }
}
