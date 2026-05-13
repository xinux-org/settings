use crate::ui::window::AppMsg;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;
use std::convert::identity;

use crate::ui::power::general_page::{GeneralPowerPageView, get_battery_path};
use crate::ui::power::power_saving::SavingPowerPageView;

#[derive(Debug)]
pub struct PowerModel {
    view_stack: adw::ViewStack,
    general_page: Controller<GeneralPowerPageView>,
    saving_page: Controller<SavingPowerPageView>,
    show_view_stack_bar: bool,
}

#[derive(Debug)]
pub enum PowerMsg {
    SetViewSwitchBar(bool),
}

#[relm4::component(pub)]
impl SimpleComponent for PowerModel {
    type Init = ();
    type Input = PowerMsg;
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
                            add: model.general_page.widget(),
                            add: model.saving_page.widget(),
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
            set_title: "General",
        },
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let view_stack = adw::ViewStack::new();
        let general_page = GeneralPowerPageView::builder()
            .launch(())
            .forward(_sender.input_sender(), identity);
        let saving_page = SavingPowerPageView::builder()
            .launch(())
            .forward(_sender.input_sender(), identity);

        let model = Self {
            view_stack: view_stack.clone(),
            general_page,
            saving_page,
            show_view_stack_bar: false,
        };

        let widgets = view_output!();

        let view_stack = model.view_stack.clone();
        let general_view_switcher = widgets.view_stack.page(model.general_page.widget());
        let saving_view_switcher = widgets.view_stack.page(model.saving_page.widget());

        general_view_switcher.set_title(Some("General"));
        general_view_switcher.set_name(Some("general")); // do not translate
        general_view_switcher.set_icon_name(Some("gnome-power-manager"));

        saving_view_switcher.set_title(Some("Power Saving"));
        saving_view_switcher.set_name(Some("power-saving")); // do not translate
        saving_view_switcher.set_icon_name(Some("power-profile-power-saver"));

        // please improve logic and add
        if get_battery_path().is_empty() {
            saving_view_switcher.set_visible(false);
            let title_stack = widgets.title_stack.clone();
            title_stack.set_visible_child_name("window_title");
        }

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            PowerMsg::SetViewSwitchBar(vsbar) => {
                self.show_view_stack_bar = vsbar;
            }
        }
    }
}
