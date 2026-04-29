use gtk::gio::Settings;
use relm4::{
    adw::prelude::*,
    gtk::{self},
    prelude::*,
};

#[derive(Debug)]
pub struct Choice {
    left: bool,
    key: String,
    settings: Settings,

    title: String,
    options: Vec<String>,
    subtitles: Vec<String>,
}

#[derive(Debug)]
pub enum ChoiceMsg {
    Change(bool),
}

#[derive(Debug)]
pub enum ChoiceOutput {
    Changed(bool),
}

#[derive(Debug)]
pub struct ChoiceInit {
    key: String,
    settings: Settings,

    title: String,
    options: Vec<String>,
    subtitles: Vec<String>,
}

#[relm4::component(pub)]
impl Component for Choice {
    type Init = ChoiceInit;
    type Input = ChoiceMsg;
    type Output = ChoiceOutput;
    type CommandOutput = ();

    view! {
    adw::PreferencesGroup {
        set_title: model.title.as_str(),

        add = &adw::ActionRow {

            add_suffix = &gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 12,
                set_homogeneous: true,
                set_hexpand: true,

                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 6,

                    append = &gtk::Frame {
                        set_hexpand: true,

                        #[wrap(Some)]
                        set_child = &gtk::Image {
                            set_icon_name: Some("input-mouse-symbolic"),
                            set_pixel_size: 64,
                        },
                    },

                    append = &gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 6,
                        set_halign: gtk::Align::Start,

                        #[name = "left"]
                        append = &gtk::CheckButton {
                            #[watch]
                            set_active: model.left,
                            connect_toggled[sender] => move |btn| {
                                if  btn.is_active() {
                                    sender.input(ChoiceMsg::Change(btn.is_active()));
                                }
                            },
                        },

                        append = &gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,

                            append = &gtk::Label {
                                set_label: model.options.first().unwrap(),
                                set_halign: gtk::Align::Start,
                            },

                            append = &gtk::Label {
                                set_label: model.subtitles.first().unwrap(),
                                set_halign: gtk::Align::Start,
                                add_css_class: "dim-label",
                                add_css_class: "caption",
                            },
                        },
                    },
                },

                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 6,

                    append = &gtk::Frame {
                        set_hexpand: true,
                        #[wrap(Some)]
                        set_child = &gtk::Image {
                            set_icon_name: Some("input-mouse-symbolic"),
                            set_pixel_size: 64,
                        },
                    },

                    append = &gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 6,
                        set_halign: gtk::Align::Start,

                        append = &gtk::CheckButton {
                            set_group: Some(&left),

                            #[watch]
                            set_active: !model.left,
                            connect_toggled[sender] => move |btn| {
                                if  btn.is_active() {
                                    sender.input(ChoiceMsg::Change(!btn.is_active()));
                                }
                            },
                        },

                        append = &gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,

                            append = &gtk::Label {
                                set_label:model.options.get(1).unwrap(),
                                set_halign: gtk::Align::Start,
                            },

                            append = &gtk::Label {
                                set_label: model.subtitles.get(1).unwrap(),
                                set_halign: gtk::Align::Start,
                                add_css_class: "dim-label",
                                    add_css_class: "caption",
                                },
                            },
                        },
                    },
                }
            },
        },
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let left = init.settings.boolean(init.key.as_str());

        let model = Self {
            settings: init.settings,
            left,
            key: init.key,
            title: init.title,
            options: init.options,
            subtitles: init.subtitles,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            ChoiceMsg::Change(state) => {
                self.left = state;

                sender.output(ChoiceOutput::Changed(state)).unwrap()
            }
        }
    }
}
