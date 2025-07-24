use gtk::{
    gdk::ffi::GDK_BUTTON_PRIMARY, glib::clone, prelude::*, DropDown, FlowBox, GestureClick, Image,
};
use rayon::prelude::*;
use std::{ffi::OsString, io::Error, path::PathBuf, process::Command};
use walkdir::WalkDir;

#[derive(Clone)]
struct Thumbnails {
    file_path: String,
    thumbnail: String,
}

// Options for the dropdown
pub const TRANSISTION_OPTIONS: [&str; 12] = [
    "random", "simple", "left", "right", "top", "bottom", "wipe", "wave", "grow", "center", "any",
    "outer",
];

pub fn add_images(
    folder_location: OsString,
    recursively_search: &bool,
    transition_types: &DropDown,
    image_grid: &FlowBox,
) {
    use std::time::Instant;
    println!("{:-<100}", "");
    let images = match search_folder(&folder_location, recursively_search) {
        Ok(entries) => {
            if *recursively_search {
                println!("Searched '{}' \n", folder_location.into_string().unwrap());
            } else {
                println!(
                    "Searched '{}' without recursion \n",
                    folder_location.into_string().unwrap()
                );
            }
            entries
        }
        Err(err) => {
            eprintln!("Error: {err}");
            return;
        }
    };

    let time_taken_dir = Instant::now();

    let cache_location = crate::config::cache_path();

    let thumbnails: Vec<Thumbnails> = images
        .par_iter()
        .map(|entry| {
            let time_taken = Instant::now();

            let get_thumbnail = create_thumbnail(entry, &cache_location);

            // Mostly a debug line
            println!(
                "Processed: {}, took {} ms",
                entry.to_str().unwrap(),
                time_taken.elapsed().as_millis()
            );
            Thumbnails {
                file_path: entry.to_str().unwrap().to_string(),
                thumbnail: get_thumbnail,
            }
        })
        .collect();

    for entry in thumbnails {
        let image = Image::from_file(&entry.thumbnail);
        image.set_size_request(200, 200); // Set image size for gallery

        // Create gesture for click event
        let gesture = GestureClick::new();
        gesture.set_button(GDK_BUTTON_PRIMARY as u32);
        gesture.connect_pressed(clone!(
            #[strong]
            entry,
            #[strong]
            transition_types,
            move |_, _, _, _| {
                swww(entry.file_path.clone().into(), &transition_types);
            }
        ));

        // Add gesture and insert image in UI
        image.add_controller(gesture);
        image_grid.append(&image);
    }
    if time_taken_dir.elapsed().as_millis() < 3000 {
        println!("Took {} ms for dir", time_taken_dir.elapsed().as_millis());
    } else {
        println!("Took {} sec for dir", time_taken_dir.elapsed().as_secs());
    }
}

pub fn search_folder(folder_path: &OsString, recursive: &bool) -> Result<Vec<PathBuf>, Error> {
    // List of file extensions to search for
    let file_extensions: [&str; 9] = [
        "png", "jpg", "jpeg", "gif", "pnm", "tga", "tiff", "webp", "bmp",
    ];

    let depth = if !recursive { 1 } else { 5 };

    // Recursively find files using WalkDir
    let entries: Vec<PathBuf> = WalkDir::new(folder_path)
        .max_depth(depth)
        .into_iter()
        .par_bridge()
        .filter_map(|entry| match entry {
            Ok(entry) if entry.file_type().is_file() => {
                let path = entry.into_path();
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    if file_extensions.contains(&ext) {
                        Some(path)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect();

    Ok(entries)
}

fn create_thumbnail(file: &PathBuf, cache_location: &str) -> String {
    use fast_image_resize::images::Image;
    use fast_image_resize::{IntoImageView, Resizer};
    use image::ImageReader;

    let thumbnail_location = format!(
        "{}/{}_{}.png",
        cache_location,                              // Cache folder
        file.file_stem().unwrap().to_str().unwrap(), // File name (excluding extension)
        file.extension().unwrap().to_str().unwrap()  // File extension
    );

    if !std::path::Path::new(&thumbnail_location).exists() {
        println!("Creating thumbnail at: {thumbnail_location}");

        // Read source image from file
        let src_image = ImageReader::open(file)
            .expect("Failed to open file")
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();

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

// Send command to swww
pub fn swww(file: PathBuf, transition: &DropDown) {
    println!("Selected: {}", &file.to_str().unwrap());
    println!("{:-<100}", "");
    Command::new("swww")
        .args([
            "img",
            "-t",
            TRANSISTION_OPTIONS[transition.selected() as usize],
            file.to_str().unwrap(),
        ])
        .spawn()
        .expect("Failed to change background");
}
