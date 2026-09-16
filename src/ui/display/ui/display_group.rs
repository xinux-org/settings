use std::sync::Arc;

use gettextrs::gettext;
use relm4::{adw::prelude::*, prelude::*};
use relm4_components::simple_adw_combo_row::SimpleComboRow;

use crate::ui::display::DisplayMonitor;

#[derive(Debug)]
pub struct DisplayGroup {
    rows: Vec<adw::ActionRow>,
    primary_display_row: Controller<SimpleComboRow<Arc<DisplayMonitor>>>,
}

#[derive(Debug)]
pub enum DisplayGroupMsg {
    ActivatedMonitor(Arc<DisplayMonitor>),
}

#[derive(Debug)]
pub enum DisplayGroupOut {
    PushDisplaySettings(Arc<DisplayMonitor>),
}

#[relm4::component(pub)]
impl SimpleComponent for DisplayGroup {
    type Init = Vec<Arc<DisplayMonitor>>;
    type Input = DisplayGroupMsg;
    type Output = DisplayGroupOut;

    view! {
        #[root]
        adw::PreferencesGroup {
            model.primary_display_row.widget() -> &adw::ComboRow {
                set_use_underline: true,
                set_title: &gettext("_Primary Display"),
                set_subtitle: &gettext("Contains top bar and Activities"),
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            rows: init
                .iter()
                .map(Arc::clone)
                .enumerate()
                .map(|(index, monitor)| display_row(index, monitor, sender.clone()))
                .collect(),
            primary_display_row: {
                let variants = init.iter().map(Arc::clone).collect::<Vec<_>>();
                let active_index = variants.iter().position(|monitor| monitor.is_primary());

                SimpleComboRow::builder()
                    .launch(SimpleComboRow {
                        variants,
                        active_index,
                    })
                    .detach()
            },
        };

        let widgets = view_output!();

        model.rows.iter().for_each(|row| root.add(row));

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            DisplayGroupMsg::ActivatedMonitor(monitor) => {
                sender
                    .output_sender()
                    .emit(DisplayGroupOut::PushDisplaySettings(monitor));
            }
        }
    }
}

fn display_row(
    index: usize,
    monitor: Arc<DisplayMonitor>,
    sender: ComponentSender<DisplayGroup>,
) -> adw::ActionRow {
    relm4::view! {
        display_row = adw::ActionRow {
            set_activatable: true,
            set_use_underline: true,
            set_title: &monitor.to_string(),
            add_prefix = &gtk::Label {
                set_align: gtk::Align::Center,
                add_css_class: "monitor-label",
                set_label: &format!("{}", index + 1),
            },
            add_suffix = &gtk::Image::from_icon_name("go-next-symbolic"),
            connect_activated[sender] => move |_| {
                sender.input(DisplayGroupMsg::ActivatedMonitor(Arc::clone(&monitor)))
            }
        }
    }

    display_row
}
