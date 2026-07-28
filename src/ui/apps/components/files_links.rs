use relm4::{
    ComponentParts, ComponentSender, SimpleComponent, adw,
    adw::prelude::*,
    factory::{DynamicIndex, FactoryComponent, FactorySender, FactoryVecDeque},
    gtk,
    gtk::{
        gio::{content_type_get_description, content_type_get_icon},
        glib::markup_escape_text,
    },
};
use gettextrs::gettext;

#[derive(Debug)]
struct MimeRow {
    mime: gtk::glib::GString,
}

#[relm4::factory]
impl FactoryComponent for MimeRow {
    type Init = gtk::glib::GString;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = adw::PreferencesGroup;
    view! {
        adw::ActionRow {
            set_title: &markup_escape_text(&content_type_get_description(&self.mime)),
            set_subtitle: &markup_escape_text(&self.mime),
            add_prefix = &gtk::Image {
                set_from_gicon: &content_type_get_icon(&self.mime),
            },
        }
    }
    fn init_model(mime: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self { mime }
    }
}

#[derive(Debug)]
pub struct FilesLinksDialog {
    app_name: String,
    rows: FactoryVecDeque<MimeRow>,
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
            set_title: &gettext("Files and Links"),
            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {},
                #[wrap(Some)]
                set_content = &adw::PreferencesPage {
                    #[watch]
                    set_description: &gettext("File and link types that are opened by <b>{}</b>")
                        .replace("{}", &model.app_name),
                    #[local_ref]
                    mime_group -> adw::PreferencesGroup {
                        #[watch]
                        set_visible: !model.rows.is_empty(),
                        #[watch]
                        set_title: &gettext(&format!(
                            "{} {}",
                            model.rows.len(),
                            if model.rows.len() == 1 { "type" } else { "types" }
                        )),
                    },
                    adw::PreferencesGroup {
                        #[watch]
                        set_visible: model.rows.is_empty(),
                        adw::StatusPage {
                            set_icon_name: Some("text-x-generic-symbolic"),
                            set_title: &gettext("No File Types"),
                            set_description: Some(&gettext("This app has not registered any file or link types")),
                            add_css_class: "compact",
                        }
                    },
                },
            }
        }
    }
    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            app_name: String::new(),
            rows: FactoryVecDeque::builder()
                .launch(adw::PreferencesGroup::new())
                .detach(),
        };
        let mime_group = model.rows.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            FilesLinksDialogMsg::Show(name, mime_types) => {
                self.app_name = name;
                let mut guard = self.rows.guard();
                guard.clear();
                for mime in mime_types {
                    guard.push_back(mime);
                }
            }
        }
    }
}
