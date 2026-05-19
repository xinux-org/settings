use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

use crate::ui::mouse::components::choice::{Alternate, Choice, ChoiceInit, ChoiceOutput, Default};
use crate::ui::mouse::components::pointer_speed::{PointerSpeed, PointerSpeedInit};
use crate::ui::mouse::mouse_page::{MouseMsg, MouseSettings};

use crate::utils::input::Interface;
use input;
use input::Libinput;
use input::event::EventTrait;

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

    natural_scroll_component: Controller<Choice>,
}

#[derive(Debug, Clone)]
pub enum MousePageMsg {
    PrimaryButton(bool),
    MouseAcceleration(bool),
    Noop,
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

                    // TODO: use adw::ToggleGroup instead of ToggleButton.
                    // See more: https://gitlab.gnome.org/GNOME/gnome-control-center/-/blob/main/panels/mouse/cc-mouse-panel.blp?ref_type=heads#L64
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
            #[name(mouse_group)]
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
        let natural_scroll = settings.mouse.boolean("natural-scroll");

        let mut input = Libinput::new_with_udev(Interface);
        input.udev_assign_seat("seat0").unwrap();
        input.dispatch().unwrap();

        let natural_scroll_default_media = gtk::MediaFile::for_filename(format!(
            "{}/src/ui/mouse/assets/scroll-traditional.webm",
            std::env::current_dir().unwrap().to_str().unwrap()
        ));
        // natural_scroll_default_media.set_playing(true);

        let natural_scroll_alternate_media = gtk::MediaFile::for_filename(format!(
            "{}/src/ui/mouse/assets/scroll-natural.webm",
            std::env::current_dir().unwrap().to_str().unwrap()
        ));
        // natural_scroll_alternate_media.set_playing(true);

        let natural_scroll_component = Choice::builder()
            .launch(ChoiceInit {
                key: "natural-scroll".to_string(),
                settings: settings.mouse.clone(),

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
            .forward(sender.input_sender(), |out| match out {
                ChoiceOutput::Noop => MousePageMsg::Noop,
            });

        let events: Vec<bool> = input
            .clone()
            .collect::<Vec<input::Event>>()
            .into_iter()
            .map(|event| event.device())
            .filter(|device| device.has_capability(input::DeviceCapability::Gesture))
            .map(|device| device.has_capability(input::DeviceCapability::Gesture))
            .collect();

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

            natural_scroll_component,
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
            // MousePageMsg::ScrollDirection(state) => {
            //     self.natural_scroll = state;
            //
            //     // let _ = self.settings.mouse.set_boolean("natural-scroll", state);
            //     let _ = self
            //         .settings
            //         .mouse
            //         .set_value("natural-scroll", &state.to_variant());
            // }
            MousePageMsg::Noop => {}
        }
    }
}
