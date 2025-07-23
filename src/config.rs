use directories::ProjectDirs;
use std::fs::create_dir;

pub fn config_path() {
    let project_dirs = ProjectDirs::from("com.github", "Ph1lll", "Gswww")
        .expect("Failed to retrieve project directories.");
    let config_dir = project_dirs.config_dir();

    if !config_dir.exists() {
        create_dir(config_dir).expect("Failed to create config directory.");
    }
}

pub fn cache_path() -> String {
    let project_dirs = ProjectDirs::from("com.github", "Ph1lll", "Gswww")
        .expect("Failed to retrieve project directories.");
    let cache_dir = project_dirs.cache_dir();

    if !cache_dir.exists() {
        create_dir(cache_dir).expect("Failed to create config directory.");
    }
    cache_dir.as_os_str().to_str().unwrap().to_string()
}
