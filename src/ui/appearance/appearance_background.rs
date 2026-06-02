use gtk;
use relm4::loading_widgets::LoadingWidgets;
use relm4::{adw::prelude::*, prelude::*, view};

#[derive(Debug, Clone)]
pub struct Background {
    pub path: String,
    pub group: gtk::ToggleButton,
    pub active: bool,
    pub thumb: String,
}

#[derive(Debug)]
pub enum BackgroundOutput {
    SetBackground(String),
    RemoveBackground(DynamicIndex, String),
}

#[relm4::factory(pub, async)]
impl AsyncFactoryComponent for Background {
    type Init = Background;
    type Input = ();
    type Output = BackgroundOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::FlowBox;

    view! {
        #[root]
        adw::Clamp{
            set_maximum_size: 130,

            gtk::FlowBoxChild {
                set_width_request: 150,
                set_height_request: 100,
                set_halign: gtk::Align::Center,
                set_accessible_role: gtk::AccessibleRole::ToggleButton,

                adw::Clamp{
                    set_maximum_size: 120,

                gtk::Overlay{
                    add_css_class: "background-thumbnail",


                    #[name="wallpaper_item"]
                    gtk::ToggleButton {
                        set_group: Some(&self.group),
                        add_css_class: "wallpaper-button",
                        set_overflow: gtk::Overflow::Hidden,

                        #[watch]
                        set_active: self.active,
                        connect_clicked[sender, path = self.path.clone()] => move |_| {
                            match sender.output(BackgroundOutput::SetBackground(path.clone())) {
                                Ok(_) => (),
                                Err(e) => eprintln!("{e:?}")
                            }
                        },

                        gtk::Picture {
                            set_content_fit: gtk::ContentFit::Fill,
                            set_isolate_contents: true,
                            // set_paintable: Some(&self.texture),
                            set_filename: Some(&self.thumb),
                            set_can_shrink: true,
                            set_height_request: 50,
                        },
                    },


                    add_overlay = &gtk::Image {
                        set_icon_name: Some("check-icon-symbolic"),
                        set_halign: gtk::Align::End,
                        set_valign: gtk::Align::End,
                        add_css_class: "remove-button",
                        add_css_class: "selected-icon",
                    },

                    add_overlay = &gtk::Button {
                        set_icon_name: "window-close",
                        set_halign: gtk::Align::End,
                        set_valign: gtk::Align::Start,
                        add_css_class: "remove-button",
                        add_css_class: "osd",
                        add_css_class: "circular",
                        add_css_class: "image-button",

                        // verify if the wallpaper is local
                        set_visible: if self.path.contains("/home") {true} else {false},

                        connect_clicked[sender, index, path = self.path.clone()] => move |_| {
                            match sender.output(BackgroundOutput::RemoveBackground(index.clone(), path.clone())) {
                                Ok(_) => (),
                                Err(e) => eprintln!("{e:?}")
                            }
                        },
                    },
                },}
            },
        },
    }

    fn init_loading_widgets(root: Self::Root) -> Option<LoadingWidgets> {
        view! {
            #[local]
            root {
                #[name(spinner)]
                gtk::Spinner {
                    start: (),
                    set_hexpand: true,
                    set_halign: gtk::Align::Center,
                    // Reserve vertical space
                    set_height_request: 34,
                }
            }
        }
        Some(LoadingWidgets::new(root, spinner))
    }

    async fn init_model(
        init: Self::Init,
        _index: &DynamicIndex,
        _sender: AsyncFactorySender<Self>,
    ) -> Self {
        Self {
            path: init.path,
            group: init.group,
            active: init.active,
            thumb: init.thumb,
        }
    }

    fn shutdown(&mut self, _widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        println!("Wallpaper with path {} was destroyed", self.path);
    }
}
