use relm4::gtk::gio::Settings;

#[derive(Debug, Clone)]
pub struct AppearanceSettings {
    pub background: Settings,
    pub interface: Settings,
}

impl AppearanceSettings {
    pub fn new() -> Self {
        Self {
            background: Settings::new("org.gnome.desktop.background"),
            interface: Settings::new("org.gnome.desktop.interface"),
        }
    }
}
