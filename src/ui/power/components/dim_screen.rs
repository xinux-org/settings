use relm4::{
    adw::prelude::*,
    gtk::{self},
    prelude::*,
};

#[derive(Debug)]
pub struct DimScreen {
    idle_dim: bool,
}

#[derive(Debug)]
pub enum DimScreenMsg {
    Toggle(bool),
}

#[derive(Debug)]
pub enum DimScreenOutput {
    Toggled(bool),
}

#[relm4::component(pub)]
impl Component for DimScreen {
    type Init = bool;
    type Input = DimScreenMsg;
    type Output = DimScreenOutput;
    type CommandOutput = ();

    view! {
        adw::ActionRow {
            set_title: "Dim Screen",
            set_subtitle: "Reduce screen brightness when the device is inactive",

            add_suffix = &gtk::Switch {
                set_valign: gtk::Align::Center,
                #[watch]
                set_active: model.idle_dim,
                connect_state_set[sender] => move |_, state| {
                    sender.input(DimScreenMsg::Toggle(state));
                    gtk::glib::Propagation::Proceed
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
            // In case there is no battery
            idle_dim: init,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            DimScreenMsg::Toggle(state) => {
                self.idle_dim = state;

                sender.output(DimScreenOutput::Toggled(state)).unwrap()
            }
        }
    }
}
