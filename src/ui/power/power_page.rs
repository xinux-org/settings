use crate::ui::window::AppMsg;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;
use std::convert::identity;

use crate::ui::power::general_page::GeneralPowerPageView;
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
        adw::ToolbarView {
            set_top_bar_style: adw::ToolbarStyle::Flat,

            add_top_bar = &adw::HeaderBar {
                // A top bar when desktop mode
                #[wrap(Some)]
                set_title_widget = &adw::ViewSwitcher {
                    set_stack: Some(&view_stack),
                    set_policy: adw::ViewSwitcherPolicy::Wide,
                },
            },

            #[wrap(Some)]
            set_content = &gtk::ScrolledWindow {
                set_hscrollbar_policy: gtk::PolicyType::Never,
                set_vexpand: true,

                adw::Clamp {
                    set_maximum_size: 600,
                    set_tightening_threshold: 400,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_margin_all: 12,
                        set_spacing: 24,

                        #[local_ref]
                        view_stack -> adw::ViewStack {
                            add: model.general_page.widget(),
                            add: model.saving_page.widget(),
                        },
                        // A bottom bar when on mobile. See more:
                        // https://github.com/blissd/fotema/blob/74160645a25d2cfe4ceb4f1935c247158ec3ab5f/src/app.rs#L401C52-L401C64
                        // #[name(switcher_bar)]
                        // adw::ViewSwitcherBar {
                        //     set_stack: Some(&view_stack),
                        //     #[track(model.show_view_stack_bar)]
                        //     // set_reveal: model.show_view_stack_bar,
                        //     set_reveal: true,
                        //     }
                        },
                    },
                },

        }
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
        general_view_switcher.set_icon_name(Some("preferences-system-symbolic"));

        saving_view_switcher.set_title(Some("Power Saving"));
        saving_view_switcher.set_name(Some("power-saving")); // do not translate
        saving_view_switcher.set_icon_name(Some("battery-symbolic"));

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
