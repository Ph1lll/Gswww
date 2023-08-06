use directories::ProjectDirs;
use std::fs::create_dir;

pub fn config_path() {
    let project_dirs = ProjectDirs::from("com", "Ph1lll", "Gswww")
        .expect("Failed to retrieve project directories.");
    let config_dir = project_dirs.config_dir();

    if !config_dir.exists() {
        create_dir(config_dir).expect("Failed to create config directory.");
    }
}
