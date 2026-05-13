use relm4::{
    adw::prelude::*,
    gtk::{self},
    prelude::*,
};

use crate::ui::power::general_page::PowerSettings;
use gettextrs::gettext;
use relm4::gtk::StringList;
#[derive(Debug)]
pub struct AutoScreenBlank {
    session_settings: gtk::gio::Settings,
    enabled: bool,
    delay: u32,
    labels: StringList,
    values: Vec<u32>,
}

#[derive(Debug)]
pub enum AutoScreenBlankMsg {
    Toggle(bool, u32),
    Delay(u32),
}

#[derive(Debug)]
pub enum AutoScreenBlankOutput {
    Noop,
}

const BLANK_SCREEN_DEFAULT: u32 = 300;

#[relm4::component(pub)]
impl Component for AutoScreenBlank {
    type Init = (PowerSettings, Vec<u32>);
    type Input = AutoScreenBlankMsg;
    type Output = AutoScreenBlankOutput;
    type CommandOutput = ();

    view! {
        #[name(blank_screen_group)]
        adw::PreferencesGroup {
            #[name(blank_screen_switch_row)]
            adw::SwitchRow {
                set_title: "Automatic Screen Blank",
                set_subtitle: "Turn the screen off after a period of inactivity",
                #[watch]
                set_active: model.enabled,

                connect_active_notify[sender, blank_screen_delay_row] => move |row| {
                    sender.input(AutoScreenBlankMsg::Toggle(row.is_active(), blank_screen_delay_row.selected()));
                }
            },

            #[name(blank_screen_delay_row)]
            adw::ComboRow {
                set_title: "Delay",
                #[watch]
                set_sensitive: model.enabled,
                set_model: Some(&model.labels),

                // example: https://github.com/blissd/fotema/blob/main/src/app/components/preferences.rs#L127-L130
                connect_selected_item_notify[sender] => move |row| {
                   sender.input(AutoScreenBlankMsg::Delay(row.selected()));
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let session_settings = init.0.session;
        let values = init.1;
        let current_delay = session_settings.uint("idle-delay");

        let screen_blank_delay_labels = [
            gettext("1 minute"),
            gettext("2 minutes"),
            gettext("3 minutes"),
            gettext("4 minutes"),
            gettext("5 minutes"),
            gettext("8 minutes"),
            gettext("10 minutes"),
            gettext("12 minutes"),
            gettext("15 minutes"),
        ]
        .to_vec();
        let labels: StringList = screen_blank_delay_labels.iter().map(gettext).collect();

        let (enabled, delay) = current_delay
            .eq(&0)
            .then(|| (false, BLANK_SCREEN_DEFAULT))
            .unwrap_or((true, current_delay));

        let model = Self {
            session_settings,
            enabled,
            delay,

            labels,
            values,
        };

        // sender.input(AutoScreenBlankMsg::Toggle(model.enabled, 0));
        // sender.input(AutoScreenBlankMsg::Delay(delay));

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            AutoScreenBlankMsg::Toggle(state, index) => {
                // FIXME: https://gitlab.gnome.org/GNOME/gnome-control-center/-/blob/main/panels/power/cc-power-panel.c?ref_type=heads#L740
                // true
                if state {
                    sender.input(AutoScreenBlankMsg::Delay(index));

                    // unwrap is used here because index is always in range, this should NOT fail
                    self.delay = *self.values.get(index as usize).unwrap();
                // false
                } else {
                    sender.input(AutoScreenBlankMsg::Delay(self.values.len() as u32));
                }
                self.enabled = state;
            }
            AutoScreenBlankMsg::Delay(index) => {
                let seconds = match self.values.get(index as usize) {
                    Some(val) => *val,
                    None => 0,
                };

                let _ = self.session_settings.set_uint("idle-delay", seconds);
            }
        }
    }
}
