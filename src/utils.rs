use directories::ProjectDirs;
use rayon::prelude::*;
use walkdir::WalkDir;
use gtk::{
    gdk::ffi::GDK_BUTTON_PRIMARY, 
    glib::{self,MainContext},
    DropDown, 
    FlowBox, 
    GestureClick, 
    Image,
    prelude::*,
};
use std::{
    fs::{read_dir, remove_file},
    io::Error,
    path::{Path, PathBuf},
    process::Command,
    collections::HashSet,
};

// Send command to swww
pub fn swww(file: &Path, transition: &DropDown, options: [&str; 12]) {
    Command::new("swww")
        .args([
            "img",
            "-t",
            options[transition.selected() as usize],
            file.to_str().unwrap(),
        ])
        .spawn()
        .expect("Failed to change background");
    let project_dirs = ProjectDirs::from("com", "Ph1lll", "Gswww")
        .expect("Failed to retrieve project directories.");

    remove_last(project_dirs.config_dir());

    let write_dir = format!(
        "{}/last.{}",
        project_dirs.config_dir().display(),
        file.extension().unwrap().to_str().unwrap()
    );
    Command::new("cp")
        .args([file.to_str().unwrap(), &write_dir])
        .spawn()
        .expect("Failed to set last image");
}

fn remove_last(config_path: &Path) {
    let entries = read_dir(config_path).unwrap();
    for entry in entries.flatten() {
        let path = entry.path();
        if let Some(filename) = path.file_name() {
            if let Some(filename_str) = filename.to_str() {
                if filename_str.starts_with("last.") {
                    let _ = remove_file(path);
                }
            }
        }
    }
}

pub fn search_folder(folder_path: &str) -> Result<Vec<PathBuf>, Error> {
    // List of file extensions to search for
    let file_extensions: HashSet<&str> = [
        "png", "jpg", "jpeg", "gif", "pnm", "tga", "tiff", "webp", "bmp",
    ]
    .iter()
    .cloned()
    .collect();

    // Recursively find files using WalkDir
    let entries: Vec<PathBuf> = WalkDir::new(folder_path)
        .into_iter()
        .par_bridge()
        .filter_map(|entry| match entry {
            Ok(entry) if entry.file_type().is_file() => {
                let path = entry.into_path();
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    if file_extensions.contains(ext) {
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
    transition_types: &DropDown,
    image_grid: &FlowBox,
    options: &'static [&str; 12],
) {
    let context = MainContext::default();
    let images = match search_folder(folder_location) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };
    
    context.spawn_local(glib::clone!(@strong transition_types, @strong image_grid => async move {
        for entry in images {
            let image_path = entry.clone();
            let image = Image::from_file(entry.clone());
            image.set_size_request(200, 200);

            // Create gesture for click event
            let gesture = GestureClick::new();
            gesture.set_button(GDK_BUTTON_PRIMARY as u32);
            gesture.connect_pressed(glib::clone!(@strong transition_types => move |_, _, _, _| {
                swww(
                    &image_path,
                    &transition_types,
                    *options,
                );
            }));

            // Add gesture and insert image in UI
            image.add_controller(gesture);
            image_grid.insert(&image, -1);
        }
    }));
}
