use relm4::{adw::prelude::*, prelude::*};

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
        adw::SwitchRow {
            set_title: "Dim Screen",
            set_subtitle: "Reduce screen brightness when the device is inactive",

            connect_active_notify[sender] => move |row| {
                    sender.input(DimScreenMsg::Toggle(row.is_active()));
            }
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
