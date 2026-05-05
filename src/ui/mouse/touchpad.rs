use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

use crate::ui::mouse::components::pointer_speed::PointerSpeed;
use crate::ui::mouse::components::pointer_speed::PointerSpeedInit;
use crate::ui::mouse::mouse_page::{MouseMsg, MouseSettings};

#[derive(Debug)]
pub struct Touchpad {
    settings: MouseSettings,
    send_events: bool,
    disable_while_typing: bool,
    speed: f64,
    speed_controller: Controller<PointerSpeed>,
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

                    add = model.speed_controller.widget(),
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

        let speed_controller = PointerSpeed::builder()
            .launch(PointerSpeedInit {
                speed,
                settings: settings.touchpad.clone(),
            })
            .detach();

        let model = Self {
            settings,

            send_events,
            disable_while_typing,
            speed,
            speed_controller,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
