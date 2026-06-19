use crate::ui::mouse::{
    components::pointer_speed::{PointerSpeed, PointerSpeedInit},
    mouse_page::{MouseMsg, MouseSettings},
};
use gettextrs::gettext;
use relm4::{adw::prelude::*, gtk, prelude::*};

#[derive(Debug)]
pub struct PointingStick {
    settings: MouseSettings,
    speed_controller: Controller<PointerSpeed>,
    accel_profile: bool,
}

#[derive(Debug)]
pub enum PointingStickMsg {
    MouseAcceleration(bool),
}

#[relm4::component(pub)]
impl SimpleComponent for PointingStick {
    type Init = MouseSettings;
    type Input = PointingStickMsg;
    type Output = MouseMsg;

    view! {
        #[root]
        adw::PreferencesPage {
            add = &adw::PreferencesGroup {
                set_title: &gettext("Pointing Stick"),

                add = model.speed_controller.widget(),

                add = &adw::SwitchRow {
                    set_title: &gettext("Pointing Stick Acceleration"),
                    set_subtitle: &gettext("Recommended for most users and applications"),
                    add_suffix = &gtk::Box {
                        gtk::MenuButton {
                            set_icon_name: "help-about",
                            set_direction: gtk::ArrowType::Down,
                            #[wrap(Some)]
                            set_popover = &gtk::Popover {
                                set_valign: gtk::Align::Center,
                                gtk::Label {
                                    set_label: &gettext("Turning pointing stick acceleration off can allow faster and more\nprecise movements, but can also make the mouse more difficult\nto use."),
                                },
                            },
                        },
                    },

                    connect_active_notify[sender] => move |row| {
                        sender.input(PointingStickMsg::MouseAcceleration(row.is_active()));
                    },
                },
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

        let acceleration = settings.pointingstick.string("accel-profile");
        let accel_profile = acceleration.as_str() == "default";

        let speed = settings.mouse.value("speed").get::<f64>().unwrap();
        let speed_controller = PointerSpeed::builder()
            .launch(PointerSpeedInit {
                speed,
                settings: settings.pointingstick.clone(),
            })
            .detach();

        let model = Self {
            settings,
            accel_profile,
            speed_controller,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            PointingStickMsg::MouseAcceleration(state) => {
                self.accel_profile = state;
                let profile = if state { "default" } else { "flat" };
                self.settings
                    .pointingstick
                    .set_string("accel-profile", profile);
            }
        }
    }
}
