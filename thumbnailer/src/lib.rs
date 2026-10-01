extern crate anyhow;
extern crate expanded_pathbuf;
extern crate magick_rust;
extern crate mime;

use anyhow::Result;
use expanded_pathbuf::ExpandedPathBuf;
use magick_rust::{magick_wand_genesis, MagickWand};
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
    // fn to_hashmap(&self) -> HashMap<String, String> {
    //     let Meta {
    //         uri,
    //         mtime,
    //         size,
    //         mimetype,
    //         description,
    //         software,
    //     } = self;
    //     let list = [
    //         ("Thumb::URI", Some(uri.to_string())),
    //         ("Thumb::MTime", Some(mtime.to_string())),
    //         ("Thumb::Size", size.and_then(|x| x.to_string().into())),
    //         (
    //             "Thumb::MimeType",
    //             mimetype.as_ref().and_then(|x| x.to_string().into()),
    //         ),
    //         (
    //             "Description",
    //             description.as_ref().and_then(|x| x.to_string().into()),
    //         ),
    //         (
    //             "Software",
    //             software.as_ref().and_then(|x| x.to_string().into()),
    //         ),
    //     ]
    //     .into_iter()
    //     .filter_map(|(key, v)| v.and_then(|value| Some(((*key).to_string(), value))))
    //     .collect::<Vec<(String, String)>>();
    //     HashMap::from(list)
    // }

    fn to_hashmap(&self) -> HashMap<&'static str, String> {
        let Meta {
            uri,
            mtime,
            size,
            mimetype,
            description,
            software,
        } = self;

        [
            ("Thumb::URI", Some(uri.clone())),
            ("Thumb::MTime", Some(mtime.to_string())),
            ("Thumb::Size", size.map(|x| x.to_string())),
            ("Thumb::MimeType", mimetype.as_ref().map(|x| x.to_string())),
            ("Description", description.clone()),
            ("Software", software.clone()),
        ]
        .into_iter()
        .filter_map(|(key, value)| value.as_ref().map(|v| (*key, v)))
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

pub fn thumbnail(filepath: &str, size: ThumbSize) -> PathBuf {
    // check if thumbnail path can be generated (folders exist and etc)
    if let Ok(thumb_path) = gen_thumb_path(filepath, size) {
        // if thumbnail already exists, return its path
        if std::fs::exists(&thumb_path).unwrap_or(false)
            || gen_thumbnail(filepath, &thumb_path, size).is_ok()
        {
            return thumb_path;
        } else {
            // if thumbnail can't be resized successfully just return empty thumbnail
            let mut failed_thumbnail_path = get_cache_fail_path();
            failed_thumbnail_path.push(gen_filename(filepath));
            let mut file = fs::File::create(&failed_thumbnail_path)
                .expect("couldn't create failed thumbnail file");
            file.write(&vec![])
                .expect("couldn't write inside failed thumbnail file");

            return failed_thumbnail_path;
        }
    } else {
        PathBuf::from(filepath)
    }
}

/// Creates and saves thumbnail, then returns its path
pub fn gen_thumbnail(original_filepath: &str, thumb_path: &Path, size: ThumbSize) -> Result<()> {
    let iiii = std::time::Instant::now();
    // initialize MagickWand to create thumbnail
    let start: Once = Once::new();
    println!("start: {:?}", iiii.elapsed());
    start.call_once(|| {
        magick_wand_genesis();
    });
    println!("start.call_once(): {:?}", iiii.elapsed());
    let wand = MagickWand::new();

    // read and resize image
    wand.read_image(original_filepath)?;
    println!("wand.read_image(): {:?}", iiii.elapsed());
    wand.fit(size.into(), size.into());
    println!("wand.fit(): {:?}", iiii.elapsed());

    // add metadata to thumbnail
    let path = PathBuf::from(original_filepath).canonicalize()?;
    let meta = Meta::read(&path, &wand)?;
    for (k, v) in meta.to_hashmap() {
        wand.set_image_property(k, &v)?;
    }

    // saving thumbnail
    let bytes = wand.write_image_blob(THUMB_IMAGE_FORMAT.into())?;
    println!("wand.write_image_blob(): {:?}", iiii.elapsed());
    let mut file = fs::File::create(&thumb_path)?;
    file.write(&bytes)?;
    println!("write(): {:?}", iiii.elapsed());

    // Return the path to thumbnail
    Ok(())
}

pub fn gen_thumb_path(filepath: &str, size: ThumbSize) -> Result<PathBuf> {
    // generates path to thumnbnails cache
    let mut cache = get_cache_path()?;
    cache.push(size.path());

    // create folders if doesn't exist
    fs::create_dir_all(&cache)?;

    // adds filename to the end of cache path
    let filename = gen_filename(filepath);
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

pub fn gen_filename(filepath: &str) -> String {
    let digest = md5::compute(filepath.as_bytes());
    format!("{digest:?}.{}", THUMB_IMAGE_FORMAT.as_str())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn generate_thumbnail() {
        let thumbnail = thumbnail("./test/reze.jxl", ThumbSize::Normal)
            .to_str()
            .map(String::from)
            .unwrap_or_default();
        assert!(
            !thumbnail.contains("fail"),
            "Thumbnbail couldn't be created"
        )
    }
}
