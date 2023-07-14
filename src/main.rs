use gtk::{glib, prelude::*, Application, ApplicationWindow};
use gtk::{
    Align, Box, Button, DropDown, FileDialog, FlowBox, Image, Orientation, ScrolledWindow,
    StringList,
};
use std::rc::Rc;

mod config;
mod utils;

// Options for the dropdown
const TRANSISTION_OPTIONS: [&str; 12] = [
    "simple", "left", "right", "top", "bottom", "wipe", "wave", "grow", "center", "any", "outer",
    "random",
];

fn main() -> glib::ExitCode {
    // Experimental folder storage
    // config::config();

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

    let window = Rc::new(
        ApplicationWindow::builder()
            .application(app)
            .title("Gswww")
            .default_width(900)
            .default_height(600)
            .child(&content)
            .build(),
    );

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

    dialog_button.connect_clicked(glib::clone!(@strong window => move |_| {
        let folder_location = glib::MainContext::default().spawn_local(file_dialog(Rc::clone(&window)));
        glib::MainContext::default().spawn_local(glib::clone!(@weak transition_types, @weak image_grid => async move {
            let folder_location = match folder_location.await {
                Ok(file) => file.unwrap().path().unwrap().to_str().map(|s| s.to_string()),
                Err(_) => None,
            };

        match utils::search_folder(&folder_location.unwrap()) {
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
                                utils::swww(
                                    entry.to_str().unwrap(),    // Path to file
                                    &transition_types,          // Dropdown selection
                                    &TRANSISTION_OPTIONS        // List of options
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

        }));
    }));

    window.present();
}

async fn file_dialog<W: IsA<gtk::Window>>(
    window: Rc<W>,
) -> Result<gtk::gio::File, gtk::glib::Error> {
    let folder_dialog = FileDialog::new();
    folder_dialog.select_folder_future(Some(&*window)).await
}
