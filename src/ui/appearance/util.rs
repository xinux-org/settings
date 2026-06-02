use allmytoes::{AMT, AMTConfiguration, ThumbSize};
use relm4::gtk;
use std::path::PathBuf;

use crate::ui::appearance::{
    appearance::{AppearanceModel, AppearanceStyle},
    appearance_background::Background,
};

pub fn add_wallpaper(path: PathBuf, model: &mut AppearanceModel, is_local: bool) {
    let configuration = AMTConfiguration::default();
    let amt = AMT::new(&configuration);

    if let Ok(rd) = std::fs::read_dir(&path) {
        for entry in rd.flatten() {
            let file_path = entry.path();
            let thumb = amt
                .get(&file_path, ThumbSize::Normal)
                .ok()
                .map(|thumb| thumb.path);

            let x = file_path.to_string_lossy().to_string();
            if is_local {
                model.recent_wallpapers.guard().push_back(Background {
                    path: x.clone(),
                    group: model.background_group.clone(),
                    active: match model.style {
                        AppearanceStyle::Default => model.wallpaper_default.ends_with(&x),
                        AppearanceStyle::Dark => model.wallpaper_dark.ends_with(&x),
                    },
                    thumb: thumb.unwrap_or(x),
                });
            } else {
                model.wallpapers.guard().push_back(Background {
                    path: x.clone(),
                    group: model.background_group.clone(),
                    active: match model.style {
                        AppearanceStyle::Default => model.wallpaper_default.ends_with(&x),
                        AppearanceStyle::Dark => model.wallpaper_dark.ends_with(&x),
                    },
                    thumb: thumb.unwrap_or(x),
                });
            }
        }
    }
}

pub fn thumb(src: &PathBuf) -> Option<String> {
    let configuration = AMTConfiguration::default();
    let amt = AMT::new(&configuration);
    let thumb_size = ThumbSize::Normal;
    amt.get(&src, thumb_size).ok().map(|thumb| thumb.path)
}

pub fn wallpaper_filters() -> Vec<gtk::FileFilter> {
    let filename_filter = gtk::FileFilter::new();
    filename_filter.add_mime_type("image/*");

    vec![filename_filter]
}
