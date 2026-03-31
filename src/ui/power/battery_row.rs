use crate::ui::power::general_page::{get_battery_percentages_float, read_file};

use relm4::{
    adw::prelude::*,
    gtk::{self},
    prelude::*,
};

#[derive(Debug, Clone)]
pub struct BatteryModel {
    pub(super) index: u8,
    pub(super) percentage: f64,
    pub(super) percentage_text: String,
    pub(super) status: String,
}

#[derive(Debug)]
pub enum BatteryMsg {
    Update,
}

#[relm4::factory(pub)]
impl FactoryComponent for BatteryModel {
    type Init = BatteryModel;
    type Input = BatteryMsg;
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        #[root]
        gtk::ListBoxRow {
            set_selectable: false,
            set_activatable: false,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_valign: gtk::Align::Center,
                set_margin_start: 12,
                set_margin_end: 12,
                set_margin_top: 16,
                set_margin_bottom: 14,
                set_spacing: 10,

                // Bluetooh battery connection has persentage text on left
                // alongside levelbar but system battery persentage text on
                // down levelbar.
                // see the image: https://www.guyrutenberg.com/2023/12/04/how-to-display-battery-percentage-for-bluetooth-headphones-in-gnome/
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 12,

                    #[name(percentage_label)]
                    gtk::Label {
                        #[watch]
                        set_label: &self.percentage_text,
                        #[watch]
                        set_visible: false,
                        set_halign: gtk::Align::End,
                        add_css_class: "dim-label",
                    },

                    #[name(levelbar)]
                    gtk::LevelBar {
                        set_min_value: 1.0,
                        set_max_value: 100.0,

                        // min/max values to activate css class
                        add_offset_value: ("low", 15.0),
                        add_offset_value: ("warning", 25.0),
                        add_offset_value: (gtk::LEVEL_BAR_OFFSET_HIGH, 79.0),
                        add_offset_value: (gtk::LEVEL_BAR_OFFSET_FULL, 100.0),
                        #[watch]
                        set_value: self.percentage,
                        set_hexpand: true,
                        set_halign: gtk::Align::Fill,
                        set_valign: gtk::Align::Center,
                    },
                },

                #[name(primary_bottom_box)]
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,

                    #[name(details_label)]
                    gtk::Label {
                        #[watch]
                        set_label: &self.status,
                        set_halign: gtk::Align::Start,
                        set_hexpand: true,
                        set_ellipsize: pango::EllipsizeMode::End,
                        set_xalign: 0.0,
                    },

                    #[name(primary_percentage_label)]
                    gtk::Label {
                        #[watch]
                        set_label: &self.percentage_text,
                        set_halign: gtk::Align::End,
                    },
                },

            }
        },

    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, sender: FactorySender<Self>) -> Self {
        let asyncsender = sender.clone();

        relm4::spawn(async move {
            loop {
                asyncsender.input(BatteryMsg::Update);
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        });

        Self {
            index: init.index,
            percentage: init.percentage,
            percentage_text: init.percentage_text,
            status: init.status,
        }
    }

    fn update(&mut self, message: Self::Input, _sender: FactorySender<Self>) {
        match message {
            BatteryMsg::Update => {
                let percentages_float =
                    get_battery_percentages_float(read_file("capacity", "0".into()));
                let percentages_text = read_file("capacity", "0".into());
                let statuses = read_file("status", "Unknown".into());

                if let (Some(pf), Some(pt), Some(st)) = (
                    percentages_float.get(self.index as usize),
                    percentages_text.get(self.index as usize),
                    statuses.get(self.index as usize),
                ) {
                    self.percentage = *pf;
                    self.percentage_text = format!("{}%", pt.trim());
                    self.status = st.trim().to_string();
                }
            }
        }
    }
}
