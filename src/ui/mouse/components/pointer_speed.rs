
use adw::glib::Variant;
use gtk::gio::Settings;
use relm4::{
    adw::prelude::*,
    gtk::{self},
    prelude::*,
};

#[derive(Debug)]
pub struct PointerSpeed {
    /// single value being chosen
    value: Variant,
    key: String,
    settings: Settings,
    speed: f64
}


#[derive(Debug)]
pub enum PointerSpeedMsg {
    PointerSpeed(f64),
}

#[derive(Debug)]
pub enum PointerSpeedOutput {
    
}

#[derive(Debug)]
pub struct ChoiceInit {
    pub key: String,
    pub speed: f64,
    pub settings: Settings,
}

#[relm4::component(pub)]
impl Component for PointerSpeed {
    type Init = ChoiceInit;
    type Input = PointerSpeedMsg;
    type Output = PointerSpeedOutput;
    type CommandOutput = ();

    view! {
        adw::PreferencesGroup {
            add = &adw::ActionRow {
                set_title: "Pointer Speed",

                add_suffix = &gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 6,
                    set_halign: gtk::Align::Fill,
                    set_valign: gtk::Align::Center,
                    set_hexpand: true,

                    append = &gtk::Label {
                        set_label: "Slow",
                        add_css_class: "dim-label",
                    },

                    append = &gtk::Scale {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_hexpand: true,
                        set_draw_value: false,
                        set_range: (-1.0, 1.0),
                        set_value: model.speed,

                        connect_value_changed[sender] => move |scale|{
                            sender.input(PointerSpeedMsg::PointerSpeed(scale.value()));
                        }
                    },

                    append = &gtk::Label {
                        set_label: "Fast",
                        add_css_class: "dim-label",
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
            key: init.key,
            speed: init.speed,
            settings: init.settings,
            value,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            PointerSpeedMsg::PointerSpeed(speed) => {
                self.speed = speed;

                let _ = self.settings.set_value("speed", &speed.to_variant());
            }
        }
    }
}
