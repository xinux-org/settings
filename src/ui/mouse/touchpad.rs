use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

use crate::ui::mouse::mouse_page::{MouseMsg, MouseSettings};

#[derive(Debug, Clone)]
pub struct Touchpad {
    settings: MouseSettings,
    send_events: bool,
    disable_while_typing: bool,
    speed: f64,
}

#[relm4::component(pub)]
impl SimpleComponent for Touchpad {
    type Init = ();
    type Input = ();
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
                    },
                },

                add = &adw::PreferencesGroup {
                    set_title: "General",

                    add = &adw::SwitchRow {
                        set_title: "Disable Touchpad While Typing",
                        set_active: model.disable_while_typing,
                    },

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
                },
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let settings = MouseSettings::new();

        let events = settings.touchpad.string("send-events");
        let send_events = events == String::from("enabled");
        let disable_while_typing = settings.touchpad.boolean("disable-while-typing");
        let speed = settings.touchpad.value("speed").get::<f64>().unwrap();

        let model = Self {
            settings,

            send_events,
            disable_while_typing,
            speed,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
