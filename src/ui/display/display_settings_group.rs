use gettextrs::gettext;
use relm4::{SimpleComponent, adw::prelude::*, prelude::*};
use relm4_components::simple_adw_combo_row::SimpleComboRow;

use super::display_panel::LogicalDisplay;
use super::monitor::Monitor;

#[derive(Debug)]
pub enum DisplaySettingsGroupMsg {
    SelectedPrimaryMonitor(usize),
    ActivatedMonitor(Box<Monitor>),
}

#[derive(Debug)]
pub enum DisplaySettingsGroupOutput {
    ChangedPrimaryMonitor(usize),
    PushDisplaySettings(Box<Monitor>),
}

#[derive(Debug)]
pub struct DisplaySettingsGroup {
    last_output_error: Option<DisplaySettingsGroupOutput>,

    primary_display_row: Controller<SimpleComboRow<Monitor>>,
}

#[relm4::component(pub)]
impl SimpleComponent for DisplaySettingsGroup {
    type Init = Vec<LogicalDisplay>;
    type Input = DisplaySettingsGroupMsg;
    type Output = DisplaySettingsGroupOutput;

    view! {
        #[root]
        adw::PreferencesGroup {
            #[name(arrangement_row)]
            adw::PreferencesRow {
                set_visible: false,
                set_activatable: false,

                // TODO: arrangement_bin
            },

            model.primary_display_row.widget() -> &adw::ComboRow {
                set_use_underline: true,
                set_title: &gettext("_Primary Display"),
                set_subtitle: &gettext("Contains top bar and Activities"),
            },
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let primary_display_row = {
            let variants = init.iter().map(|ld| ld.0.clone()).collect();
            let active_index = init
                .iter()
                .position(|ld| ld.1.as_ref().is_some_and(|lm| lm.is_primary));

            SimpleComboRow::builder()
                .launch(SimpleComboRow {
                    variants,
                    active_index,
                })
                .forward(
                    sender.input_sender(),
                    DisplaySettingsGroupMsg::SelectedPrimaryMonitor,
                )
        };

        let model = DisplaySettingsGroup {
            primary_display_row,
            last_output_error: None,
        };

        let widgets = view_output!();

        for (index, display) in init.iter().enumerate() {
            let cloned_monitor = display.0.clone();
            relm4::view! {
                display_row = adw::ActionRow {
                    set_activatable: true,
                    set_use_underline: true,
                    set_title: &display.0.get_output_ui_name(),
                    add_prefix = &gtk::Label {
                        set_align: gtk::Align::Center,
                        add_css_class: "monitor-label",
                        set_label: &format!("{}", index + 1),
                    },
                    add_suffix = &gtk::Image::from_icon_name("go-next-symbolic"),
                    connect_activated[sender] => move |_| {
                        sender.input(DisplaySettingsGroupMsg::ActivatedMonitor(Box::new(cloned_monitor.clone())));
                    }
                }
            }

            root.add(&display_row);
        }

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            DisplaySettingsGroupMsg::SelectedPrimaryMonitor(index) => {
                self.last_output_error = sender
                    .output(DisplaySettingsGroupOutput::ChangedPrimaryMonitor(index))
                    .err();
            }
            DisplaySettingsGroupMsg::ActivatedMonitor(monitor) => {
                self.last_output_error = sender
                    .output(DisplaySettingsGroupOutput::PushDisplaySettings(monitor))
                    .err();
            }
        }
    }
}
