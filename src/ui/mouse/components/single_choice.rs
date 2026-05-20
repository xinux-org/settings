use adw::glib::{Variant, property::PropertyGet};
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
        adw::ActionRow {
            add_prefix = &gtk::Label {
              set_label: model.row_option.title.as_str(),  
            },

            add_suffix = &gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 12,
                set_hexpand: true,

                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 6,

                    #[name(enable_row_option)]
                    append = &adw::SwitchRow {
                        #[watch]
                        set_active: model.row_option.enabled,
                        connect_active_notify[sender] => move |btn| {
                            sender.input(SingleChoiceMsg::Switch(btn.is_active()));
                        },
                    },

                    add_controller = gtk::EventControllerMotion {
                        connect_enter[sender] => move |_,_,_| {
                            let _ = sender.input_sender().send(SingleChoiceMsg::MediaPlay(true));
                        },

                        connect_leave[sender] => move |_| {
                            let _ = sender.input_sender().send(SingleChoiceMsg::MediaPlay(false));
                        },
                    },

                    add_controller = gtk::GestureClick {
                        connect_pressed[sender, enable_row_option] => move |_,_,_,_| {
                            let enabled = enable_row_option.is_active();   
                            let _ = sender.input_sender().send(SingleChoiceMsg::Switch(!enabled));
                        },
                    },

                    append = &gtk::Picture {
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
            }
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

                let _ = sender.output_sender().send(SingleChoiceOutput::Switch(state));
            }
        }
    }
}
