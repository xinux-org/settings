extern crate anyhow;
extern crate expanded_pathbuf;
extern crate magick_rust;
extern crate mime;

use anyhow::{Context, Result};
use expanded_pathbuf::ExpandedPathBuf;
use magick_rust::{MagickWand, magick_wand_genesis};
use mime::{Mime, Name, PNG};
use std::{
    collections::HashMap,
    fs::{self, metadata},
    io::Write,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    sync::Once,
};
use tempfile::NamedTempFile;

static THUMB_CACHE: &str = "$XDG_CACHE_HOME/thumbnails";
static THUMB_CACHE_FALLBACK: &str = "$HOME/.cache/thumbnails";
static THUMB_IMAGE_FORMAT: Name = PNG;
static MAGICK_INIT: Once = Once::new();

fn new_wand() -> MagickWand {
    MAGICK_INIT.call_once(magick_wand_genesis);
    MagickWand::new()
}

#[derive(Debug, thiserror::Error)]
pub enum ThumbError {
    #[error("could not compute thumbnail cache path")]
    NoCachePath,
    #[error("thumbnail generation previously failed for this file")]
    KnownFailure,
    #[error("thumbnail generation failed: {0}")]
    Generate(#[from] anyhow::Error),
}

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
    fn read(filepath: &Path, wand: &MagickWand) -> Result<Meta> {
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
        [
            ("Thumb::URI", Some(self.uri.clone())),
            ("Thumb::MTime", Some(self.mtime.to_string())),
            ("Thumb::Size", self.size.map(|x| x.to_string())),
            (
                "Thumb::MimeType",
                self.mimetype.as_ref().map(|x| x.to_string()),
            ),
            ("Description", self.description.clone()),
            ("Software", self.software.clone()),
        ]
        .into_iter()
        .filter_map(|(key, value)| value.map(|v| (key, v)))
        .collect()
    }
}

#[derive(Clone, Copy)]
pub enum ThumbSize {
    Normal,
    Large,
    XLarge,
    XXLarge,
}
impl ThumbSize {
    fn path(&self) -> &str {
        use ThumbSize::*;
        match self {
            Normal => "normal",
            Large => "large",
            XLarge => "x-large",
            XXLarge => "xx-large",
        }
    }
}
impl From<ThumbSize> for usize {
    fn from(value: ThumbSize) -> Self {
        use ThumbSize::*;
        match value {
            Normal => 128,
            Large => 256,
            XLarge => 512,
            XXLarge => 1024,
        }
    }
}

pub fn gen_thumbnail(src: &Path, dest: &Path, size: ThumbSize) -> anyhow::Result<()> {
    let src = src
        .canonicalize()
        .with_context(|| format!("canonicalizing {}", src.display()))?;
    let src_str = src
        .to_str()
        .with_context(|| format!("non-UTF-8 path: {}", src.display()))?;

    let mut wand = new_wand();

    let px: usize = size.into();
    wand.set_option("jpeg:size", &format!("{0}x{0}", px * 2))?;

    wand.read_image(src_str)
        .with_context(|| format!("reading image {}", src.display()))?;
    wand.fit(px, px);

    let meta = Meta::read(&src, &wand).context("fetching metadata")?;
    for (k, v) in meta.to_hashmap() {
        wand.set_image_property(k, &v)?;
    }

    let bytes = wand
        .write_image_blob(THUMB_IMAGE_FORMAT.as_str())
        .context("encoding thumbnail")?;

    let mut file = fs::File::create(&dest)
        .with_context(|| format!("creating thumbnail file in {}", dest.display()))?;
    file.write(&bytes)
        .with_context(|| format!("saving thumbnail to {}", dest.display()))?;

    Ok(())
}

pub fn thumbnail(src: &Path, size: ThumbSize) -> Result<PathBuf, ThumbError> {
    let thumb_path = gen_thumb_path(src, size).map_err(|_| ThumbError::NoCachePath)?;
    if thumb_path.is_file() {
        return Ok(thumb_path);
    }

    let path = src.to_string_lossy().to_string();
    let fail_marker = get_cache_fail_path().join(gen_filename(&path));
    if fail_marker.is_file() {
        return Err(ThumbError::KnownFailure);
    }

    match gen_thumbnail(src, &thumb_path, size) {
        Ok(()) => Ok(thumb_path),
        Err(e) => {
            if let Err(io) = std::fs::File::create(&fail_marker) {
                log::warn!("couldn't write fail marker {}: {io}", fail_marker.display());
            }
            Err(ThumbError::Generate(e))
        }
    }
}

pub fn thumbnail_or_original(src: &Path, size: ThumbSize) -> PathBuf {
    thumbnail(src, size).unwrap_or_else(|_| src.to_path_buf())
}

pub fn gen_thumb_path(filepath: &Path, size: ThumbSize) -> Result<PathBuf> {
    // generates path to thumnbnails cache
    let mut cache = get_cache_path()?;
    cache.push(size.path());

    // create folders if doesn't exist
    fs::create_dir_all(&cache)?;

    // adds filename to the end of cache path
    let path = filepath.to_string_lossy().to_string();
    let filename = gen_filename(&path);
    cache.push(filename);

    Ok(cache)
}

pub fn get_cache_path() -> Result<PathBuf> {
    let cache: ExpandedPathBuf = THUMB_CACHE.parse().or(THUMB_CACHE_FALLBACK.parse())?;
    let exists = fs::exists(&cache)?;
    if !exists {
        fs::create_dir_all(&cache)?;
    }

    Ok(cache.into())
}

pub fn get_cache_fail_path() -> PathBuf {
    let mut cache = get_cache_path().expect("Couldn't get cache path");
    cache.push("fail");
    fs::create_dir_all(&cache).expect("Couldn't create cache directory for failed thumbnails path");

    cache
}

pub fn find_mimetype(filepath: &Path) -> Option<Mime> {
    filepath
        .file_name()
        .and_then(|name| name.to_string_lossy().to_string().parse::<Mime>().ok())
}

pub fn gen_filename(path: &str) -> String {
    let digest = md5::compute(path);
    format!("{digest:?}.{}", THUMB_IMAGE_FORMAT.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_thumbnail() {
        let thumbnail = thumbnail(&PathBuf::from("./test/reze.jxl"), ThumbSize::Normal);
        assert!(thumbnail.is_ok(), "Thumbnbail couldn't be created")
    }
}
