#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use thumbnailer::{ThumbSize, thumbnail};

    #[test]
    fn generate_thumbnail() {
        let path = PathBuf::from("tests/reze.jxl");
        let thumbnail = thumbnail(&path, ThumbSize::Normal);
        assert!(thumbnail.is_ok(), "Thumbnbail couldn't be created")
    }
}
