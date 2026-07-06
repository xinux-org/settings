use relm4::{
    ComponentParts, ComponentSender, SimpleComponent, adw,
    adw::prelude::*,
    gtk,
    gtk::{
        gio::{content_type_get_description, content_type_get_icon},
        glib::markup_escape_text,
    },
};

#[derive(Debug)]
pub struct FilesLinksDialog {
    app_name: String,
    desc_label: gtk::Label,
    content_box: gtk::Box,
}

#[derive(Debug, Clone)]
pub enum FilesLinksDialogMsg {
    Show(String, Vec<gtk::glib::GString>),
}

#[relm4::component(pub)]
impl SimpleComponent for FilesLinksDialog {
    type Init = ();
    type Input = FilesLinksDialogMsg;
    type Output = ();

    view! {
        adw::NavigationPage {
            set_title: "Files and Links",

            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {},

                #[wrap(Some)]
                set_content = &gtk::ScrolledWindow {
                    #[wrap(Some)]
                    set_child = &adw::Clamp {
                        set_maximum_size: 450,
                        set_tightening_threshold: 350,

                        #[wrap(Some)]
                        set_child = &gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_spacing: 24,
                            set_margin_top: 24,
                            set_margin_bottom: 24,
                            set_margin_start: 16,
                            set_margin_end: 16,

                            #[name = "desc_label"]
                            gtk::Label {
                                set_use_markup: true,
                                set_wrap: true,
                                set_justify: gtk::Justification::Center,
                                add_css_class: "dim-label",
                            },

                            #[name = "content_box"]
                            gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                            }
                        }
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();

        let model = Self {
            app_name: String::new(),
            desc_label: widgets.desc_label.clone(),
            content_box: widgets.content_box.clone(),
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            FilesLinksDialogMsg::Show(name, mime_types) => {
                self.app_name = name;

                let desc = format!(
                    "File and link types that are opened by <b>{}</b>",
                    self.app_name
                );
                self.desc_label.set_label(&desc);

                while let Some(child) = self.content_box.first_child() {
                    self.content_box.remove(&child);
                }

                if mime_types.is_empty() {
                    let status = adw::StatusPage::builder()
                        .icon_name("text-x-generic-symbolic")
                        .title("No File Types")
                        .description("This app has not registered any file or link types")
                        .build();
                    status.add_css_class("compact");
                    self.content_box.append(&status);
                } else {
                    let pref_group = adw::PreferencesGroup::new();
                    pref_group.set_title(&format!(
                        "{} {}",
                        mime_types.len(),
                        if mime_types.len() == 1 {
                            "type"
                        } else {
                            "types"
                        }
                    ));

                    for mime_str in mime_types.iter() {
                        let description = content_type_get_description(mime_str);

                        let action_row = adw::ActionRow::builder()
                            .title(markup_escape_text(&description).as_str())
                            .subtitle(markup_escape_text(mime_str).as_str())
                            .build();

                        let icon = content_type_get_icon(mime_str);
                        let image = gtk::Image::from_gicon(&icon);
                        action_row.add_prefix(&image);

                        pref_group.add(&action_row);
                    }

                    self.content_box.append(&pref_group);
                }
            }
        }
    }
}
