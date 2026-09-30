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
    // gen_thumbnail(filepath, size).or_else(|_| {
    //     let mut failure_cache = get_cache_fail_path();
    //     failure_cache.push(gen_filename(filepath));

    //     failure_cache
    // })
    if let Some(thumb) = gen_thumbnail(filepath, size).ok() {
        return thumb;
    } else {
        let mut failed_thumbnail_path = get_cache_fail_path();
        failed_thumbnail_path.push(gen_filename(filepath));

        let mut file = fs::File::create(&failed_thumbnail_path)
            .expect("couldn't create failed thumbnail file");
        file.write(&vec![]);

        failed_thumbnail_path
    }
}

/// Creates and saves thumbnail, then returns its path
pub fn gen_thumbnail(filepath: &str, size: ThumbSize) -> Result<PathBuf> {
    // initialize MagickWand to create thumbnail
    let start: Once = Once::new();
    start.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();

    // read and resize image
    wand.read_image(filepath)?;
    wand.fit(size.into(), size.into());

    // add metadata to thumbnail
    let path = PathBuf::from(filepath).canonicalize()?;
    let meta = Meta::fetch_meta(&path, &wand)?;
    for (k, v) in meta.to_hashmap() {
        wand.set_image_property(k, &v)?;
    }

    // generate thumbnail path
    let thumb_path = gen_thumb_path(filepath, size)?;

    // saving thumbnail
    let bytes = wand.write_image_blob(THUMB_IMAGE_FORMAT.into())?;
    let mut file = fs::File::create(&thumb_path)?;
    file.write(&bytes)?;

    // Return the path to thumbnail
    Ok(thumb_path)
}

pub fn gen_thumb_path(filepath: &str, size: ThumbSize) -> Result<PathBuf> {
    // generates path to thumnbnails cache
    let mut cache = get_cache_path()?;
    cache.push(size.path());

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
    fn gen_thumbnail() {
        let thumbnail = thumbnail("./test/reze.jxl", ThumbSize::Normal);
        assert!(thumbnail.is_ok(), "Thumbnbail couldn't be created")
    }
}
