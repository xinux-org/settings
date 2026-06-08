use crate::{
    ui::{
        mouse::{mouse::Mouse, pointing_stick::PointingStick, touchpad::Touchpad},
        window::AppMsg,
    },
    utils::input::Interface,
};
use gtk::gio::Settings;
use input::{self, Device, event::EventTrait};
use relm4::{adw::prelude::*, gtk, prelude::*};
use std::convert::identity;

#[derive(Debug, Clone)]
pub struct MouseSettings {
    pub mouse: Settings,
    pub touchpad: Settings,
    pub pointingstick: Settings,
}

#[derive(Debug)]
pub struct MouseModal {
    view_stack: adw::ViewStack,
    mouse: Controller<Mouse>,
    touchpad: Controller<Touchpad>,
    pointing_stick: Controller<PointingStick>,
    show_view_stack_bar: bool,
}

impl MouseModal {
    pub fn gsettings() -> MouseSettings {
        MouseSettings {
            mouse: Settings::new("org.gnome.desktop.peripherals.mouse"),
            touchpad: Settings::new("org.gnome.desktop.peripherals.touchpad"),
            pointingstick: Settings::new("org.gnome.desktop.peripherals.pointingstick"),
        }
    }
}

#[derive(Debug)]
pub enum MouseMsg {
    SetViewSwitchBar(bool),
}

#[relm4::component(pub)]
impl SimpleComponent for MouseModal {
    type Init = ();
    type Input = MouseMsg;
    type Output = AppMsg;

