use gtk::{
    gio::ApplicationFlags, glib, prelude::*, Align, Application, ApplicationWindow, Box, Button,
    DropDown, FileDialog, FlowBox, HeaderBar, Image, Label, Orientation, ScrolledWindow,
    StringList, Switch,
};
use std::ffi::OsString;

mod config;
mod utils;

// Options for the dropdown
const TRANSISTION_OPTIONS: [&str; 12] = [
    "random", "simple", "left", "right", "top", "bottom", "wipe", "wave", "grow", "center", "any",
    "outer",
];

fn main() -> glib::ExitCode {
    // Create config directory if not added
    config::config_path();
    // Create the GTK app
    let app = Application::builder()
        .application_id("com.github.Ph1lll.Gswww")
        .flags(ApplicationFlags::HANDLES_OPEN)
        .build();

    app.connect_activate(|app| build_ui(app, None));
    app.connect_open(|app, files, _hint| {
        // Get the first file/folder path (you might want to handle multiple)
        if let Some(file) = files.first() {
            if let Some(path) = file.path() {
                if path.is_dir() {
                    build_ui(app, Some(path.into_os_string()));
                }
            }
        }
    });
    app.run()
}

fn build_ui(app: &Application, folder_path: Option<OsString>) {
    let content = Box::new(Orientation::Vertical, 0);

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
        .halign(Align::Start)
        .build();

    // Check for if we just do a surface level skim or go deeper
    let r_check = Switch::builder().active(true).build();
    let r_label = Label::new(Some("Recursive"));
    let recursive_check = Box::builder()
        .halign(Align::Start)
        .spacing(6)
        .orientation(Orientation::Horizontal)
        .build();
    recursive_check.append(&r_check);
    recursive_check.append(&r_label);

    // Dropdown for transition types
    let transition_label = Label::new(Some("Transition:"));
    let transition_types = DropDown::builder()
        .model(&StringList::new(&TRANSISTION_OPTIONS))
        .build();
    let transition_box = Box::builder()
        .halign(Align::End)
        .spacing(6)
        .orientation(Orientation::Horizontal)
        .build();
    transition_box.append(&transition_label);
    transition_box.append(&transition_types);

    // Box for all options and buttons
    let option_box = Box::builder()
        .margin_start(12)
        .margin_end(12)
        .margin_top(12)
        .margin_bottom(12)
        .hexpand_set(true)
        .spacing(12)
        .orientation(Orientation::Horizontal)
        .build();
    let option_grow = Box::builder().halign(Align::Start).hexpand(true).build();

    // Left Side
    option_box.append(&dialog_button);
    option_box.append(&recursive_check);

    // Right Side
    option_box.append(&option_grow);
    option_box.append(&transition_box);

    // Add the main boxes of content
    content.append(&HeaderBar::new());
    content.append(&gallery);
    content.append(&option_box);

    println!("{:-<100}", ""); // Here because I hate gtk's warning messages about theming
                              // Open the file dialog
    dialog_button.connect_clicked(glib::clone!(
        #[weak]
        window,
        #[weak]
        r_check,
        #[weak]
        transition_types,
        #[weak]
        image_grid,
        move |_| {
            let dialog = FileDialog::builder()
                .title("Select Folder")
                .accept_label("Select")
                .build();

            dialog.select_folder(Some(&window), gtk::gio::Cancellable::NONE, move |folder| {
                if let Ok(folder) = folder {
                    let folder = folder.path().unwrap().into_os_string();
                    add_to_gallery(folder, &r_check.is_active(), &transition_types, &image_grid);
                };
            });
        }
    ));

    // If we have someone putting folders as arguments when we can just launch with the images already
    if let Some(folder) = folder_path {
        add_to_gallery(folder, &r_check.is_active(), &transition_types, &image_grid);
    }

    window.present();
}

fn add_to_gallery(
    folder: OsString,
    recursively_search: &bool,
    transition_types: &DropDown,
    image_grid: &FlowBox,
) {
    let time_taken_dir = std::time::Instant::now();
    println!("Opening folder: {}", &folder.to_str().unwrap());
    println!("{:-<100}", "");
    let thumbnails = utils::get_thumbnails(folder, recursively_search);

    for entry in thumbnails {
        let image = Image::from_file(&entry.thumbnail_path);
        image.set_size_request(200, 200); // Set image size for gallery

        // Create gesture for click event
        let gesture = gtk::GestureClick::new();
        gesture.set_button(gtk::gdk::ffi::GDK_BUTTON_PRIMARY as u32);
        gesture.connect_pressed(glib::clone!(
            #[strong]
            transition_types,
            move |_, _, _, _| {
                swww(entry.image_path.clone(), &transition_types);
            }
        ));

        // Add gesture and insert image in UI
        image.add_controller(gesture);
        image_grid.append(&image);
    }
    println!("{:-<100}", "");
    println!("Took {} ms for dir", time_taken_dir.elapsed().as_millis());
    println!("{:-<100}", "");
}

// Send command to swww
fn swww(file: String, transition: &DropDown) {
    println!("Selected: {file}");
    println!("{:-<100}", "");
    let _ = std::process::Command::new("swww")
        .args([
            "img",
            "-t",
            TRANSISTION_OPTIONS[transition.selected() as usize],
            file.as_str(),
        ])
        .spawn()
        .expect("Failed to change background")
        .wait();
}
