use adw::glib::Variant;
use gtk::gio::Settings;
use relm4::{
    adw::prelude::*,
    gtk::{self},
    prelude::*,
};

#[derive(Debug)]
pub struct Choice {
    /// single value being chosen
    value: Variant,
    /// array of values that can be chosen
    values: Vec<Variant>,
    /// jk
    position: Position,
    key: String,
    settings: Settings,

    title: String,
    options: Vec<String>,
    subtitles: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Position {
    Left,
    Right,
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
    pub key: String,
    pub settings: Settings,
    pub values: Vec<Variant>,
    pub position: Position,

    pub title: String,
    pub options: Vec<String>,
    pub subtitles: Vec<String>,
}

#[relm4::component(pub)]
impl SimpleComponent for Choice {
    type Init = ChoiceInit;
    type Input = ChoiceMsg;
    type Output = ChoiceOutput;

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
                            set_active: model.position == Position::Left,
                            connect_toggled[sender] => move |btn| {
                                if  btn.is_active() {
                                    sender.input(ChoiceMsg::Change(!btn.is_active()));
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
                                #[iterate]
                                add_css_class: ["dim-label", "caption"],
                            },
                        },
                    },
                },

                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 6,

                    set_hexpand: true,
                    gtk::Picture {
                        set_content_fit:  gtk::ContentFit::Cover,
                        set_filename: Some("/home/sae/projects/settings/src/ui/mouse/assets/scroll-natural.webm"),
                    },

                    append = &gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 6,
                        set_halign: gtk::Align::Start,

                        append = &gtk::CheckButton {
                            set_group: Some(&left),

                            #[watch]
                            set_active: model.position == Position::Right,
                            connect_toggled[sender] => move |btn| {
                                if  btn.is_active() {
                                    sender.input(ChoiceMsg::Change(btn.is_active()));
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
                                #[iterate]
                                add_css_class: ["dim-label", "caption"],
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
        let value = init.settings.value(init.key.as_str());

        let model = Self {
            value,
            settings: init.settings,
            key: init.key,
            values: init.values,
            position: init.position,
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
                let index = if state { 0 } else { 1 };

                self.value = self.values.get(index).unwrap().to_variant();

                sender.output(ChoiceOutput::Changed(state)).unwrap()
            }
        }
    }
}
