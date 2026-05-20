use relm4::adw::AccentColor;
use relm4::adw::prelude::*;
use relm4::{
    FactorySender,
    factory::FactoryView,
    gtk,
    prelude::{DynamicIndex, FactoryComponent},
};

#[derive(Debug, Clone, PartialEq)]
pub struct AccentColorWrapped(pub AccentColor);

impl AccentColorWrapped {
    pub fn iterator() -> impl Iterator<Item = AccentColor> {
        use relm4::adw::AccentColor::*;
        [Blue, Teal, Green, Yellow, Orange, Red, Pink, Purple, Slate]
            .iter()
            .copied()
    }
}

impl From<String> for AccentColorWrapped {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "blue" => AccentColorWrapped(AccentColor::Blue),
            "teal" => AccentColorWrapped(AccentColor::Teal),
            "green" => AccentColorWrapped(AccentColor::Green),
            "yellow" => AccentColorWrapped(AccentColor::Yellow),
            "orange" => AccentColorWrapped(AccentColor::Orange),
            "red" => AccentColorWrapped(AccentColor::Red),
            "pink" => AccentColorWrapped(AccentColor::Pink),
            "purple" => AccentColorWrapped(AccentColor::Purple),
            "slate" => AccentColorWrapped(AccentColor::Slate),
            _ => AccentColorWrapped(AccentColor::Blue),
        }
    }
}

#[derive(Debug)]
pub struct AccentColorModel {
    is_active: bool,
    group: gtk::ToggleButton,
    color: String,
    accent_color: AccentColorWrapped,
}

pub struct AccentColorInit {
    pub is_active: bool,
    pub group: gtk::ToggleButton,
    pub color: String,
    pub accent_color: AccentColorWrapped,
}

#[derive(Debug)]
pub enum AccentColorOutput {
    SendPick(AccentColorWrapped),
}

#[relm4::factory(pub)]
impl FactoryComponent for AccentColorModel {
    type Init = AccentColorInit;
    type Input = ();
    type Output = AccentColorOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        #[root]
        gtk::ToggleButton {
            set_group: Some(&self.group),
            add_css_class: "accent-button",
            add_css_class: &self.color,
            set_active: self.is_active,

            connect_clicked[sender, accent_color = self.accent_color.clone()] => move |_| {
                sender.output(AccentColorOutput::SendPick(accent_color.clone()));
            },
        },
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self {
            is_active: init.is_active,
            group: init.group,
            color: init.color,
            accent_color: init.accent_color,
        }
    }

    fn init_widgets(
        &mut self,
        _index: &DynamicIndex,
        root: Self::Root,
        _returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let widgets = view_output!();
        widgets
    }
}
