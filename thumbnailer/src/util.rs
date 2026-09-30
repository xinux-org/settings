use crate::{Meta, ThumbSize, THUMB_CACHE, THUMB_CACHE_FALLBACK, THUMB_IMAGE_FORMAT};
use anyhow::Result;
use expanded_pathbuf::ExpandedPathBuf;
use magick_rust::{magick_wand_genesis, MagickWand};
use mime::Mime;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::Once,
};

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
    let meta = Meta::fetch_meta(&path, &wand)?;
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
