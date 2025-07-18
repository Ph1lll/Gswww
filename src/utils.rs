use gtk::{
    gdk::ffi::GDK_BUTTON_PRIMARY, gdk_pixbuf::Pixbuf, glib::clone, prelude::*, DropDown, FlowBox,
    GestureClick, Image,
};
use rayon::prelude::*;
use std::{io::Error, path::PathBuf, process::Command};
use walkdir::WalkDir;

// Options for the dropdown
pub const TRANSISTION_OPTIONS: [&str; 12] = [
    "random", "simple", "left", "right", "top", "bottom", "wipe", "wave", "grow", "center", "any",
    "outer",
];

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

pub fn search_folder(folder_path: &str, recursive: &bool) -> Result<Vec<PathBuf>, Error> {
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

pub fn add_images(
    folder_location: &str,
    recursively_search: &bool,
    transition_types: &DropDown,
    image_grid: &FlowBox,
) {
    println!("{:-<100}", "");
    let images = match search_folder(folder_location, recursively_search) {
        Ok(entries) => {
            if *recursively_search {
                println!("Searched '{folder_location}' \n");
            } else {
                println!("Searched '{folder_location}' without recursion \n");
            }
            entries
        }
        Err(err) => {
            eprintln!("Error: {err}");
            return;
        }
    };

    let time_taken_dir = std::time::Instant::now();
    for entry in images {
        let time_taken = std::time::Instant::now();

        // New Way
        let pixbuf = Pixbuf::from_file_at_size(&entry, 200, 200).ok();
        let image = Image::from_pixbuf(pixbuf.as_ref());

        image.set_size_request(200, 200); // Load and set image size

        // Create gesture for click event
        let gesture = GestureClick::new();
        gesture.set_button(GDK_BUTTON_PRIMARY as u32);
        gesture.connect_pressed(clone!(
            #[strong]
            entry,
            #[strong]
            transition_types,
            move |_, _, _, _| {
                swww(entry.clone(), &transition_types);
            }
        ));

        // Add gesture and insert image in UI
        image.add_controller(gesture);
        image_grid.append(&image);

        // Mostly a debug line
        println!(
            "Added: {}, took {} ms",
            entry.to_str().unwrap(),
            time_taken.elapsed().as_millis()
        );
    }
    println!("Took {} sec for dir", time_taken_dir.elapsed().as_secs());
}
