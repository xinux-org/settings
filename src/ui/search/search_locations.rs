use relm4::adw;
use relm4::adw::prelude::*;
use relm4::gtk;
use relm4::prelude::*;

// Second page modal
#[derive(Debug)]
pub struct SearchLocationsPage;

// Second page message
#[derive(Debug)]
pub enum SearchLocationsMsg {}

#[relm4::component(pub)]
impl SimpleComponent for SearchLocationsPage {
    type Init = ();
    type Input = SearchLocationsMsg;
    type Output = ();

    view! {
    adw::NavigationPage {
        set_title: "Search Locations",

        adw::ToolbarView {
                set_top_bar_style: adw::ToolbarStyle::Flat,

                add_top_bar = &adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                        set_title: "Search Locations"
                    }
                },


                gtk::Box {
                    set_halign: gtk::Align::Center,
                    set_orientation: gtk::Orientation::Vertical,

                    gtk::Box {
                        set_margin_top: 10,
                        set_hexpand: true,

                        gtk::Label {
                            set_halign: gtk::Align::Center,
                            set_label: "Filesystem locations which are selected by system apps, such as Files",
                            add_css_class: "dim-label",
                        },
                    },

                    adw::PreferencesGroup {
                        set_title: "Default Locations",
                        set_margin_top: 10,

                        adw::SwitchRow {
                            set_title: "Home",
                            set_subtitle: "Subfolders must be manually added for this location",
                        },
                    },

                    adw::PreferencesGroup {
                        set_title: "Custom Locations",

                        adw::ActionRow {
                            set_title: "Desktop",
                            set_subtitle: "Location not found",
                            set_activatable: true,

                            add_suffix = &gtk::Box {
                                set_orientation: gtk::Orientation::Horizontal,
                                set_hexpand: true,
                                set_halign: gtk::Align::End,

                                gtk::Button {
                                    set_icon_name: "document-open",
                                    add_css_class: "flat",
                                    set_valign: gtk::Align::Center,
                                },

                                gtk::Button {
                                    set_icon_name: "edit-delete",
                                    add_css_class: "flat",
                                    set_valign: gtk::Align::Center,
                                }
                            }
                        },

                        adw::ButtonRow {
                            set_start_icon_name: Some("list-add-symbolic"),
                            set_title: "Add Location",
                        },
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = SearchLocationsPage;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
