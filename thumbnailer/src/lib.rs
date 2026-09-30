pub mod util;

use anyhow::Result;
use magick_rust::MagickWand;
use mime::{Mime, Name, PNG};
use std::{collections::HashMap, fs::metadata, os::unix::fs::MetadataExt, path::Path};

use crate::util::find_mimetype;

static THUMB_CACHE: &str = "$XDG_CACHE_HOME/thumbnails";
static THUMB_CACHE_FALLBACK: &str = "$HOME/.cache/thumbnails";
static THUMB_IMAGE_FORMAT: Name = PNG;

#[derive(Debug)]
pub struct Meta {
    // TODO: write custom URI struct
    uri: String,
    mtime: i64,
    size: Option<u64>,
    mimetype: Option<Mime>,
    description: Option<String>,
    software: Option<String>,
}

impl Meta {
    fn fetch_meta(filepath: &Path, wand: &MagickWand) -> Result<Meta> {
        let metadata = metadata(filepath)?;
        let uri = format!("file://{}", filepath.to_str().unwrap_or_default());

        Ok(Meta {
            uri,
            mtime: metadata.mtime(),
            size: metadata.size().into(),
            mimetype: find_mimetype(filepath),
            description: wand.get_image_property("Description").ok(),
            software: wand.get_image_property("Software").ok(),
        })
    }
    fn to_hashmap(&self) -> HashMap<&str, String> {
        let Meta {
            uri,
            mtime,
            size,
            mimetype,
            description,
            software,
        } = self;

        let list = [
            ("Thumb::URI", Some(uri.to_string())),
            ("Thumb::MTime", Some(mtime.to_string())),
            ("Thumb::Size", size.and_then(|x| x.to_string().into())),
            (
                "Thumb::MimeType",
                mimetype.as_ref().and_then(|x| x.to_string().into()),
            ),
            (
                "Description",
                description.as_ref().and_then(|x| x.to_string().into()),
            ),
            (
                "Software",
                software.as_ref().and_then(|x| x.to_string().into()),
            ),
        ];

        let mut map = HashMap::new();
        list.into_iter()
            .filter(|(_, k)| k.is_some())
            .for_each(|(key, v)| {
                v.and_then(|value| map.insert(key, value));
            });
        map
    }
}

#[derive(Clone, Copy)]
pub enum ThumSize {
    Normal,
    Large,
    XLarge,
    XXLarge,
}
impl ThumSize {
    fn path(&self) -> &str {
        use ThumSize::*;
        match self {
            Normal => "normal",
            Large => "large",
            XLarge => "x-large",
            XXLarge => "xx-large",
        }
    }
}
impl From<ThumSize> for usize {
    fn from(value: ThumSize) -> Self {
        use ThumSize::*;
        match value {
            Normal => 128,
            Large => 256,
            XLarge => 512,
            XXLarge => 1024,
        }
    }
}
