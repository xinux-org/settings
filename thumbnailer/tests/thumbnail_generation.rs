#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use thumbnailer::{ThumbSize, Thumbnail};

    #[test]
    fn generate_thumbnail() {
        let thumbnail = Thumbnail {
            origin: PathBuf::from("tests/reze.jxl"),
            size: ThumbSize::Large,
        }
        .generate();
        assert!(thumbnail.is_ok(), "Thumbnbail couldn't be created")
    }
}
