use std::{fs, path::Path};

use imagesize::ImageSize;
use eframe::egui::{load::Bytes, ImageSource};

pub struct Image<'a> {
    pub image_size: ImageSize,
    pub image_source: ImageSource<'a>
}

impl Image<'_> {

    fn from_path(path: &Path) -> Self {
        let image_string_path = path.to_str().expect("Failed to convert image path to str!");

        let image_bytes = fs::read(path).expect(&format!("Failed to read image at '{}'!", &image_string_path));

        let image_size = imagesize::blob_size(&image_bytes).expect(
            "Failed to retrieve the dimensions of the image!"
        );

        let image_source = ImageSource::Bytes { 
            uri: std::borrow::Cow::Owned(
                format!("bytes://{}", image_string_path)
            ),
            bytes: Bytes::Shared(image_bytes.into())
        };

        Self { image_size, image_source }
    }
}