    view! {
        #[root]
        adw::BreakpointBin {
            // when no trackpad is found itʻs shows only general page,
            // otherwise view_switcher_bar
            add_breakpoint = adw::Breakpoint::new(adw::BreakpointCondition::new_length(
                adw::BreakpointConditionLengthType::MinWidth,
                560.0,
                adw::LengthUnit::Sp,
            )) {
                add_setter: (&header_bar, "show-title", Some(&true.into())),
                add_setter: (&view_switcher_title, "policy", Some(&adw::ViewSwitcherPolicy::Wide.into())),
            },
            // tablet
            add_breakpoint = adw::Breakpoint::new(adw::BreakpointCondition::new_length(
                adw::BreakpointConditionLengthType::MaxWidth,
                550.0,
                adw::LengthUnit::Sp,
            )) {
                add_setter: (&header_bar, "show-title", Some(&true.into())),
                add_setter: (&view_switcher_title, "policy", Some(&adw::ViewSwitcherPolicy::Narrow.into())),
            },
            // mobile
            add_breakpoint = adw::Breakpoint::new(adw::BreakpointCondition::new_length(
                adw::BreakpointConditionLengthType::MaxWidth,
                450.0,
                adw::LengthUnit::Sp,
            )) {
                add_setter: (&header_bar, "show-title", Some(&false.into())),
                add_setter: (&view_switcher_bar, "reveal", Some(&true.into())),
            },

            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                set_top_bar_style: adw::ToolbarStyle::Flat,

                #[name(header_bar)]
                add_top_bar = &adw::HeaderBar {
                    #[wrap(Some)]
                    #[name(title_stack)]
                    set_title_widget = &gtk::Stack {
                        add_named: (&view_switcher_title, Some("view_switcher")),
                        add_named: (&window_title, Some("window_title")),
                        set_vhomogeneous: false,
                        set_hhomogeneous: false,
                    },
                },

                #[wrap(Some)]
                set_content = &gtk::ScrolledWindow {
                    set_hscrollbar_policy: gtk::PolicyType::Never,
                    set_vexpand: true,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,

                        #[local_ref]
                        view_stack -> adw::ViewStack {
                            add: model.mouse.widget(),
                            add: model.touchpad.widget(),
                            add: model.pointing_stick.widget(),
                        },
                    },
                },

                #[name(view_switcher_bar)]
                add_bottom_bar = &adw::ViewSwitcherBar {
                    set_stack: Some(&view_stack),
                },
            }
        },
        view_switcher_title = &adw::ViewSwitcher {
            set_stack: Some(&view_stack),
            #[watch]
            set_policy: adw::ViewSwitcherPolicy::Wide,
        },
        window_title = &adw::WindowTitle {
            set_title: "Mouse & Touchpad",
        },
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let settings = Self::gsettings();

        let view_stack = adw::ViewStack::new();
        let mouse = Mouse::builder()
            .launch(settings.clone())
            .forward(_sender.input_sender(), identity);

        let touchpad = Touchpad::builder()
            .launch(settings.clone())
            .forward(_sender.input_sender(), identity);

        let pointing_stick = PointingStick::builder()
            .launch(settings.clone())
            .forward(_sender.input_sender(), identity);

        let model = Self {
            view_stack: view_stack.clone(),
            mouse,
            touchpad,
            pointing_stick,
            show_view_stack_bar: false,
        };

        let widgets = view_output!();
        let view_stack = model.view_stack.clone();

        let mouse_switcher = widgets.view_stack.page(model.mouse.widget());
        let touchpad_swticher = widgets.view_stack.page(model.touchpad.widget());
        let pointing_stick_switcher = widgets.view_stack.page(model.pointing_stick.widget());

        mouse_switcher.set_title(Some("Mouse"));
        mouse_switcher.set_name(Some("mouse")); // do not translate
        mouse_switcher.set_icon_name(Some("input-mouse"));

        touchpad_swticher.set_title(Some("Touchpad"));
        touchpad_swticher.set_name(Some("touchpad")); // do not translate
        touchpad_swticher.set_icon_name(Some("input-touchpad"));

        pointing_stick_switcher.set_title(Some("Pointing Stick"));
        pointing_stick_switcher.set_name(Some("pointing_stick")); // do not translate
        pointing_stick_switcher.set_icon_name(Some("pointer thinkpad"));

        let mut input = input::Libinput::new_with_udev(Interface);

        // `udev_assign_seat` succeeds even if no input devices are
        // currently available on this seat, or if devices are available
        // but fail to open in `LibinputInterface::open_restricted`.
        input.udev_assign_seat("seat0").unwrap();
        input.dispatch().unwrap();

        // All Devices that have the Pointer capability
        // are being filtered out because
        // Touchpads have Pointer and Gesture
        // TrackPad doesn't have Gesture, only Pointer
        // All Mouses have Pointer but not Gesture
        //          Touchpad | Mouse | TrackPad
        // Gesture |   ✅    |  ❌   |   ❌
        // Pointer |   ✅    |  ✅   |   ✅
        let events: Vec<Device> = input
            .clone()
            .collect::<Vec<input::Event>>()
            .into_iter()
            .map(|event| event.device())
            .filter(|device| device.has_capability(input::DeviceCapability::Pointer))
            .collect();

        // All Devices but in String
        let devices: Vec<String> = events
            .clone()
            .into_iter()
            .map(|device| device.name().to_string())
            .collect();

        // Filter out touchpads with Gesture
        let touchpads: Vec<String> = events
            .clone()
            .into_iter()
            .filter(|device| device.has_capability(input::DeviceCapability::Gesture))
            .map(|device| device.name().to_string())
            .collect();

        // All Mouses and Touchpads has Pointer but only TrackPoint is called so
        let trackpoint: Vec<String> = events
            .clone()
            .into_iter()
            .filter(|device| device.has_capability(input::DeviceCapability::Pointer))
            .map(|device| device.name().to_string())
            .filter(|name| name.contains("TrackPoint"))
            .collect();

        println!("Devices: {:#?}", devices);
        println!("Touchpads: {:#?}", touchpads);
        println!("Trackpoint: {:#?}", trackpoint);

        if touchpads.is_empty() {
            touchpad_swticher.set_visible(false);
        }

        if trackpoint.is_empty() {
            pointing_stick_switcher.set_visible(false);
        }

        if touchpads.is_empty() && trackpoint.is_empty() {
            let title_stack = widgets.title_stack.clone();
            title_stack.set_visible_child_name("window_title");
        }

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            MouseMsg::SetViewSwitchBar(vsbar) => {
                self.show_view_stack_bar = vsbar;
            }
        }
    }
}
