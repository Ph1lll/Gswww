use directories::ProjectDirs;

pub fn _config() {
    let project_dirs = ProjectDirs::from("com", "Ph1lll", "Gswww")
        .expect("Failed to retrieve project directories.");
    let config_dir = project_dirs.config_dir();

    if !config_dir.exists() {
        std::fs::create_dir(config_dir).expect("Failed to create config directory.");
    }

    let _file_path = config_dir.join("config.ron");
}
