use adw::glib::Variant;
use gtk::gio::Settings;
use relm4::{
    adw::prelude::*,
    gtk::{self},
    prelude::*,
};

#[derive(Debug)]
pub struct Default {
    pub value: Variant,
    pub media: gtk::MediaFile,
    pub title: String,
    pub subtitle: String,

    pub enabled: bool,
}

#[derive(Debug)]
pub struct Alternate {
    pub value: Variant,
    pub media: gtk::MediaFile,
    pub title: String,
    pub subtitle: String,

    pub enabled: bool,
}

#[derive(Debug)]
pub struct Choice {
    pub key: String,
    pub settings: Settings,
    pub title: String,

    pub default: Default,
    pub alternate: Alternate,
}

#[derive(Debug)]
pub enum ChoiceMsg {
    Default(bool),
    Alternate(bool),
    DefaultMedia(bool),
    AlternateMedia(bool),
}

#[derive(Debug)]
pub struct ChoiceInit {
    pub key: String,
    pub settings: Settings,
    pub title: String,

    pub default: Default,
    pub alternate: Alternate,
}

#[relm4::component(pub)]
impl SimpleComponent for Choice {
    type Init = ChoiceInit;
    type Input = ChoiceMsg;
    type Output = ();

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
                            set_label: model.title.as_str(),
                            set_align: gtk::Align::Start,
                        },
                    },

                    #[name(suffixes)]
                    gtk::Box {
                        set_visible: false,
                        add_css_class: "suffixes",
                    },
                },

                // gif starts from here
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
                        set_can_target: true,
                        set_focus_on_click: false,
                        set_focusable: true,
                        set_receives_default: true,

                        #[iterate]
                        add_css_class: ["activatable"],

                        add_controller = gtk::EventControllerMotion {
                            connect_enter[sender, default_option_box] => move |_,_,_| {
                                default_option_box.add_css_class("card");
                                let _ = sender.input_sender().send(ChoiceMsg::DefaultMedia(true));
                            },

                            connect_leave[sender, default_option_box] => move |_| {
                                default_option_box.remove_css_class("card");
                                let _ = sender.input_sender().send(ChoiceMsg::DefaultMedia(false));
                            },
                        },

                        add_controller = gtk::GestureClick {
                            connect_pressed[sender] => move |_,_,_,_| {
                                let _ = sender.input_sender().send(ChoiceMsg::Default(true));
                            },
                        },

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

                                set_paintable: Some(&model.default.media),
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

                                #[name = "traditional"]
                                gtk::CheckButton {
                                    #[watch]
                                    set_active: model.default.enabled,
                                    connect_toggled[sender] => move |btn| {
                                        sender.input(ChoiceMsg::Default(btn.is_active()));
                                    },
                                },

                                gtk::Box {
                                    set_orientation: gtk::Orientation::Vertical,

                                    #[name(default_option_title)]
                                    gtk::Label {
                                        set_use_underline: true,
                                        set_xalign: 0.0,
                                        set_wrap: true,
                                        set_wrap_mode: pango::WrapMode::WordChar,
                                        set_label: model.default.title.as_str(),
                                        add_css_class: "title",
                                    },
                                    #[name(default_option_subtitle)]
                                    gtk::Label {
                                        set_xalign: 0.0,
                                        set_wrap: true,
                                        set_wrap_mode: pango::WrapMode::WordChar,
                                        set_label: model.default.subtitle.as_str(),
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
                        set_can_target: true,
                        set_focusable: true,
                        set_receives_default: true,

                        #[iterate]
                        add_css_class: ["activatable"],

                        add_controller = gtk::EventControllerMotion {
                            connect_enter[sender, alternative_option_box] => move |_,_,_| {
                                alternative_option_box.add_css_class("card");
                                let _ = sender.input_sender().send(ChoiceMsg::AlternateMedia(true));
                            },

                            connect_leave[sender, alternative_option_box] => move |_| {
                                alternative_option_box.remove_css_class("card");
                                let _ = sender.input_sender().send(ChoiceMsg::AlternateMedia(false));
                            },
                        },

                        add_controller = gtk::GestureClick {
                            connect_pressed[sender] => move |_,_,_,_| {
                                let _ = sender.input_sender().send(ChoiceMsg::Alternate(true));
                            },
                        },

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

                                set_paintable: Some(&model.alternate.media),
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

                                #[name = "natural"]
                                gtk::CheckButton {
                                    set_group: Some(&traditional),

                                    #[watch]
                                    set_active: model.alternate.enabled,
                                    connect_toggled[sender] => move |btn| {
                                        sender.input(ChoiceMsg::Alternate(btn.is_active()));
                                    },
                                },

                                gtk::Box {
                                    set_orientation: gtk::Orientation::Vertical,

                                    #[name(alternative_option_title)]
                                    gtk::Label {
                                        set_use_underline: true,
                                        set_xalign: 0.0,
                                        set_wrap: true,
                                        set_wrap_mode: pango::WrapMode::WordChar,
                                        set_label: model.alternate.title.as_str(),
                                        add_css_class: "title",
                                    },
                                    #[name(alternative_option_subtitle)]
                                    gtk::Label {
                                        set_xalign: 0.0,
                                        set_wrap: true,
                                        set_wrap_mode: pango::WrapMode::WordChar,
                                        set_label: model.alternate.subtitle.as_str(),
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

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            key: init.key,
            settings: init.settings,
            title: init.title,

            default: init.default,
            alternate: init.alternate,
        };

        model.default.media.set_loop(true);
        model.alternate.media.set_loop(true);

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            ChoiceMsg::Default(state) => {
                self.default.enabled = state;
                self.alternate.enabled = !state;

                let _ = self
                    .settings
                    .set_value(self.key.as_str(), &self.default.value);
            }
            ChoiceMsg::Alternate(state) => {
                self.alternate.enabled = state;
                self.default.enabled = !state;

                let _ = self
                    .settings
                    .set_value(self.key.as_str(), &self.alternate.value);
            }
            ChoiceMsg::DefaultMedia(state) => {
                if state {
                    self.default.media.play();
                } else {
                    self.default.media.pause();
                }
            }
            ChoiceMsg::AlternateMedia(state) => {
                if state {
                    self.alternate.media.play();
                } else {
                    self.alternate.media.pause();
                }
            }
        }
    }
}
