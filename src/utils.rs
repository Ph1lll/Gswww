use rayon::prelude::*;
use std::{ffi::OsString, path::PathBuf};
use walkdir::WalkDir;

// List of file extensions to search for
const FILE_EXTENSIONS: [&str; 9] = [
    "png", "jpg", "jpeg", "gif", "pnm", "tga", "tiff", "webp", "bmp",
];

#[derive(Clone)]
pub struct Thumbnail {
    pub image_path: String,
    pub thumbnail_path: String,
}

pub fn get_thumbnails(folder_path: OsString, recursive: &bool) -> Vec<Thumbnail> {
    let cache_location = crate::config::cache_path();
    let depth = if !recursive { 1 } else { 5 };

    WalkDir::new(folder_path)
        .max_depth(depth)
        .into_iter()
        .par_bridge()
        .filter_map(|entry| match entry {
            Ok(entry) if entry.file_type().is_file() => {
                let path = entry.into_path();
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    if FILE_EXTENSIONS.contains(&ext) {
                        let thumbnail_path = create_thumbnail(&path, &cache_location);

                        Some(Thumbnail {
                            image_path: path.to_str().unwrap().to_string(),
                            thumbnail_path,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect()
}

fn create_thumbnail(file: &PathBuf, cache_location: &str) -> String {
    use fast_image_resize::images::Image;
    use fast_image_resize::{IntoImageView, Resizer};
    use image::ImageReader; // Read source image from file
    use xxhash_rust::xxh3::xxh3_64; // Source Image to a unique hash for cache

    let src_image = ImageReader::open(file)
        .expect("Failed to open file")
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    let image_hash = xxh3_64(src_image.as_bytes());
    let thumbnail_location = format!("{cache_location}/{image_hash}.png");

    if !std::path::Path::new(&thumbnail_location).exists() {
        println!("Creating thumbnail at: {thumbnail_location}");

        // Create container for data of destination image
        let dst_width: u32 = 200;
        let dst_height: u32 = 112;
        let mut dst_image = Image::new(dst_width, dst_height, src_image.pixel_type().unwrap());

        // Create Resizer instance and resize source image
        // into buffer of destination image
        let mut resizer = Resizer::new();
        resizer.resize(&src_image, &mut dst_image, None).unwrap();

        // Write destination image as a PNG
        let _ = image::save_buffer(
            std::path::Path::new(&thumbnail_location),
            dst_image.buffer(),
            dst_width,
            dst_height,
            src_image.color(),
        );
    } else {
        println!("Hit Cache!");
    }

    thumbnail_location
}
