use directories::ProjectDirs;
use gtk::{gdk::ffi::GDK_BUTTON_PRIMARY, glib, prelude::*, DropDown, FlowBox, GestureClick, Image};
use rayon::prelude::*;
use std::{
    fs::{read_dir, remove_file},
    io::Error,
    path::{Path, PathBuf},
    process::Command,
    sync::{Arc, Mutex},
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
    let file_extensions = vec![
        "png", "jpg", "jpeg", "gif", "pnm", "tga", "tiff", "webp", "bmp",
    ];

    let entries = Arc::new(Mutex::new(Vec::new()));

    // Read the contents of the folder
    let folder_entries = read_dir(folder_path)?;

    // Parallelize the search of image files
    folder_entries.par_bridge().for_each(|entry_result| {
        if let Ok(entry) = entry_result {
            let entry_path = entry.path();

            // Check if the entry is a file or a subfolder
            if entry_path.is_file() {
                if file_extensions
                    .par_iter()
                    .any(|&ext| ext == entry_path.extension().unwrap())
                {
                    entries.lock().unwrap().push(entry_path);
                }
            } else if entry_path.is_dir() {
                // Recursively search subfolders
                if let Ok(subfolder_entries) = search_folder(entry_path.to_str().unwrap()) {
                    let mut locked_entries = entries.lock().unwrap();
                    locked_entries.extend(subfolder_entries);
                }
            }
        }
    });

    // Retrieve and return the results from the Arc<Mutex<Vec<PathBuf>>>
    let result = entries.lock().unwrap();
    Ok(result.clone())
}

pub fn add_images(
    folder_location: &str,
    transition_types: &DropDown,
    image_grid: &FlowBox,
    options: &'static [&str; 12],
) {
    match search_folder(folder_location) {
        // Use those paths to create images
        Ok(entries) => {
            for entry in entries {
                let image = Image::from_file(&entry);
                image.set_size_request(200, 200);

                // Clicking on image will send a command to swww
                let gesture = GestureClick::new();
                gesture.set_button(GDK_BUTTON_PRIMARY as u32);
                gesture.connect_pressed(glib::clone!(@weak transition_types => move |_, _, _, _| {
                    swww(
                        &entry,    // Path to file
                        &transition_types,          // Dropdown selection
                        *options                    // Dropdown options
                    )
                }));

                image.add_controller(gesture); // Make sure the command is sent
                image_grid.insert(&image, -1); // Add it to the grid
            }
        }
        // Just in case
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
