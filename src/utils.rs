use rayon::prelude::*;

// Send command to swww
pub fn swww(file: &str, transition: &gtk::DropDown, options: &[&'static str; 12]) {
    std::process::Command::new("swww")
        .args(["img", "-t", options[transition.selected() as usize], file])
        .spawn()
        .expect("Failed to change background");
}

pub fn search_folder(folder_path: &str) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
    // List of file extensions to search for
    let file_extensions = vec![
        "png", "jpg", "jpeg", "gif", "pnm", "tga", "tiff", "webp", "bmp",
    ];

    let entries = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));

    // Read the contents of the folder
    let folder_entries = std::fs::read_dir(folder_path)?;

    // Parallelize the search of image files
    folder_entries.par_bridge().for_each(|entry_result| {
        if let Ok(entry) = entry_result {
            let entry_path = entry.path();

            // Check if the entry is a file or a subfolder
            if entry_path.is_file() {
                if file_extensions
                    .iter()
                    .any(|&ext| ext == entry_path.extension().unwrap())
                {
                    let mut locked_entries = entries.lock().unwrap();
                    locked_entries.push(entry_path);
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
