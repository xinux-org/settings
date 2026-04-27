use crate::ui::appearance::AppearanceSettings;
use relm4::{adw::prelude::*, gtk, prelude::*};

#[derive(Debug, Clone)]
pub struct Background {
    pub path: String,
    pub group: gtk::ToggleButton,
}

#[derive(Debug)]
pub enum BackgroundMsg {
    SetBackground(String),
}

#[derive(Debug)]
pub enum BackgroundOutput {
    SetBackground(String),
}

#[relm4::factory(pub)]
impl FactoryComponent for Background {
    type Init = Background;
    type Input = BackgroundMsg;
    type Output = BackgroundOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::FlowBox;

    view! {
        #[root]
        gtk::FlowBoxChild {
            // set_width_request: 50,
            // set_halign: gtk::Align::Fill,
            // set_valign: gtk::Align::Fill,
            set_size_request: (200, 150),
            set_halign: gtk::Align::Center,
            set_accessible_role: gtk::AccessibleRole::ToggleButton,

            // set_width_request: 144,
            // set_height_request: 144,
            //

            // #[name="wallpaper_group"]
            gtk::ToggleButton {
                set_group: Some(&self.group),
                add_css_class: "style-toggle",
                set_overflow: gtk::Overflow::Hidden,
                connect_clicked[sender, path = self.path.clone()] => move |_| {
                    println!("HAHAHAHAHAHAHA");
                    sender.input(BackgroundMsg::SetBackground(path.clone()))
                },
                gtk::Overlay{
                    add_css_class: "background-thumbnail",

                    gtk::Picture {
                        // set_width_request: 144,
                        // set_height_request: 120,
                        set_content_fit: gtk::ContentFit::Fill,
                        set_filename: Some(&self.path.clone()),
                        set_can_shrink: true,
                        set_size_request: (200, 150),

                    },
                    // add_overlay = &gtk::Button {
                    //     // set_icon: "cross-small-symbolic",
                    //     set_halign: gtk::Align::Center,
                    //     set_valign: gtk::Align::Center,
                    //     add_css_class: "osd",
                    //     add_css_class: "circular",
                    //     add_css_class: "remove-button",
                    // }
                }
            }

        },

    }

    fn init_model(init: Self::Init, _index: &Self::Index, _sender: FactorySender<Self>) -> Self {
        Self {
            path: init.path,
            group: init.group,
        }
    }

    fn update(&mut self, message: Self::Input, _sender: FactorySender<Self>) {
        let settings = AppearanceSettings::new();
        println!("BACKGROUND: ");
        match message {
            BackgroundMsg::SetBackground(path) => {
                let _ = settings.background.set(
                    match settings.interface.get::<String>("color-scheme").as_str() {
                        "prefer-dark" => "picture-uri-dark",
                        _ => "picture-uri",
                    },
                    &format!("file://{}", path),
                );
                println!("BACKGROUND: {}", &path.clone())
            }
        }
    }
}
