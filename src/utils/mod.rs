pub mod error;
pub mod input;
pub mod language;
pub mod modules;
pub mod power;
pub mod state;

use relm4::gtk;
use std::path::PathBuf;

use crate::ui::appearance::{
    appearance::{AppearanceModel, AppearanceStyle},
    appearance_background::Background,
};

use thumbnailer::*;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("{command} exited with status code {code}")]
    CommandFailed { command: String, code: i32 },
}

// Source - https://stackoverflow.com/a/69812881
// Posted by yolenoyer, modified by community. See post 'Timeline' for change history
// Retrieved 2026-04-08, License - CC BY-SA 4.0
pub fn parse_dconf(mut str: String) -> String {
    // removes file protocol
    if str.starts_with("file://") {
        str = str[7..].into();
    }

    str
}

// add all wallpapers from directory provided
pub fn add_wallpapers(wallpapers_dir: PathBuf, model: &mut AppearanceModel, is_local: bool) {
    if let Ok(rd) = std::fs::read_dir(&wallpapers_dir) {
        for entry in rd.flatten() {
            let wallpaper = entry.path().to_string_lossy().to_string();
            let thumbnail = Thumbnail::make(PathBuf::from(&wallpaper), ThumbSize::Large);
            let background = Background {
                path: wallpaper.clone(),
                group: model.background_group.clone(),
                active: match model.style {
                    AppearanceStyle::Default => model.wallpaper_default.ends_with(&wallpaper),
                    AppearanceStyle::Dark => model.wallpaper_dark.ends_with(&wallpaper),
                },
                thumbnail,
            };
            if is_local {
                model.recent_wallpapers.guard().push_back(background);
            } else {
                model.wallpapers.guard().push_back(background);
            }
        }
    }
}

pub fn wallpaper_filters() -> Vec<gtk::FileFilter> {
    let filename_filter = gtk::FileFilter::new();
    filename_filter.add_mime_type("image/*");

    vec![filename_filter]
}
