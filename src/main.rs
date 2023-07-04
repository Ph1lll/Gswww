use gtk::{glib, prelude::*, Application, ApplicationWindow, Window};
use gtk::{
    Align, Box, Button, DropDown, FileChooserAction, FileChooserNative, FlowBox, Image,
    Orientation, ScrolledWindow, StringList,
};
use rayon::prelude::*;

// Options for the dropdown
const TRANSISTION_OPTIONS: [&str; 12] = [
    "simple", "left", "right", "top", "bottom", "wipe", "wave", "grow", "center", "any", "outer",
    "random",
];

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.github.Ph1lll.Gswww")
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let content = Box::new(Orientation::Vertical, 0);
    content.append(&gtk::HeaderBar::new());

    let image_grid = FlowBox::new(); // Allows to change the rows and columns depending on the size of the window
    let gallery = ScrolledWindow::builder() // Allows to scroll through the wallpapers
        .child(&image_grid)
        .vexpand(true)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Gswww")
        .default_width(900)
        .default_height(600)
        .child(&content)
        .build();

    // Button to open dialog
    let dialog_button = Button::builder()
        .label("Select Folder")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .halign(Align::Start)
        .build();

    // Dropdown for transition types
    let transition_types = DropDown::builder()
        .model(&StringList::new(&TRANSISTION_OPTIONS))
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .halign(Align::End)
        .build();

    // Box for all options and buttons
    let option_box = Box::new(Orientation::Horizontal, 0);
    option_box.append(&dialog_button);
    option_box.append(&transition_types);

    // Add the main boxes of content
    content.append(&gallery);
    content.append(&option_box);

    let dialog = FileChooserNative::new(
        Some("Select Folder"),
        Window::NONE,
        FileChooserAction::SelectFolder,
        Some("Select"),
        Some("Cancel"),
    );
    dialog.set_transient_for(Some(&window));

    // When you Select Folder
    dialog.connect_response(move |dialog, response| {
        // Hide the dialog
        dialog.hide();

        // If folder selected
        if response == gtk::ResponseType::Accept {
            // Get the path to folder
            let folder_path = dialog.file().unwrap();
            let folder_path = folder_path.path().unwrap();
            match search_folder(folder_path.to_str().unwrap()) {
                // Use those paths to create images
                Ok(entries) => {
                    for entry in entries {
                        let image = Image::from_file(&entry);
                        image.set_size_request(200, 200);

                        // Clicking on image will send a command to swww
                        let gesture = gtk::GestureClick::builder()
                            .button(gtk::gdk::ffi::GDK_BUTTON_PRIMARY as u32) // Left Click
                            .build();
                        gesture.connect_pressed(
                            glib::clone!(@weak transition_types => move |_, _, _, _| {
                                swww(
                                    entry.to_str().unwrap(),    // Path to file
                                    &transition_types,          // Dropdown selection
                                    &TRANSISTION_OPTIONS,       // Options to transistion
                                )
                            }),
                        );

                        image.add_controller(gesture); // Make sure the command is sent
                        image_grid.insert(&image, -1); // Add image to grid
                    }
                }
                // Just in case
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            }
        }
    });

    dialog_button.connect_clicked(move |_| {
        dialog.show();
    });

    window.present();
}

// Send command to swww
fn swww(file: &str, transition: &DropDown, options: &[&str]) {
    std::process::Command::new("swww")
        .args(["img", "-t", options[transition.selected() as usize], file])
        .spawn()
        .expect("Failed to change background");
}

fn search_folder(folder_path: &str) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
    // List of file extensions to search for
    let file_extensions = vec![
        "png", "jpg", "jpeg", "gif", "pnm", "tga", "tiff", "webp", "bmp",
    ];

    let entries = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));

    // Read the contents of the folder
    let folder_entries = std::fs::read_dir(folder_path)?;

    // Parallelize the search of image files (Slight HACK)
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
