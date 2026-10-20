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
    // Used to make sure MagickWand is initialized exactly once. Note that we do not
    // bother shutting down, we simply exit when we're done.
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
            mimetype: Thumbnail::find_mimetype(filepath),
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

pub struct Thumbnail {
    // original file path
    pub origin: PathBuf,
    // generated thumbnail path, file is stored in cache
    pub size: ThumbSize,
}

impl Thumbnail {
    fn compress_save(&self, des: PathBuf) -> Result<PathBuf> {
        let Self { origin, size } = self;
        let src = origin
            .canonicalize()
            .context(format!("canonicalizing {:?}", origin))?;
        let src_str = src.to_str().context(format!("non-UTF-8 path: {:?}", src))?;

        let wand = new_wand();

        let px: usize = *size as usize;
        wand.read_image(src_str)
            .context(format!("reading image {:?}", src))?;
        wand.fit(px, px);

        let meta = Meta::read(&src, &wand).context("fetching metadata")?;
        for (k, v) in meta.to_hashmap() {
            wand.set_image_property(k, &v)?;
        }

        let bytes = wand
            .write_image_blob(THUMB_IMAGE_FORMAT.as_str())
            .context("encoding thumbnail")?;

        let mut file =
            fs::File::create(&des).context(format!("creating thumbnail file in {:?}", &des))?;
        file.write(&bytes)
            .context(format!("saving thumbnail to {:?}", &des))?;

        Ok(des)
    }

    // tries to generate thumbnail if can't, then tries to generate empty thumbnail
    pub fn generate(&self) -> Result<PathBuf, ThumbError> {
        // if thumbnail already exists, return that
        let thumbnail_path = self.gen_thumb_path().map_err(|_| ThumbError::NoCachePath)?;
        if thumbnail_path.exists() {
            return Ok(thumbnail_path);
        }

        let cache = Self::get_cache_fail_path()?;
        let filename = Self::gen_filename(&self.origin);
        let fail_marker = cache.join(filename);
        if fail_marker.is_file() {
            return Err(ThumbError::KnownFailure);
        }

        self.compress_save(thumbnail_path).map_err(|e| {
            let _ = std::fs::File::create(&fail_marker)
                .inspect_err(|io| log::warn!("couldn't write fail marker {fail_marker:?}: {io}"));

            ThumbError::Generate(e)
        })
    }

    pub fn make(origin: PathBuf, size: ThumbSize) -> Option<String> {
        let thumbnail = Self { origin, size };
        thumbnail
            .generate()
            .unwrap_or(thumbnail.origin)
            .to_str()
            .map(String::from)
    }

    fn gen_thumb_path(&self) -> Result<PathBuf> {
        // generates path to thumnbnails cache
        let mut cache = Self::get_cache_path()?;
        cache.push(self.size.path());

        // create folders if doesn't exist
        fs::create_dir_all(&cache)?;

        // adds filename to the end of cache path
        let filename = Self::gen_filename(&self.origin);
        cache.push(filename);

        Ok(cache)
    }

    fn get_cache_path() -> Result<PathBuf> {
        let cache: ExpandedPathBuf = THUMB_CACHE.parse().or(THUMB_CACHE_FALLBACK.parse())?;
        let exists = fs::exists(&cache)?;
        if !exists {
            fs::create_dir_all(&cache)?;
        }

        Ok(cache.into())
    }

    fn get_cache_fail_path() -> Result<PathBuf> {
        let mut cache = Self::get_cache_path().context(format!("Couldn't get cache path"))?;
        cache.push("fail");
        fs::create_dir_all(&cache).context(format!(
            "Couldn't create cache directory for failed thumbnails path"
        ))?;

        Ok(cache)
    }

    fn find_mimetype(filepath: &Path) -> Option<Mime> {
        filepath
            .file_name()
            .and_then(|name| name.to_string_lossy().to_string().parse::<Mime>().ok())
    }

    pub fn gen_filename(path: &Path) -> String {
        let path = path.to_str().unwrap_or_default();
        let digest = md5::compute(format!("file://{path}"));
        format!("{digest:?}.{}", THUMB_IMAGE_FORMAT.as_str())
    }
}
