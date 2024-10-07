use std::{borrow::Borrow, fs, path::Path};

use imagesize::ImageSize;

pub struct Image {
    pub image_size: ImageSize,
    pub image_path: String,
    pub image_bytes: Vec<u8>
}

impl Image {

    pub fn from_path(path: &Path) -> Self {
        let image_string_path = path.to_string_lossy();

        let image_bytes = fs::read(path).expect(&format!("Failed to read image at '{}'!", &image_string_path));

        let image_size = imagesize::blob_size(&image_bytes).expect(
            "Failed to retrieve the dimensions of the image!"
        );

        Self {
            image_size,
            image_path: image_string_path.to_string(),
            image_bytes: image_bytes
        }
    }
}