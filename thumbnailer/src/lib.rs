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

static THUMB_CACHE: &str = "$XDG_CACHE_HOME/thumbnails";
static THUMB_CACHE_FALLBACK: &str = "$HOME/.cache/thumbnails";
static THUMB_IMAGE_FORMAT: Name = PNG;
static MAGICK_INIT: Once = Once::new();

// Prior to using the ImageMagick system,
// the application should invoke `magick_wand_genesis()`, which maps directly to `MagickWandGenesis`
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

pub fn find_mimetype(filepath: &Path) -> Option<Mime> {
    filepath
        .file_name()
        .and_then(|name| name.to_string_lossy().to_string().parse::<Mime>().ok())
}

#[derive(Clone, Copy, Debug)]
pub enum ThumbSize {
    Normal = 128,
    Large = 256,
    XLarge = 512,
    XXLarge = 1024,
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

#[derive(Debug)]
pub struct Thumbnail {
    uri: PathBuf, // original file path
    // path is wrapped in Option, because thumbnail generation might fail
    path: Option<PathBuf>,
    size: ThumbSize,
}

impl Thumbnail {
    // returns generated thumbnail path stored in system cache. If it fails returns original file path
    pub fn make(uri: PathBuf, thumb_size: ThumbSize) -> PathBuf {
        let record = Thumbnail {
            uri,
            path: None,
            size: thumb_size,
        };
        record
            .thumbnail()
            .unwrap_or_else(|_| record.src.to_path_buf())
    }

    pub fn gen_thumb_path(&self) -> Result<PathBuf> {
        // generates path to thumnbnails cache
        let mut cache = self.get_cache_path()?;
        cache.push(self.size.path());

        // create cache folders if doesn't exist
        fs::create_dir_all(&cache)?;

        // adds filename to the end of cache path
        let path = self.src.to_string_lossy().to_string();
        let filename = self.gen_filename(&path);
        cache.push(filename);

        Ok(cache)
    }

    pub fn thumbnail(&self) -> Result<PathBuf, ThumbError> {
        // if thumbnail already exists, return that
        let thumb_path = self.gen_thumb_path().map_err(|_| ThumbError::NoCachePath)?;
        if thumb_path.is_file() {
            return Ok(thumb_path);
        }

        let path = self.src.to_string_lossy().to_string();
        let cache = self.get_cache_fail_path()?;
        let fail_marker = cache.join(self.gen_filename(&path));
        if fail_marker.is_file() {
            return Err(ThumbError::KnownFailure);
        }

        self.gen_thumbnail(&thumb_path)
            .map(|_| thumb_path)
            .map_err(|e| {
                // #[allow(unused)]
                let a = std::fs::File::create(&fail_marker).inspect_err(|io| {
                    log::warn!("couldn't write fail marker {fail_marker:?}: {io}")
                });

                ThumbError::Generate(e)
            })
    }

    pub fn gen_thumbnail(&self, dest: &Path) -> anyhow::Result<()> {
        let src = self
            .src
            .canonicalize()
            .with_context(|| format!("canonicalizing {}", self.src.display()))?;
        let src_str = src
            .to_str()
            .with_context(|| format!("non-UTF-8 path: {}", src.display()))?;

        let wand = new_wand();

        let px: usize = self.size as usize;
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

    pub fn get_cache_path(&self) -> Result<PathBuf> {
        let cache: ExpandedPathBuf = THUMB_CACHE.parse().or(THUMB_CACHE_FALLBACK.parse())?;
        let exists = fs::exists(&cache)?;
        if !exists {
            fs::create_dir_all(&cache)?;
        }

        Ok(cache.into())
    }

    pub fn gen_filename(&self, path: &str) -> String {
        let digest = md5::compute(format!("file://{path}"));
        format!("{digest:?}.{}", THUMB_IMAGE_FORMAT.as_str())
    }

    pub fn get_cache_fail_path(&self) -> Result<PathBuf> {
        let mut cache = self
            .get_cache_path()
            .with_context(|| format!("Couldn't get cache path"))?;
        cache.push("fail");
        fs::create_dir_all(&cache).with_context(|| {
            format!("Couldn't create cache directory for failed thumbnails path")
        })?;

        Ok(cache)
    }
}
