use relm4::{
    adw::prelude::*,
    gtk::{self},
    prelude::*,
};


#[relm4::widget_template(pub)]
impl WidgetTemplate for ChoiceWidget {
    view! {
        adw::PreferencesRow {
            set_activatable: false,

            #[wrap(Some)]
            set_child = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_hexpand: false,
                

                #[name(header)]
                gtk::Box {
                    set_valign: gtk::Align::Center,

                    #[name(prefixes)]
                    gtk::Box {
                      set_visible: false,
                    },

                    #[name(title_box)]
                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_valign: gtk::Align::Center,
                        set_hexpand: true,

                        #[name(title)]
                        gtk::Label {
                            set_margin_all: 16,
                            set_align: gtk::Align::Start,
                        },
                    },

                    #[name(suffixes)]
                    gtk::Box {
                        set_visible: false,
                        add_css_class: "suffixes",
                    },
                },

                #[name(contents)]
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_hexpand: true,
                    set_homogeneous: true,
                    set_spacing: 3,

                    #[name(default_option_box)]
                    gtk::Box {
                        set_accessible_role: gtk::AccessibleRole::Radio,
                        set_orientation: gtk::Orientation::Vertical,
                        set_can_focus: true,
                        set_focus_on_click: false,
                        set_focusable: false,
                        set_receives_default: true,

                        #[iterate]
                        add_css_class: ["activatable"],

                        #[name(default_choice_bin)]
                        adw::Bin {
                            set_margin_top: 9,
                            set_margin_bottom: 9,
                            set_margin_start: 9,
                            set_margin_end: 9,

                            #[iterate]
                            add_css_class: ["background","frame"],

                            #[name(default_option_picture)]
                            gtk::Picture {
                                set_hexpand: true,
                                set_halign: gtk::Align::Center,
                                set_margin_top: 6,
                                set_margin_bottom: 6,
                                set_margin_start: 6,
                                set_margin_end: 6,
                                set_height_request: 128,
                            },
                        },
                        gtk::Box {
                            set_margin_start: 15,
                            set_margin_bottom: 9,
                            #[name(default_checkbutton_image)]
                            adw::Bin {
                                add_css_class: "radio",
                                set_can_target: false,
                                set_valign: gtk::Align::Center,
                                set_halign: gtk::Align::Center
                            },
                            gtk::Box {
                                set_valign: gtk::Align::Center,
                                set_margin_start: 6,
                                set_orientation: gtk::Orientation::Horizontal,
                                add_css_class: "title",

                                #[name = "default_check_button"]
                                gtk::CheckButton {
                                    #[watch]
                                    set_active: true,
                                },

                                gtk::Box {
                                    set_orientation: gtk::Orientation::Vertical,

                                    #[name(default_option_title)]
                                    gtk::Label {
                                        set_use_underline: true,
                                        set_xalign: 0.0,
                                        set_wrap: true,
                                        set_wrap_mode: pango::WrapMode::WordChar,
                                        add_css_class: "title",
                                    },
                                    #[name(default_option_subtitle)]
                                    gtk::Label {
                                        set_xalign: 0.0,
                                        set_wrap: true,
                                        set_wrap_mode: pango::WrapMode::WordChar,
                                        add_css_class: "subtitle",
                                    },
                                },
                            }
                        },
                    },
                    #[name(alternative_option_box)]
                    gtk::Box {
                        set_accessible_role: gtk::AccessibleRole::Radio,
                        set_orientation: gtk::Orientation::Vertical,
                        set_can_focus: true,
                        set_focusable: true,
                        set_receives_default: true,

                        #[iterate]
                        add_css_class: ["activatable"],

                        #[name(alternative_choice_bin)]
                        adw::Bin {
                            set_margin_top: 9,
                            set_margin_bottom: 9,
                            set_margin_start: 9,
                            set_margin_end: 9,

                            #[iterate]
                            add_css_class: ["background","frame"],

                            #[name(alternative_option_picture)]
                            gtk::Picture {
                                set_hexpand: true,
                                set_halign: gtk::Align::Center,
                                set_margin_top: 6,
                                set_margin_bottom: 6,
                                set_margin_start: 6,
                                set_margin_end: 6,
                                set_height_request: 128,

                            },
                        },

                        gtk::Box {
                            set_margin_start: 15,
                            set_margin_bottom: 9,
                            #[name(alternative_checkbutton_image)]
                            adw::Bin {
                                add_css_class: "radio",
                                set_can_target: false,
                                set_valign: gtk::Align::Center,
                                set_halign: gtk::Align::Center
                            },
                            gtk::Box {
                                set_valign: gtk::Align::Center,
                                set_margin_start: 6,
                                set_orientation: gtk::Orientation::Horizontal,
                                add_css_class: "title",

                                #[name = "alternative_check_button"]
                                gtk::CheckButton {
                                    set_group: Some(&default_check_button),
                                },

                                gtk::Box {
                                    set_orientation: gtk::Orientation::Vertical,

                                    #[name(alternative_option_title)]
                                    gtk::Label {
                                        set_use_underline: true,
                                        set_xalign: 0.0,
                                        set_wrap: true,
                                        set_wrap_mode: pango::WrapMode::WordChar,
                                        add_css_class: "title",
                                    },
                                    #[name(alternative_option_subtitle)]
                                    gtk::Label {
                                        set_xalign: 0.0,
                                        set_wrap: true,
                                        set_wrap_mode: pango::WrapMode::WordChar,
                                        add_css_class: "subtitle",
                                    },
                                },
                            },
                        },
                    },
                },
            },

        },
    }
}
