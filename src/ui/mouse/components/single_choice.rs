use adw::glib::Variant;
use gtk::gio::Settings;
use relm4::{
    adw::prelude::*,
    gtk::{self},
    prelude::*,
};

#[derive(Debug, Clone)]
pub struct RowOption {
    pub value: Variant,
    pub media: gtk::MediaFile,
    pub title: String,
    pub subtitle: String,

    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct SingleChoice {
    pub key: String,
    pub settings: Settings,

    pub row_option: RowOption,
}

#[derive(Debug)]
pub enum SingleChoiceMsg {
    Switch(bool),
    MediaPlay(bool),
}

#[derive(Debug)]
pub enum SingleChoiceOutput {
    Switch(bool),
}

#[derive(Debug)]
pub struct SingleChoiceInit {
    pub key: String,
    pub settings: Settings,

    pub row_option: RowOption,
}

#[relm4::component(pub)]
impl SimpleComponent for SingleChoice {
    type Init = SingleChoiceInit;
    type Input = SingleChoiceMsg;
    type Output = SingleChoiceOutput;

    view! {
        adw::PreferencesRow {
            set_activatable: false,

            add_controller = gtk::GestureClick {
                connect_pressed[sender, enable_row_option] => move |_,_,_,_| {
                    let enabled = enable_row_option.is_active();
                    let _ = sender.input_sender().send(SingleChoiceMsg::Switch(!enabled));
                },
            },

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
                            set_label: model.row_option.title.as_str(),
                            set_align: gtk::Align::Start,
                        },

                        #[name(subtitle)]
                        gtk::Label {
                            set_xalign: 0.06,
                            set_wrap: true,
                            set_wrap_mode: pango::WrapMode::WordChar,
                            set_label: model.row_option.subtitle.as_str(),
                            add_css_class: "subtitle",
                        },
                    },

                    #[name(suffixes)]
                    gtk::Box {
                        set_visible: false,
                        add_css_class: "suffixes",
                    },

                    #[name(enable_row_option)]
                    append = &adw::SwitchRow {
                        #[watch]
                        set_active: model.row_option.enabled,
                        connect_active_notify[sender] => move |btn| {
                            sender.input(SingleChoiceMsg::Switch(btn.is_active()));
                        },
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

                        adw::Bin {
                            set_margin_top: 9,
                            set_margin_bottom: 9,
                            set_margin_start: 9,
                            set_margin_end: 9,

                            #[iterate]
                            add_css_class: ["background","frame"],

                            add_controller = gtk::EventControllerMotion {
                                connect_enter[sender] => move |_,_,_| {
                                    let _ = sender.input_sender().send(SingleChoiceMsg::MediaPlay(true));
                                },

                                connect_leave[sender] => move |_| {
                                    let _ = sender.input_sender().send(SingleChoiceMsg::MediaPlay(false));
                                },
                            },

                            #[name(default_option_picture)]
                            gtk::Picture {
                                set_hexpand: true,
                                set_halign: gtk::Align::Center,
                                set_margin_top: 6,
                                set_margin_bottom: 6,
                                set_margin_start: 6,
                                set_margin_end: 6,
                                set_height_request: 128,

                                set_paintable: Some(&model.row_option.media),
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

            row_option: init.row_option,
        };

        model.row_option.media.set_loop(true);

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            SingleChoiceMsg::MediaPlay(state) => {
                if state {
                    self.row_option.media.play();
                } else {
                    self.row_option.media.pause();
                }
            }

            SingleChoiceMsg::Switch(state) => {
                self.row_option.enabled = state;

                let _ = sender
                    .output_sender()
                    .send(SingleChoiceOutput::Switch(state));
            }
        }
    }
}
