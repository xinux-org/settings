use relm4::adw::AccentColor;
use std::path::PathBuf;
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppearanceStyle {
    Default,
    Dark,
}

#[derive(Debug)]
pub enum AppearanceMsg {
    SetStyle(AppearanceStyle),
    SendPick(AccentColorWrapped),
    OpenRequest,
    OpenResponse(PathBuf),
    Ignore,
}
