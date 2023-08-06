use gtk::{
    gio::File,
    glib::{self, clone, Error, ExitCode, MainContext},
    prelude::*,
    Align, Application, ApplicationWindow, Box, Button, DropDown, FileDialog, FlowBox, HeaderBar,
    Orientation, ScrolledWindow, StringList, Window,
};
use std::rc::Rc;

mod config;
mod utils;

// Options for the dropdown
const TRANSISTION_OPTIONS: [&str; 12] = [
    "simple", "left", "right", "top", "bottom", "wipe", "wave", "grow", "center", "any", "outer",
    "random",
];

fn main() -> ExitCode {
    // Create config directory if not added
    config::config_path();
    // Create the GTK app
    let app = Application::builder()
        .application_id("com.github.Ph1lll.Gswww")
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let content = Box::new(Orientation::Vertical, 0);

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
    content.append(&HeaderBar::new());
    content.append(&gallery);
    content.append(&option_box);

    // Open the file dialog
    dialog_button.connect_clicked(clone!(@strong window => move |_| {
        let folder_location = MainContext::default().spawn_local(file_dialog(Rc::clone(&window)));
        MainContext::default().spawn_local(clone!(@weak transition_types, @weak image_grid => async move {
            // I want the location of the folder
            let folder_location = match folder_location.await {
                Ok(file) => file.unwrap().path().unwrap().to_str().map(|s| s.to_string()),
                Err(_) => None,
            };
            // Add images to gallery
            utils::add_images(&folder_location.unwrap(), &transition_types, &image_grid, &TRANSISTION_OPTIONS);
        }));
    }));

    window.present();
}

// Dialog to get folder location
async fn file_dialog<W: IsA<Window>>(window: Rc<W>) -> Result<File, Error> {
    FileDialog::new().select_folder_future(Some(&*window)).await
}
