#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use thumbnailer::{Thumbnail, ThumbSize};

    #[test]
    fn generate_thumbnail() {
        let path = PathBuf::from("tests/reze.jxl");

        let thumbnail = Thumbnail::new(&path, ThumbSize::Normal);
        let thumb = thumbnail.thumbnail();

        assert!(thumb.is_ok(), "Thumbnbail couldn't be created")
    }
}
