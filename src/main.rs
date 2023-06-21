use adw::gtk::{
    Align, Box, Button, DropDown, FileChooserAction, FileChooserNative, FlowBox, Image,
    Orientation, PolicyType, ScrolledWindow, StringList,
};
use adw::{glib, prelude::*, Application, ApplicationWindow, Window};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.github.dwogo.Gswww")
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let content = Box::new(Orientation::Vertical, 0);
    content.append(
        &adw::gtk::HeaderBar::builder()
            .title_widget(&adw::WindowTitle::new("Gswww", ""))
            .build(),
    );

    let image_grid = FlowBox::builder().column_spacing(1).row_spacing(1).build();
    let gallery = ScrolledWindow::builder()
        .child(&image_grid)
        .hexpand(true)
        .vexpand(true)
        .hscrollbar_policy(PolicyType::Automatic)
        .vscrollbar_policy(PolicyType::Automatic)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Gswww")
        .content(&content)
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

    // Options for the dropdown
    let transition_options = [
        "simple", "left", "right", "top", "bottom", "wipe", "grow", "center", "any", "outer",
        "random",
    ];

    // Dropdown for transition types
    let transition_types = DropDown::builder()
        .model(&StringList::new(&transition_options))
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

    dialog.connect_response(move |dialog, response| {
        if response == adw::gtk::ResponseType::Accept {
            if let Some(path) = dialog.file() {
                if let Some(folder_path) = path.path() {
                    match search_folder(folder_path.to_str().unwrap()) {
                        Ok(entries) => {
                            for entry in entries {
                                let image = Image::from_file(&entry);
                                let gesture = adw::gtk::GestureClick::new();
                                gesture.set_button(adw::gtk::gdk::ffi::GDK_BUTTON_PRIMARY as u32);
                                gesture.connect_pressed(
                                    glib::clone!(@weak transition_types => move |_, _, _, _| {
                                        swww(
                                            entry.to_str().unwrap(),
                                            &transition_types,
                                            &transition_options,
                                        )
                                    }),
                                );
                                image.add_controller(gesture);
                                image.set_size_request(200, 200);
                                image_grid.insert(&image, -1);
                            }
                        }
                        Err(err) => {
                            eprintln!("Error: {}", err);
                        }
                    }
                }
            }

            dialog.hide();
        }
    });

    dialog_button.connect_clicked(move |_| {
        dialog.show();
    });

    window.present();
}

fn swww(file: &str, transition: &DropDown, options: &[&str]) {
    std::process::Command::new("swww")
        .args(["img", "-t", options[transition.selected() as usize], file])
        .spawn()
        .expect("Failed to change background");
}

fn search_folder(folder_path: &str) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
    // Specify the file extensions you want to search for
    let file_extensions = vec!["png", "jpg", "gif", "pnm", "tga", "tiff", "webp", "bmp"];

    let mut entries = Vec::new();

    // Read the contents of the folder
    let folder_entries = std::fs::read_dir(folder_path)?;

    for entry_result in folder_entries {
        let entry = entry_result?;
        let entry_path = entry.path();

        // Check if the entry is a file or a subfolder
        if entry_path.is_file() {
            if let Some(extension) = entry_path.extension() {
                if file_extensions.iter().any(|&ext| ext == extension) {
                    entries.push(entry_path.clone());
                }
            }
        } else if entry_path.is_dir() {
            // Recursively search subfolders
            let subfolder_entries = search_folder(entry_path.to_str().unwrap())?;
            entries.extend(subfolder_entries);
        }
    }

    Ok(entries)
}
