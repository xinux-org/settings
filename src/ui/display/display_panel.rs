use crate::ui::window::AppMsg;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct DisplayModel {
    
}

#[relm4::component(pub)]
impl SimpleComponent for DisplayModel {
    type Init = ();
    type Input = ();
    type Output = AppMsg;

    view! {
        #[root]
        adw::ToolbarView {
            set_top_bar_style: adw::ToolbarStyle::Flat,

            add_top_bar = &adw::HeaderBar {
                #[wrap(Some)]
                set_title_widget = &adw::WindowTitle {
                    set_title: "Displays",
                }
            },
            adw::PreferencesPage {
                adw::PreferencesGroup {
                    // set_title: "Devices",
                    adw::ComboRow {
                        set_title: "Orientantion",
                        #[wrap(Some)]
                        set_model = &gtk::StringList::new(&[
                            "Landscape",
                            "Portrait Right",
                            "Portrait Left",
                            "Landscape (flipped)",
                        ]),
                        set_selected: 0,
                    },

                    adw::ComboRow {
                        set_title: "Resolution",
                        #[wrap(Some)]
                        set_model = &gtk::StringList::new(&[
                            "3440 × 1440 (21:9)",
                            "2560 × 1440 (16:9)",
                            "1920 × 1080 (16:9)",
                            "1680 × 1050 (16:10)",
                            "1440 × 900 (16:10)",
                            "1280 × 1024 (5:4)",
                            "1024 × 768 (4:3)",
                            "800 × 600 (4:3)",
                        ]),
                        set_selected: 0,
                    },

                    adw::ComboRow {
                        set_title: "Refresh Rate",
                        #[wrap(Some)]
                        set_model = &gtk::StringList::new(&[
                            "60.00 Hz",
                            "50.00 Hz",
                        ]),
                        set_selected: 0,
                    },

                    adw::SwitchRow {
                        set_title: "HDR (High Dynamic Range)"
                    },

                    adw::ActionRow {
                        set_title: "Scale",

                        add_suffix = &gtk::Box {
                            set_spacing: 0,
                            set_halign: gtk::Align::End,
                            set_valign: gtk::Align::Center,
                            add_css_class: "linked",

                            #[name="left"]
                            gtk::ToggleButton {
                                set_group: Some(&right),
                                set_label: "100 %",
                                set_active: true,
                                // add_css_class: "flat",
                            },

                            #[name="right"]
                            gtk::ToggleButton {
                                set_label: "200 %",
                                // add_css_class: "flat",
                            },
                        }
                    },
                },

                adw::PreferencesGroup {
                    adw::ActionRow {
                        set_title: "Night Light",
                        set_activatable: true,

                        add_prefix = &gtk::Image {
                            set_icon_name: Some("night-light-symbolic"),
                        },

                        add_suffix = &gtk::Box {
                            set_spacing: 12,
                            set_halign: gtk::Align::End,

                            gtk::Label {
                                set_label: "Off",
                                add_css_class: "dim-label",
                            },

                            gtk::Image {
                                set_icon_name: Some("go-next-symbolic"),
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
        let model = Self {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
