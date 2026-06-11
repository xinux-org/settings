pub mod error;
pub mod input;
pub mod language;
pub mod modules;
pub mod power;
pub mod state;

use allmytoes::{AMT, AMTConfiguration, ThumbSize};
use relm4::gtk;
use std::path::PathBuf;

use crate::ui::appearance::{
    appearance::{AppearanceModel, AppearanceStyle},
    appearance_background::Background,
};

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

pub fn add_wallpaper(path: PathBuf, model: &mut AppearanceModel, is_local: bool) {
    if let Ok(rd) = std::fs::read_dir(&path) {
        for entry in rd.flatten() {
            let file_path = entry.path();

            let x = file_path.to_string_lossy().to_string();
            if is_local {
                model.recent_wallpapers.guard().push_back(Background {
                    path: x.clone(),
                    group: model.background_group.clone(),
                    active: match model.style {
                        AppearanceStyle::Default => model.wallpaper_default.ends_with(&x),
                        AppearanceStyle::Dark => model.wallpaper_dark.ends_with(&x),
                    },
                    thumb: thumb(&PathBuf::from(&x), ThumbSize::Large).unwrap_or(x),
                });
            } else {
                model.wallpapers.guard().push_back(Background {
                    path: x.clone(),
                    group: model.background_group.clone(),
                    active: match model.style {
                        AppearanceStyle::Default => model.wallpaper_default.ends_with(&x),
                        AppearanceStyle::Dark => model.wallpaper_dark.ends_with(&x),
                    },
                    thumb: thumb(&PathBuf::from(&x), ThumbSize::Large).unwrap_or(x),
                });
            }
        }
    }
}

pub fn thumb(src: &PathBuf, thumb_size: ThumbSize) -> Option<String> {
    let configuration = AMTConfiguration::default();
    let amt = AMT::new(&configuration);
    amt.get(&src, thumb_size).ok().map(|thumb| thumb.path)
}

pub fn wallpaper_filters() -> Vec<gtk::FileFilter> {
    let filename_filter = gtk::FileFilter::new();
    filename_filter.add_mime_type("image/*");

    vec![filename_filter]
}
