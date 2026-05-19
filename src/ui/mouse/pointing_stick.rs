use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

use crate::ui::mouse::mouse_page::MouseMsg;

#[derive(Debug, Clone)]
pub struct PointingStick {}

#[relm4::component(pub)]
impl SimpleComponent for PointingStick {
    type Init = ();
    type Input = ();
    type Output = MouseMsg;

    view! {
        #[root]
        adw::PreferencesPage {
            add = &adw::PreferencesGroup {
                set_title: "Pointic stick",
                add = &adw::ActionRow {
                    set_title: "Primary Button",
                    set_subtitle: "Order of physical buttons on mice and touchpads",
                    add_suffix = &gtk::Box {
                        set_spacing: 0,
                        set_halign: gtk::Align::End,
                        set_valign: gtk::Align::Center,
                        add_css_class: "linked",

                        #[name= "left" ]
                        append = &gtk::ToggleButton {
                            set_group: Some(&right),
                            set_label: "Left",
                            set_active: true,
                        },

                        #[name= "right" ]
                        append = &gtk::ToggleButton {
                            set_label: "Right",
                        },
                    }
                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
