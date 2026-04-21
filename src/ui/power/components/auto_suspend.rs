use relm4::{
    adw::prelude::*,
    gtk::{self},
    prelude::*,
};

use crate::ui::power::general_page::PowerSettings;
use gettextrs::gettext;
use relm4::gtk::StringList;
#[derive(Debug)]
pub struct AutomaticSuspend {
    power_settings: gtk::gio::Settings,
    suspend_text: String,
    enabled: bool,

    key: String,
    labels: StringList,
    values: Vec<u32>,
}

#[derive(Debug)]
pub enum AutomaticSuspendMsg {
    Toggle(bool),
    Delay(u32),
}

#[derive(Debug)]
pub enum AutomaticSuspendOutput {
    Noop,
    Toggled(bool),
}

#[derive(Debug)]
pub struct AutomaticSuspendInit {
    pub suspend_text: String,
    pub key: String,
    pub power_settings: PowerSettings,

    pub values: Vec<u32>,
}

#[relm4::component(pub)]
impl Component for AutomaticSuspend {
    // Label Text, Key, Settings, Labels, Values
    type Init = AutomaticSuspendInit;
    type Input = AutomaticSuspendMsg;
    type Output = AutomaticSuspendOutput;
    type CommandOutput = ();

    view! {
        #[root]
        adw::PreferencesGroup {
            adw::ActionRow {
                set_title: model.suspend_text.as_str(),

                add_suffix = &gtk::Switch {
                    set_valign: gtk::Align::Center,
                    #[watch]
                    set_active: model.enabled,
                    connect_state_set[sender] => move |_, state| {
                        sender.input(AutomaticSuspendMsg::Toggle(state));
                        gtk::glib::Propagation::Proceed
                    },
                },
            },

            adw::ComboRow {
                #[watch]
                set_sensitive: model.enabled,

                set_title: "Delay",
                set_model: Some(&model.labels),

                connect_selected_item_notify[sender] => move |row| {
                    sender.input(AutomaticSuspendMsg::Delay(row.selected()));
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let power_settings = init.power_settings.power;
        let key = init.key;

        let suspend_delay_labels: Vec<String> = [
            gettext("15 minute"),
            gettext("20 minute"),
            gettext("25 minute"),
            gettext("30 minute"),
            gettext("45 minute"),
            gettext("1 hour"),
            gettext("1 hour 20 minute"),
            gettext("1 hour 30 minute"),
            gettext("1 hour 40 minute"),
            gettext("2 hours"),
        ]
        .to_vec();

        let labels: StringList = suspend_delay_labels.iter().map(gettext).collect();
        let values = init.values;

        let sleep_inactive_type =
            power_settings.string(format!("sleep-inactive-{}-type", key).as_str());
        let enabled = sleep_inactive_type.as_str() == "suspend";

        let model = Self {
            suspend_text: init.suspend_text,
            power_settings,
            enabled,

            key,
            labels,
            values,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            AutomaticSuspendMsg::Toggle(state) => {
                self.enabled = state;

                let status = if state { "suspend" } else { "nothing" };

                let _ = self
                    .power_settings
                    .set_string(format!("sleep-inactive-{}-type", self.key).as_str(), status);

                sender
                    .output_sender()
                    .emit(AutomaticSuspendOutput::Toggled(state));
            }

            AutomaticSuspendMsg::Delay(index) => {
                let seconds = match self.values.get(index as usize) {
                    Some(val) => *val,
                    None => 0,
                };

                println!("Seconds: {:?}\nIndex: {:?}\n\n\n\n\n", seconds, index);
                let _ = self.power_settings.set_int(
                    format!("sleep-inactive-{}-timeout", self.key).as_str(),
                    seconds as i32,
                );
            }
        }
    }
}
