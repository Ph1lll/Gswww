use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{self, create_dir, File};

pub fn _config_path() -> String {
    let project_dirs = ProjectDirs::from("com", "Ph1lll", "Gswww")
        .expect("Failed to retrieve project directories.");
    let config_dir = project_dirs.config_dir();
    let path = format!("{config_dir:?}");
    let path = format!("{}/folders.ron", &path[1..path.len() - 1]);

    if !config_dir.exists() {
        create_dir(config_dir).expect("Failed to create config directory.");
        File::create(&path).expect("Failed to create folders.ron");
    }
    path
}

#[derive(Serialize, Deserialize)]
pub struct _Locations {
    folder_paths: Vec<String>,
}

impl _Locations {
    fn _check_folders() {
        // Read folders.ron
        let contents = fs::read_to_string(_config_path()).expect("Failed to read config file");
        // Deserialize the data from .ron format
        let _data: _Locations = ron::from_str(&contents).expect("Failed to deserialize data.");
    }

    fn _add_folder_path(&mut self, folder_path: &str) {
        if self.folder_paths.contains(&folder_path.to_string()) {
            println!("The path '{folder_path}' already exists. Skipping command execution.");
        } else {
            self.folder_paths.push(folder_path.to_string());
        }
    }
}
