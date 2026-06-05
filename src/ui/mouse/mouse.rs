use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

use crate::ui::mouse::{
    components::{
        choice::{Alternate, Choice, ChoiceInit, Default},
        pointer_speed::{PointerSpeed, PointerSpeedInit},
    },
    mouse_page::{MouseMsg, MouseSettings},
};

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

    natural_scroll_component: Controller<Choice>,
}

#[derive(Debug, Clone)]
pub enum MousePageMsg {
    PrimaryButton(u32),
    MouseAcceleration(bool),
}

#[relm4::component(pub)]
impl SimpleComponent for Mouse {
    type Init = MouseSettings;
    type Input = MousePageMsg;
    type Output = MouseMsg;

    view! {
        #[root]
        #[name(mouse_stack_page)]
        adw::PreferencesPage {
            add = &adw::PreferencesGroup {
                set_title: "General",
                #[name(primary_button_row)]
                add = &adw::ActionRow {
                    set_title: "Primary Button",
                    set_subtitle: "Order of physical buttons on mice and touchpads",

                    add_suffix = &gtk::Box {
                        set_spacing: 0,
                        set_halign: gtk::Align::End,
                        set_valign: gtk::Align::Center,
                        add_css_class: "linked",

                        adw::ToggleGroup {
                            add = adw::Toggle {
                                set_label: Some("Left"),
                                set_name: Some("Left"),
                            },

                            add = adw::Toggle {
                                set_label: Some("Right"),
                                set_name: Some("Right"),
                            },

                            connect_active_name_notify[sender] => move |toggle| {
                                sender.input(MousePageMsg::PrimaryButton(toggle.active()));
                            },
                        },
                    }
                },
            },

            #[name(mouse_group)]
            add = &adw::PreferencesGroup {
                set_title: "Mouse",

                add = model.speed_controller.widget(),

                add = &adw::SwitchRow {
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

                    connect_active_notify[sender] => move |row| {
                            sender.input(MousePageMsg::MouseAcceleration(row.is_active()));
                    }
                },

                add = model.natural_scroll_component.widget(),
            },
            add = &adw::PreferencesGroup {
                add = &adw::ButtonRow {
                    set_title: "Test Settings",
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
        let key = "natural-scroll".to_string();
        let value = settings.mouse.value(key.as_str());

        let acceleration = settings.mouse.string("accel-profile");
        let accel_profile = acceleration.as_str() == "default";
        let left_handed = settings.mouse.boolean("left-handed");
        let speed = settings.mouse.value("speed").get::<f64>().unwrap();

        let natural_scroll_default_media = gtk::MediaFile::for_filename(format!(
            "{}/src/ui/mouse/assets/scroll-traditional.webm",
            std::env::current_dir().unwrap().to_str().unwrap()
        ));

        let natural_scroll_alternate_media = gtk::MediaFile::for_filename(format!(
            "{}/src/ui/mouse/assets/scroll-natural.webm",
            std::env::current_dir().unwrap().to_str().unwrap()
        ));

        let natural_scroll_component = Choice::builder()
            .launch(ChoiceInit {
                key: "natural-scroll".to_string(),
                settings: settings.mouse.clone(),
                title: "Scroll Direction".to_string(),

                default: Default {
                    value: false.to_variant(),
                    media: natural_scroll_default_media,
                    title: "Traditional".to_string(),
                    subtitle: "Scrolling moves the view".to_string(),

                    enabled: false.to_variant() == value,
                },

                alternate: Alternate {
                    value: true.to_variant(),
                    media: natural_scroll_alternate_media,
                    title: "Natural".to_string(),
                    subtitle: "Scrolling moves the view".to_string(),

                    enabled: true.to_variant() == value,
                },
            })
            .detach();

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

            natural_scroll_component,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            MousePageMsg::PrimaryButton(state) => {
                let primary = state.eq(&0);
                self.left_handed = primary;
                self.settings.mouse.set_boolean("left-handed", primary);
            }
            MousePageMsg::MouseAcceleration(state) => {
                self.accel_profile = state;
                let profile = if state { "default" } else { "flat" };
                self.settings.mouse.set_string("accel-profile", profile);
            }
        }
    }
}
