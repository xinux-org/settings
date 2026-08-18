use gettextrs::gettext;
use relm4::{adw::prelude::*, prelude::*};

#[relm4::widget_template(pub)]
impl WidgetTemplate for ApplyWidget {
    view! {
        adw::HeaderBar {
            set_show_end_title_buttons: false,
            set_show_start_title_buttons: false,

            #[wrap(Some)]
            set_title_widget = &adw::WindowTitle {
                set_title: &gettext("Apply Changes?"),
            },

            pack_start = &gtk::Button {
                set_can_shrink: true,
                set_use_underline: true,
                set_label: &gettext("Cancel"),
            },
            pack_end = &gtk::Button {
                set_can_shrink: true,
                set_use_underline: true,
                set_label: &gettext("Apply"),
                add_css_class: "suggested-action",
            },
        },
    }
}
