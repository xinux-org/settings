use gtk::gio::Settings;
use relm4::{
    adw::prelude::*,
    gtk::{self},
    prelude::*,
};

#[derive(Debug)]
pub struct PointerSpeed {
    settings: Settings,
    speed: f64,
    mouse_speed_scale: Option<gtk::Scale>,
}

#[derive(Debug)]
pub enum PointerSpeedMsg {
    PointerSpeed(f64),
}

#[derive(Debug)]
pub enum PointerSpeedOutput {}

#[derive(Debug)]
pub struct PointerSpeedInit {
    pub settings: Settings,
    pub speed: f64,
}

#[relm4::component(pub)]
impl Component for PointerSpeed {
    type Init = PointerSpeedInit;
    type Input = PointerSpeedMsg;
    type Output = PointerSpeedOutput;
    type CommandOutput = ();

    view! {
        #[name(mouse_row)]
        adw::ActionRow {
            set_title: "Pointer Speed",
            set_use_underline: true,
            // set_activatable_widget: model.mouse_speed_scale,
            #[name(mouse_speed_scale)]
            add_suffix = &gtk::Scale {
                set_hexpand: true,
                set_value: model.speed,
                set_range: (-1.0, 1.0),
                add_mark: (-1.0, gtk::PositionType::Bottom, Some("Slow")),
                add_mark: (0.0, gtk::PositionType::Bottom, None),
                add_mark: (1.0, gtk::PositionType::Bottom, Some("Fast")),
                set_adjustment = &gtk::Adjustment {
                    set_lower: -1.0,
                    set_upper: 1.0,
                    set_step_increment: 0.1,
                    set_page_increment: 0.1,
                },
                connect_value_changed[sender] => move |scale|{
                    sender.input(PointerSpeedMsg::PointerSpeed(scale.value()));
                }
            },
        },
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut model = Self {
            settings: init.settings,
            speed: init.speed,
            mouse_speed_scale: None,
        };

        let widgets = view_output!();
        let mouse_speed_scale = widgets.mouse_speed_scale.clone();
        model.mouse_speed_scale = Some(mouse_speed_scale);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            PointerSpeedMsg::PointerSpeed(speed) => {
                self.speed = speed;

                let _ = self.settings.set_value("speed", &speed.to_variant());
            }
        }
    }
}
