use crate::ui::mouse::mouse::Mouse;
use crate::ui::mouse::pointing_stick::PointingStick;
use crate::ui::window::AppMsg;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;
use std::convert::identity;

use crate::ui::mouse::touchpad::Touchpad;

#[derive(Debug)]
pub struct MouseModal {
    view_stack: adw::ViewStack,
    mouse: Controller<Mouse>,
    touchpad: Controller<Touchpad>,
    pointing_stick: Controller<PointingStick>,
    show_view_stack_bar: bool,
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
            // when no battery found itʻs shows only general page,
            // otherwise shows battery and view_switcher_bar
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
        let view_stack = adw::ViewStack::new();
        let mouse = Mouse::builder()
            .launch(())
            .forward(_sender.input_sender(), identity);
        let touchpad = Touchpad::builder()
            .launch(())
            .forward(_sender.input_sender(), identity);

        let pointing_stick = PointingStick::builder()
            .launch(())
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
