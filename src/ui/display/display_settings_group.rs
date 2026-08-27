use gettextrs::gettext;
use relm4::{adw::prelude::*, prelude::*};
use relm4_components::simple_adw_combo_row::{SimpleComboRow, SimpleComboRowMsg};

use super::config::CcDisplayMonitor;

#[derive(Debug)]
pub enum DisplaySettingsGroupMsg {
    SelectedPrimaryMonitor(usize),
    Update(Option<Vec<CcDisplayMonitor>>),
    ActivatedMonitor(Box<CcDisplayMonitor>),
}

#[derive(Debug)]
pub enum DisplaySettingsGroupOutput {
    ChangedPrimaryMonitor(usize),
    PushDisplaySettings(Box<CcDisplayMonitor>),
}

#[derive(Debug)]
pub struct DisplaySettingsGroup {
    is_visible: bool,
    rows: Vec<adw::ActionRow>,
    primary_display_row: Controller<SimpleComboRow<CcDisplayMonitor>>,
}

#[relm4::component(pub)]
impl Component for DisplaySettingsGroup {
    type Init = Vec<CcDisplayMonitor>;
    type Input = DisplaySettingsGroupMsg;
    type Output = DisplaySettingsGroupOutput;
    type CommandOutput = ();

    view! {
        #[root]
        adw::PreferencesGroup {
            #[watch]
            set_visible: model.is_visible,

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
        let (variants, active_index) = primary_combo_data(&init);

        let primary_display_row = SimpleComboRow::builder()
            .launch(SimpleComboRow {
                variants,
                active_index,
            })
            .forward(
                sender.input_sender(),
                DisplaySettingsGroupMsg::SelectedPrimaryMonitor,
            );

        let rows = init
            .iter()
            .enumerate()
            .map(|(index, monitor)| monitor_row(index, monitor, sender.clone()))
            .collect::<Vec<_>>();

        let model = DisplaySettingsGroup {
            rows,
            primary_display_row,
            is_visible: !init.is_empty(),
        };

        let widgets = view_output!();

        model.rows.iter().for_each(|row| root.add(row));

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
        match message {
            DisplaySettingsGroupMsg::SelectedPrimaryMonitor(index) => {
                let _ = sender.output(DisplaySettingsGroupOutput::ChangedPrimaryMonitor(index));
            }
            DisplaySettingsGroupMsg::ActivatedMonitor(monitor) => {
                let _ = sender.output(DisplaySettingsGroupOutput::PushDisplaySettings(monitor));
            }
            DisplaySettingsGroupMsg::Update(monitors) => {
                self.is_visible = monitors.is_some();

                if let Some(monitors) = monitors.as_ref() {
                    let (variants, active_index) = primary_combo_data(monitors);

                    self.primary_display_row
                        .sender()
                        .emit(SimpleComboRowMsg::UpdateData(SimpleComboRow {
                            variants,
                            active_index,
                        }));

                    self.rows.iter().for_each(|row| root.remove(row));
                    self.rows = monitors
                        .iter()
                        .enumerate()
                        .map(|(index, monitor)| monitor_row(index, monitor, sender.clone()))
                        .inspect(|row| root.add(row))
                        .collect()
                } else {
                    self.rows.iter().for_each(|row| root.remove(row));
                    self.rows.clear();
                }
            }
        }
    }
}

fn monitor_row(
    index: usize,
    monitor: &CcDisplayMonitor,
    sender: ComponentSender<DisplaySettingsGroup>,
) -> adw::ActionRow {
    let cloned_monitor = monitor.clone();

    relm4::view! {
        display_row = adw::ActionRow {
            set_activatable: true,
            set_use_underline: true,
            set_title: &monitor.get_output_ui_name(),
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

    display_row
}

fn primary_combo_data(monitors: &[CcDisplayMonitor]) -> (Vec<CcDisplayMonitor>, Option<usize>) {
    let variants: Vec<CcDisplayMonitor> = monitors.to_vec();
    let active_index = monitors.iter().position(|m| m.is_primary());
    (variants, active_index)
}
